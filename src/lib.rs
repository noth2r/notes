mod note;
mod questions;
mod ui;

use colored::Colorize;
use note::Notebook;
use questions::{Questions, QuestionsLists};
use std::{collections::HashMap, io};

type FnFromApp<'a> = for<'r> fn(&'r mut App<'a>);

pub struct App<'a> {
    notebook: Option<Notebook>,
    tasks: Vec<FnFromApp<'a>>,
    path: String,
    die: bool,
}

// Base
impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            notebook: None,
            tasks: Vec::new(),
            path: String::new(),
            die: false,
        }
    }

    pub fn run(&mut self) {
        self.greeting();
        self.cycle();
    }

    fn greeting(&mut self) {
        self.tasks.push(App::home);
    }

    fn cycle(&mut self) {
        while self.die == false {
            self.complete_tasks();
        }
    }

    fn complete_tasks(&mut self) {
        let tasks = self.tasks.to_owned();
        self.tasks = Vec::new();

        for task in tasks {
            task(&mut *self);
        }
    }

    fn stop(&mut self) {
        self.die = true
    }
}

// IO
impl<'a> App<'a> {
    fn user_choice<Closure>(&mut self, question_list: QuestionsLists, choise_fn: Closure)
    where
        Closure: FnOnce(&Option<&Questions>) -> FnFromApp<'a>,
    {
        let list = question_list.as_list();

        print!("{}\n", list);

        if let Ok(num) = ui::user_choice("Write a number") {
            let vec = question_list.as_vec();
            let choice = vec.get(num as usize);
            let ptr = choise_fn(&choice);

            self.tasks.push(ptr)
        } else {
            self.tasks.push(App::greeting)
        }
    }

    fn fields<'b>(
        &self,
        mut keys: Vec<&'b str>,
        mut questions: Vec<&str>,
    ) -> Result<HashMap<&'b str, String>, io::Error> {
        let mut map: HashMap<&'b str, String> = HashMap::with_capacity(keys.len());

        while let (Some(key), Some(question)) = (keys.get(0), questions.get(0)) {
            println!("{question}");

            // Get user input
            let input = ui::input()?;

            // Add value from input to field
            map.insert(key, input);

            // Remove unusable values
            keys.remove(0);
            questions.remove(0);

            ui::clear_terminal();
        }

        Ok(map)
    }
}

// Tabs
impl<'a> App<'a> {
    fn home(&mut self) {
        ui::clear_terminal();

        let title = format!("{}", "-- Notes --\n").yellow();
        println!("{title}");

        self.user_choice(QuestionsLists::Home, |&choice| match choice {
            Some(&Questions::AddNotebook) => App::create_notebook,
            Some(&Questions::RmNotebook) => App::rm_notebook,
            Some(&Questions::UseNotebook) => App::use_notebook,
            Some(&Questions::Exit) => App::stop,
            _ => App::greeting,
        })
    }

    fn rm_notebook(&mut self) {}

    fn use_notebook(&mut self) {
        if let Some(_) = &mut self.notebook {
            self.tasks.push(App::notebook_menu);
        } else {
            self.tasks.push(App::home);
        }
    }

    fn create_notebook(&mut self) {
        ui::clear_terminal();

        let colored_str = format!("{}", "Notebook name:").yellow();
        println!("{}", colored_str);

        match ui::input() {
            Ok(name) => {
                self.notebook = Some(Notebook::new(name));
                self.tasks.push(App::notebook_menu);
            }
            Err(error) => {
                eprintln!("Unknown error: {}", error);
                self.tasks.push(App::home);
            }
        }
    }

    fn notebook_menu(&mut self) {
        ui::clear_terminal();

        self.user_choice(QuestionsLists::NotebookMenu, |&choice| match choice {
            Some(&Questions::ShowNotes) => App::show_notes,
            Some(&Questions::AddNote) => App::add_note,
            Some(&Questions::Back) => App::home,
            _ => App::notebook_menu,
        })
    }

    fn show_notes(&mut self) {
        ui::clear_terminal();

        if let Some(notebook) = &mut self.notebook {
            println!("{}", notebook.as_list());

            match ui::input() {
                Ok(_) => self.tasks.push(App::notebook_menu),
                Err(error) => eprintln!("{error}"),
            };
        } else {
            self.tasks.push(App::home);
        }
    }

    fn add_note(&mut self) {
        ui::clear_terminal();

        let map_keys = vec!["name", "description"];
        let map_questions = QuestionsLists::AddNote
            .as_vec()
            .iter()
            .map(|q| q.as_str())
            .collect::<Vec<&str>>();

        // Add note or go home
        if let Ok(mut map) = self.fields(map_keys, map_questions) {
            let vec = map.drain().map(|(_k, v)| v).collect::<Vec<_>>();
            let key = vec[0].to_owned();
            let description = vec[1].to_owned();

            if let Some(notebook) = &mut self.notebook {
                notebook.add(key, description);
            } else {
                self.tasks.push(App::home);
            }
        }

        // Go to notebook menu
        self.tasks.push(App::notebook_menu);
    }
}
