use myh::parsing::{escape_str, unescape_str};
use myh::Primitive;

fn main() {
    let escaped = escape_str("hello\n 🦀, 丠, 両, \x7f \u{7f}");
    println!("{escaped}");
    let unescaped = unescape_str("hello\\n 🦀, 丠, 両, \\x7f \\u{7f}").unwrap();
    println!("{unescaped}");
    let e = Primitive::stringify(&('c', '\x7f'));
    println!("{e}");
    let c: (char, char, char, String, String) = Primitive::from_string("'a', 'b', '\\x7f', \"foo\", \"foo \\\", bar\"").unwrap();
    println!("{:?}", c);
}