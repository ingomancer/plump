use protocol::message::Message;
use serde_reflection::Tracer;
use serde_reflection::TracerConfig;
use std::fs::File;
use std::io::Write;
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
            let generator = serde_generate::python3::CodeGenerator::new(config);
            let mut source = Vec::new();
            let _ = generator.output(&mut source, registry);
            let filepath = "langs/python/protocol.py";
            let mut file = File::create(filepath).unwrap();
            file.write_all(&source).unwrap();
        }
        Languages::Cpp => {
            let generator = serde_generate::cpp::CodeGenerator::new(config);
            let mut source = Vec::new();
            let _ = generator.output(&mut source, registry);
            let filepath = "langs/cpp/protocol.cpp";
            let mut file = File::create(filepath).unwrap();
            file.write_all(&source).unwrap();
        }
        Languages::Java => {
            let generator = serde_generate::java::CodeGenerator::new(config);
            let _ = generator.write_source_files("langs/java/".into(), registry);
        }
        Languages::Typescript => {
            let generator = serde_generate::typescript::CodeGenerator::new(config);
            let mut source = Vec::new();
            let _ = generator.output(&mut source, registry);
            let filepath = "langs/typescript/protocol.ts";
            let mut file = File::create(filepath).unwrap();
            file.write_all(&source).unwrap();
        }
        Languages::Csharp => {
            let generator = serde_generate::csharp::CodeGenerator::new(config);
            let _ = generator.write_source_files("langs/csharp/".into(), registry);
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
