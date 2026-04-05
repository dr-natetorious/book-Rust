// This file is intentionally broken for chapter exercises.

// tag::unsafe_bounds_broken[]
pub unsafe fn write_past_end(buffer: &mut [u8]) {
    let ptr = buffer.as_mut_ptr();
    // Intentionally wrong: this writes one byte past the buffer end.
    *ptr.add(buffer.len()) = 0xFF;
}
// end::unsafe_bounds_broken[]
