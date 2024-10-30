pub struct StringContext {
    pub include_verse_numbers: bool,
    pub include_new_lines: bool,
    pub include_footnotes: bool,
    pub include_cross_references: bool,
}

pub const DEFAULT_CONTEXT: StringContext = StringContext {
    include_new_lines: true,
    include_footnotes: false,
    include_cross_references: false,
    include_verse_numbers: false,
};

impl StringContext {
    pub fn new(
        include_verse_numbers: bool,
        include_new_lines: bool,
        include_footnotes: bool,
        include_cross_references: bool,
    ) -> Self {
        Self {
            include_verse_numbers,
            include_new_lines,
            include_footnotes,
            include_cross_references,
        }
    }
}
