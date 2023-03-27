use std::any::Any;
use std::collections::HashMap;

pub struct CommandHandler {
    pub prefix: &'static str,
    pub commands: HashMap<&'static str, Command>,
    pub ordered_commands: Vec<Command>,
}

impl CommandHandler {
    /// Creates a command handler with a specific command prefix.
    pub fn new(prefix: &'static str) -> Self {
        Self {
            prefix,
            commands: HashMap::new(),
            ordered_commands: Vec::new(),
        }
    }

    pub fn set_prefix(&mut self, prefix: &'static str) {
        self.prefix = prefix;
    }

    pub fn get_prefix(&self) -> &'static str {
        self.prefix
    }

    /// Handles a message with optional extra parameters. Runs the command if successful.
    /// * return a response detailing whether the command was handled, and what went wrong, if applicable.
    pub fn handle_message(
        &self,
        mut message: Option<String>,
        params: Option<Vec<dyn Any>>,
    ) -> CommandResponse {
        if message.is_none() || (!message.unwrap().starts_with(self.prefix)) {
            return CommandResponse::new(ResponseType::NoCommand, None);
        }

        let message = message.unwrap().replace(self.prefix, "");

        let commandstr = if message.contains(" ") {
            message.split(" ").collect::<Vec<&str>>()[0]
        } else {
            message.as_str()
        };
        let mut argstr = if message.contains(" ") {
            message.split(" ").collect::<Vec<&str>>()[1..].join(" ")
        } else {
            "".to_string()
        };

        let mut result = vec![];

        let command = self.commands.get(commandstr);

        if command.is_none() {
            return CommandResponse::new(ResponseType::UnknownCommand, None);
        }
        let command = command.unwrap();
        let mut index = 0;
        let mut satisfied = false;

        loop {
            if index >= command.params.len() && !argstr.is_empty() {
                return CommandResponse {
                    ty: ResponseType::ManyArguments,
                    command: Some(command),
                    run_command: Some(commandstr),
                };
            } else if argstr.is_empty() {
                break;
            }

            if command.params[index].optional
                || index >= command.params.len() - 1
                || command.params[index + 1].optional
            {
                satisfied = true;
            }

            if command.params[index].variadic {
                result.push(argstr);
                break;
            }

            let next = argstr.find(" ");
            if !next.is_none() {
                if !satisfied {
                    return CommandResponse {
                        ty: ResponseType::FewArguments,
                        command: Some(command),
                        run_command: Some(commandstr),
                    };
                }
                result.push(argstr);
                break;
            } else {
                let arg = argstr[..next.unwrap()].to_string();
                argstr = argstr[next.unwrap() + 1..].to_string();
                result.push(arg);
            }

            index += 1;
        }

        if !satisfied && command.params.len() > 0 && !command.params[0].optional {
            return CommandResponse {
                ty: ResponseType::FewArguments,
                command: Some(command),
                run_command: Some(commandstr),
            };
        }

        // finally run the command
        command.runner.accept(result, params);

        CommandResponse {
            ty: ResponseType::Valid,
            command: Some(command),
            run_command: Some(commandstr),
        }
    }
}

pub enum ResponseType {
    NoCommand,
    UnknownCommand,
    FewArguments,
    ManyArguments,
    Valid,
}

pub struct CommandResponse<'a> {
    pub ty: ResponseType,
    pub command: Option<&'a Command>,
    pub run_command: Option<&'static str>,
}

impl<'a> CommandResponse<'a> {
    pub fn new(ty: ResponseType, command: Option<&'a Command>) -> Self {
        Self {
            ty,
            command,
            run_command: None,
        }
    }
}

impl<'a> Default for CommandResponse<'a> {
    fn default() -> Self {
        Self {
            ty: ResponseType::NoCommand,
            command: None,
            run_command: None,
        }
    }
}

// todo: fix this part, cause currently its a hack to hide the T generic
pub trait Entity {}

impl Entity for bool {}

trait CR<T: Entity> {
    fn accept(&self, args: Vec<String>, parameter: T);
}

pub trait CommandRunner {
    fn accept<T: Entity>(&self, args: Vec<String>, parameter: T);
}

impl<T: Entity> CommandRunner for dyn CR<T> {
    fn accept<K: Entity>(&self, args: Vec<String>, parameter: K) {
        self.accept(args, parameter);
    }
}

pub fn create_command_runner<T: Entity>(
    runner: Box<dyn Fn(Vec<String>, T)>,
) -> Box<dyn CommandRunner> {
    struct C<K: Entity> {
        runner: Box<dyn Fn(Vec<String>, K)>,
    }
    impl<K: Entity> CommandRunner for C<K> {
        fn accept<T: Entity>(&self, args: Vec<String>, parameter: T) {
            self.accept(args, parameter);
        }
    }
    impl<K: Entity> CR<K> for C<K> {
        fn accept(&self, args: Vec<String>, parameter: K) {
            (self.runner)(args, parameter);
        }
    }
    Box::new(C { runner })
}

pub struct CommandParam {
    pub name: &'static str,
    pub optional: bool,
    pub variadic: bool,
}

impl Default for CommandParam {
    fn default() -> Self {
        Self {
            name: "",
            optional: false,
            variadic: false,
        }
    }
}

pub struct Command {
    pub name: &'static str,
    pub param_text: &'static str,
    pub description: &'static str,
    pub params: Vec<CommandParam>,
    pub runner: Box<dyn CommandRunner>,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            name: "",
            param_text: "",
            description: "",
            params: Vec::new(),
            runner: create_command_runner(Box::new(|_, _: bool| {})),
        }
    }
}

impl Command {
    pub fn new(
        name: &'static str,
        param_text: &'static str,
        description: &'static str,
        runner: Box<dyn CommandRunner>,
    ) -> Self {
        let mut c = Self {
            name,
            param_text,
            description,
            params: Vec::new(),
            runner,
        };
        let psplit = param_text.split(" ").collect::<Vec<&str>>();
        if param_text.len() == 0 {
            return c;
        } else {
            c.params = vec![CommandParam::default(); psplit.count()];

            let mut had_optional = false;

            for i in 0..c.params.len() {
                let param = psplit[i];

                if param.len() <= 2 {
                    panic!("Invalid parameter text: {}", param_text);
                }

                let l = param.chars().nth(0).unwrap();
                let r = param.chars().nth(param.len() - 1).unwrap();
                let mut optional = false;
                let mut variadic = false;

                if l == '<' && r == '>' {
                    if had_optional {
                        panic!("Can't have non-optional param after optional param!");
                    }
                    optional = false;
                } else if l == '[' && r == ']' {
                    optional = true;
                } else {
                    panic!("Invalid parameter text: {}", param_text);
                }

                if optional {
                    had_optional = true;
                }

                let mut fname = param[1..param.len() - 1].to_string();
                if fname.ends_with("...") {
                    if i != param.len() - 1 {
                        panic!("Variadic parameter must be last!");
                    }
                    fname = fname[0..fname.len() - 3].to_string();
                    variadic = true;
                }

                c.params[i] = CommandParam {
                    name: &*fname,
                    optional,
                    variadic,
                };
            }
        }
        c
    }
}
