pub fn as_byte_slice<'a, T>(x: &'a T) -> &'a [u8] {
    unsafe {
        ::std::slice::raw::buf_as_slice(
            x as *_ as *u8,
            ::std::mem::size_of::<T>(),
            |v| ::std::mem::transmute(v)
        )
    }
}
