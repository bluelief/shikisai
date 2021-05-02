use std::io::Write;

use shikisai::*;

pub fn print(s: String) {
    writeln!(&mut std::io::stdout(), "{}", s).unwrap();
}

#[test]
fn test_stdout_color() {
    print("black".black());
    print("red".red());
    print("green".green());
    print("yellow".yellow());
    print("blue".blue());
    print("magenta".magenta());
    print("cyan".cyan());
    print("white".white());
    print("red-bold".red().bold());
}
