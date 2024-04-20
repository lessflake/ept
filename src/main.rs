use std::{fs, io, num::NonZeroUsize, path::PathBuf};

use crossterm::event::{self, Event, KeyEvent};
use lepu::Epub;

use ept::term::Display;

// TODO: features
// - nicer virtual styling
// - book select maybe?
// - progress saving
// - scorescreen; wpm/acc display at end
// - score annotations per paragraph
// - window resize
// - sixel images

// TODO: annoyances/bugs
// - british english uses single quotation marks for speech, prefer double--
//   convert single quotation marks to double quotations? messes up quotes
//   within quotes, so will have to parse nested and convert accordingly

fn main() -> anyhow::Result<()> {
    xflags::xflags! {
        cmd ept {
            /// Path to book.
            required path: PathBuf
            /// Width of text view, in characters.
            /// Defaults to 60.
            optional -w,--width width: NonZeroUsize
        }
    };

    let args = Ept::from_env()?;
    let book = fs::read(args.path)
        .map_err(Into::into)
        .and_then(Epub::new)?;

    let width = args
        .width
        .and_then(|x| x.get().try_into().ok())
        .unwrap_or(60u16);

    println!("{}'s {}", book.author().unwrap(), book.title());

    let (term_w, term_h) = crossterm::terminal::size()?;

    let mut w = io::stdout();
    let mut display = Display::new(book, width, term_w, term_h);

    display.enter(&mut w)?;

    loop {
        let ev = next_key_event()?;
        if display.handle_input(ev)? {
            break;
        }

        display.render(&mut w)?;
    }

    Display::exit(&mut w)?;

    Ok(())
}

fn next_key_event() -> anyhow::Result<KeyEvent> {
    loop {
        if let Ok(Event::Key(event)) = event::read() {
            return Ok(event);
        }
    }
}
