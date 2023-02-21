use microkv::MicroKV;
use home::home_dir;

pub fn list() {
    let db: MicroKV = open();
    let db_keys: Vec<String> = db.keys().expect("Couldn't find keys in db.");

    match db_keys.len() {
        0 => println!("No sticky notes."),
        _ => {
            for key in db_keys {
                let full_note: String = db.get_unwrap(&key).unwrap();
                let preview: String = note_preview(full_note);
                println!("{} => {}", key, preview);
            }
        }
    }
}

pub fn read_sticky_note(sticky_note_name: &String) {
    let retrieved_value: String = open().get_unwrap(sticky_note_name).unwrap();
    println!("{}\n===\n{}", sticky_note_name, retrieved_value);
}

pub fn write_sticky_note(name: &String, text: &String, force: bool) {
    let db: MicroKV = open();

    if db.exists(name).expect("Failed to check db for sticky note.") && !force {
        println!("Sticky note named {} already exists, use '-W' to overwrite.", name);
    } else {
        db.put(name, text).expect("Failed to write note.");
        println!("Wrote sticky note named {}.", name);
    }
}

pub fn delete_sticky_note(name: &String) {
    open().delete(name).expect("Failed to delete note.");
    println!("Deleted sticky note named {}", name);
}

fn path() -> std::path::PathBuf {
    let mut path: std::path::PathBuf = home_dir().expect("Couldn't find the user's home directory");
    path.push("microkv_db");
    path
}

fn open() -> MicroKV {
    MicroKV::open_with_base_path("sticky_note_db", path())
        .expect("Failed to create MicroKV from a stored file or create MicroKV for this file")
        .set_auto_commit(true)
}

fn note_preview(note: String) -> String {
    if note.len() > 32 {
        let mut preview: String = String::from(note.split_at(29).0);
        preview.push_str("...");
        preview
    } else {
        note
    }
}
