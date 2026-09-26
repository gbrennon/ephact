use ephact::{
    domain::value_objects::{ActionDefinition, ActionRuntime},
    infrastructure::workflows::yaml::ActionDefinitionYaml,
};

fn action_from(yaml: &str) -> ActionDefinition {
    serde_yaml::from_str::<ActionDefinitionYaml>(yaml)
        .unwrap()
        .into_domain()
}

#[test]
fn a_composite_action_keeps_its_steps_in_order() {
    let yaml = "name: Test Action\ndescription: Does stuff\nruns:\n  using: composite\n  steps:\n    - run: echo hello\n      shell: bash\n    - name: Check\n      run: cargo test\n";

    let action = action_from(yaml);

    assert_eq!(action.name(), "Test Action");
    assert_eq!(action.description(), Some("Does stuff"));
    let ActionRuntime::Composite { steps } = action.runs() else {
        panic!("expected a composite runtime");
    };
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].run(), Some("echo hello"));
    assert_eq!(steps[1].name(), Some("Check"));
}

#[test]
fn action_inputs_keep_their_requirement_and_default() {
    let yaml = "name: With Inputs\ninputs:\n  path:\n    description: Files to cache\n    required: true\n  key:\n    description: Cache key\n    default: default-key\nruns:\n  using: composite\n  steps:\n    - run: echo done\n";

    let action = action_from(yaml);

    assert_eq!(action.inputs().len(), 2);
    assert!(action.inputs()["path"].required());
    assert_eq!(action.inputs()["key"].default(), Some("default-key"));
}

#[test]
fn the_using_tag_selects_the_runtime() {
    let node = action_from("name: Node Action\nruns:\n  using: node16\n  main: index.js\n");
    let docker = action_from("name: Docker Action\nruns:\n  using: docker\n  image: Dockerfile\n");

    assert!(matches!(node.runs(), ActionRuntime::Node16 { main } if main == "index.js"));
    assert!(matches!(docker.runs(), ActionRuntime::Docker { image } if image == "Dockerfile"));
}

#[test]
fn an_unknown_using_tag_is_rejected() {
    assert!(
        serde_yaml::from_str::<ActionDefinitionYaml>(
            "name: Bad\nruns:\n  using: deno\n  main: index.js\n"
        )
        .is_err()
    );
}
