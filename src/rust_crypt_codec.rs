use crypto::{
    aes::KeySize,
    blockmodes::PkcsPadding,
    buffer::{self, BufferResult, ReadBuffer, WriteBuffer},
    symmetriccipher::{Decryptor, Encryptor},
};

use crate::{
    codec::Codec,
    codec_util::{parse_bytes_2_hex_str, parse_hex_str_2_bytes},
};

pub struct AesCodec {
    encryptor: Box<dyn Encryptor>,
    decryptor: Box<dyn Decryptor>,
}

impl AesCodec {
    pub fn new(key: String) -> Self {
        AesCodec {
            encryptor: crypto::aes::ecb_encryptor(KeySize::KeySize128, key.as_bytes(), PkcsPadding),
            decryptor: crypto::aes::ecb_decryptor(KeySize::KeySize128, key.as_bytes(), PkcsPadding),
        }
    }
}

impl Codec for AesCodec {
    fn encode<'a, 'b: 'a>(&'a mut self, data: impl Into<&'b str>) -> anyhow::Result<String> {
        let data: &'b str = data.into();
        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(data.as_bytes());
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = self
                .encryptor
                .encrypt(&mut read_buffer, &mut write_buffer, true);

            let result = result.unwrap();

            final_result.extend(
                write_buffer
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|&i| i),
            );

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        Ok(parse_bytes_2_hex_str(final_result))
    }

    fn decode<'a, 'b: 'a>(&'a mut self, data: impl Into<&'b str>) -> anyhow::Result<String> {
        let mut final_result = Vec::<u8>::new();
        let v = parse_hex_str_2_bytes(data);

        let mut read_buffer = buffer::RefReadBuffer::new(&v);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = self
                .decryptor
                .decrypt(&mut read_buffer, &mut write_buffer, true);

            let result = result.unwrap();

            final_result.extend(
                write_buffer
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|&i| i),
            );
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        Ok(String::from_utf8(final_result).unwrap())
    }
}
