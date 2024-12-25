#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;
    // Enable wasm-bindgen-test for this module
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_math_function() {
        assert_eq!(2 + 2, 4);
    }

    #[wasm_bindgen_test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[wasm_bindgen_test]
    async fn test_async_behavior() {
        let result = async_function().await;
        assert_eq!(result, "Success");
    }

    async fn async_function() -> &'static str {
        "Success"
    }
}
