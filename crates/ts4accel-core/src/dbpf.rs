use anyhow::{bail, Context, Result};
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use memmap2::Mmap;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Seek, Write};
use std::path::{Path, PathBuf};

pub const DBPF_HEADER_SIZE: usize = 96;
pub const DBPF_INDEX_VERSION: u32 = 3;
pub const ENTRY_EXTENDED_BIT: u32 = 0x8000_0000;
pub const ENTRY_SIZE_NO_CONSTANTS: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ResourceKey {
    pub type_id: u32,
    pub group_id: u32,
    pub instance_id: u64,
}

impl ResourceKey {
    pub fn display_hex(&self) -> String {
        format!("{:08X}-{:08X}-{:016X}", self.type_id, self.group_id, self.instance_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbpfHeader {
    pub major: u32,
    pub minor: u32,
    pub index_count: u32,
    pub index_offset: u64,
    pub index_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceEntry {
    pub key: ResourceKey,
    pub source_offset: u32,
    pub stored_size: u32,
    pub uncompressed_size: u32,
    pub compression_type: u16,
    pub committed: u16,
    pub source_entry_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbpfPackage {
    pub path: PathBuf,
    pub header: DbpfHeader,
    pub index_flags: u32,
    pub entries: Vec<ResourceEntry>,
    pub index_hash: String,
    pub duplicate_keys_inside_package: bool,
}

#[derive(Debug, Clone)]
pub struct ShardInput<'a> {
    pub package_path: &'a Path,
    pub entry: &'a ResourceEntry,
}

impl DbpfPackage {
    pub fn parse(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path).with_context(|| format!("open DBPF read-only: {}", path.display()))?;
        let mmap = unsafe { Mmap::map(&file) }.with_context(|| format!("map DBPF: {}", path.display()))?;
        Self::parse_bytes(path.to_path_buf(), &mmap)
    }

    pub fn parse_bytes(path: PathBuf, bytes: &[u8]) -> Result<Self> {
        if bytes.len() < DBPF_HEADER_SIZE {
            bail!("{} is too small to be DBPF", path.display());
        }
        if &bytes[0..4] != b"DBPF" {
            bail!("{} does not start with DBPF", path.display());
        }

        let major = le_u32(bytes, 4)?;
        let minor = le_u32(bytes, 8)?;
        if major != 2 || minor != 1 {
            bail!("{} has unsupported DBPF version {}.{}", path.display(), major, minor);
        }

        let index_count = le_u32(bytes, 36)?;
        let short_offset = le_u32(bytes, 40)?;
        let index_size = le_u32(bytes, 44)?;
        let index_version = le_u32(bytes, 60)?;
        let long_offset = le_u64(bytes, 64)?;
        if index_version != DBPF_INDEX_VERSION {
            bail!("{} has unsupported index version {}", path.display(), index_version);
        }
        let index_offset = if short_offset == 0 { long_offset } else { short_offset as u64 };
        let end = index_offset
            .checked_add(index_size as u64)
            .context("DBPF index range overflow")?;
        if end > bytes.len() as u64 {
            bail!("{} DBPF index is outside file bounds", path.display());
        }

        if index_offset < DBPF_HEADER_SIZE as u64 {
            bail!("{} DBPF index overlaps the header", path.display());
        }

        let index = &bytes[index_offset as usize..end as usize];
        if index.len() < 4 {
            bail!("{} DBPF index is truncated", path.display());
        }
        let index_flags = LittleEndian::read_u32(&index[0..4]);
        if index_flags & !0x7 != 0 {
            bail!("{} has unsupported DBPF index flags 0x{:08X}", path.display(), index_flags);
        }

        let constant_count = (index_flags & 0x7).count_ones() as usize;
        let minimum_entry_size = 28usize
            .checked_sub(constant_count * 4)
            .context("invalid DBPF index constant flags")?;
        let minimum_index_size = 4usize
            .checked_add(constant_count * 4)
            .and_then(|value| value.checked_add((index_count as usize).checked_mul(minimum_entry_size)?))
            .context("DBPF index size overflow")?;
        if minimum_index_size > index.len() {
            bail!("{} DBPF index cannot contain {} declared entries", path.display(), index_count);
        }

        let mut cursor = 4usize;
        let constant_type = if index_flags & 0x1 != 0 { Some(read_index_u32(index, &mut cursor)?) } else { None };
        let constant_group = if index_flags & 0x2 != 0 { Some(read_index_u32(index, &mut cursor)?) } else { None };
        let constant_instance_hi = if index_flags & 0x4 != 0 { Some(read_index_u32(index, &mut cursor)?) } else { None };

        let mut entries = Vec::with_capacity(index_count as usize);
        let mut seen = HashSet::with_capacity(index_count as usize);
        let mut duplicate_keys_inside_package = false;

        for source_entry_index in 0..index_count {
            let type_id = match constant_type { Some(v) => v, None => read_index_u32(index, &mut cursor)? };
            let group_id = match constant_group { Some(v) => v, None => read_index_u32(index, &mut cursor)? };
            let instance_hi = match constant_instance_hi { Some(v) => v, None => read_index_u32(index, &mut cursor)? };
            let instance_lo = read_index_u32(index, &mut cursor)?;
            let source_offset = read_index_u32(index, &mut cursor)?;
            let compressed_field = read_index_u32(index, &mut cursor)?;
            let extended = compressed_field & ENTRY_EXTENDED_BIT != 0;
            let stored_size = compressed_field & !ENTRY_EXTENDED_BIT;
            let uncompressed_size = read_index_u32(index, &mut cursor)?;
            let (compression_type, committed) = if extended {
                (read_index_u16(index, &mut cursor)?, read_index_u16(index, &mut cursor)?)
            } else {
                (0u16, 1u16)
            };

            let data_end = (source_offset as u64)
                .checked_add(stored_size as u64)
                .context("resource range overflow")?;
            if data_end > bytes.len() as u64 {
                bail!(
                    "{} resource {} points outside file (offset {}, size {})",
                    path.display(), source_entry_index, source_offset, stored_size
                );
            }

            let key = ResourceKey {
                type_id,
                group_id,
                instance_id: ((instance_hi as u64) << 32) | instance_lo as u64,
            };
            if !seen.insert(key) {
                duplicate_keys_inside_package = true;
            }
            entries.push(ResourceEntry {
                key,
                source_offset,
                stored_size,
                uncompressed_size,
                compression_type,
                committed,
                source_entry_index,
            });
        }

        if cursor > index.len() {
            bail!("{} DBPF index parse exceeded index bounds", path.display());
        }

        let index_hash = blake3::hash(index).to_hex().to_string();
        Ok(Self {
            path,
            header: DbpfHeader { major, minor, index_count, index_offset, index_size },
            index_flags,
            entries,
            index_hash,
            duplicate_keys_inside_package,
        })
    }
}

pub fn write_front_indexed_shard(output: &Path, resources: &[ShardInput<'_>]) -> Result<u64> {
    let output_parent = output.parent().context("shard output has no parent")?;
    std::fs::create_dir_all(output_parent)?;

    let index_size = 4usize
        .checked_add(resources.len().checked_mul(ENTRY_SIZE_NO_CONSTANTS).context("index size overflow")?)
        .context("index size overflow")?;
    let index_offset = DBPF_HEADER_SIZE as u64;
    let payload_start = align_up(index_offset + index_size as u64, 16);

    let mut resource_offsets = Vec::with_capacity(resources.len());
    let mut next_offset = payload_start;
    for item in resources {
        if item.entry.stored_size & ENTRY_EXTENDED_BIT != 0 {
            bail!("resource stored size exceeds DBPF 31-bit field");
        }
        if next_offset > u32::MAX as u64 {
            bail!("shard resource offset exceeds DBPF 32-bit limit");
        }
        resource_offsets.push(next_offset as u32);
        next_offset = next_offset
            .checked_add(item.entry.stored_size as u64)
            .context("shard size overflow")?;
        next_offset = align_up(next_offset, 4);
    }
    if next_offset > u32::MAX as u64 {
        bail!("shard exceeds DBPF 4 GiB address limit");
    }

    let temp = output.with_extension("package.tmp");
    let file = OpenOptions::new().create(true).truncate(true).write(true).open(&temp)
        .with_context(|| format!("create shard temp: {}", temp.display()))?;
    let mut writer = BufWriter::with_capacity(4 * 1024 * 1024, file);

    let resource_count = u32::try_from(resources.len()).context("too many resources for DBPF index")?;
    let index_size_u32 = u32::try_from(index_size).context("DBPF index exceeds 32-bit size")?;
    write_header(&mut writer, resource_count, index_offset, index_size_u32)?;
    writer.write_u32::<byteorder::LittleEndian>(0)?;
    for (item, offset) in resources.iter().zip(resource_offsets.iter().copied()) {
        let key = item.entry.key;
        writer.write_u32::<byteorder::LittleEndian>(key.type_id)?;
        writer.write_u32::<byteorder::LittleEndian>(key.group_id)?;
        writer.write_u32::<byteorder::LittleEndian>((key.instance_id >> 32) as u32)?;
        writer.write_u32::<byteorder::LittleEndian>(key.instance_id as u32)?;
        writer.write_u32::<byteorder::LittleEndian>(offset)?;
        writer.write_u32::<byteorder::LittleEndian>(item.entry.stored_size | ENTRY_EXTENDED_BIT)?;
        writer.write_u32::<byteorder::LittleEndian>(item.entry.uncompressed_size)?;
        writer.write_u16::<byteorder::LittleEndian>(item.entry.compression_type)?;
        writer.write_u16::<byteorder::LittleEndian>(item.entry.committed)?;
    }

    let current = writer.stream_position()?;
    if current > payload_start {
        bail!("front index exceeded computed payload start");
    }
    write_zeros(&mut writer, (payload_start - current) as usize)?;

    let mut mapped_path: Option<PathBuf> = None;
    let mut mapped_source: Option<Mmap> = None;
    for (item, expected_offset) in resources.iter().zip(resource_offsets.iter().copied()) {
        let position = writer.stream_position()?;
        if position != expected_offset as u64 {
            bail!("internal shard offset mismatch: {} != {}", position, expected_offset);
        }
        if mapped_path.as_deref() != Some(item.package_path) {
            let file = File::open(item.package_path)
                .with_context(|| format!("open source package read-only: {}", item.package_path.display()))?;
            mapped_source = Some(unsafe { Mmap::map(&file) }
                .with_context(|| format!("map source package: {}", item.package_path.display()))?);
            mapped_path = Some(item.package_path.to_path_buf());
        }
        let source = mapped_source.as_ref().context("source map missing")?;
        let start = item.entry.source_offset as usize;
        let end = start.checked_add(item.entry.stored_size as usize).context("source range overflow")?;
        if end > source.len() {
            bail!("source range outside package: {}", item.package_path.display());
        }
        writer.write_all(&source[start..end])?;
        let aligned = align_up(writer.stream_position()?, 4);
        let current = writer.stream_position()?;
        write_zeros(&mut writer, (aligned - current) as usize)?;
    }
    writer.flush()?;
    writer.get_ref().sync_all()?;
    drop(writer);
    std::fs::rename(&temp, output)
        .with_context(|| format!("atomically install shard {}", output.display()))?;
    Ok(next_offset)
}

fn write_header<W: Write>(writer: &mut W, count: u32, index_offset: u64, index_size: u32) -> Result<()> {
    writer.write_all(b"DBPF")?;
    writer.write_u32::<byteorder::LittleEndian>(2)?;
    writer.write_u32::<byteorder::LittleEndian>(1)?;
    writer.write_u32::<byteorder::LittleEndian>(0)?;
    writer.write_u32::<byteorder::LittleEndian>(0)?;
    writer.write_u32::<byteorder::LittleEndian>(0)?;
    writer.write_u32::<byteorder::LittleEndian>(0)?;
    writer.write_u32::<byteorder::LittleEndian>(0)?;
    writer.write_u32::<byteorder::LittleEndian>(0)?;
    writer.write_u32::<byteorder::LittleEndian>(count)?;
    writer.write_u32::<byteorder::LittleEndian>(0)?;
    writer.write_u32::<byteorder::LittleEndian>(index_size)?;
    writer.write_all(&[0u8; 12])?;
    writer.write_u32::<byteorder::LittleEndian>(DBPF_INDEX_VERSION)?;
    writer.write_u64::<byteorder::LittleEndian>(index_offset)?;
    writer.write_all(&[0u8; 24])?;
    Ok(())
}

fn read_index_u32(index: &[u8], cursor: &mut usize) -> Result<u32> {
    let end = cursor.checked_add(4).context("index cursor overflow")?;
    if end > index.len() { bail!("truncated DBPF index"); }
    let value = LittleEndian::read_u32(&index[*cursor..end]);
    *cursor = end;
    Ok(value)
}

fn read_index_u16(index: &[u8], cursor: &mut usize) -> Result<u16> {
    let end = cursor.checked_add(2).context("index cursor overflow")?;
    if end > index.len() { bail!("truncated DBPF index"); }
    let value = LittleEndian::read_u16(&index[*cursor..end]);
    *cursor = end;
    Ok(value)
}

fn le_u32(bytes: &[u8], offset: usize) -> Result<u32> {
    let end = offset.checked_add(4).context("offset overflow")?;
    if end > bytes.len() { bail!("truncated DBPF header"); }
    Ok(LittleEndian::read_u32(&bytes[offset..end]))
}

fn le_u64(bytes: &[u8], offset: usize) -> Result<u64> {
    let end = offset.checked_add(8).context("offset overflow")?;
    if end > bytes.len() { bail!("truncated DBPF header"); }
    Ok(LittleEndian::read_u64(&bytes[offset..end]))
}

fn align_up(value: u64, alignment: u64) -> u64 {
    (value + alignment - 1) & !(alignment - 1)
}

fn write_zeros<W: Write>(writer: &mut W, count: usize) -> Result<()> {
    const ZEROES: [u8; 4096] = [0; 4096];
    let mut remaining = count;
    while remaining > 0 {
        let amount = remaining.min(ZEROES.len());
        writer.write_all(&ZEROES[..amount])?;
        remaining -= amount;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write_source(path: &Path, key: ResourceKey, payload: &[u8]) -> Result<()> {
        let entry = ResourceEntry {
            key,
            source_offset: 0,
            stored_size: payload.len() as u32,
            uncompressed_size: payload.len() as u32,
            compression_type: 0,
            committed: 1,
            source_entry_index: 0,
        };
        std::fs::write(path, payload)?;
        let out = path.with_extension("package");
        write_front_indexed_shard(&out, &[ShardInput { package_path: path, entry: &entry }])?;
        let parsed = DbpfPackage::parse(&out)?;
        assert_eq!(parsed.entries.len(), 1);
        assert_eq!(parsed.entries[0].key, key);
        Ok(())
    }

    fn fixture_with_index_flags(flags: u32) -> Result<Vec<u8>> {
        let constants = (flags & 0x7).count_ones() as usize;
        let entry_size = ENTRY_SIZE_NO_CONSTANTS - constants * 4;
        let index_size = 4 + constants * 4 + entry_size;
        let payload_offset = align_up((DBPF_HEADER_SIZE + index_size) as u64, 16) as u32;
        let payload = b"fixture-payload";

        let mut bytes = Vec::new();
        write_header(&mut bytes, 1, DBPF_HEADER_SIZE as u64, index_size as u32)?;
        bytes.write_u32::<byteorder::LittleEndian>(flags)?;
        if flags & 0x1 != 0 { bytes.write_u32::<byteorder::LittleEndian>(0x1111_1111)?; }
        if flags & 0x2 != 0 { bytes.write_u32::<byteorder::LittleEndian>(0x2222_2222)?; }
        if flags & 0x4 != 0 { bytes.write_u32::<byteorder::LittleEndian>(0x3333_3333)?; }
        if flags & 0x1 == 0 { bytes.write_u32::<byteorder::LittleEndian>(0x1111_1111)?; }
        if flags & 0x2 == 0 { bytes.write_u32::<byteorder::LittleEndian>(0x2222_2222)?; }
        if flags & 0x4 == 0 { bytes.write_u32::<byteorder::LittleEndian>(0x3333_3333)?; }
        bytes.write_u32::<byteorder::LittleEndian>(0x4444_4444)?;
        bytes.write_u32::<byteorder::LittleEndian>(payload_offset)?;
        bytes.write_u32::<byteorder::LittleEndian>(payload.len() as u32 | ENTRY_EXTENDED_BIT)?;
        bytes.write_u32::<byteorder::LittleEndian>(payload.len() as u32)?;
        bytes.write_u16::<byteorder::LittleEndian>(0)?;
        bytes.write_u16::<byteorder::LittleEndian>(7)?;
        bytes.resize(payload_offset as usize, 0);
        bytes.extend_from_slice(payload);
        Ok(bytes)
    }

    #[test]
    fn writes_and_reads_front_indexed_package() -> Result<()> {
        let temp = tempdir()?;
        write_source(
            &temp.path().join("payload.bin"),
            ResourceKey { type_id: 1, group_id: 2, instance_id: 3 },
            b"hello-ts4",
        )
    }

    #[test]
    fn parses_every_dbpf_index_constant_flag_combination() -> Result<()> {
        for flags in 0..=7 {
            let bytes = fixture_with_index_flags(flags)?;
            let parsed = DbpfPackage::parse_bytes(
                PathBuf::from(format!("flags-{flags}.package")),
                &bytes,
            )?;
            assert_eq!(parsed.index_flags, flags);
            assert_eq!(parsed.entries.len(), 1);
            assert_eq!(parsed.entries[0].key.type_id, 0x1111_1111);
            assert_eq!(parsed.entries[0].key.group_id, 0x2222_2222);
            assert_eq!(parsed.entries[0].key.instance_id, 0x3333_3333_4444_4444);
            assert_eq!(parsed.entries[0].committed, 7);
        }
        Ok(())
    }

    #[test]
    fn rejects_impossible_declared_index_count() -> Result<()> {
        let mut bytes = fixture_with_index_flags(0)?;
        LittleEndian::write_u32(&mut bytes[36..40], u32::MAX);
        assert!(DbpfPackage::parse_bytes(PathBuf::from("bad.package"), &bytes).is_err());
        Ok(())
    }

    #[test]
    fn writer_preserves_committed_field() -> Result<()> {
        let temp = tempdir()?;
        let payload_path = temp.path().join("payload.bin");
        std::fs::write(&payload_path, b"data")?;
        let entry = ResourceEntry {
            key: ResourceKey { type_id: 5, group_id: 6, instance_id: 7 },
            source_offset: 0,
            stored_size: 4,
            uncompressed_size: 4,
            compression_type: 0,
            committed: 9,
            source_entry_index: 0,
        };
        let output = temp.path().join("committed.package");
        write_front_indexed_shard(
            &output,
            &[ShardInput { package_path: &payload_path, entry: &entry }],
        )?;
        let parsed = DbpfPackage::parse(&output)?;
        assert_eq!(parsed.entries[0].committed, 9);
        Ok(())
    }
}
