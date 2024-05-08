use std::{collections::HashMap, fmt::Display};

use clap::{value_parser, Arg, ArgAction, Command};

struct Project {
    name: String,
    exp: u64,
    rank: u8,
}

impl Project {
    fn new(name: &str, exp: u64, rank: u8) -> Project {
        Project {
            name: String::from(name),
            exp,
            rank,
        }
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: [{}] exp: [{}] rank: [{}]",
            self.name, self.exp, self.rank
        )
    }
}

fn load_projects() -> HashMap<String, Project> {
    let mut map: HashMap<String, Project> = HashMap::new();

    map.insert(String::from("libft"), Project::new("libft", 462, 0));
    map.insert(String::from("ft_printf"), Project::new("ft_printf", 882, 1));
    map.insert(
        String::from("Born2beroot"),
        Project::new("Born2beroot", 577, 1),
    );
    map.insert(
        String::from("get_next_line"),
        Project::new("get_next_line", 882, 1),
    );
    map.insert(String::from("minitalk"), Project::new("minitalk", 1142, 2));
    map.insert(String::from("pipex"), Project::new("pipex", 1142, 2));
    map.insert(
        String::from("push_swap"),
        Project::new("push_swap", 1855, 2),
    );
    map.insert(String::from("so_long"), Project::new("so_long", 1000, 2));
    map.insert(String::from("Fdf"), Project::new("Fdf", 1000, 2));
    map.insert(String::from("fract-ol"), Project::new("fract-ol", 1000, 2));
    map.insert(
        String::from("Philosophers"),
        Project::new("Philosophers", 3360, 3),
    );
    map.insert(
        String::from("minishell"),
        Project::new("minishell", 2814, 3),
    );
    map.insert(
        String::from("NetPractice"),
        Project::new("NetPractice", 3160, 4),
    );
    map.insert(String::from("cub3d"), Project::new("cub3d", 5775, 4));
    map.insert(String::from("miniRT"), Project::new("miniRT", 5775, 4));
    map.insert(String::from("cpp04"), Project::new("cpp04", 9660, 4));
    map.insert(
        String::from("inception"),
        Project::new("inception", 10042, 5),
    );
    map.insert(String::from("cpp09"), Project::new("cpp09", 10042, 5));
    map.insert(String::from("webserv"), Project::new("webserv", 21630, 5));
    map.insert(String::from("ft_irc"), Project::new("ft_irc", 21630, 5));
    map
}

fn main() {
    // define cli params
    let app = Command::new("gsaiago's bh_cli")
        .arg(
            Arg::new("initial_xp")
                .help("Your current xp")
                .required(true)
                .action(ArgAction::Set)
                .default_value("0")
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("projects")
                .help("The projects you want to add")
                .required(true)
                .action(ArgAction::Append),
        )
        .get_matches();

    // load map and args
    let project_map = load_projects();
    let initial_xp = app.get_one::<u64>("initial_xp").unwrap();
    let args = app
        .get_many::<String>("projects")
        .unwrap_or_default()
        .map(|item| -> String { String::from(item) })
        .collect::<Vec<String>>();

    // debub printing
    println!("Initial_xp value is: {:?}", initial_xp);
    println!("Args passed: {:?}", &args);
    println!("found projects:");
    for arg in args.iter() {
        if let Some(project) = project_map.get(arg) {
            println!("{}", project);
        }
    }
}
