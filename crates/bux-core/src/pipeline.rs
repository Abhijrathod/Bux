//! Pipeline execution (for pipes like cmd1 | cmd2)

use crate::parser::Token;

/// A single stage in a command pipeline.
/// Example: `ls | grep txt` → 2 stages (`ls`, `grep txt`)
#[derive(Debug, Clone)]
pub struct PipelineStage {
    pub command: String,
    pub args: Vec<String>,
}

/// Parse tokens into pipeline stages.
/// Splits the input tokens into separate command segments divided by `|`.
pub fn parse_pipeline(tokens: &[Token]) -> Vec<PipelineStage> {
    let mut stages = Vec::new();
    let mut current_stage = PipelineStage {
        command: String::new(),
        args: Vec::new(),
    };

    for token in tokens {
        match token {
            Token::Word(word) => {
                if current_stage.command.is_empty() {
                    current_stage.command = word.clone();
                } else {
                    current_stage.args.push(word.clone());
                }
            }
            Token::Pipe => {
                if !current_stage.command.is_empty() {
                    stages.push(current_stage);
                    current_stage = PipelineStage {
                        command: String::new(),
                        args: Vec::new(),
                    };
                }
            }
            _ => {} // Ignore unsupported tokens for now
        }
    }

    if !current_stage.command.is_empty() {
        stages.push(current_stage);
    }

    stages
}
