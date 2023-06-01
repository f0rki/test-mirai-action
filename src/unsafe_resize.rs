pub fn copy_and_fill(into: &mut Vec<u8>, first: &[u8], with_len: usize, with: u8) {
    let total_len = first.len() + with_len;
    // bug because potentially total_len < first.len()
    into.reserve(total_len);

    unsafe {
        use core::ptr;
        ptr::copy(first.as_ptr(), into.as_mut_ptr(), first.len());
        let mut i = first.len();
        while i < total_len {
            *into.get_unchecked_mut(i) = with;
            i += 1;
        }

        into.set_len(total_len);
    }
}

pub fn do_the_resize() -> anyhow::Result<()> {
    let with_len = std::env::var("WITH_LEN")?.parse::<usize>()?;

    let mut v = vec![];
    let mut first = Vec::new();
    first.resize(64, 'B' as u8);

    copy_and_fill(&mut v, &first, with_len, 0x41);

    println!("{:?}", v);
    Ok(())
}
