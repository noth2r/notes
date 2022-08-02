#[derive(Debug, PartialEq)]
pub struct Note<'a> {
    name: &'a str,
    description: &'a str,
}

impl<'a> Note<'a> {
    pub fn new(name: &'a str, description: &'a str) -> Self {
        Self { name, description }
    }
}

#[derive(Debug)]
pub struct Notebook<'a> {
    pub name: String,
    pub notes: Vec<Note<'a>>,
}

impl<'a> Notebook<'a> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            notes: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &'a str, description: &'a str) {
        let note = Note::new(name, description);
        self.notes.push(note);
    }

    pub fn rm(&mut self, name: &'a str) {
        self.notes.retain(|note| note.name.contains(name) == false);
    }

    pub fn upd(&mut self, name: &'a str, note: Note<'a>) {
        self.rm(name);
        self.add(note.name, note.description);
    }
}

#[cfg(test)]
mod notebook {
    use super::*;

    #[test]
    fn rm_note() {
        let mut notebook = Notebook::new("hey".to_owned());

        notebook.add("some name", "qwroqwrooqwro");
        notebook.add("some", "qwroqwrooqwro");

        notebook.rm("some name");

        assert_eq!(notebook.notes, vec![Note::new("some", "qwroqwrooqwro")]);
    }
}
