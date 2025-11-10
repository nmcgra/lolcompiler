use std::fs;
use std::path::Path;
use std::process::Command;

pub struct HtmlGenerator;

impl HtmlGenerator {
    //Constructor!
    pub fn new() -> Self {
        HtmlGenerator
    }

    //I do not know enough about HTML to rightfully comment on most of this TODO: HTML
    //Essentially, we're just matching LOLCODE tags to their HTML equivalent
    pub fn generate_html(&self, tokens: &[String], input_file: &str) {
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<body>\n");

        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].to_uppercase().as_str() {
                "#HAI" => html.push_str("<!-- Program Start -->\n"),
                "#KTHXBYE" => html.push_str("<!-- Program End -->\n"),
                "#MAEK" => {
                    if i + 1 < tokens.len() {
                        match tokens[i + 1].to_uppercase().as_str() {
                            "PARAGRAF" => html.push_str("<p>\n"),
                            "LIST" => html.push_str("<ul>\n"),
                            "HEAD" => html.push_str("<h1>\n"),
                            _ => {}
                        }
                    }
                }
                "#OIC" => {
                    html.push_str("</p>\n");
                    html.push_str("</ul>\n");
                    html.push_str("</h1>\n");
                }
                "#GIMMEH" => {
                    if i + 1 < tokens.len() {
                        match tokens[i + 1].to_uppercase().as_str() {
                            "BOLD" => {
                                if i + 2 < tokens.len() {
                                    html.push_str(&format!("<b>{}</b>\n", tokens[i + 2]));
                                    i += 2;
                                }
                            }
                            "ITALICS" => {
                                if i + 2 < tokens.len() {
                                    html.push_str(&format!("<i>{}</i>\n", tokens[i + 2]));
                                    i += 2;
                                }
                            }
                            "ITEM" => {
                                if i + 2 < tokens.len() {
                                    html.push_str(&format!("<li>{}</li>\n", tokens[i + 2]));
                                    i += 2;
                                }
                            }
                            "NEWLINE" => html.push_str("<br/>\n"),
                            "SOUNDZ" => {
                                if i + 2 < tokens.len() {
                                    html.push_str(&format!(
                                        "<audio controls><source src=\"{}\"></audio>\n",
                                        tokens[i + 2]
                                    ));
                                    i += 2;
                                }
                            }
                            "VIDZ" => {
                                if i + 2 < tokens.len() {
                                    html.push_str(&format!(
                                        "<video controls width=\"480\"><source src=\"{}\"></video>\n",
                                        tokens[i + 2]
                                    ));
                                    i += 2;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                "#LEMME" => {
                    if i + 2 < tokens.len() && tokens[i + 1].eq_ignore_ascii_case("SEE") {
                        html.push_str(&format!("<span>{}</span>\n", tokens[i + 2]));
                        i += 2;
                    }
                }
                _ => {
                    if !tokens[i].starts_with('#') {
                        html.push_str(&format!("{} ", tokens[i]));
                    }
                }
            }
            i += 1;
        }

        html.push_str("\n</body>\n</html>\n");

        // print the HTML output..,
        let output_path = Path::new(input_file).with_extension("html");
        if let Err(e) = fs::write(&output_path, html) {
            eprintln!("Error writing HTML file: {}", e);
            return;
        }
        println!("HTML output saved to {:?}", output_path);

        // Try opening in Chrome (please work)
        Self::open_in_chrome(&output_path);
    }

    fn open_in_chrome(path: &Path) {
        let file_str = path.to_string_lossy();

        #[cfg(target_os = "windows")]
        {
                if let Ok(abs_path) = std::fs::canonicalize(path) {
                    let chrome_path = r"C:\Program Files\Google\Chrome\Application\chrome.exe";
                    let file_path = abs_path.to_string_lossy().to_string();
                    let _ = Command::new("chrome")
                        .arg(&file_path)
                        .spawn()
                        .or_else(|_| {
                            // fallback, because I'm feeling auspicious: try opening via `explorer` if Chrome not in PATH
                            Command::new("explorer")
                                .arg(&file_path)
                                .spawn()
                        });
                } else {
                    eprintln!("Could not resolve path for Chrome launch.");
                }
        }

        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("open")
                .arg("-a")
                .arg("Google Chrome")
                .arg(&*file_str)
                .spawn();
        }

        println!("Attempted to open HTML output in Chrome.");
    }
}

