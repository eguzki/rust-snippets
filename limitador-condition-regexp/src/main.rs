use regex::Regex;

fn main() {
    let re = Regex::new(r"([\w_\.]+)\s*==\s*'(.+)'").unwrap();

    let conditions_str = [
        "foo=='bar '",
        "foo == 'bar '",
        "  foo == 'bar ' ",
        "  foo   ==   'bar ' ",
        "var == 42",
        " 変数 == '  💖 '",
        "foo == 'ba",
        r#""x != 5 && x > 12""#,
    ];

    for i in 0..conditions_str.len() {
        match re.captures(conditions_str[i]) {
            Some(caps) => println!(
                "{}: identifier: {}, predicate: EQUAL, operand: {}",
                conditions_str[i],
                caps.get(1).unwrap().as_str(),
                caps.get(2).unwrap().as_str(),
            ),
            None => println!("{}: does not parse", conditions_str[i]),
        };
    }
}
