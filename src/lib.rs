use std::{collections::HashMap, marker::PhantomData};

pub enum Action<'a> {
    Unknown(String),
    Incorrect(String, &'a Verb<'a>),
    Run(Box<dyn CommandFunc<'a>>),
    Help(Box<dyn CommandFunc<'a>>),
    Exit,
}

pub trait CommandFunc<'a> {
    fn execute(&self, parameterValues: Vec<ParameterValue<'a>>);
}

pub trait Informational {
    fn get_help(&self) -> &Manual;
}

pub struct Manual<'a> {
    short_description: &'a str,
    detailed_help: Vec<&'a str>,
}

impl<'a> Manual<'a> {
    pub fn new(short_description: &'a str, detailed_help: Vec<&'a str>) -> Manual<'a> {
        Manual {
            short_description,
            detailed_help,
        }
    }
}

pub struct Runtime {}

pub struct Parser<'a> {
    verbs: HashMap<String, Verb<'a>>,
}

impl<'a> Parser<'a> {
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

pub struct Verb<'a> {
    name: String,
    verbs: HashMap<String, Verb<'a>>,
    commands: HashMap<String, Command>,
    manual: Manual<'a>,
}

impl<'a> Verb<'a> {
    pub fn new(name: &str, verbs: Option<Vec<Verb<'a>>>, commands: Option<Vec<Command>>, manual: Manual<'a>) -> Verb<'a> {
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

        if self.verbs.contains_key(&command_name) {
            let command_verb = self.verbs.get(&command_name).expect(format!("Expected there would be a verb named '{}' but couldn't find it.", command_name).as_str());

            return command_verb.parse(remaining_commands);
        }
        else if self.commands.contains_key(&command_name) {
            let command = self.commands.get(&command_name).expect(format!("Expected there would be a command named '{}' but couldn't find it.", command_name).as_str());

            return command.parse(remaining_commands);
        }

        Action::Incorrect(input.to_owned(), self)
    }
}

impl<'a> Informational for Verb<'a> {
    fn get_help(&self) -> &Manual {
        &self.manual
    }
}

pub struct Command {
    name: String,
    parameters: Vec<Parameter>,
    parameters_by_short_name: HashMap<String, usize>,
    parameters_by_long_name: HashMap<String, usize>,
}

impl Command {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn parse(&self, parameters: &str) -> Action {
        let tokens = parameters.split_whitespace();

        let mut parameter_values = Vec::new();
        let mut parameter_value = None as Option<ParameterValue>;

        for token in tokens {
            let token_parameter_type_index = {
                if token.starts_with("--") {
                    let parameter_name = token.trim_matches('-');

                    match self.parameters_by_long_name.get(parameter_name) {
                        Some(parameter_type_index) => Some(*parameter_type_index),
                        None => None,
                    }
                }
                else if token.starts_with("-") {
                    let parameter_name = token.trim_matches('-');

                    match self.parameters_by_short_name.get(parameter_name) {
                        Some(parameter_type_index) => Some(*parameter_type_index),
                        None => None,
                    }
                }
                else {
                    None
                }
            };

            parameter_value = match token_parameter_type_index {
                Some(parameter_type_index) => {
                    match self.parameters.get(parameter_type_index) {
                        Some(parameter_type) => {
                            if let Some(parameter_value) = parameter_value {
                                parameter_values.push(parameter_value);
                            }

                            Some(ParameterValue {
                                parameter_type,
                                values: Vec::new(),
                            })
                        },
                        None => return Action::Unknown(token.to_owned()),
                    }
                },
                None => parameter_value,
            };

            if let None = token_parameter_type_index {
                if let Some(parameter_value) = &mut parameter_value {
                    parameter_value.values.push(token.to_owned());
                }
            }
        }

        todo!()
    }
}

pub struct Parameter {
    short_name: char,
    long_name: String,
}

pub struct ParameterValue<'a> {
    parameter_type: &'a Parameter,
    values: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::{Manual, Parser, Verb};

    #[test]
    fn parse_line() {
        let parser = Parser::new(vec![
            Verb::new(
                "exit",
                None,
                None,
                Manual::new(
                    "exits the tool",
                    vec![
                        "exits the tool"
                    ],
                ),
            ),
        ]);
    }
}
