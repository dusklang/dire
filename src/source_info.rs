use std::cmp::{min, max};

#[derive(Copy, Clone, Debug)]
pub struct SourceRange {
    pub start: usize,
    pub end: usize,
}

impl Default for SourceRange {
    fn default() -> Self {
        SourceRange {
            start: usize::MAX,
            end: usize::MAX,
        }
    }
}

impl SourceRange {
    pub fn from_single_char(index: usize) -> SourceRange {
        SourceRange {
            start: index,
            end: index+1
        }
    }
}

pub fn concat(a: SourceRange, b: SourceRange) -> SourceRange {
    SourceRange {
        start: min(a.start, b.start),
        end:   max(a.end, b.end),
    }
}