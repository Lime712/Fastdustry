use std::collections::HashSet;

pub fn get_comment(mut s: &str) -> (&str, &str) {
    // get the comment
    let comment = if let Some(start) = advance_to_next_char(s, '/') {
        // println!("\nstart: {}", start);
        // skip the first 2 *
        s = &s[start + 3..];
        // print_s(s);
        // save to comment until / occurs
        let end = advance_to_next_string(&s[0..], "*/").unwrap();
        let comment = &s[0..end - 2];
        // println!("end: {}: {}", end, s.char_indices().nth(end).unwrap().1);
        s = &s[end + 2..];
        // print_s(s);
        // println!("comment: {}", comment);
        comment
    } else {
        ""
    };
    // s = &s.trim();
    // s = skip(s, '*');
    // s = skip(s, '/');
    (comment, s)
}

pub fn convert_to_rust_type(s: &str) -> String {
    // check if @Nullable is in the string and replace it with Option
    let mut s = s.to_string();
    if s.contains("@Nullable") {
        s = s.replace("@Nullable", "");
        s = format!("Option<{}>", s.trim());
    }
    // replace Seq<T> with HashSet<T>
    if s.contains("Seq<") {
        s = s.replace("Seq<", "HashSet<");
    }
    let s = s.trim();
    match s {
        "int" => "i32",
        "long" => "i64",
        "float" => "f32",
        "double" => "f64",
        "boolean" => "bool",
        "char" => "char",
        "byte" => "u8",
        "short" => "i16",
        _ => s,
    }.parse().unwrap()
}

fn skip_whitespace(s: &str) -> &str {
    let mut i = 0;
    for ch in s.chars() {
        if ch != ' ' {
            return &s[i..];
        }
        i += 1;
    }
    ""
}

pub fn skip_whitespace_and_newline(s: &str) -> &str {
    let mut i = 0;
    for ch in s.chars() {
        if ch != ' ' && ch != '\r' && ch != '\t' && ch != '\n' {
            return &s[i..];
        }
        i += 1;
    }
    ""
}

pub fn skip(s: &str, c: char) -> &str {
    let mut i = 0;
    for ch in s.chars() {
        if ch != c {
            return &s[i..];
        }
        i += 1;
    }
    ""
}

fn advance_n(s: &str, n: usize) -> &str {
    &s[n..]
}

pub fn advance_to_next_char(s: &str, c: char) -> Option<usize> {
    for (i, ch) in s.char_indices() {
        if ch == c {
            return Some(i);
        }
    }
    None
}

pub fn next_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn next_string(s: &str) -> Option<&str> {
    let mut i = 0;
    for ch in s.chars() {
        if ch == ' ' {
            return Some(&s[..i]);
        }
        i += 1;
    }
    None
}

pub fn advance_to_next_string(s: &str, string: &str) -> Option<usize> {
    for (i, ch) in s.char_indices() {
        if ch == string.chars().next().unwrap() {
            let mut j = 0;
            for ch in s[i..].chars() {
                if ch != string.chars().nth(j).unwrap() {
                    break;
                }
                j += 1;
                if j == string.len() {
                    return Some(i);
                }
            }
        }
    }
    None
}

pub fn advance_to_next_string_or_string(s: &str, string: &str, string2: &str) -> Option<usize> {
    for (i, ch) in s.char_indices() {
        if ch == string.chars().next().unwrap() {
            let mut j = 0;
            for ch in s[i..].chars() {
                if ch != string.chars().nth(j).unwrap() {
                    break;
                }
                j += 1;
                if j == string.len() {
                    return Some(i);
                }
            }
        }
        if ch == string2.chars().next().unwrap() {
            let mut j = 0;
            for ch in s[i..].chars() {
                if ch != string2.chars().nth(j).unwrap() {
                    break;
                }
                j += 1;
                if j == string2.len() {
                    return Some(i);
                }
            }
        }
    }
    None
}

pub fn print_s(s: &str) {
    let b = {
        let mut v = String::new();
        for i in 0..30 {
            v.push(s.char_indices().nth(i).unwrap().1);
        }
        v
    };
    println!("s: {:?}", b);
}

// convert list of types to imports with "use"
pub fn convert_to_imports(types: HashSet<String>) -> String {
    let mut imports = HashSet::new();
    for t in types {
        // call convert_to_import
        let mut new_imports = convert_to_import(&t);
        // add the new imports to the imports
        for i in new_imports.drain() {
            imports.insert(i);
        }
    }
    println!("imports: {:?}", imports);
    // now we have all the imports in the imports HashSet
    // now we need to match the imports to the correct use
    let mut imports_string = String::new();
    for import in imports {
        let mut usage = match &*import {
            "String" => "",
            "HashSet" => "std::collections::HashSet",
            "HashMap" => "std::collections::HashMap",
            "Vec" => "std::vec::Vec",
            "Option" => "",
            "i32" => "",
            "i64" => "",
            "f32" => "",
            "f64" => "",
            "bool" => "",
            "char" => "std::char",
            "u8" => "std::u8",
            "i16" => "std::i16",
            "Item" => "crate::r#type::item::Item",
            "Liquid" => "crate::r#type::liquid::Liquid",
            "Vec2" => "arc::arc_core::math::geom::vec2::Vec2",
            _ => "",
        }.to_string();
        if usage != "" {
            usage = format!("use {};", usage);
            imports_string.push_str(&*usage);
            imports_string.push_str("\r\n");
        }
    }
    imports_string
}


pub fn convert_to_import(s: &str) -> HashSet<String> {
    // println!("convert_to_import: {}", s);
    // we check if theres something like blablha<blabla> and if so we add the blabla to the imports
    // and we need to that recursively
    let mut s = s.to_string();
    let mut imports = HashSet::new();
    // read the type and look if the next char is <
    let mut i = 0;
    if let Some(start) = advance_to_next_char(&s, '<') {
        // println!("found <");
        // we found a < so we need to find the next >, should be at the end, if not well then we have a problem
        let end = &s.len() - 1;
        // now we save the type before the < to the imports
        let ty = &s[0..start];
        // println!("ty: {}", ty);
        imports.insert(ty.to_string());
        // now call this function again with the type between the < and >
        let sub_imports = convert_to_import(&s[start + 1..end]);
        for i in sub_imports {
            imports.insert(i);
        }
    } else {
        // we didnt find a < so we just add the type to the imports
        imports.insert(match_import(s));
    }
    imports
}

// converts java types to rust types
pub fn match_import(ty: String) -> String {
    // todo: convert eg Seq to HashSet
    ty
}