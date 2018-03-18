pub mod headers {
    extern crate lzf;
    extern crate byteorder;

    use byteorder::{WriteBytesExt, BigEndian};

    #[derive(Debug)]
    pub struct LzfHeader {
        pub compress_method: String,
        pub is_compressed: bool,
        pub original_len: usize,
        pub compressed_len: usize
    }

    #[derive(Debug)]
    pub struct LzfStructure {
        pub header: LzfHeader,
        pub body: Vec<u8>
    }

    pub fn lzf_structure_from_compressed(payload: &[u8]) -> Result<LzfStructure, &str> {
        let header = header_from_compressed(&payload[..7]);
        
        let body = if header.is_compressed {
            payload[7..].to_vec()
        } else {
            payload[5..].to_vec()
        };

        Ok(LzfStructure {
            header: header,
            body: body
        })
    }

    fn convert_to_size(payload: &[u8]) -> usize {
        let s = format!("{:X}{:X}", &payload[0], &payload[1]);
        match i64::from_str_radix(&s, 16) {
            Ok(x) => x as usize,
            Err(why) => panic!("Could not parse size from {:?}, {}", &payload, why)
        }
    }

    fn header_from_compressed(payload: &[u8]) -> LzfHeader {
        let is_compressed = payload[2] as u8 != 48;

        let compressed_len = convert_to_size(&payload[3..5]);

        let original_len = if is_compressed {
            convert_to_size(&payload[5..7])
        } else {
            0 as usize
        };

        LzfHeader {
            compress_method: String::from("ZV"),
            is_compressed: is_compressed,
            original_len: original_len,
            compressed_len: compressed_len
        }
    }

    pub fn lzf_structure_from_uncompressed(uncompressed_payload: &[u8]) -> Result<LzfStructure, String> {
        let original_len = uncompressed_payload.len();

        match lzf::compress(uncompressed_payload) {
            Ok(compressed) => {
                let compressed_len = compressed.len();
                let header = LzfHeader {
                    is_compressed: true,
                    original_len: original_len,
                    compressed_len: compressed_len,
                    compress_method: String::from("ZV")
                };

                let structure = LzfStructure {
                    header: header,
                    body: compressed
                };

                Ok(structure)

            },
            Err(error) => Err(format!("{}", error))
        }
    }

    fn size_to_bytes(size: usize) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u16::<BigEndian>(size as u16).unwrap();
        wtr
    }

    impl LzfStructure {
        pub fn to_compressed(&self) -> Vec<u8> {
            let compress_method = self.header.compress_method.as_bytes().into_iter().cloned();
            let compressed_indicator = if self.header.is_compressed { vec!(0x01) } else { vec!(0x00) };
            let compressed_length = size_to_bytes(self.header.compressed_len);
            let original_length = size_to_bytes(self.header.original_len);
            
            compress_method
                .chain(compressed_indicator)
                .chain(compressed_length.into_iter())
                .chain(original_length.into_iter())
                .chain(self.body.iter().cloned())
                .collect::<Vec<_>>()
        }

        pub fn get_decompressed_payload(&self) -> Result<Vec<u8>, String> {
            match lzf::decompress(&self.body, self.header.original_len) {
                Ok(x) => Ok(x),
                Err(error) => Err(format!("{}", error))
            }
        }
    }

}
