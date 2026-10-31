//! The fixed shape of the generated Java that the mod binds to: class names, one entry method per
//! route, and the `HostConsts` fields. None of it depends on the worldgen that was compiled, so it
//! isn't reported per compile; [`check`] verifies that the generated source still matches it.

/// Package of the generated top-level class.
pub const PACKAGE: &str = "tungsten.generated";
/// The generated top-level class.
pub const CLASS_NAME: &str = "GeneratedWorldgen";
/// Nested class: public mutable host inputs, see [`HOST_CONSTS`].
pub const HOST_CONSTS_CLASS: &str = "HostConsts";
/// Nested class: seed-derived tables, immutable and shareable.
pub const AUTO_CONSTS_CLASS: &str = "AutoConsts";
/// `public static AutoConsts create(long seed)` on [`AUTO_CONSTS_CLASS`].
pub const AUTO_CONSTS_FACTORY: &str = "create";
/// Nested class, constructed as `new Orchestration(AutoConsts)`, one per thread.
pub const ORCHESTRATION_CLASS: &str = "Orchestration";

/// Fully qualified name of the generated top-level class.
pub fn qualified_class_name() -> String {
    format!("{}.{}", PACKAGE, CLASS_NAME)
}

/// A noise-router route. Every compile generates every route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    FinalDensityWithBeardify,
    FinalDensity,
    VeinGap,
    VeinRidged,
    VeinToggle,
    PreliminarySurfaceLevel,
    Aquifer,
}

impl Route {
    pub const ALL: [Route; 7] = [
        Route::FinalDensityWithBeardify,
        Route::FinalDensity,
        Route::VeinGap,
        Route::VeinRidged,
        Route::VeinToggle,
        Route::PreliminarySurfaceLevel,
        Route::Aquifer,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Route::FinalDensityWithBeardify => "final_density_with_beardify",
            Route::FinalDensity => "final_density",
            Route::VeinGap => "vein_gap",
            Route::VeinRidged => "vein_ridged",
            Route::VeinToggle => "vein_toggle",
            Route::PreliminarySurfaceLevel => "preliminary_surface_level",
            Route::Aquifer => "aquifer",
        }
    }

    /// The `Orchestration` instance method that computes the route.
    pub fn method(self) -> String {
        format!("orchestrate_{}", self.name())
    }

    /// Element type of the returned array.
    pub fn element_type(self) -> &'static str {
        match self {
            Route::Aquifer => "byte",
            _ => "double",
        }
    }

    /// The method's declaration as generated:
    /// `public <element_type>[] <method>(double x, double y, double z, HostConsts host_consts)`.
    /// The returned array is an internal buffer, overwritten by the next call.
    pub fn signature(self) -> String {
        format!(
            "public {}[] {}(double x, double y, double z, {} host_consts)",
            self.element_type(),
            self.method(),
            HOST_CONSTS_CLASS
        )
    }
}

/// `HostConsts` fields as `(field, java_type)`, in declaration order.
pub const HOST_CONSTS: [(&str, &str); 6] = [
    ("base_box", "BlockBox"),
    ("jigsaw_array", "Jigsaw[]"),
    ("piece_array", "Piece[]"),
    ("aquifer_max_y", "int"),
    ("splitter", "XoroshiroRandomSplitter"),
    ("estimate_surface_height", "EstimatedSurfaceHeight"),
];

