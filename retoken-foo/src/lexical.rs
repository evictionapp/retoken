use retoken::{relex, Span, Token};

relex! {
    pub alphabet Alphabet
    pub grammar Grammar

    pub skip Whitespace r"\s+"

    pub char OpenAngle '<'
    pub char CloseAngle '>'
    pub char OpenBracket '{'
    pub char CloseBracket '}'
    pub char Slash '/'
    pub char Equal '='

    pub borrowed Quoted

    pub re Ident "[a-zA-Z_][a-zA-Z_0-9]*"
}

#[derive(Debug, Clone)]
pub struct Quoted<'a> {
    content: &'a str,
    span: Span,
}

impl<'a> Token<'a, Alphabet> for Quoted<'a> {
    fn content(&'a self) -> &'a str {
        self.content
    }

    fn span(&'a self) -> Span {
        self.span.clone()
    }

    fn peek(tokenizer: &'a retoken::Tokenizer) -> bool {
        tokenizer.peek_char('"')
    }

    fn token(tokenizer: &'a retoken::Tokenizer) -> retoken::Result<Self, Alphabet> {
        if !tokenizer.peek_char('"') {
            return Err(retoken::Error::expected_token(
                tokenizer.cursor(),
                Alphabet::Quoted,
            ));
        }

        let (slice, start) = tokenizer.slice_with_cursor()?;

        let mut len = 0;
        let mut prev_char = '\\';
        for ch in slice.chars() {
            len += ch.len_utf8();
            if ch == '"' && prev_char != '\\' && len > 0 {
                break;
            }
            prev_char = ch;
        }

        let span = Span {
            start,
            end: start + len,
        };
        let content = tokenizer.slice_with_span(span.clone())?;
        if !content.ends_with('"') {
            return Err(retoken::Error::expected_token(
                tokenizer.cursor(),
                Alphabet::Quoted,
            ));
        }

        tokenizer.advance(len);

        Ok(Self { content, span })
    }
}
