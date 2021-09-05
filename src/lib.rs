use std::{collections::HashMap, marker::PhantomData};

pub enum Action {
    Unknown(String),
    Run(Command),
    Help(Command),
    Exit,
}

pub trait Informational {
    fn get_help(&self) -> &Manual;
}

pub struct Manual {
    short_description: &'static str,
    detailed_help: Vec<&'static str>,
}

pub struct Runtime {}

pub struct Parser {
    verbs: HashMap<String, Verb>,
}

impl Parser {
    pub fn new(verbs: Vec<Verb>) -> Parser {
        let mut verbs_map = HashMap::with_capacity(verbs.len());

        for verb in verbs {
            verbs_map.insert(verb.name().clone(), verb);
        }

        Parser {
            verbs: verbs_map,
        }
    }

    pub fn parse(&self, input: String) -> Action {
        let (verb_name, remaining_commands, matching_verb) = match input.split_once(" ") {
            Some((verb_name, remaining_commands)) => {
                (verb_name.to_owned(), remaining_commands, self.verbs.get(verb_name))
            },
            None => {
                (input.clone(), "", self.verbs.get(&input))
            },
        };

        let action = match matching_verb {
            Some(verb) => {
                verb.parse(remaining_commands)
            },
            None => Action::Unknown(verb_name),
        };

        action
    }
}

pub struct Verb {
    name: String,
    verbs: HashMap<String, Verb>,
    commands: HashMap<String, Command>,
    manual: Box<Manual>,
}

impl Verb {
    pub fn new(name: &str, verbs: Option<Vec<Verb>>, commands: Option<Vec<Command>>, manual: Box<Manual>) -> Verb {
        let mut verbs_map = HashMap::new();

        if let Some(verbs) = verbs {
            for verb in verbs {
                verbs_map.insert(verb.name().clone(), verb);
            }
        }

        let mut commands_map = HashMap::new();

        if let Some(commands) = commands {
            // TODO: Need to check if a verb with the same name already exists.
            for command in commands {
                commands_map.insert(command.name().clone(), command);
            }
        }

        Verb {
            name: name.to_owned(),
            verbs: verbs_map,
            commands: commands_map,
            manual,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn parse(&self, input: &str) -> Action {
        let (command_name, remaining_commands) = match input.split_once(" ") {
            Some((command_name, remaining_commands)) => (command_name.to_owned(), remaining_commands),
            None => (input.to_owned(), ""),
        };

        if self.verbs.contains_key(command_name) {

        }
        else if self.commands.contains_key(command_name) {

        }

        todo!()
    }
}

impl Informational for Verb {
    fn get_help(&self) -> &Manual {
        self.manual.as_ref()
    }
}

pub struct Command {
    name: String,
    parameters: HashMap<u8, Parameter>,
}

impl Command {
    pub fn name(&self) -> &String {
        &self.name
    }
}

pub struct Parameter {
    short_name: char,
    long_name: String,
}

#[cfg(test)]
mod tests {
    use crate::{Parser, Verb};

    #[test]
    fn parse_line() {
        let parser = Parser::new(vec![
            Verb::new(
                "exit",
                None,
                None,
                "exits the tool",
                vec![
                    "exits the tool"
                ],
            ),
        ]);
    }
}