/// Checks that `source` declares exactly the contract: every class, every route method and every
/// `HostConsts` field, and no other entry method or field. A mismatch is a compiler bug.
pub fn check(source: &str) -> Result<(), String> {
    let mut problems = Vec::new();

    if !source.contains(&format!("package {};", PACKAGE)) {
        problems.push(format!("no `package {};`", PACKAGE));
    }
    if !source.contains(&format!("public final class {} {{", CLASS_NAME)) {
        problems.push(format!("no top-level class {}", CLASS_NAME));
    }
    for class in [HOST_CONSTS_CLASS, AUTO_CONSTS_CLASS, ORCHESTRATION_CLASS] {
        if !source.contains(&format!("public static final class {} {{", class)) {
            problems.push(format!("no nested class {}", class));
        }
    }
    if !source.contains(&format!(
        "public static {} {}(long seed)",
        AUTO_CONSTS_CLASS, AUTO_CONSTS_FACTORY
    )) {
        problems.push(format!("no factory {}.{}(long seed)", AUTO_CONSTS_CLASS, AUTO_CONSTS_FACTORY));
    }

    let routes: Vec<String> = Route::ALL.iter().map(|r| r.signature()).collect();
    for route in &routes {
        if !source.contains(route.as_str()) {
            problems.push(format!("no entry method `{}`", route));
        }
    }
    for line in source.lines().map(str::trim) {
        if line.contains(" orchestrate_") && line.starts_with("public ") {
            let declared = line.trim_end_matches('{').trim_end();
            if !routes.iter().any(|r| r == declared) {
                problems.push(format!("unexpected entry method `{}`", declared));
            }
        }
    }

    match host_consts_fields(source) {
        Some(fields) => {
            let expected: Vec<String> = HOST_CONSTS
                .iter()
                .map(|(field, java_type)| format!("public {} {};", java_type, field))
                .collect();
            if fields != expected {
                problems.push(format!("HostConsts fields are {:?}, expected {:?}", fields, expected));
            }
        }
        None => problems.push("HostConsts has no closing brace".into()),
    }

    if problems.is_empty() {
        Ok(())
    } else {
        Err(format!("generated Java breaks the fixed contract: {}", problems.join("; ")))
    }
}

/// The trimmed member lines of the `HostConsts` class, or `None` if the class isn't closed.
fn host_consts_fields(source: &str) -> Option<Vec<String>> {
    let header = format!("public static final class {} {{", HOST_CONSTS_CLASS);
    let mut lines = source.lines().map(str::trim).skip_while(|l| *l != header).skip(1);
    let mut fields = Vec::new();
    loop {
        let line = lines.next()?;
        if line == "}" {
            return Some(fields);
        }
        if !line.is_empty() && !line.starts_with("//") {
            fields.push(line.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conforming_source() -> String {
        let mut s = format!("package {};\n\npublic final class {} {{\n", PACKAGE, CLASS_NAME);
        s += &format!("    public static final class {} {{\n", HOST_CONSTS_CLASS);
        for (field, java_type) in HOST_CONSTS {
            s += &format!("        public {} {};\n", java_type, field);
        }
        s += "    }\n";
        s += &format!("    public static final class {} {{\n", AUTO_CONSTS_CLASS);
        s += &format!("        public static {} {}(long seed) {{ }}\n    }}\n", AUTO_CONSTS_CLASS, AUTO_CONSTS_FACTORY);
        s += &format!("    public static final class {} {{\n", ORCHESTRATION_CLASS);
        for route in Route::ALL {
            s += &format!("        {} {{\n        }}\n", route.signature());
        }
        s + "    }\n}\n"
    }

    #[test]
    fn conforming_source_passes() {
        assert_eq!(check(&conforming_source()), Ok(()));
    }

    #[test]
    fn missing_route_fails() {
        let source = conforming_source().replace(&Route::VeinGap.signature(), "void other()");
        let err = check(&source).unwrap_err();
        assert!(err.contains("orchestrate_vein_gap"), "{}", err);
    }

    #[test]
    fn renamed_route_fails() {
        let source = conforming_source().replace("orchestrate_vein_gap(", "orchestrate_vein_gap_1(");
        let err = check(&source).unwrap_err();
        assert!(err.contains("unexpected entry method") && err.contains("vein_gap_1"), "{}", err);
    }

    #[test]
    fn changed_host_consts_fail() {
        let source = conforming_source().replace("public int aquifer_max_y;", "public long aquifer_max_y;");
        let err = check(&source).unwrap_err();
        assert!(err.contains("HostConsts fields"), "{}", err);
        let source = conforming_source().replace("public int aquifer_max_y;\n", "");
        assert!(check(&source).is_err());
    }
}
