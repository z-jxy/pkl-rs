#[cfg(test)]
mod tests {

    #[cfg(feature = "codegen")]
    use std::path::PathBuf;
    #[cfg(feature = "codegen")]
    use pkl_rs;

    #[test]
    #[cfg(feature = "codegen")]
    fn test_appconfig() {
        let mut evaluator = pkl_rs::api::evaluator::Evaluator::new().expect("failed to initialize evaluator");
        let pkl_mod = evaluator.evaluate_module(
            PathBuf::from("tests/codegen/pkl_files/local/appconfig.pkl")).expect("failed to evaluate");

        pkl_mod.codegen().expect("failed to codegen");
    }

    #[test]
    #[cfg(feature = "codegen")]
    fn test_nested_config() {
        let mut evaluator = pkl_rs::api::evaluator::Evaluator::new().expect("failed to initialize evaluator");
        let pkl_mod = evaluator.evaluate_module(
            PathBuf::from("tests/codegen/pkl_files/local/nesting.pkl")).expect("failed to evaluate");

        pkl_mod.codegen().expect("failed to codegen");
    }

    #[test]
    #[cfg(feature = "codegen")]
    fn test_primitives() {
        let mut evaluator = pkl_rs::api::evaluator::Evaluator::new().expect("failed to initialize evaluator");
        let pkl_mod = evaluator.evaluate_module(
            PathBuf::from("tests/codegen/pkl_files/local/primitives.pkl")).expect("failed to evaluate");

        pkl_mod.codegen().expect("failed to codegen");

    }

    #[test]
    #[cfg(feature = "codegen")]
    fn test_map() {
        let mut evaluator = pkl_rs::api::evaluator::Evaluator::new().expect("failed to initialize evaluator");
        let pkl_mod = evaluator.evaluate_module(
            PathBuf::from("tests/codegen/pkl_files/local/map.pkl")).expect("failed to evaluate");

        pkl_mod.codegen().expect("failed to codegen");

    }

    #[test]
    #[cfg(feature = "codegen")]
    fn test_set() {
        let mut evaluator = pkl_rs::api::evaluator::Evaluator::new().expect("failed to initialize evaluator");
        let pkl_mod = evaluator.evaluate_module(
            PathBuf::from("tests/codegen/pkl_files/local/set.pkl")).expect("failed to evaluate");

        pkl_mod.codegen().expect("failed to codegen");
    }

    #[test]
    #[cfg(feature = "codegen")]
    fn test_list() {
        let mut evaluator = pkl_rs::api::evaluator::Evaluator::new().expect("failed to initialize evaluator");
        let pkl_mod = evaluator.evaluate_module(
            PathBuf::from("tests/codegen/pkl_files/local/list.pkl")).expect("failed to evaluate");

        pkl_mod.codegen().expect("failed to codegen");
    }

    #[test]
    #[cfg(feature = "codegen")]
    fn test_listing() {
        let mut evaluator = pkl_rs::api::evaluator::Evaluator::new().expect("failed to initialize evaluator");
        let pkl_mod = evaluator.evaluate_module(
            PathBuf::from("tests/codegen/pkl_files/local/listing.pkl")).expect("failed to evaluate");

        pkl_mod.codegen().expect("failed to codegen");
    }
}
