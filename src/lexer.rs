use std::process;

pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
    fn next_lexeme(&mut self) -> Option<String>;
}

pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub current_lexeme: String,
}

//Reads file name
impl Lexer {
    pub fn new(filename: &str) -> Self {
        let contents = std::fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Error: could not read file '{}'", filename));
        Lexer {
            input: contents.chars().collect(),
            position: 0,
            current_lexeme: String::new(),
        }
    }
}

impl LexicalAnalyzer for Lexer {
    //Reads one character and advances position
    fn get_char(&mut self) -> char {
        let c = self.input[self.position];
        self.position += 1;
        c
    }

    // Appends character to current token 
    fn add_char(&mut self, c: char) {
        self.current_lexeme.push(c);
    }

    // Checks whether a lexeme is a valid keyword
    fn lookup(&self, s: &str) -> bool {
        const VALID_TOKENS: [&str; 21] = [
            "#HAI", "#KTHXBYE", "#OBTW", "#TLDR", "#MAEK", "#GIMMEH", "#OIC", "#MKAY",
            "#I", "#HAZ", "#IT", "#IZ", "#LEMME", "#SEE", "#NEWLINE",
            "#SOUNDZ", "#VIDZ", "#HEAD", "#PARAGRAF", "#LIST", "#ITEM",
        ];
        VALID_TOKENS.contains(&s.to_uppercase().as_str())
    }

   fn next_lexeme(&mut self) -> Option<String> {
    self.current_lexeme.clear();

    // Skip whitespace, tabs, and (maybe) newlines
    while self.position < self.input.len() && self.input[self.position].is_whitespace() {
        self.position += 1;
    }

    // End of file
    if self.position >= self.input.len() {
        return None;
    }

    let c = self.get_char();

    if c == '#' {
        self.add_char(c);

        // Read until whitespace or EOF
        while self.position < self.input.len() && !self.input[self.position].is_whitespace() {
            let nc = self.get_char();
            self.add_char(nc);
        }

        let lex = self.current_lexeme.clone();
        if self.lookup(&lex) {
            return Some(lex);
        } else {
            eprintln!("Lexical Error: invalid token '{}'", lex);
            process::exit(1);
        }
    } else {
        // Handle normal text
        self.add_char(c);
        while self.position < self.input.len()
            && self.input[self.position] != '#'
            && !self.input[self.position].is_whitespace()
        {
            let nc = self.get_char();
            self.add_char(nc);
        }

        // prevent infinite loop at EOF or empty lexeme
        if self.position >= self.input.len() && self.current_lexeme.is_empty() {
            return None;
        }

        return Some(self.current_lexeme.clone());
    }
}

}
