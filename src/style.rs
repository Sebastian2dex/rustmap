use crossterm::style::{Color, Stylize};

pub fn banner() -> () {
    let banner = r#"
 ____            _   __  __             
|  _ \ _   _ ___| |_|  \/  | __ _ _ __  
| |_) | | | / __| __| |\/| |/ _` | '_ \ 
|  _ <| |_| \__ \ |_| |  | | (_| | |_) |
|_| \_\\__,_|___/\__|_|  |_|\__,_| .__/ 
                                  |_|
    "#;

    println!("{}", banner.with(Color::DarkRed).bold());
    println!(
        "{}",
        "        A minimalist port scanner written in Rust 🦀".with(Color::DarkGrey)
    );
    println!(
        "{}",
        "        Github: https://github.com/Sebastian2dex/rustmap\n".with(Color::DarkGrey)
    );
}
