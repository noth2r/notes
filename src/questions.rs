#[derive(Debug)]
pub enum QuestionsLists {
    Home,
    NotebookMenu,
    AddNote,
}

impl QuestionsLists {
    pub fn as_vec<'a>(&self) -> Vec<Questions> {
        match self {
            QuestionsLists::Home => vec![
                Questions::AddNotebook,
                Questions::RmNotebook,
                Questions::UseNotebook,
                Questions::Exit,
            ],
            QuestionsLists::NotebookMenu => vec![
                Questions::ShowNotes,
                Questions::AddNote,
                Questions::RmNote,
                Questions::Back,
            ],
            QuestionsLists::AddNote => vec![Questions::NoteName, Questions::NoteDescription],
        }
    }

    pub fn as_list(&self) -> String {
        let mut string = String::new();
        let vec = self.as_vec();

        for (i, quest) in vec.iter().enumerate() {
            string = format!("{}{}. {}\n", string, i, quest.as_str());
        }

        string
    }
}

#[derive(Debug)]
pub enum Questions {
    ShowNotes,
    AddNote,
    RmNote,

    AddNotebook,
    RmNotebook,
    UseNotebook,

    NoteName,
    NoteDescription,

    Back,
    Exit,
}

impl Questions {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Questions::ShowNotes => "Show notebook.",
            Questions::AddNote => "Do you want to create a note?",
            Questions::RmNote => "Do you want to remove a note?",

            Questions::AddNotebook => "Do you want to create a notebook?",
            Questions::RmNotebook => "Do you want to remove a notebook?",
            Questions::UseNotebook => "Use an existing notebook.",

            Questions::NoteName => "Note name:",
            Questions::NoteDescription => "Note description:",

            Questions::Back => "Back",
            Questions::Exit => "Exit",
        }
    }
}
