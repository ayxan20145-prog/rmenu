use x11rb::{COPY_FROM_PARENT, connect, connection::Connection, protocol::xproto::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let window = conn.generate_id()?;

    let aux = CreateWindowAux::new()
        .background_pixel(screen.black_pixel)
        .override_redirect(1)
        .event_mask(EventMask::EXPOSURE | EventMask::KEY_PRESS);

    conn.create_window(
        COPY_FROM_PARENT as u8,
        window,
        screen.root,
        0,
        0,
        screen.width_in_pixels,
        28,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &aux,
    )?;

    conn.map_window(window)?;
    conn.flush()?;

    loop {
        let event = conn.wait_for_event()?;

        match event {
            x11rb::protocol::Event::Expose(_) => {
                conn.map_window(window)?;
                conn.flush()?;
            }
            _ => {}
        }
    }
}
