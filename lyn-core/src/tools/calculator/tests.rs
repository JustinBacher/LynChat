use super::*;

#[async_std::test]
async fn test_calculator_basic_operations() {
    let calculator = Calculator;

    // Test basic addition
    let params = CalculatorParams {
        expression: "2 + 2".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 4.0);

    // Test multiplication
    let params = CalculatorParams {
        expression: "6 * 7".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42.0);

    // Test division
    let params = CalculatorParams {
        expression: "100 / 4".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 25.0);

    // Test complex expression
    let params = CalculatorParams {
        expression: "(5 + 3) * 2 / 4".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 4.0);
}

#[async_std::test]
async fn test_calculator_advanced_functions() {
    let calculator = Calculator;

    // Test sine function
    let params = CalculatorParams {
        expression: "sin(pi/2)".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!((value - 1.0).abs() < 1e-10); // Should be very close to 1.0

    // Test cosine function
    let params = CalculatorParams {
        expression: "cos(pi)".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!((value + 1.0).abs() < 1e-10); // Should be very close to -1.0

    // Test square root
    let params = CalculatorParams {
        expression: "sqrt(16)".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 4.0);

    // Test power
    let params = CalculatorParams {
        expression: "3^2".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 9.0);
}

#[async_std::test]
async fn test_calculator_errors() {
    let calculator = Calculator;

    // Test syntax error
    let params = CalculatorParams {
        expression: "2 +".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_err());

    // Test division by zero
    let params = CalculatorParams {
        expression: "1/0".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_err());

    // Test invalid function
    let params = CalculatorParams {
        expression: "invalid_func(2)".to_string(),
    };
    let result = calculator.execute(params).await;
    assert!(result.is_err());
}

#[async_std::test]
async fn test_calculator_tool_trait() {
    let calculator = Calculator;

    // Test definition method
    let definition = calculator.definition("".to_string()).await;
    assert_eq!(definition.name, "calculator");
    assert!(!definition.description.is_empty());

    // Test call method
    let args = CalculatorParams {
        expression: "5 * 5".to_string(),
    };
    let result = calculator.call(args).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 25.0);
}

#[async_std::test]
async fn test_calculator_embedding_trait() {
    let calculator = Calculator;

    // Test embedding_docs method
    let docs = calculator.embedding_docs();
    assert!(!docs.is_empty());

    // Test init method
    let result = Calculator::init((), calculator.context());
    assert!(result.is_ok());
}
