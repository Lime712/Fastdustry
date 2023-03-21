use convert_case::Casing;

use help_scripts::scanner::{advance_to_next_string, convert_to_rust_type};

fn main() {
    let s = "private static final int windowSize = 3;
    private static final Interval flowTimer = new Interval(2);
    private static final float pollScl = 20f;

    private static WindowedMean[] cacheFlow;
    private static float[] cacheSums;
    private static float[] displayFlow;
    private static final Bits cacheBits = new Bits();

    private float[] liquids = new float[content.liquids().size];
    private Liquid current = content.liquid(0);

    private @Nullable WindowedMean[] flow;
";

    let mut statics = "".to_string();
    let mut struct_vars = "".to_string();
    let mut struct_init = "".to_string();

    for line in s.lines() {
        // ignore empty lines
        if line == "" {
            continue;
        }
        let mut line = &*line.replace("final", "");
        let mut line = &*line.replace("transient", "");
        let mut line = &*line.replace("protected", "");
        line = line.trim();
        if line.starts_with("@") {
            continue;
        }
        let mut code = "".to_string();
        // replace public with pub
        if let Some(start) = advance_to_next_string(&line, "public") {
            code.push_str("pub");
            line = &line[start + 6..];
        }
        line = line.trim();
        // replace static with static
        if let Some(start) = advance_to_next_string(&line, "static") {
            // create a static variable
            code.push_str(" static");
            line = &line[start + 6..];
            line = line.trim();
            // get type
            let line: Vec<&str> = line.split(" ").collect();
            let type_ = convert_to_rust_type(line[0]);
            // get name
            let name = line[1];
            // get value
            let value = line[3..].join(" ");
            code.push_str(&format!(" {}: {} = {}\n", name, type_, value));
            statics.push_str(&code);
        } else {
            // create a variable for the struct
            let line: Vec<&str> = line.split(" ").collect();
            let type_ = convert_to_rust_type(line[0]);
            let name = line[1].replace(";", "");
            if line.len() > 2 {
                let value = line[3..].join(" ").replace(";", "");
                code.push_str(&format!(" {}: {},\n", name.to_case(convert_case::Case::Camel), type_));
                struct_init.push_str(&format!("{}: {},\n", name.to_case(convert_case::Case::Camel), value));
            } else {
                code.push_str(&format!(" {}: {},\n", name.to_case(convert_case::Case::Camel), type_));
                struct_init.push_str(&format!("{}: {}::default(),\n", name.to_case(convert_case::Case::Camel), type_));
            }
            struct_vars.push_str(&code);
        }
    }
    println!("statics: \n{}", statics);
    println!("struct_vars: \n{}", struct_vars);
    println!("struct_init: \n{}", struct_init);
}