
pub fn print_vector(vec: &Vec<u8>) {
    for i in &vec[..3] {
        println!("{i:#010b}");
    }
}
