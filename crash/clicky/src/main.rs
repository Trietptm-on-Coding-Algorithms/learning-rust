use gio::prelude::*;
use gtk::prelude::*;

use gtk::Button;
use gtk::{Application, ApplicationWindow};
use gtk::{Window, WindowType};

use std::cell::RefCell;
use std::error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn write_file() {
    let file_name = "clicky.log";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .unwrap_or_else(|_| panic!("Cannot open {} for append", file_name));
    if false {
        writeln!(file, "Hello!")
    } else {
        file.write_all(b"Hello, world!\n")
    }
    .expect("Cannot write to file");
    file.flush().expect("Cannot flush file");
}

fn write_init_cell_file(file: &RefCell<File>) {
    let mut file = file.borrow_mut();
    writeln!(file, "Initialising clicky...").expect("Cannot write to file");
    file.flush().expect("Cannot flush file");
}

fn old_way() -> Result<(), Box<dyn error::Error>> {
    gtk::init()?;

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("Click me!");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let file_name = "clicky.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("clicky.log")?;
    let cell_file = RefCell::new(file);
    button.connect_clicked(move |_| {
        println!("Clicked!");
        let mut file = cell_file.borrow_mut();
        file.write_all(b"Clicked\n")
            .unwrap_or_else(|err| eprintln!("Cannot write message to {}: {}", file_name, err));
    });

    gtk::main();

    Ok(())
}

fn new_way() {
    let app = Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
        .expect("Failed to initialise GTK application");

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Application");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");
        let file_name = "clicky.log";
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("clicky.log")
            .unwrap_or_else(|_| panic!("Cannot open {}", file_name));
        let cell_file = RefCell::new(file);
        write_init_cell_file(&cell_file);
        button.connect_clicked(move |_| {
            println!("Clicked!");

            // Two borrow_mut's in the same scope cause a dynamic panic!
            // https://doc.rust-lang.org/std/cell/index.html#introducing-mutability-inside-of-something-immutable
            // thread 'main' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:1165:5
            {
                let mut file = cell_file.borrow_mut();
                file.write_all(b"Clicked\n")
                    .unwrap_or_else(|err| panic!("Cannot write Clicked to {}: {}", file_name, err));
            }

            let mut file = cell_file.borrow_mut();
            file.write_all(b"Toot!\n")
                .unwrap_or_else(|err| panic!("Cannot write Toot to {}: {}", file_name, err));
        });
        window.add(&button);
        window.show_all();
    });

    app.run(&[]);
}

fn test_write_file() {
    for _ in 1..=5 {
        write_file();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if false {
        test_write_file();
    }
    let enable_new_way = false;
    if enable_new_way {
        new_way();
        Ok(())
    } else {
        old_way()
    }
}
