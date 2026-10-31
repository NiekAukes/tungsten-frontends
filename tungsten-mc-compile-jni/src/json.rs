//! The JSON document `NativeCompiler.compile` returns. The schema is described in `docs/jni-api.md`;
//! bump [`SCHEMA_VERSION`] on any incompatible change.

use serde::Serialize;
use tungsten_mc_compile::{CompileError, JavaOutput, ResourceKind, compiler_version};

pub const SCHEMA_VERSION: u32 = 2;

#[derive(Serialize)]
struct Document<'a> {
    schema: u32,
    compiler_version: &'static str,
    #[serde(flatten)]
    result: Status<'a>,
}

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum Status<'a> {
    Success(&'a JavaOutput),
    Unsupported {
        kind: ResourceKind,
        id: &'a str,
        message: &'a str,
    },
    Internal {
        message: &'a str,
    },
}

/// Serializes a compile result.
pub fn result_json(result: &Result<JavaOutput, CompileError>) -> String {
    let result = match result {
        Ok(output) => Status::Success(output),
        Err(CompileError::Unsupported { kind, id, message }) => Status::Unsupported {
            kind: *kind,
            id,
            message,
        },
        Err(CompileError::Internal(message)) => Status::Internal { message },
    };
    let document = Document {
        schema: SCHEMA_VERSION,
        compiler_version: compiler_version(),
        result,
    };
    serde_json::to_string(&document).expect("the result document always serializes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use tungsten_mc_compile::{JavaUnit, Layout, ReachableId};

    fn parse(result: &Result<JavaOutput, CompileError>) -> Value {
        serde_json::from_str(&result_json(result)).unwrap()
    }

    #[test]
    fn success_document() {
        let output = JavaOutput {
            units: vec![JavaUnit { name: "tungsten.generated.GeneratedWorldgen".into(), source: "class X {}".into() }],
            reachable: vec![
                ReachableId { kind: ResourceKind::DensityFunction, id: "minecraft:overworld/continents".into() },
                ReachableId { kind: ResourceKind::Noise, id: "minecraft:continentalness".into() },
            ],
            layout: Layout { min_y: -64, height: 384 },
            warnings: vec![],
        };
        assert_eq!(
            parse(&Ok(output)),
            json!({
                "schema": 2,
                "compiler_version": compiler_version(),
                "status": "success",
                "units": [{"name": "tungsten.generated.GeneratedWorldgen", "source": "class X {}"}],
                "reachable": [
                    {"kind": "density_function", "id": "minecraft:overworld/continents"},
                    {"kind": "noise", "id": "minecraft:continentalness"}
                ],
                "layout": {"min_y": -64, "height": 384},
                "warnings": []
            })
        );
    }

    #[test]
    fn error_documents() {
        let unsupported = Err(CompileError::Unsupported {
            kind: ResourceKind::DensityFunction,
            id: "minecraft:a".into(),
            message: "unknown density function type `x`".into(),
        });
        assert_eq!(
            parse(&unsupported),
            json!({
                "schema": 2,
                "compiler_version": compiler_version(),
                "status": "unsupported",
                "kind": "density_function",
                "id": "minecraft:a",
                "message": "unknown density function type `x`"
            })
        );
        assert_eq!(
            parse(&Err(CompileError::Internal("boom".into()))),
            json!({"schema": 2, "compiler_version": compiler_version(), "status": "internal", "message": "boom"})
        );
    }
}
