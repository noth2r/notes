mod note;
mod questions;
mod user_interface;

use colored::Colorize;
use note::Notebook;
use questions::{Questions, QuestionsLists};
use std::collections::HashMap;
use std::io;

type F<'a> = for<'r> fn(&'r mut App<'a>);

pub struct App<'a> {
    pub data: HashMap<String, Notebook<'a>>,
    tasks: Vec<F<'a>>,
    die: bool,
}

// Base
impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            data: HashMap::new(),
            tasks: Vec::new(),
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
        Closure: FnOnce(&Option<&Questions>) -> F<'a>,
    {
        let list = question_list.as_str();

        print!("{}\n", list);

        if let Ok(num) = user_interface::user_choice("Write a number") {
            let vec = question_list.as_vec();
            let choice = vec.get(num as usize);
            let ptr = choise_fn(&choice);

            self.tasks.push(ptr)
        } else {
            self.tasks.push(App::greeting)
        }
    }

    fn fields(&mut self, map: &mut HashMap<&'a str, Option<String>>) -> Option<io::Error> {
        let mut keys = map.keys().cloned().collect::<Vec<&str>>();

        // While keys is not empty
        while let Some(key) = keys.get(0) {
            println!("{key}");

            match user_interface::input() {
                Ok(input) => {
                    map.insert(key, Some(input));
                    keys.remove(0);
                }
                Err(error) => return Some(error),
            }
        }

        None
    }
}

// Tabs
impl<'a> App<'a> {
    fn home(&mut self) {
        user_interface::clear_terminal();

        let title = format!("{}", "-- Notes --\n").yellow();
        println!("{title}");

        self.user_choice(QuestionsLists::Home, |&choice| match choice {
            Some(&Questions::AddNotebook) => App::create_notebook,
            Some(&Questions::RmNotebook) => App::rm_notebook,
            Some(&Questions::Exit) => App::stop,
            _ => App::greeting,
        })
    }

    fn create_notebook(&mut self) {
        user_interface::clear_terminal();

        let colored_str = format!("{}", "Notebook name:").yellow();
        println!("{}", colored_str);

        match user_interface::input() {
            Ok(name) => {
                let name = name.trim().to_string();
                let notebook = Notebook::new(name);

                self.data.insert(notebook.name.clone(), notebook);
                self.tasks.push(App::notebook_menu);
            }
            Err(error) => {
                eprintln!("{}", format!("Unknown error: {}", error));
                self.tasks.push(App::home);
            }
        }
    }

    fn notebook_menu(&mut self) {
        user_interface::clear_terminal();
        self.user_choice(QuestionsLists::NotebookMenu, |&choice| match choice {
            Some(&Questions::AddNote) => App::add_note,
            Some(&Questions::Back) => App::home,
            _ => App::notebook_menu,
        })
    }

    fn add_note(&mut self) {
        user_interface::clear_terminal();
    }

    fn rm_notebook(&mut self) {}
}
