fn is_qmd_file(path: &str) -> bool {
    path.ends_with(".qmd")
}

fn main() {
    let paths = ["notes/example.md", "notes/example.qmd", "notes/example.txt"];

    for path in paths {
        if is_qmd_file(path) {
            println!("{path} is a QMD file.");
        } else {
            println!("{path} is not a QMD file.");
        }
    }
}
