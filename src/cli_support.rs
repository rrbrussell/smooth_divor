/// prints the provided buffer to stdout.
pub fn print_buffer_as_hex(buffer: &[u8]) {
    for base in 0..buffer.len() / 16 {
        print!("\n{base:04X}:\t");
        for inside in 0..16 {
            if inside > 1 && inside % 2 == 0 {
                print!(" ");
            }
            print!("{:02X}", buffer[base * 16 + inside]);
        }
    }
}
