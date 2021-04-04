use termcolor::{Color, NoColor};
use termcolor_output as tco;

fn main() {
    let mut w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    match tco::write!(
        w,
        "{}{}{}{}{}{}{}",
        bold!(true),
        underline!(true),
        bold!(false),
        fg!(Some(Color::White)),
        bg!(Some(Color::Black)),
        reset!(),
        intense!(true)
    ) {
        Ok(_) => {}
        Err(_) => {}
    };
}
