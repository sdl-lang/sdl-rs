pub struct FormatterConfig {
    /// <hr> -> <hr/>
    fix_tag_unclosed: bool,
    /// <input unchecked/> -> <input unchecked="unchecked"/>
    fix_attribute_unpaired: bool,
    /// <input :_a.-1/> -> <input __a__1/>
    fix_attribute_bad_name: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self { fix_attribute_bad_name: true, fix_tag_unclosed: true, fix_attribute_unpaired: false }
    }
}
