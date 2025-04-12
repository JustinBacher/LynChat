use std::future::Future;

use ollama_rs::generation::tools::Tool as OllamaRsTool;
use schemars::JsonSchema; // Added JsonSchema
use serde::Deserialize;

use crate::prelude::*;

async fn calculate_expression(expression: String) -> std::result::Result<String, Box<dyn std::error::Error + Send + Sync>> {
    info!("Calculating expression: {}", expression);
    meval::eval_str(&expression).map(|r| r.to_string()).map_err(|e| {
        error!("Calculation error: {}", e);
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
    })
}

#[derive(Deserialize, Debug, JsonSchema)] // Added JsonSchema derive
pub struct CalculatorParams {
    expression: String,
}

#[derive(Debug, Clone, Default)]
pub struct Calculator;

impl OllamaRsTool for Calculator {
    type Params = CalculatorParams;

    fn name() -> &'static str {
        "calculator"
    }

    fn description() -> &'static str {
        "Performs mathematical calculations. Takes a single string argument 'expression' containing the expression to evaluate (e.g., '2 + 2 * 3')."
    }

    fn call(
        &mut self,
        paramaters: Self::Params,
    ) -> impl Future<Output = std::result::Result<String, Box<dyn std::error::Error + Send + Sync>>> {
        // Call the actual calculation logic
        calculate_expression(paramaters.expression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_calculator_call_simple_addition() {
        let mut tool = Calculator;
        let params = CalculatorParams { expression: "2 + 2".to_string() };
        let result = tool.call(params).await.unwrap();
        assert_eq!(result, "4");
    }

    #[async_std::test]
    async fn test_calculator_call_complex_expression() {
        let mut tool = Calculator;
        let params = CalculatorParams { expression: "(3 * 4) / 2 + 5 - 1".to_string() };
        let result = tool.call(params).await.unwrap();
        assert_eq!(result, "10");
    }

     #[async_std::test]
    async fn test_calculator_call_with_floats() {
        let mut tool = Calculator;
        let params = CalculatorParams { expression: "1.5 * 2.0".to_string() };
        let result = tool.call(params).await.unwrap();
        assert!(result == "3" || result == "3.0");
    }

    #[async_std::test]
    async fn test_calculator_call_invalid_expression() {
        let mut tool = Calculator;
        let params = CalculatorParams { expression: "2 +".to_string() };
        let result = tool.call(params).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<meval::Error>());
    }

    #[async_std::test]
    async fn test_calculator_call_division_by_zero() {
        let mut tool = Calculator;
        let params = CalculatorParams { expression: "1 / 0".to_string() };
        let result = tool.call(params).await;
        assert!(result.is_err());
         assert!(result.unwrap_err().is::<meval::Error>());
    }

    // Test name and description (reverted to methods)
    #[test]
    fn test_calculator_static_info() {
        assert_eq!(Calculator::name(), "calculator");
        assert_eq!(Calculator::description(), "Performs mathematical calculations. Takes a single string argument 'expression' containing the expression to evaluate (e.g., '2 + 2 * 3').");
    }
}
