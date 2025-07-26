use std::env;

#[derive(Default, Debug, Clone, Copy)]
pub enum UserSelection {
    #[default] Help,
    TriangleList,
    TriangleStrip,
}

pub fn parse_user_input() -> UserSelection {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "help" {
        print_help();
        return UserSelection::Help;
    }

    match args[1].as_str() {
        "triangle-list" => UserSelection::TriangleList,
        "triangle-strip" => UserSelection::TriangleStrip,
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
    println!("  triangle-list   - Render using PointList");
    println!("  triangle-strip  - Render using LineList");
    println!("  help            - Show this help message");
}
