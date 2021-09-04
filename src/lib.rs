use std::collections::HashMap;

pub enum Action<'a> {
    Run(&'a Command),
    Help(&'a Command),
    Exit,
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
}

pub struct Verb {
    name: String,
    verbs: HashMap<String, Verb>,
    commands: HashMap<String, Command>,
    short_description: String,
    detailed_help: Vec<String>,
}

impl Verb {
    pub fn new(name: &str, verbs: Option<Vec<Verb>>, commands: Option<Vec<Command>>, short_description: &str, detailed_help: Vec<&str>) -> Verb {
        let mut verbs_map = HashMap::new();

        if let Some(verbs) = verbs {
            for verb in verbs {
                verbs_map.insert(verb.name().clone(), verb);
            }
        }

        let mut commands_map = HashMap::new();

        if let Some(commands) = commands {
            for command in commands {
                commands_map.insert(command.name().clone(), command);
            }
        }

        let mut owned_detailed_help = Vec::with_capacity(detailed_help.len());

        for line in detailed_help {
            owned_detailed_help.push(line.to_owned());
        }

        Verb {
            name: name.to_owned(),
            verbs: verbs_map,
            commands: commands_map,
            short_description: short_description.to_owned(),
            detailed_help: owned_detailed_help,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
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
