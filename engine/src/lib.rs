pub mod note;
pub mod que;
pub mod sample;

pub fn construct_lib(path: String, que_width: u8) -> std::collections::HashMap<u8, note::Note> {
    let mut notes: std::collections::hash_map::HashMap<u8, note::Note> = Default::default();
    let folders = std::fs::read_dir(path).unwrap();
    for folder in folders {
        let note_path = folder.unwrap().path().to_str().unwrap().to_string();
        let note = note::Note::new_from_folder(note_path.clone(), que_width.into());
        let number = note_path.clone().split("/").last().unwrap().to_string()[0..2]
            .to_string()
            .parse::<u8>()
            .unwrap();
        notes.insert(number, note);
    }
    notes
}

