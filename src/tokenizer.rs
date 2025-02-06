pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    fn next_token(&mut self) -> Option<&'a str> {
        let token = self.input[self.position..].split_whitespace().next()?;
        self.position += token.len() + 1;
        Some(token)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

pub trait Tokenizable {
    fn tokens(&self) -> Tokenizer;
}

impl Tokenizable for str {
    fn tokens(&self) -> Tokenizer {
        Tokenizer::new(self)
    }
}
