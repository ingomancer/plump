use protocol::message::Message;
use serde_generate::SourceInstaller;
use serde_reflection::Tracer;
use serde_reflection::TracerConfig;
use std::path::PathBuf;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Languages {
    Python,
    Cpp,
    Java,
    Typescript,
    Csharp,
}

fn main() {
    let (registry, config) = trace_message();

    for language in Languages::iter() {
        generate_spec(&config, &registry, language);
    }
}

fn generate_spec(
    config: &serde_generate::CodeGeneratorConfig,
    registry: &std::collections::BTreeMap<String, serde_reflection::ContainerFormat>,
    language: Languages,
) {
    match language {
        Languages::Python => {
            let path = "langs/python/";
            let path = PathBuf::from_str(path).unwrap();
            let installer = serde_generate::python3::Installer::new(path, None);
            installer.install_module(config, registry).unwrap();
            installer.install_serde_runtime().unwrap();
            installer.install_bincode_runtime().unwrap();
            installer.install_bcs_runtime().unwrap();
        }
        Languages::Cpp => {
            let path = "langs/cpp/";
            let path = PathBuf::from_str(path).unwrap();
            let installer = serde_generate::cpp::Installer::new(path);
            installer.install_module(config, registry).unwrap();
            installer.install_serde_runtime().unwrap();
            installer.install_bincode_runtime().unwrap();
            installer.install_bcs_runtime().unwrap();
        }
        Languages::Java => {
            let path = "langs/java/";
            let path = PathBuf::from_str(path).unwrap();
            let installer = serde_generate::java::Installer::new(path);
            installer.install_module(config, registry).unwrap();
            installer.install_serde_runtime().unwrap();
            installer.install_bincode_runtime().unwrap();
            installer.install_bcs_runtime().unwrap();
        }
        Languages::Typescript => {
            let path = "langs/ts/";
            let path = PathBuf::from_str(path).unwrap();
            let installer = serde_generate::typescript::Installer::new(path);
            installer.install_module(config, registry).unwrap();
            installer.install_serde_runtime().unwrap();
            installer.install_bincode_runtime().unwrap();
            installer.install_bcs_runtime().unwrap();
        }
        Languages::Csharp => {
            let path = "langs/csharp/";
            let path = PathBuf::from_str(path).unwrap();
            let installer = serde_generate::csharp::Installer::new(path);
            installer.install_module(config, registry).unwrap();
            installer.install_serde_runtime().unwrap();
            installer.install_bincode_runtime().unwrap();
            installer.install_bcs_runtime().unwrap();
        }
    }
}

fn trace_message() -> (
    std::collections::BTreeMap<String, serde_reflection::ContainerFormat>,
    serde_generate::CodeGeneratorConfig,
) {
    let mut tracer = Tracer::new(TracerConfig::default());
    tracer.trace_simple_type::<Message>().unwrap();

    let registry = tracer.registry().unwrap();

    let config = serde_generate::CodeGeneratorConfig::new("plump_message".to_string())
        .with_encodings(vec![serde_generate::Encoding::Bincode]);
    (registry, config)
}
