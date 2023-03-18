use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use convert_case::{Case, Casing};
use convert_case::Case::{Camel, Pascal, Snake};

use help_scripts::scanner;
use help_scripts::scanner::{advance_to_next_char, advance_to_next_string, advance_to_next_string_or_string, next_string, print_s, skip_whitespace_and_newline};

#[derive(Debug, Clone)]
struct Method {
    comment: String,
    name: String,
    parameters: Vec<Parameter>,
    return_type: String,
    body: String,
}

#[derive(Debug, Clone)]
struct Parameter {
    name: String,
    type_: String,
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
    // usage: <name> <input_folder> <output_folder>
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input_folder> <output_folder>", args[0].split("\\").last().unwrap());
        return;
    }

    // read the code from a file
    let input_folder = args[1].clone();
    let paths = fs::read_dir(input_folder).unwrap();
    for path in paths {
        let p = path.unwrap().path();
        println!("\nConverting: {}", p.clone().display());
        let mut s: &str = &*fs::read_to_string(p).unwrap();
        let mut final_code = String::new();

        // now we try to convert this to a trait in rust
        // first we search for "public interface" and then we search for "{"
        // we first extract the name of the interface
        // but first we need to get the comments
        // println!("s: {}", s);
        // check if there is a comment
        let mut i = advance_to_next_string_or_string(s, "public", "/*");
        if i == None {
            i = advance_to_next_string_or_string(s, "interface", "/*");
        }
        if i == None {
            println!("Error: expected public or /*");
            continue;
        }
        if &s[i.unwrap()..i.unwrap() + 1] == "/" {
            let (mut comment, mut s) = scanner::get_comment(s);
            let r = comment.replace("*", "").replace("*/", "");
            let b = r.trim();
            comment = &*b;
            // print_s(s);
            final_code = format!("/// {}\r\n", comment).to_string();
            s = s.trim();
        }

        // println!("{}", next_string(s).unwrap()); // public
        // search for the public keyword
        let p = advance_to_next_string(s, "public");
        if let Some(i) = p {
            s = &s[i..];
            s = s.trim();
            s = &s[next_string(s).unwrap().len()..];
            s = s.trim();
        }
        let i = advance_to_next_string(s, "interface");
        if let Some(i) = i {
            s = &s[i..];
            s = s.trim();
        }
        println!("interface: {}", next_string(s).unwrap()); // interface
        if next_string(s).unwrap() != "interface" {
            println!("Error: expected interface");
            continue;
        }
        s = &s[next_string(s).unwrap().len()..];
        s = s.trim();
        let interface_name = next_string(s).unwrap().to_case(Pascal);
        s = &s[interface_name.len()..];
        s = s.trim();

        print_s(s);
        let mut extends: Vec<String> = Vec::new();
        let mut imports = String::new();
        if let Some(start) = advance_to_next_string(s, "extends") {
            // check if its still on this line
            let mut exit = false;
            for i in 0..start {
                if &s[i..i + 1] == "\n" {
                    exit = true;
                    break;
                }
            }
            if !exit {
                let end = advance_to_next_char(&s[start..], '{').unwrap();
                extends = s[start..start + end].split(", ").map(|x| { x.replace("extends", "").trim().to_string() }).collect::<Vec<String>>();
                for e in extends.clone() {
                    // add to imports
                    imports.push_str(&*format!("use crate::gen::{}::{};\n", e.to_string().to_case(Snake), e.to_string()));
                }
                println!("extends: {:?}", extends);
                s = &s[start + end..];
            }
        }
        if extends.len() > 0 {
            final_code += &*format!("pub trait {} : {} {{", interface_name, extends.join(" + ")).to_string();
        } else {
            final_code += &*format!("pub trait {} {{", interface_name).to_string();
        }

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

        // we collect all types so we can create imports for them later (or try to at least)
        let mut types = HashSet::new();
        let mut methods = Vec::new();
        let mut method_names = HashSet::new();
        while let Some(char) = scanner::next_char(s) {
            if char == '{' {
                s = &s[1..];
            }
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
            s = scanner::skip_whitespace_and_newline(s);
            // println!("s: {}", s);
            // check if theres something like "@Nullable" or "@Override" before it, just skip everything if the line starts with @
            // then we change the return type to Option<return_type>
            let mut option = false;
            if let Some(char) = scanner::next_char(s) {
                if char == '@' {
                    let mut i = 0;
                    for ch in s.chars() {
                        if ch == '\n' {
                            break;
                        }
                        i += 1;
                    }
                    s = &s[i..];
                    s = scanner::skip_whitespace_and_newline(s);
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
            s = scanner::skip(s, '(');
            // get the parameters
            let mut parameters = Vec::new();
            while let Some(char) = scanner::next_char(s) {
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
                if let Some(char) = scanner::next_char(s) {
                    if char == ',' {
                        s = &s[1..];
                    } else if char == ')' {
                        s = &s[1..];
                        break;
                    }
                }
            }
            // println!("parameters: {:?}", parameters);
            method.parameters = parameters.iter().map(|parameter| {
                // get name of the parameter
                let name = parameter.split(' ').last().unwrap().to_case(Case::Snake).to_string();
                // get the type of the parameter
                let parameter = parameter.split(' ').take(parameter.split(' ').count() - 1).collect::<Vec<&str>>().join(" ");
                // convert the type to rust
                let parameter = scanner::convert_to_rust_type(&parameter);
                // add the type to the types
                types.insert(parameter.clone());
                Parameter {
                    name,
                    type_: parameter.to_string(),
                }
            }).collect();
            s = s.trim();
            if let Some(char) = scanner::next_char(s) {
                if char != ';' {
                    // get the body
                    let mut body = String::new();
                    while let Some(char) = scanner::next_char(s) {
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
            if method_names.contains(&method.name) {
                // if there is, we need to add the parameters to the method name
                let mut i = 0;
                while method_names.contains(&method.name) && i < method.parameters.len() {
                    method.name = format!("{}_{}", method.name.to_case(Case::Snake), method.parameters[i].name);
                    i += 1;
                }
            }
            method_names.insert(method.name.clone());
            method_code.push_str(&format!("    fn {}(", method.name.to_case(Case::Snake)));
            for (i, parameter) in method.parameters.iter().enumerate() {
                method_code.push_str(&format!("{}: {}", parameter.name, parameter.type_));
                if i != method.parameters.len() - 1 {
                    method_code.push_str(", ");
                }
            }
            // now the fun part: converting the parameter to rust
            // first check if its void, because then we can just skip it
            let mut return_type = method.return_type.clone();
            println!("method: {:?}", method);
            if return_type != "void" && return_type != "{" {
                return_type = scanner::convert_to_rust_type(&return_type);
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

        // print all used types
        println!("types: {:?}", types);
        imports.push_str(&scanner::convert_to_imports(types));
        final_code = format!("{}{}", imports, final_code);

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
            continue;
        }
        let mut module_code = String::new();
        // append the new module
        module_code.push_str(&format!("\npub mod {};", interface_name.to_case(Case::Snake)));
        // write the file
        match module_file.write_all(module_code.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", path, why.description()),
            Ok(_) => println!("successfully wrote to {}", path),
        }
    }
}