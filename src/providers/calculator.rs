use crate::{ProviderResult, engine::Provider};


#[derive(Debug)]
pub struct CalculatorProvider {
}

impl CalculatorProvider {
    pub fn new() -> Self {
        CalculatorProvider { }
    }
}

impl Provider for CalculatorProvider  {

    fn search(&self, query: &str) -> Option<ProviderResult> {
        match meval::eval_str(query) {
            Ok(result) => Some(ProviderResult::Calculator(
                CalculatorResult { expression: query.to_string(), result: Some(result.to_string()) }
            )),
            Err(_) => Some(ProviderResult::Calculator(
                CalculatorResult { expression: query.to_string(), result: None }
            )),
        }
    }

    fn init(&mut self) { }

}

#[derive(Debug, Clone)]
pub struct CalculatorResult {
    pub expression: String,
    pub result: Option<String>,
}
