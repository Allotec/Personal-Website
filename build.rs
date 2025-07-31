use std::process::Command;

fn main() {
    Command::new("pdflatex")
        .arg("./Matthew_Champagne_Resume.tex")
        .status()
        .expect("Failed to execute pdflatex command");

    std::fs::copy(
        "./Matthew_Champagne_Resume.pdf",
        "./public/Matthew_Champagne_Resume.pdf",
    )
    .unwrap();

    std::fs::remove_file("./Matthew_Champagne_Resume.pdf").unwrap();
}
