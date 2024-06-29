pub fn unary_op(s: &str) -> Option<String> {
    let u = "U".to_string();
    match s {
        "-" => Some(u + "-"),
        "not" => Some(u + "!"),
        "string-to-int" => Some(u + "#"),
        "int-to-string" => Some(u + "$"),
        _ => None,
    }
}

pub fn binary_op(s: &str) -> Option<String> {
    let b = "B".to_string();
    match s {
        "+" | "-" | "*" | "/" | "%" | "<" | ">" | "=" | "|" | "&" => Some(b + s),
        "string-append" => Some(b + "."),
        "string-head" => Some(b + "T"),
        "string-tail" => Some(b + "D"),

        "string=?" => Some(b + "="),
        _ => None,
    }
}

pub fn variable(n: i32) -> String {
    format!("V{}", n)
}
