use gtk::glib::{self, clone};
use gtk::{prelude::*, Picture};
use gtk::{Box, Orientation};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
pub fn set_image(id: String, mutex: Arc<Mutex<()>>) -> Box {
    let imgbox = Box::new(Orientation::Vertical, 5);

    let (sender, receiver) = async_channel::bounded::<String>(1);

    let image = gtk::Picture::new();
    image.set_halign(gtk::Align::Center);

    let path = format!(
        "{}/.local/share/tsukimi/{}.png",
        dirs::home_dir().expect("msg").display(),
        id
    );
    let pathbuf = PathBuf::from(&path);
    let idfuture = id.clone();
    if pathbuf.exists() {
        image.set_file(Some(&gtk::gio::File::for_path(&path)));
    } else {
        crate::ui::network::runtime().spawn(async move {
            let _lock = mutex.lock().await;
            let mut retries = 0;
            while retries < 3 {
                match crate::ui::network::get_image(id.clone()).await {
                    Ok(id) => {
                        sender
                            .send(id.clone())
                            .await
                            .expect("The channel needs to be open.");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Failed to get image: {}, retrying...", e);
                        retries += 1;
                    }
                }
            }
        });
    }

    glib::spawn_future_local(clone!(@weak image => async move {
        while let Ok(_) = receiver.recv().await {
            let path = format!("{}/.local/share/tsukimi/{}.png",dirs::home_dir().expect("msg").display(), idfuture);
            let file = gtk::gio::File::for_path(&path);
            image.set_file(Some(&file));
        }
    }));

    imgbox.append(&image);
    imgbox
}

pub fn set_thumbimage(id: String, mutex: Arc<Mutex<()>>) -> Box {
    let imgbox = Box::new(Orientation::Vertical, 5);

    let (sender, receiver) = async_channel::bounded::<String>(1);

    let image = gtk::Picture::new();
    image.set_halign(gtk::Align::Center);

    let path = format!(
        "{}/.local/share/tsukimi/t{}.png",
        dirs::home_dir().expect("msg").display(),
        id
    );
    let pathbuf = PathBuf::from(&path);
    let idfuture = id.clone();
    if pathbuf.exists() {
        image.set_file(Some(&gtk::gio::File::for_path(&path)));
    } else {
        crate::ui::network::runtime().spawn(async move {
            let _lock = mutex.lock().await;
            let mut retries = 0;
            while retries < 3 {
                match crate::ui::network::get_thumbimage(id.clone()).await {
                    Ok(id) => {
                        sender
                            .send(id.clone())
                            .await
                            .expect("The channel needs to be open.");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Failed to get image: {}, retrying...", e);
                        retries += 1;
                    }
                }
            }
        });
    }

    glib::spawn_future_local(clone!(@weak image => async move {
        while let Ok(_) = receiver.recv().await {
            let path = format!("{}/.local/share/tsukimi/{}.png",dirs::home_dir().expect("msg").display(), idfuture);
            let file = gtk::gio::File::for_path(&path);
            image.set_file(Some(&file));
        }
    }));

    imgbox.append(&image);
    imgbox
}

pub fn set_backdropimage(id: String, mutex: Arc<Mutex<()>>) -> Box {
    let imgbox = Box::new(Orientation::Vertical, 5);

    let (sender, receiver) = async_channel::bounded::<String>(1);

    let image = gtk::Picture::new();
    image.set_halign(gtk::Align::Center);

    let path = format!(
        "{}/.local/share/tsukimi/b{}.png",
        dirs::home_dir().expect("msg").display(),
        id
    );
    let pathbuf = PathBuf::from(&path);
    let idfuture = id.clone();
    if pathbuf.exists() {
        image.set_file(Some(&gtk::gio::File::for_path(&path)));
    } else {
        crate::ui::network::runtime().spawn(async move {
            let _lock = mutex.lock().await;
            let mut retries = 0;
            while retries < 3 {
                match crate::ui::network::get_backdropimage(id.clone()).await {
                    Ok(id) => {
                        sender
                            .send(id.clone())
                            .await
                            .expect("The channel needs to be open.");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Failed to get image: {}, retrying...", e);
                        retries += 1;
                    }
                }
            }
        });
    }

    glib::spawn_future_local(clone!(@weak image => async move {
        while let Ok(_) = receiver.recv().await {
            let path = format!("{}/.local/share/tsukimi/{}.png",dirs::home_dir().expect("msg").display(), idfuture);
            let file = gtk::gio::File::for_path(&path);
            image.set_file(Some(&file));
        }
    }));

    imgbox.append(&image);
    imgbox
}

pub fn setimage(id: String,mutex: Arc<Mutex<()>>) -> Picture {
    let (sender, receiver) = async_channel::bounded::<String>(1);

    let image = gtk::Picture::new();
    image.set_halign(gtk::Align::Center);

    let path = format!(
        "{}/.local/share/tsukimi/{}.png",
        dirs::home_dir().expect("msg").display(),
        id
    );
    let pathbuf = PathBuf::from(&path);
    let idfuture = id.clone();
    if pathbuf.exists() {
        image.set_file(Some(&gtk::gio::File::for_path(&path)));
    } else {
        crate::ui::network::runtime().spawn(async move {
            let _lock = mutex.lock().await;
            let mut retries = 0;
            while retries < 3 {
                match crate::ui::network::get_image(id.clone()).await {
                    Ok(id) => {
                        sender
                            .send(id.clone())
                            .await
                            .expect("The channel needs to be open.");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Failed to get image: {}, retrying...", e);
                        retries += 1;
                    }
                }
            }
        });
    }

    glib::spawn_future_local(clone!(@weak image => async move {
        while let Ok(_) = receiver.recv().await {
            let path = format!("{}/.local/share/tsukimi/{}.png",dirs::home_dir().expect("msg").display(), idfuture);
            let file = gtk::gio::File::for_path(&path);
            image.set_file(Some(&file));
        }
    }));

    image
}

pub fn setlogoimage(id: String) -> Picture {
    let (sender, receiver) = async_channel::bounded::<String>(1);

    let image = gtk::Picture::new();
    image.set_halign(gtk::Align::Center);

    let path = format!(
        "{}/.local/share/tsukimi/{}.png",
        dirs::home_dir().expect("msg").display(),
        id
    );
    let pathbuf = PathBuf::from(&path);
    let idfuture = id.clone();
    if pathbuf.exists() {
        image.set_file(Some(&gtk::gio::File::for_path(&path)));
    } else {
        crate::ui::network::runtime().spawn(async move {
            let id = crate::ui::network::get_image(id.clone())
                .await
                .expect("msg");
            sender
                .send(id.clone())
                .await
                .expect("The channel needs to be open.");
        });
    }

    glib::spawn_future_local(clone!(@weak image => async move {
        while let Ok(_) = receiver.recv().await {
            let path = format!("{}/.local/share/tsukimi/{}.png",dirs::home_dir().expect("msg").display(), idfuture);
            let file = gtk::gio::File::for_path(&path);
            image.set_file(Some(&file));
        }
    }));

    image
}
