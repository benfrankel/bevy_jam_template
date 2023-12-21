use std::cell::OnceCell;

use bevy::prelude::*;
use bevy::utils::HashMap;
use lazy_regex::regex;

// Parses a "rich text" string.
//
// Format:
// - The text style will be set to `styles[start_tag]` initially.
// - "[tag]" will set the text style to `styles["tag"]` for the following text.
// - If `styles["tag"]` is not found, "[tag]" will be interpreted as literal text.
// - Tags cannot be escaped. To allow literal "[tag]", don't use "tag" as a key.
pub fn parse_rich(text: &str, styles: &HashMap<&str, TextStyle>, start_tag: &str) -> Text {
    let mut sections = vec![];

    let mut lo = 0;
    let mut style = &styles[start_tag];
    let mut section = TextSection::new("", style.clone());

    let mut push_str = |s: &str, style: &TextStyle| {
        if s.is_empty() {
            return;
        }

        // If the new text uses a different style, create a new section for it
        if section.style.font != style.font
            || section.style.font_size != style.font_size
            || section.style.color != style.color
        {
            let mut old_section = TextSection::new("", style.clone());
            std::mem::swap(&mut old_section, &mut section);
            if !old_section.value.is_empty() {
                sections.push(old_section);
            }
        }
        section.value.push_str(s);
    };

    for tag in regex!(r"\[((?:\w|\d|-)+)\]").captures_iter(text) {
        // Skip invalid tags to include them as literal text instead
        let Some(next_style) = styles.get(&tag[1]) else {
            continue;
        };

        let delim = tag.get(0).unwrap();
        push_str(&text[lo..delim.start()], style);
        lo = delim.end();
        style = next_style;
    }
    push_str(&text[lo..text.len()], style);
    if !section.value.is_empty() {
        sections.push(section);
    }

    Text::from_sections(sections)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::BOLD_FONT_HANDLE;
    use crate::ui::FONT_HANDLE;

    fn get_styles() -> HashMap<&'static str, TextStyle> {
        let r = TextStyle {
            font: FONT_HANDLE,
            ..default()
        };
        let b = TextStyle {
            font: BOLD_FONT_HANDLE,
            ..default()
        };
        HashMap::from([("regular", r.clone()), ("bold", b.clone())])
    }

    #[test]
    #[should_panic]
    fn test_invalid_start_tag() {
        let _ = parse_rich("hello", &get_styles(), "invalid");
    }

    #[test]
    fn test_text() {
        let styles = get_styles();
        let r = &styles["regular"].clone();
        let b = &styles["bold"].clone();
        for (case, want) in [
            ("", vec![]),
            ("[bold]", vec![]),
            ("[bold", vec![TextSection::new("[bold", r.clone())]),
            ("bold]", vec![TextSection::new("bold]", r.clone())]),
            ("[[bold]", vec![TextSection::new("[", r.clone())]),
            ("[bold]]", vec![TextSection::new("]", b.clone())]),
            (
                "[[bold]]",
                vec![
                    TextSection::new("[", r.clone()),
                    TextSection::new("]", b.clone()),
                ],
            ),
            ("[invalid]", vec![TextSection::new("[invalid]", r.clone())]),
            ("[][][]", vec![TextSection::new("[][][]", r.clone())]),
            ("hello [bold]", vec![TextSection::new("hello ", r.clone())]),
            ("[bold] hello", vec![TextSection::new(" hello", b.clone())]),
            (
                "[bold][bold] hello",
                vec![TextSection::new(" hello", b.clone())],
            ),
            (
                "hello [bold] world",
                vec![
                    TextSection::new("hello ", r.clone()),
                    TextSection::new(" world", b.clone()),
                ],
            ),
            (
                "hello [invalid] world",
                vec![TextSection::new("hello [invalid] world", r.clone())],
            ),
            (
                "hello [] world",
                vec![TextSection::new("hello [] world", r.clone())],
            ),
            (
                "hello [[bold]] world",
                vec![
                    TextSection::new("hello [", r.clone()),
                    TextSection::new("] world", b.clone()),
                ],
            ),
            (
                "hello \\[bold] world",
                vec![
                    TextSection::new("hello \\", r.clone()),
                    TextSection::new(" world", b.clone()),
                ],
            ),
            (
                "hello [regular] world",
                vec![TextSection::new("hello  world", r.clone())],
            ),
            (
                "hello [regular] w[regular][regular]orld",
                vec![TextSection::new("hello  world", r.clone())],
            ),
            (
                "hello [regular][bold] world",
                vec![
                    TextSection::new("hello ", r.clone()),
                    TextSection::new(" world", b.clone()),
                ],
            ),
            (
                "hello [bold][regular] world",
                vec![TextSection::new("hello  world", r.clone())],
            ),
        ] {
            let got = parse_rich(case, &styles, "regular");
            assert_eq!(got.sections.len(), want.len());
            for (got, want) in got.sections.iter().zip(&want) {
                assert_eq!(got.value, want.value);
                assert_eq!(got.style.font, want.style.font);
                assert_eq!(got.style.font_size, want.style.font_size);
                assert_eq!(got.style.color, want.style.color);
            }
        }
    }
}
