use termcolor::NoColor;
use termcolor_output as tco;

struct NonDebuggable;

fn main() {
    let mut w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    match tco::write!(w, "{}{}{:?}", Some(()), vec![1u8], NonDebuggable) {
        Ok(_) => {}
        Err(_) => {}
    };
}
