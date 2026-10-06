
enum Error {
    IoError(std::io::Error),
    InvalidPath(String),
    Other(String),
}