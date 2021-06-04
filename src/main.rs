use structopt::StructOpt;

fn read_from_stdin() -> Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;
    Ok(buf)
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "l", long = "lower")]
    lower: bool,

    #[structopt(short = "u", long = "upper")]
    upper: bool,

    #[structopt(short = "p", long = "pascal")]
    pascal: bool,

    #[structopt(short = "c", long = "camel")]
    camel: bool,

    #[structopt(short = "s", long = "snake")]
    snake: bool,

    #[structopt(short = "ss", long = "screaming-snake")]
    screamingSnake: bool,

    #[structopt(short = "k", long = "kebab")]
    kebab: bool,

    #[structopt(short = "sk", long = "screaming-kebab")]
    screamingKebab: bool,

    #[structopt(name = "INPUT")]
    input: Option<String>,
}

pub fn apply_to_variant(&self, variant: &str) -> String {
    match *self {
        None | PascalCase => variant.to_owned(),
        LowerCase => variant.to_ascii_lowercase(),
        UpperCase => variant.to_ascii_uppercase(),
        CamelCase => variant[..1].to_ascii_lowercase() + &variant[1..],
        SnakeCase => {
            let mut snake = String::new();
            for (i, ch) in variant.char_indices() {
                if i > 0 && ch.is_uppercase() {
                    snake.push('_');
                }
                snake.push(ch.to_ascii_lowercase());
            }
            snake
        }
        ScreamingSnakeCase => SnakeCase.apply_to_variant(variant).to_ascii_uppercase(),
        KebabCase => SnakeCase.apply_to_variant(variant).replace('_', "-"),
        ScreamingKebabCase => ScreamingSnakeCase
            .apply_to_variant(variant)
            .replace('_', "-"),
    }
}

fn main() -> Result<()> {
    let opt = Cli::from_args();
    let input = match opt.input {
        Some(i) => i,
        None => read_from_stdin()?
    };
    if input.is_empty() {
        Opt::clap().get_matches().usage();
    }
}
