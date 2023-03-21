use help_scripts::scanner::{get_comment, get_comment_option, print_s};

fn main() {
    // todo: change this, ill do it today evening
    let path = "C:\\Users\\janwi\\mindustryServer\\v7\\mods\\Mindustry\\core\\src\\mindustry\\world\\modules\\ItemModule.java";
    let mut s = &*std::fs::read_to_string(path).unwrap();
    let max = 20;
    let mut i = 0;
    let mut final_code = String::new();
    while let Some(_) = get_comment_option(s) {
        println!("\nfound comment: ");
        // print_s(s);
        let mut c = get_comment(s).0.to_string();
        s = get_comment(s).1;
        // print_s(s);
        c = c.replace("*", "").replace("*/", "");
        c = c.trim().to_string();
        // println!("{:?}", c);
        let mut first_args = true;
        for mut line in c.to_string().lines() {
            if line.trim().is_empty() {
                continue;
            }
            line = line.trim();
            if line.starts_with("@param") {
                let mut parts = line.split_whitespace();
                parts.next();
                let name = parts.next().unwrap();
                let desc = parts.collect::<Vec<&str>>().join(" ");
                if first_args {
                    final_code += "/// # Arguments\n";
                    first_args = false;
                }
                final_code += &*(format!("/// * `{}` - {}\n", name, desc));
                continue;
            }
            if line.starts_with("@return") {
                let mut parts = line.split_whitespace();
                parts.next();
                let desc = parts.collect::<Vec<&str>>().join(" ");
                final_code += &*format!("/// * returns {}\n", desc);
                continue;
            } else { final_code += &*format!("/// {}\n", line.trim()); }
        }
        final_code += &*format!("\n");
        i += 1;
        if i > max {
            break;
        }
    }

    println!("{}", final_code);
    // same here
    let mut out_path = "C:\\Users\\janwi\\rust\\MindRustry\\mind\\help_scripts\\output\\comments.rs";
    std::fs::write(out_path, final_code).unwrap();
}