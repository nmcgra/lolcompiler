use std::process;

pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self) -> Result<(), String>;
    fn parse_head(&mut self) -> Result<(), String>;
    fn parse_title(&mut self) -> Result<(), String>;
    fn parse_body(&mut self) -> Result<(), String>;
    fn parse_paragraph(&mut self) -> Result<(), String>;
    fn parse_list(&mut self) -> Result<(), String>;
    fn parse_list_items(&mut self) -> Result<(), String>;
    fn parse_audio(&mut self) -> Result<(), String>;
    fn parse_video(&mut self) -> Result<(), String>;
    fn parse_newline(&mut self) -> Result<(), String>;
    fn parse_variable_define(&mut self) -> Result<(), String>;
    fn parse_variable_use(&mut self) -> Result<(), String>;
    fn parse_text(&mut self) -> Result<(), String>;
}

pub struct Parser {
    pub tokens: Vec<String>, //all tokens contained here
    pub pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<String>) -> Self {
        Parser { tokens, pos: 0 }
    }

    //curent chararacter return
    fn current(&self) -> Option<&String> {
        self.tokens.get(self.pos)
    }

    //move forward a character
    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: &str) -> Result<(), String> {
        if let Some(tok) = self.current() {
            if tok.eq_ignore_ascii_case(expected) {
                self.advance();
                Ok(())
            } else {
                Err(format!(
                    "Syntax Error: expected '{}', found '{}' at position {}",
                    expected, tok, self.pos
                ))
            }
        } else {
            Err(format!(
                "Syntax Error: expected '{}', but reached end of input",
                expected
            ))
        }
    }
}

//-----------------------------------------------------------------------
// The functions below will be responsible for recognizing specific tags
//-----------------------------------------------------------------------
impl SyntaxAnalyzer for Parser {
    fn parse_lolcode(&mut self) -> Result<(), String> {
        self.expect("#HAI")?;
        self.parse_body()?;
        self.expect("#KTHXBYE")?;
        Ok(())
    }

    fn parse_body(&mut self) -> Result<(), String> {
        while let Some(tok) = self.current() {
            let up = tok.to_uppercase();
            match up.as_str() {
                "#KTHXBYE" => break,
                "#MAEK" => {
                    self.advance();
                    if let Some(next) = self.current() {
                        match next.to_uppercase().as_str() {
                            "HEAD" => self.parse_head()?,
                            "PARAGRAF" => self.parse_paragraph()?,
                            "LIST" => self.parse_list()?,
                            _ => {
                                return Err(format!(
                                    "Syntax Error: unexpected MAEK type '{}'",
                                    next
                                ))
                            }
                        }
                    } else {
                        return Err("Syntax Error: expected block type after #MAEK".to_string());
                    }
                }
                "#GIMMEH" => {
                    self.advance();
                    if let Some(next) = self.current() {
                        match next.to_uppercase().as_str() {
                            "NEWLINE" => self.parse_newline()?,
                            "SOUNDZ" => self.parse_audio()?,
                            "VIDZ" => self.parse_video()?,
                            _ => self.parse_text()?,
                        }
                    } else {
                        return Err("Syntax Error: expected token after #GIMMEH".to_string());
                    }
                }
                _ => self.parse_text()?, // now guaranteed to advance properly
            }
        }
        Ok(())
    }

    fn parse_head(&mut self) -> Result<(), String> {
        self.expect("HEAD")?;
        while let Some(tok) = self.current() {
            if tok.eq_ignore_ascii_case("#OIC") {
                break;
            }
            if tok.eq_ignore_ascii_case("#GIMMEH") {
                self.advance();
                self.parse_title()?;
            } else {
                return Err(format!("Syntax Error in HEAD: unexpected token '{}'", tok));
            }
        }
        self.expect("#OIC")?;
        Ok(())
    }

    fn parse_title(&mut self) -> Result<(), String> {
        self.expect("TITLE")?;
        while let Some(tok) = self.current() {
            if tok.eq_ignore_ascii_case("#MKAY") {
                break;
            }
            self.advance();
        }
        self.expect("#MKAY")
    }

    fn parse_paragraph(&mut self) -> Result<(), String> {
        self.expect("PARAGRAF")?;
        while let Some(tok) = self.current() {
            if tok.eq_ignore_ascii_case("#OIC") {
                break;
            }
            self.advance();
        }
        self.expect("#OIC")
    }

    fn parse_list(&mut self) -> Result<(), String> {
        self.expect("LIST")?;
        self.parse_list_items()?;
        self.expect("#OIC")?;
        Ok(())
    }

    fn parse_list_items(&mut self) -> Result<(), String> {
        while let Some(tok) = self.current() {
            if tok.eq_ignore_ascii_case("#OIC") {
                break;
            }
            if tok.eq_ignore_ascii_case("#GIMMEH") {
                self.advance();
                self.expect("ITEM")?;
                while let Some(inner) = self.current() {
                    if inner.eq_ignore_ascii_case("#MKAY") {
                        break;
                    }
                    self.advance();
                }
                self.expect("#MKAY")?;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn parse_audio(&mut self) -> Result<(), String> {
        self.expect("SOUNDZ")?;
        while let Some(tok) = self.current() {
            if tok.eq_ignore_ascii_case("#MKAY") {
                break;
            }
            self.advance();
        }
        self.expect("#MKAY")
    }

    fn parse_video(&mut self) -> Result<(), String> {
        self.expect("VIDZ")?;
        while let Some(tok) = self.current() {
            if tok.eq_ignore_ascii_case("#MKAY") {
                break;
            }
            self.advance();
        }
        self.expect("#MKAY")
    }

    fn parse_newline(&mut self) -> Result<(), String> {
        self.expect("NEWLINE")
    }

    fn parse_variable_define(&mut self) -> Result<(), String> {
        self.expect("#I")?;
        self.expect("HAZ")?;
        self.advance();
        self.expect("#IT")?;
        self.expect("IZ")?;
        self.advance();
        self.expect("#MKAY")
    }

    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.expect("#LEMME")?;
        self.expect("SEE")?;
        self.advance();
        self.expect("#MKAY")
    }

    // 🧩 FIXED FUNCTION: always advances, even if the token starts with '#'
    fn parse_text(&mut self) -> Result<(), String> {
        if let Some(tok) = self.current() {
            if !tok.starts_with('#') {
                // consume all non-# tokens
                while let Some(t) = self.current() {
                    if t.starts_with('#') {
                        break;
                    }
                    self.advance();
                }
            } else {
                // ensure progress even if token starts with '#'
                self.advance();
            }
        }
        Ok(())
    }
}
