use std::process::Command;

fn main() {
    let status = Command::new("pdflatex")
        .arg("./Matthew_Champagne_Resume.tex")
        .status()
        .expect("Failed to execute pdflatex command");

    if !status.success() {
        panic!("pdflatex failed");
    }

    std::fs::copy(
        "./Matthew_Champagne_Resume.pdf",
        "./public/docs/Matthew_Champagne_Resume.pdf",
    )
    .expect("Failed to copy generated resume PDF");

    std::fs::remove_file("./Matthew_Champagne_Resume.pdf")
        .expect("Failed to remove generated resume PDF");
}
