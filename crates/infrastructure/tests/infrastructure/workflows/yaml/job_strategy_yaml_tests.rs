use ephact::{
    domain::value_objects::{ContextValue, JobStrategy},
    infrastructure::workflows::yaml::JobStrategyYaml,
};

fn strategy_from(yaml: &str) -> JobStrategy {
    serde_yaml::from_str::<JobStrategyYaml>(yaml)
        .unwrap()
        .into_domain()
}

#[test]
fn matrix_variables_become_context_values() {
    let yaml = "matrix:\n  os: [ubuntu-latest, macos-latest]\n  rust: [stable, nightly]\nfail-fast: false\nmax-parallel: 2\n";

    let strategy = strategy_from(yaml);

    assert!(!strategy.fail_fast());
    assert_eq!(strategy.max_parallel(), Some(2));
    assert_eq!(strategy.combination_count(), 4);
    assert_eq!(
        strategy.matrix().unwrap().variables()["os"][0],
        ContextValue::text("ubuntu-latest")
    );
}

#[test]
fn include_and_exclude_combinations_are_kept_apart_from_variables() {
    let yaml = "matrix:\n  os: [ubuntu-latest]\n  include:\n    - os: macos-latest\n      rust: nightly\n  exclude:\n    - os: ubuntu-latest\n      rust: stable\n";

    let matrix = strategy_from(yaml).matrix().unwrap().clone();

    assert_eq!(matrix.include().len(), 1);
    assert_eq!(matrix.exclude().len(), 1);
    assert_eq!(
        matrix.include()[0].get("os"),
        Some(&ContextValue::text("macos-latest"))
    );
    assert_eq!(matrix.variables().len(), 1);
}

#[test]
fn fail_fast_defaults_to_true_and_max_parallel_to_none() {
    let strategy = strategy_from("matrix:\n  os: [ubuntu-latest]\n");

    assert!(strategy.fail_fast());
    assert_eq!(strategy.max_parallel(), None);
}

#[test]
fn an_empty_matrix_yields_no_combinations() {
    assert_eq!(strategy_from("matrix: {}\n").combination_count(), 0);
}

#[test]
fn non_string_matrix_values_keep_their_scalar_type() {
    let strategy = strategy_from("matrix:\n  node: [18, 20.5, true]\n");

    let values = strategy.matrix().unwrap().variables()["node"].clone();
    assert_eq!(
        values,
        vec![
            ContextValue::Integer(18),
            ContextValue::Decimal(20.5),
            ContextValue::Boolean(true)
        ]
    );
}
