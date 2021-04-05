use termcolor::NoColor;
use termcolor_output as tco;

fn main() {
    let w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    let mut w = tco::ResetGuard::Owned(w);
    match tco::writeln!(w, "Hello world!") {
        Ok(_) => {}
        Err(_) => {}
    };
}
