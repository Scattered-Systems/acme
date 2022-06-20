fn main() {
    println!("Welcome to acme");

    let app = application::Application::constructor();

    for _ in 0..app.commands.count {
        println!("Hello {}!", app.commands.name)
    }
}

mod application {
    use clap::Parser;

    /// Simple program to greet a person
    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    pub struct Args {
        /// Name of the person to greet
        #[clap(short, long, value_parser, default_value = "World")]
        pub name: String,

        /// Number of times to greet
        #[clap(short, long, value_parser, default_value_t = 1)]
        pub count: u8,
    }

    pub struct Application {
        pub commands: Args,
    }

    impl Application {
        pub fn constructor() -> Self {
            Self {
                commands: Args::parse()
            }
        }
    }
}