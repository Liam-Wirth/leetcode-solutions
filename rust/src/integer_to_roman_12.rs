pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut out = String::new();
    let conv = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];
    for &(symbol, val) in conv.iter() {
        while num >= val {
            out.push_str(&symbol);
            num -= val;
        }
    }

    out
}
fn main() {
    println!("{}", int_to_roman(954));
}
