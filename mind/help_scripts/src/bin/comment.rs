use help_scripts::scanner::{get_comment, get_comment_option, print_s};

fn main() {
    // usage: <name> <input_file> <output_file>
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!(
            "Usage: {} <input_file> <output_file>",
            args[0].split("\\").last().unwrap()
        );
        return;
    }
    let input_path = &args[1];
    let mut s = &*std::fs::read_to_string(input_path).unwrap();
    let max = 20;
    let mut i = 0;
    let mut final_code = String::new();
    while let Some(_) = get_comment_option(s) {
        // println!("\nfound comment: ");
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
            } else {
                final_code += &*format!("/// {}\n", line.trim());
            }
        }
        final_code += &*format!("\n");
        i += 1;
        if i > max {
            break;
        }
    }

    println!("{}", final_code);
    let output_path = &args[2];
    std::fs::write(output_path, final_code).unwrap();
}
