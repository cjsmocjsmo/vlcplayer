// extern crate vlc;

// use std::sync::mpsc::channel;

// use vlc::{Instance, Media, MediaPlayer, Event, EventType, State};

// fn main() {
//     let args: Vec<String> = std::env::args().collect();

//     let path = match args.get(1) {
//         Some(s) => s,
//         None => {
//             println!("Usage: cli_audio_player path_to_a_media_file");
//             return;
//         }
//     };
//     let instance = Instance::new().unwrap();

//     let md = Media::new_path(&instance, path).unwrap();
//     let mdp = MediaPlayer::new(&instance).unwrap();

//     let (tx, rx) = channel::<()>();

//     let em = md.event_manager();
//     let _ = em.attach(EventType::MediaStateChanged, move |e, _| {
//         match e {
//             Event::MediaStateChanged(s) => {
//                 println!("State : {:?}", s);
//                 if s == State::Ended || s == State::Error {
//                     tx.send(()).unwrap();
//                 } else if s == State::Paused || s == State::Stopped {
//                     tx.send(()).unwrap();
//                 }
//             },
//             _ => (),
//         }
//     });

//     mdp.set_media(&md);

//     // Start playing
//     mdp.play().unwrap();



//     // Wait for end state
//     rx.recv().unwrap();
// }

use fltk::{
    enums::Color,
    prelude::*,
    *,
    // image::IcoImage
};
use vlc::*;

#[derive(Copy, Clone)]
pub enum Message {
    Play,
    Stop,
}

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Media Player");
    // let icon: IcoImage = IcoImage::load(&std::path::Path::new("src/fltk.ico")).unwrap();
    win.make_resizable(true);
    // win.set_icon(Some(icon));

    // Create inner window to act as embedded media player
    let mut vlc_win = window::Window::new(10, 10, 780, 520, "");
    vlc_win.end();
    vlc_win.set_color(Color::Black);

    let mut but_play = button::Button::new(320, 545, 80, 40, "Play");
    let mut but_stop = button::Button::new(400, 545, 80, 40, "Stop");

    win.end();
    win.show();
    win.make_resizable(true);

    // Take in same args as vlc
    let args: Vec<String> = std::env::args().collect();

    // Instantiate vlc instance and media player
    let instance = Instance::new().unwrap();
    let md = Media::new_path(&instance, "/home/charliepi/testvid2.mkv").unwrap();
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Get vlc_win handle that we'll pass to libvlc
    // Linux u32, windows HWND, Mac NSWindow
    let handle = vlc_win.raw_handle();

    // Pass the handle to vlc
    // Method depends on the platform
    // For Linux
    #[cfg(target_os = "linux")]
    mdp.set_xwindow(handle as u32);
    // For Windows
    #[cfg(target_os = "windows")]
    mdp.set_hwnd(handle);
    // For MacOS
    #[cfg(target_os = "macos")]
    mdp.set_nsobject(utils::content_view(&vlc_win) as _);

    // Disable event handling on vlc's side
    // Do it thru fltk
    mdp.set_key_input(false);
    mdp.set_mouse_input(false);

    let (s, r) = app::channel::<Message>();

    but_play.emit(s, Message::Play);
    but_stop.emit(s, Message::Stop);

    while app.wait() {
        match r.recv() {
            Some(val) => match val {
                Message::Play => mdp.play().unwrap(),
                Message::Stop => mdp.stop(),
            },
            None => (),
        }
    }
}