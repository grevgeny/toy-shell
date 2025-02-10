use core::str;

pub struct Tokenizer<'a> {
    iter: std::iter::Peekable<str::Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<String> {
        while let Some(&c) = self.iter.peek() {
            if c.is_whitespace() {
                self.iter.next();
            } else {
                break;
            }
        }

        let mut token = String::new();

        while let Some(char) = self.iter.next() {
            match char {
                '\'' => {
                    let quoted_iter = self.iter.by_ref().take_while(|&char| char != '\'');
                    token.extend(quoted_iter);
                }
                char if char.is_whitespace() => break,
                char => token.push(char),
            }
        }

        if token.is_empty() {
            None
        } else {
            Some(token)
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = String;

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
