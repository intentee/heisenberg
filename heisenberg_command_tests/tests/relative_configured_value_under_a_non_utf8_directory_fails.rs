use std::fs;
use std::io;
use std::path::Path;

use cargo::GlobalContext;
use cargo::util::data_structures::HashMap;
use cargo_util_terminal::Shell;
use heisenberg_command_tests::non_utf8_os_string::non_utf8_os_string;
use heisenberg_project_model::configured_environment::ConfiguredEnvironment;

#[test]
fn relative_configured_value_under_a_non_utf8_directory_fails() {
    let base = Path::new(env!("CARGO_TARGET_TMPDIR")).join("non_utf8_configured_environment");
    let directory = base.join(non_utf8_os_string());
    let cargo_home = base.join("cargo_home");

    fs::create_dir_all(&base)
        .and_then(|()| fs::remove_dir_all(&base))
        .and_then(|()| fs::create_dir_all(&directory))
        .and_then(|()| fs::create_dir_all(&cargo_home))
        .expect("the non-UTF-8 directory must be created");

    let mut context = GlobalContext::new(
        Shell::from_write(Box::new(io::sink())),
        directory,
        cargo_home,
    );
    let environment: HashMap<String, String> = HashMap::default();

    context.set_env(environment.clone());

    let failure = context
        .configure(
            0,
            true,
            None,
            true,
            true,
            true,
            &None,
            &[],
            &[
                "env.PROBE.value = \"value\"".to_owned(),
                "env.PROBE.relative = true".to_owned(),
            ],
        )
        .map_err(|error| error.to_string())
        .map(|()| {
            ConfiguredEnvironment {
                context: &context,
                environment: &environment,
            }
            .variables()
            .err()
            .map(|error| error.to_string())
        });

    assert_eq!(
        failure,
        Ok(Some(
            "the configured value of `PROBE` is not UTF-8".to_owned()
        ))
    );
}
