use std::process::Command;

fn main() {
    Command::new("pdflatex")
        .arg("./Matthew_Champagne_Resume.tex")
        .status()
        .expect("Failed to execute pdflatex command");

    Command::new("sed")
        .arg(r"'/\/ID \[/d'")
        .arg("./Matthew_Champagne_Resume.pdf")
        .arg(">")
        .arg("public/docs/Matthew_Champagne_Resume.pdf")
        .status()
        .expect("Failed to execute sed command");

    std::fs::copy("./release/index.html", "./release/404.html").unwrap();

    std::fs::remove_file("./Matthew_Champagne_Resume.pdf").unwrap();
}
