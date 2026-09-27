//! The world's simplest "renderer": throw away the tags, keep the text.
//!
//! Real browsers build a tree out of HTML (Phase 5). For now we walk the
//! characters one by one and remember a single thing: "am I inside a <tag>?"

/// Remove HTML tags, and skip the contents of <script> and <style> entirely
/// (their text is code, not something a reader should see).
pub fn extract_text(html: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    let mut tag = String::new(); // everything between '<' and '>'
    let mut skipping = false; // true while inside <script> or <style>

    for c in html.chars() {
        if in_tag {
            if c == '>' {
                in_tag = false;
                // `<style type="text/css">` → tag name "style"
                let name = tag.split_whitespace().next().unwrap_or("").to_lowercase();
                match name.as_str() {
                    "script" | "style" => skipping = true,
                    "/script" | "/style" => skipping = false,
                    _ => {}
                }
            } else {
                tag.push(c);
            }
        } else if c == '<' {
            in_tag = true;
            tag.clear();
        } else if !skipping {
            text.push(c);
        }
    }

    text
}

/// Trim each line and squeeze runs of blank lines down to one,
/// so the output reads nicely in a terminal.
pub fn tidy(text: &str) -> String {
    let mut out = String::new();
    let mut last_was_blank = true; // start "blank" so we skip leading empty lines

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !last_was_blank {
                out.push('\n');
            }
            last_was_blank = true;
        } else {
            out.push_str(line);
            out.push('\n');
            last_was_blank = false;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_tags() {
        assert_eq!(extract_text("<p>Hello <b>world</b></p>"), "Hello world");
    }

    #[test]
    fn skips_style_and_script() {
        let html = "<style>body { color: red }</style><p>Hi</p><script>alert(1)</script>";
        assert_eq!(extract_text(html), "Hi");
    }

    #[test]
    fn tidy_collapses_blank_lines() {
        assert_eq!(tidy("\n\n  a  \n\n\n\n b\n"), "a\n\nb\n");
    }
}
