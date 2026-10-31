use crate::parse::{
    MinecraftData, ParseResult,
    density::DensityParseFunctions,
    model::{Spline, SplinePoint, SplineType, SplineValue},
};

pub trait SplineParseFunctions<'m> {
    fn parse_spline(&self, value: &serde_json::Value) -> ParseResult<Spline<'m>>;
}

impl<'m> SplineParseFunctions<'m> for MinecraftData<'m> {
    fn parse_spline(&self, value: &serde_json::Value) -> ParseResult<Spline<'m>> {
        let Some(obj) = value.as_object() else {
            return Err(self.unsupported(format!("a spline must be an object, found `{}`", value)));
        };
        // parse coordinate as density function
        let coordinate = self.parse_density_function_from_value(self.field(obj, "spline", "coordinate")?)?;

        // parse spline points
        let Some(points) = self.field(obj, "spline", "points")?.as_array() else {
            return Err(self.unsupported("spline `points` must be an array"));
        };
        if points.is_empty() {
            return Err(self.unsupported("a spline needs at least one point"));
        }
        let mut splinepoints = vec![];
        for p in points {
            let Some(point_obj) = p.as_object() else {
                return Err(self.unsupported(format!("a spline point must be an object, found `{}`", p)));
            };
            let derivative = self.f64_field(point_obj, "spline point", "derivative")?;
            let location = self.f64_field(point_obj, "spline point", "location")?;
            let value = self.field(point_obj, "spline point", "value")?;
            let spline_value = if let Some(c) = value.as_f64() {
                // value is a const
                SplineValue::Const(c)
            } else {
                // otherwise it must be another spline
                SplineValue::Spline(self.parse_spline(value)?)
            };

            splinepoints.push(SplinePoint {
                derivative,
                location,
                value: spline_value,
            });
        }
        let spline = SplineType {
            coordinate,
            spline_points: self.arena.alloc_slice_clone(&splinepoints),
        };
        Ok(self.arena.alloc(spline))
    }
}
