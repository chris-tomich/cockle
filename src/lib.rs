use std::{collections::HashMap};

#[derive(Debug)]
pub enum Action<'a> {
    Unknown(String),
    Incorrect(String, &'a Verb<'a>),
    BadParameter(String, &'a Command),
    Run(Vec<ParameterValue<'a>>),
    Help(Vec<ParameterValue<'a>>),
    Exit,
}

pub trait Informational {
    fn get_help(&self) -> &Manual;
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Command {
    name: String,
    parameters: Vec<Parameter>,
    parameters_by_short_name: HashMap<char, usize>,
    parameters_by_long_name: HashMap<String, usize>,
}

impl Command {
    pub fn new(name: &str, parameters: Vec<Parameter>) -> Command {
        let parameters_ref = &parameters;
        let parameters_by_short_name = parameters_ref.into_iter().enumerate().map(|(i, x)|(x.short_name, i)).collect::<HashMap<char, usize>>();
        let parameters_by_long_name = parameters_ref.into_iter().enumerate().map(|(i, x)|(x.long_name.clone(), i)).collect::<HashMap<String, usize>>();

        Command {
            name: name.to_owned(),
            parameters,
            parameters_by_short_name,
            parameters_by_long_name,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn parse(&self, parameters: &str) -> Action {
        let tokens = parameters.split_whitespace();

        let mut parameter_values = Vec::new();

        for token in tokens {
            if token.starts_with("--") {
                let parameter_type_token = token.trim_matches('-');

                if let Some(parameter_type_index) = self.parameters_by_long_name.get(parameter_type_token) {
                    if let Some(parameter_type) = self.parameters.get(*parameter_type_index) {
                        parameter_values.push(ParameterValue::new(parameter_type))
                    }
                }
            }
            else if token.starts_with("-") {
                let parameter_type_token = token.trim_matches('-');

                if parameter_type_token.len() > 1 {
                    return Action::BadParameter(parameter_type_token.to_owned(), self);
                }

                if let Some(first_char) = parameter_type_token.chars().next() {
                    if let Some(parameter_type_index) = self.parameters_by_short_name.get(&first_char) {
                        if let Some(parameter_type) = self.parameters.get(*parameter_type_index) {
                            parameter_values.push(ParameterValue::new(parameter_type))
                        }
                    }
                }
            }
            else {
                if let Some(parameter_value) = parameter_values.last_mut() {
                    parameter_value.values.push(token.to_owned());
                }
            }
        }

        Action::Run(parameter_values)
    }
}

#[derive(Debug)]
pub struct Parameter {
    short_name: char,
    long_name: String,
}

impl Parameter {
    pub fn new(short_name: char, long_name: &str) -> Parameter {
        Parameter {
            short_name,
            long_name: long_name.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct ParameterValue<'a> {
    pub parameter_type: &'a Parameter,
    pub values: Vec<String>,
}

impl<'a> ParameterValue<'a> {
    pub fn new(parameter_type: &'a Parameter) -> ParameterValue<'a> {
        ParameterValue {
            parameter_type,
            values: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, Command, Manual, Parameter, Parser, Verb};

    #[test]
    fn parse_command_with_one_parameter_short_name() {
        let parser = Parser::new(vec![
            Verb::new(
                "list",
                None,
                Some(
                    vec![
                        Command::new(
                            "table",
                            vec![
                                Parameter::new('i', "name"),
                            ],
                        ),
                    ],
                ),
                Manual::new(
                    "list all the elements",
                    vec![
                        "",
                    ]
                )
            ),
        ]);

        let action = parser.parse("list table -i my_table_name".to_string());

        if let Action::Run(parameter_value) = action {
            assert_eq!('i', parameter_value.get(0).unwrap().parameter_type.short_name);
            assert_eq!("name", parameter_value.get(0).unwrap().parameter_type.long_name);
            assert_eq!("my_table_name", parameter_value.get(0).unwrap().values.get(0).unwrap());
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn parse_command_with_multiple_parameters() {
        let parser = Parser::new(vec![
            Verb::new(
                "list",
                None,
                Some(
                    vec![
                        Command::new(
                            "table",
                            vec![
                                Parameter::new('i', "name"),
                                Parameter::new('n', "count"),
                            ],
                        ),
                    ],
                ),
                Manual::new(
                    "list all the elements",
                    vec![
                        "",
                    ]
                )
            ),
        ]);

        let action = parser.parse("list table -i my_table_name -n 10".to_string());
        
        if let Action::Run(parameter_value) = action {
            assert_eq!('i', parameter_value.get(0).unwrap().parameter_type.short_name);
            assert_eq!("name", parameter_value.get(0).unwrap().parameter_type.long_name);
            assert_eq!("my_table_name", parameter_value.get(0).unwrap().values.get(0).unwrap());

            assert_eq!('n', parameter_value.get(1).unwrap().parameter_type.short_name);
            assert_eq!("count", parameter_value.get(1).unwrap().parameter_type.long_name);
            assert_eq!("10", parameter_value.get(1).unwrap().values.get(0).unwrap());
        }
        else {
            assert!(false);
        }
    }
}
