use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;

use glib::Bytes as GlibBytes;

use super::guii::Size2Di;
use super::image_area::render_image_on_thread;

pub enum RenderThreadMsg {
    Quit,
    Render(GlibBytes, Size2Di),
}

pub fn start_render_thread() -> (std::thread::JoinHandle<()>, Sender<RenderThreadMsg>) {
    let (tx, rx) = channel();
    //
    let handle = std::thread::spawn(move || {
        let mut push_back_buf = Vec::new();
        //
        'loop_main: loop {
            let msg = rx.recv().expect("Unable to receive from channel");
            match msg {
                RenderThreadMsg::Quit => break,
                RenderThreadMsg::Render(bytes, iwh) => {
                    push_back_buf.push((bytes, iwh));
                }
            }
            loop {
                match rx.try_recv() {
                    Ok(msg) => match msg {
                        RenderThreadMsg::Quit => break 'loop_main,
                        RenderThreadMsg::Render(bytes, iwh) => {
                            push_back_buf.push((bytes, iwh));
                        }
                    },
                    Err(TryRecvError::Empty) => {
                        if let Some((bytes, iwh)) = push_back_buf.pop() {
                            render_image_on_thread(&bytes, iwh);
                        }
                        push_back_buf.clear();
                    }
                    Err(TryRecvError::Disconnected) => {
                        break 'loop_main;
                    }
                }
            }
        }
        push_back_buf.clear();
    });
    (handle, tx)
}
