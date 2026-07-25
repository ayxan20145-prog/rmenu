use std::{env, fs, process::Command};
use x11rb::{
    COPY_FROM_PARENT, CURRENT_TIME, connect,
    connection::Connection,
    protocol::{Event, xproto::*},
};

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
        210,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &aux,
    )?;

    conn.map_window(window)?;
    conn.flush()?;

    conn.set_input_focus(InputFocus::PARENT, window, CURRENT_TIME)?;
    conn.flush()?;

    let mut input = String::new();
    let programs = get_programs();
    let mut selected = 0;

    let gc = conn.generate_id()?;

    conn.create_gc(
        gc,
        window,
        &CreateGCAux::new().foreground(screen.white_pixel),
    )?;

    conn.flush()?;

    loop {
        let event = conn.wait_for_event()?;

        let old_input = input.clone();

        match event {
            Event::KeyPress(ev) => {
                match ev.detail {
                    // Letters
                    38 => input.push('a'),
                    56 => input.push('b'),
                    54 => input.push('c'),
                    40 => input.push('d'),
                    26 => input.push('e'),
                    41 => input.push('f'),
                    42 => input.push('g'),
                    43 => input.push('h'),
                    31 => input.push('i'),
                    44 => input.push('j'),
                    45 => input.push('k'),
                    46 => input.push('l'),
                    58 => input.push('m'),
                    57 => input.push('n'),
                    32 => input.push('o'),
                    33 => input.push('p'),
                    24 => input.push('q'),
                    27 => input.push('r'),
                    39 => input.push('s'),
                    28 => input.push('t'),
                    30 => input.push('u'),
                    55 => input.push('v'),
                    25 => input.push('w'),
                    53 => input.push('x'),
                    29 => input.push('y'),
                    52 => input.push('z'),

                    // Numbers
                    10 => input.push('1'),
                    11 => input.push('2'),
                    12 => input.push('3'),
                    13 => input.push('4'),
                    14 => input.push('5'),
                    15 => input.push('6'),
                    16 => input.push('7'),
                    17 => input.push('8'),
                    18 => input.push('9'),
                    19 => input.push('0'),

                    // Arrow keys
                    111 => {
                        if selected > 0 {
                            selected -= 1;
                            draw(&conn, window, gc, &input, &programs, selected)?;
                        }
                    }
                    116 => {
                        let matches = filter_programs(&programs, &input);

                        if selected + 1 < matches.len() {
                            selected += 1;
                            draw(&conn, window, gc, &input, &programs, selected)?;
                        }
                    }
                    // Other
                    65 => input.push(' '),
                    22 => {
                        input.pop();
                    }

                    // Enter
                    36 => {
                        let matches = filter_programs(&programs, &input);

                        if let Some(program) = matches.get(selected) {
                            Command::new(program).spawn()?;
                        }
                        return Ok(());
                    }

                    // Escape
                    9 => return Ok(()),
                    _ => {}
                }
                if old_input != input {
                    selected = 0;
                    draw(&conn, window, gc, &input, &programs, selected)?;
                }
            }
            Event::Expose(_) => {
                draw(&conn, window, gc, &input, &programs, selected)?;
            }
            _ => {}
        }
    }
}
fn draw(
    conn: &impl Connection,
    window: Window,
    gc: Gcontext,
    input: &str,
    programs: &[String],
    selected: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.clear_area(false, window, 0, 0, 0, 0)?;

    let cursor = "|";
    let text = format!("{}{}", input, cursor);

    conn.image_text8(window, gc, 10, 18, text.as_bytes())?;

    let matches = filter_programs(programs, input);

    for (i, program) in matches.iter().enumerate() {
        let text = if i == selected {
            format!("> {}", program)
        } else {
            format!("  {}", program)
        };

        conn.image_text8(window, gc, 10, 40 + (i as i16 * 18), text.as_bytes())?;
    }

    conn.flush()?;

    Ok(())
}
fn get_programs() -> Vec<String> {
    let mut programs = Vec::new();

    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    if path.is_file() {
                        if let Some(name) = path.file_name() {
                            if let Some(name) = name.to_str() {
                                programs.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    programs.sort();
    programs.dedup();

    programs
}
fn filter_programs(programs: &[String], input: &str) -> Vec<String> {
    let input_lower = input.to_lowercase();
    programs
        .iter()
        .filter(|p| p.to_lowercase().contains(&input_lower))
        .take(10)
        .cloned()
        .collect()
}
