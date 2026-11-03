use arrays::buffer::Buffer;

pub fn collected() -> Vec<u8> {
    Buffer { storage: [0_u8; 1] }.into_iter().collect()
}
