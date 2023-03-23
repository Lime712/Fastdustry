pub fn strip_colors(str: &str) -> String {
    let mut out = String::new();

    let mut i = 0;
    while i < str.len() {
        let c = str.chars().nth(i).unwrap();

        // possible color tag
        if c == '[' {
            let length = parse_color_markup(str, i + 1,  str.len());
            if length >= 0 {
                i += length as usize + 2;
                continue;
            } else {
                out.push(c);
                // escaped string
                i+=1;
            }
        } else {
            out.push(c);
            i+=1;
        }
    }

    out
}

/// returns None if the string is not a valid color markup
pub fn parse_color_markup(str: &str, start: usize, end: usize) -> i32 {
    if start >= end { return -1; } // String ended with "["
    match str.chars().nth(start).unwrap() {
        '#' => {
            // Parse hex color RRGGBBAA where AA is optional and defaults to 0xFF if less than 6 chars are used.
            for i in start + 1..end {
                let ch = str.chars().nth(i).unwrap();
                if ch == ']' {
                    if i < start + 2 || i > start + 9 {
                        break;
                    }
                    return (i - start) as i32;
                }
                if !ch.is_digit(16) {
                    break;
                }
            }
            return -1;
        }
        '[' => { return -2; } // "[[" is an escaped left square bracket. todo: handle this case
        ']' => { // "[]" is a "pop" color tag.
            // pop the color stack here if needed
            return 0; }
        _ => {}
    }
    // parse named color
    for i in start + 1..end {
        let ch = str.chars().nth(i).unwrap();
        if ch == ']' {
            return (i - start) as i32;
        }
        if !ch.is_alphanumeric() {
            break;
        }
        // todo: create the color struct and check if its a valid color
    }
    -1
}