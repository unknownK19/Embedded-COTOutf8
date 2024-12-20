use embedded_cotoutf8::{COtoUTF8, DebugODisplay};

fn main() {
    let num: f32 = 1.0005;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);

    let num: f32 = -123.0005;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);

    let num: f32 = 1524.001;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);

    let num: f32 = 44245.12;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);

    let num: f32 = 442400005.001;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);

    // Alternative use this
    let num = DebugODisplay(44245.12f32);
    println!("{}", num)
}
