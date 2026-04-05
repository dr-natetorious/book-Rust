// tag::aliasing_xor_mutability[]
pub fn mutate_while_borrowed(buffer: &mut [u8]) {
    let stock = &buffer[15..23];
    buffer[15] = b'Z';
    println!("{:?}", stock);
}
// end::aliasing_xor_mutability[]