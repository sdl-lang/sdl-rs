pub struct FormatterConfig {
    /// <hr> -> <hr/>
    fix_unclosed_void_tag: bool,
    /// <input unchecked/> -> <input unchecked="unchecked"/>
    fix_unpaired_attribute: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self { fix_unclosed_void_tag: true, fix_unpaired_attribute: false }
    }
}
