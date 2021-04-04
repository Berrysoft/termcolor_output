use termcolor::NoColor;
use termcolor_output as tco;

fn main() {
    let mut w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    match tco::ResetGuard::<std::io::Error>::run(&mut w, |w| {
        tco::writeln!(w, "Hello world!")?;
        Ok(())
    }) {
        Ok(_) => {}
        Err(_) => {}
    };
}
