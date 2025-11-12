//! bux-core: Core shell runtime, parser, and execution engine
//! Similar to System.Management.Automation in PowerShell

pub mod parser;
pub mod runtime;
pub mod executor;
pub mod pipeline;
pub mod value;

pub use parser::*;
pub use runtime::*;
pub use executor::*;
pub use pipeline::*;
pub use value::*;

