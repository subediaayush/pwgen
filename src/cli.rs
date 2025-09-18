use clap::Parser;

#[derive(Parser)]
#[command(name = "MyApp", version = "1.0", about = "Generates random password for use")]
pub struct PwGenCli {
    #[arg(short, value_parser = validate_pattern)]
    pub pattern: Option<String>,
    #[arg(short)]
    pub len: Option<usize>,
    #[arg(short, action=clap::ArgAction::SetTrue)]
    pub format: Option<bool>,
    #[arg(short, action=clap::ArgAction::SetTrue)]
    pub copy: Option<bool>,
    #[arg(long, short, action=clap::ArgAction::SetTrue)]
    pub verbose: Option<bool>,
}

/// Validates that the pattern contains only allowed characters.
fn validate_pattern(s: &str) -> Result<String, String> {
    const ALLOWED_CHARS: &[char] = &['s', 'n', 'u', 'l', '(', ')'];
    for c in s.chars() {
        if !ALLOWED_CHARS.contains(&c.to_ascii_lowercase()) {
            return Err(format!(
                "Invalid character `{}` in pattern. Allowed characters: s, n, u, l, (, )",
                c
            ));
        }
    }
    Ok(s.to_string())
}
