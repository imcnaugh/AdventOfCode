fn main() {
    let prefix = "iwrupvqb";
    let result = (0..).find(|i| {
        let test = format!("{}{}", prefix, i);
        let hashed = md5::compute(test);
        let output = format!("{:x}", hashed);
        output.chars().take(6).all(|b| b == '0')
    });
    if let Some(i) = result {
        println!("{}", i);
    }
}
