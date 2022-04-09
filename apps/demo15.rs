


use curl::easy::Easy;

// Capture output into a local `Vec`.
fn main() {
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        dst.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
}