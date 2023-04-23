#[derive(Debug, Clone, Copy)]
pub enum Error {
    ParamNotEnough,
    BindPipelineFailed,
    InvalidCStr,
    InvalidCommand,
    CommandExecuteFailed,
}
