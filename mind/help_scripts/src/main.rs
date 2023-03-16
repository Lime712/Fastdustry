use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use convert_case::{Case, Casing};

#[derive(Debug, Clone)]
struct Method {
    comment: String,
    name: String,
    parameters: Vec<String>,
    return_type: String,
    body: String,
}

impl Default for Method {
    fn default() -> Self {
        Self {
            comment: "".to_string(),
            name: "".to_string(),
            parameters: Vec::new(),
            return_type: "".to_string(),
            body: "".to_string(),
        }
    }
}


fn main() {


    // usage: <name> <file_name.java> <output_folder>
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <file_name.java> <output_folder>", args[0].split("\\").last().unwrap());
        return;
    }

    // read the code from a file
    let file_name = args[1].clone();
    let mut s: &str = &*std::fs::read_to_string(file_name).unwrap();
    //    let mut s = "bla
    //    /**
    //       * Interface for {@link mindustry.entities.comp.BuildingComp}
    // */
    //    public interface Buildingc extends QuadTreeObject, Sized, Entityc, Healthc, Posc, Teamc, Timerc, Controllable, Senseable, Displayable {
    //    /**
    //     * @return the block that this building is built on
    //     */";


    let mut final_code = "".to_string();

    // now we try to convert this to a trait in rust
    // first we search for "public interface" and then we search for "{"
    // we first extract the name of the interface
    // but first we need to get the comments
    // println!("s: {}", s);
    let (mut comment, mut s) = get_comment(s);
    let r = comment.replace("*", "").replace("*/", "");
    let b = r.trim();
    comment = &*b;
    // print_s(s);
    final_code = format!("/// {}\r\n", comment).to_string();
    s = s.trim();
    // println!("{}", next_string(s).unwrap()); // public
    // search for the public keyword
    let i = advance_to_next_string(s, "public").unwrap();
    s = &s[i..];
    s = s.trim();
    s = &s[next_string(s).unwrap().len()..];
    s = s.trim();
    // println!("{}", next_string(s).unwrap()); // interface
    s = &s[next_string(s).unwrap().len()..];
    s = s.trim();
    let interface_name = next_string(s).unwrap();
    final_code += &*format!("pub trait {} {{", interface_name).to_string();
    s = &s[interface_name.len()..];

    let mut interface = "";
    while let Some(start) = advance_to_next_char(s, '{') {
        let end = advance_to_next_char(&s[start..], '}').unwrap();
        interface = &s[start..start + end];
        // println!("i: {}", interface);
        s = &s[start + end..];
    }
    s = interface;
    // now we have the interface
    // we need to extract the methods
    // get the comment and convert it from // or /**/ to ///
    // get the method name
    // get the parameters
    // get the return type
    // get the body
    // convert the body to rust
    // convert the parameters to rust
    // convert the return type to rust
    // convert the method to rust
    // add the method to the trait
    let mut methods = Vec::new();
    while let Some(char) = next_char(s) {
        // println!("{}", char);
        let mut method = Method::default();
        if char == '}' {
            break;
        }
        // get the comment
        // let (mut comment, mut s) = get_comment(s);
        let mut comment = if let Some(start) = advance_to_next_char(s, '/') {
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
        let r = comment.replace("*", "").replace("*/", "");
        let b = r.trim();
        comment = &*b;
        // println!("comment: {}", comment);
        method.comment = comment.to_string();
        s = skip_whitespace_and_newline(s);
        // println!("s: {}", s);
        // check if theres something like "@Nullable" or "@Override" before it, just skip everything if the line starts with @
        // then we change the return type to Option<return_type>
        let mut option = false;
        if let Some(char) = next_char(s) {
            if char == '@' {
                let mut i = 0;
                for ch in s.chars() {
                    if ch == '\n' {
                        break;
                    }
                    i += 1;
                }
                s = &s[i..];
                s = skip_whitespace_and_newline(s);
                option = true;
            }
        }
        // get the return type
        let return_type = {
            let r = next_string(s).unwrap();
            s = &s[r.len()..];
            if option {
                format!("Option<{}>", r).to_string()
            } else {
                r.to_string()
            }
        };
        // println!("return type: {}", return_type);
        method.return_type = return_type.to_string();
        s = s.trim();
        // get the method name
        let method_name = {
            let mut i = 0;
            for ch in s.chars() {
                if ch == '(' {
                    break;
                }
                i += 1;
            }
            &s[..i]
        };
        s = &s[method_name.len()..];
        // println!("method_name: {}", method_name);
        method.name = method_name.to_string();
        s = s.trim();
        s = skip(s, '(');
        // get the parameters
        let mut parameters = Vec::new();
        while let Some(char) = next_char(s) {
            if char == ')' {
                s = &s[1..];
                break;
            }
            let parameter = {
                let mut i = 0;
                for ch in s.chars() {
                    if ch == ',' || ch == ')' {
                        break;
                    }
                    i += 1;
                }
                &s[..i]
            };
            s = &s[parameter.len()..];
            s = s.trim();
            parameters.push(parameter);
            if let Some(char) = next_char(s) {
                if char == ',' {
                    s = &s[1..];
                } else if char == ')' {
                    s = &s[1..];
                    break;
                }
            }
        }
        // println!("parameters: {:?}", parameters);
        method.parameters = parameters.iter().map(|x| x.to_string()).collect();
        s = s.trim();
        if let Some(char) = next_char(s) {
            if char != ';' {
                // get the body
                let mut body = String::new();
                while let Some(char) = next_char(s) {
                    if char == '}' {
                        s = &s[1..];
                        break;
                    }
                    body.push(char);
                    s = &s[1..];
                }
                // println!("body: {}", body);
                method.body = body;
            } else {
                s = &s[1..];
            }
        }
        // println!("method: {:?}", method);
        methods.push(method);
    }

    // now we have all the methods
    // println!("methods: {:?}", methods);

    for mut method in methods.clone() {
        let mut method_code = String::new();
        method_code.push_str("\n");
        // format the comment a little bit
        method.comment = method.comment.trim().to_string();
        method.comment = method.comment.replace("@return", "# Returns\n");
        for line in method.comment.lines() {
            method_code.push_str(&format!("    /// {}\n", line.trim()));
        }
        // convert the method name to snake case
        // check if there is another method with the same name
        if methods.iter().filter(|x| x.name == method.name).count() > 1 {
            // if there is, we need to add the parameters to the method name
            if method.parameters.len() != 0 {
                method.name = format!("{}_{}", method.name.to_case(Case::Snake), method.parameters.first().unwrap().to_case(Case::Snake));
            }
        }
        method_code.push_str(&format!("    fn {}(", method.name.to_case(Case::Snake)));
        for (i, parameter) in method.parameters.iter().enumerate() {
            // get name of the parameter
            let name = parameter.split(' ').last().unwrap().to_case(Case::Snake).to_string();
            // get the type of the parameter
            let parameter = parameter.split(' ').take(parameter.split(' ').count() - 1).collect::<Vec<&str>>().join(" ");
            // convert the type to rust
            let parameter = convert_to_rust_type(&parameter);
            method_code.push_str(&format!("{}: {}", name, parameter));
            if i != method.parameters.len() - 1 {
                method_code.push_str(", ");
            }
        }
        // now the fun part: converting the parameter to rust
        // first check if its void, because then we can just skip it
        let mut return_type = method.return_type.clone();
        if return_type != "void" {
            return_type = convert_to_rust_type(&return_type);
            method_code.push_str(&format!(") -> {}", return_type));
        } else {
            method_code.push_str(")");
        }

        if method.body.len() > 0 {
            method_code.push_str(&format!(" {{
        {}  }}", method.body));
        } else {
            method_code.push_str(";");
        }
        method_code.push_str("\n");
        final_code.push_str(&method_code);
    }
    // append the closing brace
    final_code.push_str("}\n");

    // println!("final code: {}", final_code);

    // write the code to a file
    let path = format!("{}/{}.rs", args[2], interface_name.to_case(Case::Snake));
    let mut file = File::create(path.clone()).unwrap();
    println!("writing to file: {}", path);
    match file.write_all(final_code.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path, why.description()),
        Ok(_) => println!("successfully wrote to {}", path),
    }
    // search for mod.rs in that directory
    let path = format!("{}/mod.rs", args[2]);
    let mut module_file = OpenOptions::new().write(true).append(true).read(true).open(path.clone()).unwrap();
    let mut module_code = String::new();
    // read the file
    match module_file.read_to_string(&mut module_code) {
        Err(why) => panic!("couldn't read {}: {}", path, why.description()),
        Ok(_) => println!("successfully read {}", path),
    }
    if module_code.contains(&format!("pub mod {};", interface_name.to_case(Case::Snake))) {
        println!("module already exists");
        return;
    }
    let mut module_code = String::new();
    // append the new module
    module_code.push_str(&format!("\npub mod {};\n", interface_name.to_case(Case::Snake)));
    // write the file
    match module_file.write_all(module_code.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path, why.description()),
        Ok(_) => println!("successfully wrote to {}", path),
    }
}

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

fn convert_to_rust_type(s: &str) -> String {
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

fn skip_whitespace_and_newline(s: &str) -> &str {
    let mut i = 0;
    for ch in s.chars() {
        if ch != ' ' && ch != '\r' && ch != '\t' && ch != '\n' {
            return &s[i..];
        }
        i += 1;
    }
    ""
}

fn skip(s: &str, c: char) -> &str {
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

fn advance_to_next_char(s: &str, c: char) -> Option<usize> {
    for (i, ch) in s.char_indices() {
        if ch == c {
            return Some(i);
        }
    }
    None
}

fn next_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn next_string(s: &str) -> Option<&str> {
    let mut i = 0;
    for ch in s.chars() {
        if ch == ' ' {
            return Some(&s[..i]);
        }
        i += 1;
    }
    None
}

fn advance_to_next_string(s: &str, string: &str) -> Option<usize> {
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

fn print_s(s: &str) {
    let b = {
        let mut v = String::new();
        for i in 0..30 {
            v.push(s.char_indices().nth(i).unwrap().1);
        }
        v
    };
    println!("s: {:?}", b);
}