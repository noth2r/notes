#[derive(Debug)]
pub enum QuestionsLists {
    Home,
    NotebookMenu,
}

impl QuestionsLists {
    pub fn as_vec<'a>(&self) -> Vec<Questions> {
        match self {
            QuestionsLists::Home => vec![
                Questions::AddNotebook,
                Questions::RmNotebook,
                Questions::Exit,
            ],
            QuestionsLists::NotebookMenu => vec![
                Questions::AddNote,
                Questions::RmNote,
                Questions::UpdNote,
                Questions::Back,
            ],
        }
    }

    pub fn as_str(&self) -> String {
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
    AddNote,
    RmNote,
    UpdNote,

    AddNotebook,
    RmNotebook,

    Back,
    Exit,
}

impl Questions {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Questions::AddNote => "Do you want to create a note?",
            Questions::RmNote => "Do you want to remove a note?",
            Questions::UpdNote => "Do you want to update a note?",

            Questions::AddNotebook => "Do you want to create a notebook?",
            Questions::RmNotebook => "Do you want to remove a notebook?",

            Questions::Back => "Back",
            Questions::Exit => "Exit",
        }
    }
}
