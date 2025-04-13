//! Calculator tool for mathematical expressions

mod error;
#[cfg(test)]
mod tests;

use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::prelude::*;
pub(super) use error::CalculatorError;

use super::ToolError;

/// Calculator tool for evaluating mathematical expressions
#[derive(Debug, Clone)]
pub struct Calculator;

impl Calculator {
    pub async fn execute(&self, params: CalculatorParams) -> Result<f64> {
        meval::eval_str(&params.expression)
            .map_err(CalculatorError::from)
            .map_err(ToolError::from)
            .map_err(Error::Tool)
    }
}

/// Parameters for the calculator tool
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CalculatorParams {
    /// The mathematical expression to evaluate
    pub expression: String,
}

impl Tool for Calculator {
    const NAME: &'static str = "calculator";

    type Error = CalculatorError;
    type Args = CalculatorParams;
    type Output = f64;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "A calculator that can solve math problems, evaluate arithmetic expressions, \
            perform calculations, and handle mathematical operations. \
            Examples: '2+2', 'sin(0.5)*5', 'sqrt(16)', '(7*8)/2'"
                    .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "The mathematical expression to evaluate",
                    },
                }
            }),
        }
    }
    async fn call(&self, args: Self::Args) -> std::result::Result<Self::Output, Self::Error> {
        meval::eval_str(&args.expression).map_err(CalculatorError::from)
    }
}

impl ToolEmbedding for Calculator {
    type InitError = CalculatorError;
    type Context = CalculatorParams;
    type State = ();

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "A calculator that can solve math problems, evaluate arithmetic expressions, \
            perform calculations, and handle mathematical operations. \
            Examples: '2+2', 'sin(0.5)*5', 'sqrt(16)', '(7*8)/2'"
                .to_string(),
        ]
    }

    fn context(&self) -> Self::Context {
        CalculatorParams {
            expression: "2+2".to_string(),
        }
    }
    fn init(
        _state: Self::State,
        _context: Self::Context,
    ) -> std::result::Result<Self, Self::InitError> {
        Ok(Self)
    }
}
