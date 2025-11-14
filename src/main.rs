
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: hello-rust <words...>");
        std::process::exit(2);
    }
    print_report(args.iter().map(|s| s.as_str()));
}

fn print_report<'a, I>(words: I)
where
    I: IntoIterator<Item = &'a str>,
{
    let collected: Vec<&str> = words.into_iter().collect();
    println!("You gave me {} words:", collected.len());
    for (i, w) in collected.iter().enumerate() {
        println!("{:>3}: {}", i + 1, w);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity() {
        let x = 2 + 2;
        assert_eq!(x, 4);
    }
}
