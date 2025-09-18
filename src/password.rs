#[derive(Clone)]
pub struct PwArgs {
    pub pattern: Option<String>,
    pub length: usize,
    pub format: bool,
}

impl PwArgs {
    pub fn from_cli(cli: &crate::cli::PwGenCli) -> Self {
        PwArgs {
            pattern: cli.pattern.clone().map(|v| v.to_lowercase()),
            length: cli.len.unwrap_or(9),
            format: cli.format.unwrap_or(false),
        }
    }
}