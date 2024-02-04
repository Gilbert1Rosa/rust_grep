pub mod main {
    use std::error::Error;
    use std::fs;

    pub fn read_file(file_path: &String) -> String {
        return fs::read_to_string(file_path).expect("DeezNuts");
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let file_content = read_file(&(config.file_path));

        for line in search(&config.query, &file_content) {
            println!("{line}");
        }

        Ok(())
    }

    pub struct Config {
        query: String,
        file_path: String,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, &'static str> {

            if args.len() < 3 {
                return Err("Not enough arguments Bitch");
            }

            let query = args[1].clone();
            let file_path = args[2].clone();

            Ok(Config { query, file_path })
        }
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut found = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                found.push(line);
            }
        }

        return found;
    }
}