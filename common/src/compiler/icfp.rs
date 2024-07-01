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
        "string-take" => Some(b + "T"),
        "string-drop" => Some(b + "D"),

        "string=?" => Some(b + "="),
        "modulo" => Some(b + "%"),
        "div" => Some(b + "/"),
        "or" => Some(b + "|"),
        _ => None,
    }
}
