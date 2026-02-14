use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Configuration error: {message}"))]
    Config { message: String },

    #[snafu(display("Theme error: {message}"))]
    Theme { message: String },

    #[snafu(display("Render error: {message}"))]
    Render { message: String },
}
