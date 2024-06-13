use pkl::PklSerialize;

pub mod api;
pub mod pkl;
pub mod types;
// #[macro_export]
// macro_rules! include_pkl {
//     ($package: tt) => {
//         include!(concat!(env!("OUT_DIR"), concat!("/", $package, ".rs")));
//     };
// }

/// Evaluates a `.pkl` file and interprets it as `T`
///
/// `path`: The path to the `.pkl` file
///
/// # Example
///
/// ```pkl
/// ip = "127.0.0.1"
/// database {
///     username = "root"
///     password = "password"
/// }
/// ```
/// -------------
/// ```rust
///
/// #[derive(Deserialize)]
/// struct Config {
///     ip: String,
///     database: Database,
/// }
///
/// #[derive(Deserialize)]
/// struct Database {
///     username: String,
///     password: String,
/// }
///
/// let config: Database = pkl_rs::value_from_config("config.pkl")?;
/// ```
pub fn value_from_config<T>(path: impl AsRef<std::path::Path>) -> anyhow::Result<String>
where
    T: Sized + for<'de> serde::Deserialize<'de>,
{
    {
        let mut evaluator = api::Evaluator::new()?;
        let pkl_mod = evaluator.evaluate_module(path.as_ref().to_path_buf())?;
        // let json = pkl_mod.serialize_json()?;
        let pkld = pkl_mod.serialize_pkl()?;
        println!("{:?}", pkld);
        // let v: T = serde_json::from_value(serde_json::Value::Object(json))?;
        // let v = serde_
        Ok(String::new())
    }
}
