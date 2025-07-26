use std::env;

#[derive(Default, Debug, Clone, Copy)]
pub enum UserSelection {
    #[default] Help,
    PointList,
    LineList,
    LineStrip,
}

pub fn parse_user_input() -> UserSelection {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "help" {
        print_help();
        return UserSelection::Help;
    }

    match args[1].as_str() {
        "point-list" => UserSelection::PointList,
        "line-list" => UserSelection::LineList,
        "line-strip" => UserSelection::LineStrip,
        _ => {
            eprintln!("Unknown mode: '{}'", args[1]);
            print_help();
            UserSelection::Help
        }
    }
}

fn print_help() {
    println!("Usage: cargo run -- <mode>");
    println!("Supported modes:");
    println!("  point-list   - Render using PointList");
    println!("  line-list    - Render using LineList");
    println!("  line-strip   - Render using LineStrip");
    println!("  help         - Show this help message");
}
