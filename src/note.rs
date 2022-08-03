#[derive(Debug, PartialEq)]
pub struct Note {
    name: String,
    description: String,
}

impl Note {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

#[derive(Debug)]
pub struct Notebook {
    pub name: String,
    pub notes: Vec<Note>,
}

impl Notebook {
    pub fn new(name: String) -> Self {
        Self {
            name,
            notes: Vec::new(),
        }
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn add(&mut self, name: String, description: String) {
        let note = Note::new(name, description);
        self.notes.push(note);
    }

    pub fn rm(&mut self, name: String) {
        self.notes.retain(|note| note.name.contains(&name) == false);
    }

    pub fn upd(&mut self, name: String, note: Note) {
        self.rm(name);
        self.add(note.name, note.description);
    }

    pub fn as_list(&self) -> String {
        let mut list = String::new();

        for (i, note) in self.notes.iter().enumerate() {
            list += format!("{} #{}\n", note.name, i).as_str();
            list += format!("{}\n\n", note.description).as_str();
        }

        list
    }
}

#[cfg(test)]
mod notebook {
    use super::*;

    #[test]
    fn rm_note() {
        let mut notebook = Notebook::new("hey".to_owned());

        notebook.add(String::from("some name"), String::from("qwroqwrooqwro"));
        notebook.add(String::from("some"), String::from("qwroqwrooqwro"));

        notebook.rm(String::from("some name"));

        assert_eq!(
            notebook.notes,
            vec![Note::new(
                String::from("some"),
                String::from("qwroqwrooqwro")
            )]
        );
    }
}
