use x11rb::{connect, connection::Connection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    println!("{}", screen.root);

    Ok(())
}
