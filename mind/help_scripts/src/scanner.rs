use std::cmp::{max, min};
use std::collections::HashSet;

pub fn get_comment(mut s: &str) -> (&str, &str) {
    // get the comment
    match get_comment_option(s) {
        Some((comment, s)) => (comment, s),
        None => ("", s),
    }
}

pub fn get_comment_option(mut s: &str) -> Option<(&str, &str)> {
    // get the comment
    let comment = if let Some(start) = advance_to_next_string(s, "/**") {
        s = &s[start + 3..];
        let end = match advance_to_next_string(&s[0..], "*/") {
            Some(end) => end,
            None => return None,
        };
        let comment = &s[0..end - 1];
        s = &s[end + 2..];
        comment
    } else {
        return None;
    };
    Some((comment, s))
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
    }.to_string()
}

fn skip_whitespace(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[i..],
        _ => "",
    }
}

pub fn skip_whitespace_and_newline(s: &str) -> &str {
    for (i, ch) in s.char_indices() {
        if matches!(ch, ' ' | '\r' | '\t' | '\n') {
            return &s[i..];
        }
    }
    ""
}

pub fn skip(s: &str, c: char) -> &str {
    for (i, ch) in s.char_indices() {
        if ch != c {
            return &s[i..];
        }
    }
    ""
}

fn advance_n(s: &str, n: usize) -> &str {
    &s[n..]
}

pub fn advance_to_next_char(s: &str, c: char) -> Option<usize> {
    s.find(c)
}

pub fn next_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn next_string(s: &str) -> Option<&str> {
    s.find(' ').map(|i| &s[..i])
}

pub fn advance_to_next_string(s: &str, string: &str) -> Option<usize> {
    s.find(string)
}

pub fn advance_to_next_string_or_string(s: &str, string: &str, string2: &str) -> Option<usize> {
    s.find(string).or(s.find(string2))
}

pub fn print_s(s: &str) {
    let b = {
        let mut v = String::new();
        for i in 0..min(s.len() - 1, 30) {
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
        }
        .to_string();
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
    let s = s.to_string();
    let mut imports = HashSet::new();
    // read the type and look if the next char is <
    if let Some(start) = advance_to_next_char(&s, '<') {
        // println!("found <");
        // we found a < so we need to find the next >, should be at the end, if not well then we have a problem
        let end = &s.len() - 1;
        // now we save the type before the < to the imports
        let ty = &s[0..start];
        // println!("ty: {}", ty);
        imports.insert(match_import(ty.to_string()));
        // now call this function again with the type between the < and >
        let sub_imports = convert_to_import(&s[start + 1..end]);
        for i in sub_imports {
            imports.insert(match_import(i));
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
    match ty.as_str() {
        "Seq" => "HashSet",
        ty => ty,
    }.to_string()
}

