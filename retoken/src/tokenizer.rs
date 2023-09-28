use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use lazy_regex::{Lazy, Regex};

use crate::{Skip, SliceError, Span, Token};

pub struct Tokenizer {
    content: Arc<str>,
    cursor: AtomicUsize,
}

impl Tokenizer {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.into(),
            cursor: AtomicUsize::new(0),
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor.load(Ordering::SeqCst)
    }

    pub fn slice_with_cursor(&self) -> Result<(&str, usize), SliceError> {
        let start = self.cursor();
        let slice = match self.content.get(start..) {
            Some(slice) => slice,
            None => return Err(SliceError(start)),
        };
        Ok((slice, start))
    }

    pub fn slice_with_span(&self, span: Span) -> Result<&str, SliceError> {
        let slice = match self.content.get(span.start..span.end) {
            Some(slice) => slice,
            None => return Err(SliceError(span.start)),
        };
        Ok(slice)
    }

    pub fn advance(&self, len: usize) -> usize {
        self.cursor.fetch_add(len, Ordering::SeqCst)
    }

    pub fn peek_char(&self, ch: char) -> bool {
        let slice = match self.slice_with_cursor() {
            Ok(ok) => ok.0,
            _ => return false,
        };

        match slice.chars().next() {
            Some(first_char) => first_char == ch,
            None => false,
        }
    }

    pub fn peek_re(&self, re: &Lazy<Regex>) -> bool {
        let slice = match self.slice_with_cursor() {
            Ok(ok) => ok.0,
            _ => return false,
        };
        re.is_match(slice)
    }

    pub fn token_char(&self, ch: char) -> Result<Option<Span>, SliceError> {
        let (slice, start) = self.slice_with_cursor()?;

        let ok = match slice.chars().next() {
            Some(first_char) => {
                if first_char != ch {
                    return Ok(None);
                }

                let len = ch.len_utf8();
                let end = start + len;
                self.advance(len);

                Some(Span { start, end })
            }
            None => None,
        };

        Ok(ok)
    }

    pub fn token_re(&self, re: &Lazy<Regex>) -> Result<Option<(&str, Span)>, SliceError> {
        let (slice, start) = self.slice_with_cursor()?;

        let m = match re.find(slice) {
            Some(m) => m,
            None => return Ok(None),
        };
        let m_str = m.as_str();
        let len = m_str.len();
        let end = start + len;
        self.advance(len);

        Ok(Some((m_str, Span { start, end })))
    }

    pub fn skip<T: Skip>(&self) -> bool {
        T::skip(self)
    }

    pub fn peek<'a, Alphabet, T: Token<'a, Alphabet>>(&'a self) -> bool {
        T::peek(self)
    }

    pub fn token<'a, Alphabet, T: Token<'a, Alphabet>>(&'a self) -> crate::Result<T, Alphabet> {
        T::token(self)
    }
}
