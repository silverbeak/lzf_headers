mod utils;

extern crate lzf;
extern crate byteorder;

/// Takes a _compressed_ payload (with LZF header) and returns the decompressed payload
pub fn decompress_lzf(payload: &[u8]) -> Result<Vec<u8>, String> {
    match utils::headers::lzf_structure_from_compressed(payload) {
        Ok(structure) => structure.get_decompressed_payload(),
        Err(error) => Err(String::from(error))
    }
}

/// Takes an _uncompressed_ payload and returns the compressed payload (with LZF headers)
pub fn compress_lzf(payload: &[u8]) -> Result<Vec<u8>, String> {
    match utils::headers::lzf_structure_from_uncompressed(payload) {
        Ok(structure) => Ok(structure.to_compressed()),
        Err(error) => Err(String::from(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_STRING: &str = "On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain";

    #[test]
    fn test_compress_decompress() {
        
        let compressed_structure = utils::headers::lzf_structure_from_uncompressed(TEST_STRING.as_bytes()).unwrap();

        match decompress_lzf(&compressed_structure.to_compressed()) {
            Ok(decompressed) => {
                println!("Decompressed: {:?}", decompressed);
                assert_eq!(decompressed.len(), compressed_structure.header.original_len);
                assert_eq!(390, compressed_structure.header.original_len);
                assert_eq!(351, compressed_structure.header.compressed_len);
            },
            Err(error) => {
                println!("Got error: {}", error);
                assert!(false);
            }
        };
    }

    #[test]
    fn test_decompress() {
        let compressed = compress_lzf(TEST_STRING.as_bytes()).unwrap();

        match decompress_lzf(&compressed) {
            Ok(decompressed) => {
                println!("Decompressed: {:?}", decompressed);
                assert_eq!(358, compressed.len());
            },
            Err(error) => {
                println!("Got error: {}", error);
                assert!(false);
            }
        };
    }
}
