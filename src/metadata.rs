use std::io::Read;

use tracing::instrument;

use crate::compressor;
use crate::error::SquashfsError;
use crate::squashfs::SuperBlock;

const METDATA_UNCOMPRESSED: u16 = 1 << 15;

#[instrument(skip_all)]
pub fn read_block<R: Read>(
    reader: &mut R,
    superblock: &SuperBlock,
) -> Result<Vec<u8>, SquashfsError> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    let metadata_len = u16::from_le_bytes(buf);

    tracing::trace!("read: 0x{:02x?}", metadata_len);
    let byte_len = len(metadata_len);
    let mut buf = vec![0u8; byte_len as usize];
    reader.read_exact(&mut buf)?;

    let bytes = if is_compressed(metadata_len) {
        tracing::trace!("compressed");
        compressor::decompress(buf, superblock.compressor)?
    } else {
        buf
    };

    Ok(bytes)
}

/// Check is_compressed bit within raw `len`
pub fn is_compressed(len: u16) -> bool {
    len & METDATA_UNCOMPRESSED == 0
}

/// Get actual length of `data` following `len` from unedited `len`
pub fn len(len: u16) -> u16 {
    len & !(METDATA_UNCOMPRESSED)
}

pub fn set_if_compressed(len: u16) -> u16 {
    len | METDATA_UNCOMPRESSED
}
