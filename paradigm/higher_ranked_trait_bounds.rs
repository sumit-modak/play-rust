trait Formatter {
    fn format<T: std::fmt::Display>(&self, value: T) -> String;
}

struct SimpleFormatter;

impl Formatter for SimpleFormatter {
    fn format<T: std::fmt::Display>(&self, value: T) -> String {
        format!("Value: {}", value)
    }
}

fn apply_format(formatter: impl Formatter) -> impl for<'a> Fn(&'a str) -> String {
    move |s| formatter.format(s)
}

// this means the input string that the return closure accepts
// must live as long as the lifetime of the closure
fn apply_format2<'a>(formatter: impl Formatter) -> impl Fn(&'a str) -> String {
    move |s| formatter.format(s)
}

fn main() {
    let formatter = SimpleFormatter;
    let format_fn = apply_format(formatter);

    let s1 = "Hello";
    let s2 = String::from("World");

    println!("{}", format_fn(s1));
    println!("{}", format_fn(&s2));
}
