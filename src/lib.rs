mod note;
mod questions;
mod ui;

use colored::Colorize;
use note::Notebook;
use questions::{Questions, QuestionsLists};
use std::collections::HashMap;

type FnFromApp<'a> = for<'r> fn(&'r mut App<'a>) -> Result<(), &'a str>;

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
        if let Ok(()) = self.greeting() {
            self.cycle();
        }
    }

    fn greeting(&mut self) -> Result<(), &'a str> {
        self.tasks.push(App::home);

        Ok(())
    }

    fn cycle(&mut self) {
        while self.die == false {
            if let Err(error) = self.complete_tasks() {
                eprintln!("{error}");
                self.tasks.push(App::home);
            }
        }
    }

    fn complete_tasks(&mut self) -> Result<(), &'a str> {
        let tasks = self.tasks.to_owned();
        self.tasks = Vec::new();

        for task in tasks {
            task(&mut *self)?;
        }

        Ok(())
    }

    fn stop(&mut self) -> Result<(), &'a str> {
        self.die = true;

        Ok(())
    }
}

// IO
impl<'a> App<'a> {
    fn user_choice<Closure>(
        &mut self,
        question_list: QuestionsLists,
        choise_fn: Closure,
    ) -> Result<(), &'a str>
    where
        Closure: FnOnce(&Option<&Questions>) -> FnFromApp<'a>,
    {
        let list = question_list.as_list();

        print!("{}\n", list);

        if let Ok(num) = ui::user_choice("Write a number") {
            let vec = question_list.as_vec();
            let choice = vec.get(num as usize);
            let ptr = choise_fn(&choice);

            Ok(self.tasks.push(ptr))
        } else {
            Err("Can't get user input")
        }
    }

    fn fields<'b>(
        &self,
        mut keys: Vec<&'b str>,
        mut questions: Vec<&str>,
    ) -> Result<HashMap<&'b str, String>, &'a str> {
        let mut map: HashMap<&'b str, String> = HashMap::with_capacity(keys.len());

        while let (Some(key), Some(question)) = (keys.get(0), questions.get(0)) {
            println!("{question}");

            match ui::input() {
                Ok(input) => {
                    // Add value from input to field
                    map.insert(key, input);

                    // Remove unusable values
                    keys.remove(0);
                    questions.remove(0);

                    ui::clear_terminal();
                }
                Err(_) => return Err("Can't get user input"),
            }
        }

        Ok(map)
    }
}

// Tabs
impl<'a> App<'a> {
    fn home(&mut self) -> Result<(), &'a str> {
        ui::clear_terminal();

        let title = format!("{}", "-- Notes --\n").yellow();
        println!("{title}");

        self.user_choice(QuestionsLists::Home, |&choice| match choice {
            Some(&Questions::AddNotebook) => App::create_notebook,
            Some(&Questions::RmNotebook) => App::rm_notebook,
            Some(&Questions::UseNotebook) => App::use_notebook,
            Some(&Questions::Exit) => App::stop,
            _ => App::greeting,
        })?;

        Ok(())
    }

    fn rm_notebook(&mut self) -> Result<(), &'a str> {
        self.notebook = None;
        Ok(self.tasks.push(App::home))
    }

    fn use_notebook(&mut self) -> Result<(), &'a str> {
        if let Some(_) = &mut self.notebook {
            Ok(self.tasks.push(App::notebook_menu))
        } else {
            Err("Notebook is unavailable")
        }
    }

    fn create_notebook(&mut self) -> Result<(), &'a str> {
        ui::clear_terminal();

        let colored_str = format!("{}", "Notebook name:").yellow();
        println!("{}", colored_str);

        match ui::input() {
            Ok(name) => {
                self.notebook = Some(Notebook::new(name));
                self.tasks.push(App::notebook_menu);

                Ok(())
            }
            Err(_) => Err("Can't get user input"),
        }
    }

    fn notebook_menu(&mut self) -> Result<(), &'a str> {
        ui::clear_terminal();

        self.user_choice(QuestionsLists::NotebookMenu, |&choice| match choice {
            Some(&Questions::ShowNotes) => App::show_notes,
            Some(&Questions::AddNote) => App::add_note,
            Some(&Questions::RmNote) => App::rm_note,
            Some(&Questions::Back) => App::home,
            _ => App::notebook_menu,
        })?;

        Ok(())
    }

    fn show_notes(&mut self) -> Result<(), &'a str> {
        ui::clear_terminal();

        if let Some(notebook) = &mut self.notebook {
            println!("Notebook name: {}", notebook.name);
            println!("{}", notebook.as_list());
        }

        match ui::input() {
            Ok(_) => Ok(self.tasks.push(App::notebook_menu)),
            Err(_) => Err("Can't get user input"),
        }
    }

    fn add_note(&mut self) -> Result<(), &'a str> {
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

        Ok(())
    }

    fn rm_note(&mut self) -> Result<(), &'a str> {
        ui::clear_terminal();

        if let Some(notebook) = &mut self.notebook {
            // Print all note names
            for (i, note) in notebook.notes.iter().enumerate() {
                println!("{}. {}", i, note.get_name());
            }

            println!("Number of note that you want to delete:");

            if let Ok(input) = ui::input() {
                match input.trim().parse::<usize>() {
                    Ok(index) => {
                        notebook.rm(index);
                        Ok(self.tasks.push(App::notebook_menu))
                    }
                    Err(_) => Err("Can't parse string"),
                }
            } else {
                Err("Can't get user input")
            }
        } else {
            Err("Notebook is unavailable")
        }
    }
}
