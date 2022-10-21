use std::{
    fs::{self, File},
    io::Write,
};

fn main() {
    let contents =
        fs::read_to_string("placeholder.svg").expect("Should have been able to read the file");

    let modified = contents
        .replace("%TITLE%", "CONTRIBUTOR OF OCTOBER 2022")
        .replace("%PRS%", "3000")
        .replace("%ISSUES%", "100")
        .replace("%PRREVIEWS%", "20");

    let mut file = File::create("output.svg").expect("Error creating modified file");

    file.write(modified.as_bytes()).expect("WTF");
}
