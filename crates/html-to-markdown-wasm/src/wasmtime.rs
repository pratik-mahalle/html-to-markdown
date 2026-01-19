#[cfg(feature = "wasmtime-testing")]
mod runtime {
    use core::{mem, slice, str};
    use html_to_markdown_rs::{ConversionOptions, HeadingStyle};
    use std::cell::RefCell;

    thread_local! {
        static RESULT_BUFFER: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }

    fn write_result(bytes: &[u8]) -> u32 {
        RESULT_BUFFER.with(|buf| {
            let mut buffer = buf.borrow_mut();
            buffer.clear();
            buffer.extend_from_slice(bytes);
            buffer.len() as u32
        })
    }

    fn read_utf8(ptr: u32, len: u32) -> String {
        let slice = unsafe { slice::from_raw_parts(ptr as *const u8, len as usize) };
        str::from_utf8(slice).expect("input must be valid UTF-8").to_owned()
    }

    fn parse_options(ptr: u32, len: u32) -> Option<ConversionOptions> {
        if len == 0 {
            return None;
        }
        let json = read_utf8(ptr, len);
        html_to_markdown_bindings_common::json::parse_conversion_options(Some(&json))
            .expect("options JSON must be valid")
    }

    fn convert_internal(html_ptr: u32, html_len: u32, options: Option<ConversionOptions>) -> u32 {
        let html = read_utf8(html_ptr, html_len);
        let result = {
            #[cfg(feature = "visitor")]
            {
                html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, options))
            }
            #[cfg(not(feature = "visitor"))]
            {
                html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, options))
            }
        };
        match result {
            Ok(markdown) => write_result(markdown.as_bytes()),
            Err(err) => write_result(format!("ERROR:{}", err).as_bytes()),
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn htmd_alloc(len: u32) -> u32 {
        let mut buffer = vec![0u8; len as usize];
        let ptr = buffer.as_mut_ptr();
        mem::forget(buffer);
        ptr as u32
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn htmd_dealloc(ptr: u32, len: u32) {
        unsafe {
            Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn htmd_result_ptr() -> u32 {
        RESULT_BUFFER.with(|buf| buf.borrow().as_ptr() as u32)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn htmd_convert(ptr: u32, len: u32) -> u32 {
        convert_internal(ptr, len, None)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn htmd_convert_with_options(
        html_ptr: u32,
        html_len: u32,
        options_ptr: u32,
        options_len: u32,
    ) -> u32 {
        let options = parse_options(options_ptr, options_len);
        convert_internal(html_ptr, html_len, options)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn htmd_convert_underlined(html_ptr: u32, html_len: u32) -> u32 {
        let options = ConversionOptions {
            heading_style: HeadingStyle::Underlined,
            wrap: true,
            wrap_width: 12,
            ..Default::default()
        };
        convert_internal(html_ptr, html_len, Some(options))
    }
}
