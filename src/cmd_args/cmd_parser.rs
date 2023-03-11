pub struct ArgParse {
    command: String,
    option: String,
    file_name: Option<String>,
}

impl ArgParse {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() > 2 {
            return Err("Too few arguments");
        } else if args.len() > 3 {
            return Err("Too many arguments");
        }
        let command = args[1].to_owned();
        let option = args[2].to_owned();
        let file_name = args[3].to_owned();
        Ok(ArgParse {
            command,
            option,
            file_name: Some(file_name),
        })
    }
}
