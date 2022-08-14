use crate::sequences::FIBONACCI;
use serde::Deserialize;
use std::collections::HashMap;
use svg::{
    node::element::{Circle, Line},
    Document,
};
use url::Url;

fn default_radius() -> f32 {
    100.0
}
fn default_padding() -> f32 {
    20.0
}
fn default_rotation() -> f32 {
    std::f32::consts::PI / 2.0
}
fn default_bounding_circle() -> bool { true }


#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Deserialize)]
pub struct Plot {
    pub modulus: usize,
    #[serde(default = "default_radius")]
    pub radius: f32,
    #[serde(default = "default_padding")]
    pub padding: f32,
    #[serde(default = "default_rotation")]
    pub rotation: f32,
    #[serde(default = "default_bounding_circle")]
    pub with_bunding_circle: bool,
}

impl Plot {
    pub fn render(self) -> Document {
        let Plot {
            radius,
            padding,
            modulus,
            rotation,
            with_bunding_circle,
        } = self;

        let mut document = Document::new().set(
            "viewBox",
            (
                0,
                0,
                radius * 2.0 + padding + 2.0,
                radius * 2.0 + padding * 2.0,
            ),
        );

        let points: Vec<Point> = (1..=modulus)
            .map(|i| {
                let theta = (std::f32::consts::PI * 2.0) / modulus as f32;
                let angle = theta * i as f32 + rotation;

                Point {
                    x: radius * angle.cos(),
                    y: radius * angle.sin(),
                }
            })
            .collect();

        let sequence = &FIBONACCI;

        for (i, element) in sequence.iter().map(|i| i % modulus).enumerate() {
            let from = &points[element];
            let to = &points[sequence[(i + 1) % sequence.len()] % points.len()];

            document = document.add(
                Line::new()
                    .set("x1", from.x + radius + padding)
                    .set("y1", from.y + radius + padding)
                    .set("x2", to.x + radius + padding)
                    .set("y2", to.y + radius + padding)
                    .set("stroke", "black"),
            );
        }

        if with_bunding_circle {
            document = document.add(
                Circle::new()
                    .set("cx", radius + padding)
                    .set("cy", radius + padding)
                    .set("r", radius)
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1.5),
            );
        }

        document
    }
}

impl TryFrom<Url> for Plot {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let parameters: HashMap<String, String> =
            HashMap::from_iter(value.query_pairs().into_owned());

        let modulus: usize = parameters.get("modulus").unwrap().parse()?;

        let radius: f32 = parameters
            .get("radius")
            .map(|v| v.parse())
            .unwrap_or(Ok(default_radius()))?;

        let padding: f32 = parameters
            .get("padding")
            .map(|v| v.parse())
            .unwrap_or(Ok(default_padding()))?;

        let rotation: f32 = parameters
            .get("rotation")
            .map(|v| v.parse())
            .unwrap_or(Ok(default_rotation()))?;

        let with_bunding_circle: bool = parameters
            .get("with-bounding-circle")
            .map(|v| v.parse())
            .unwrap_or(Ok(default_bounding_circle()))?;

        Ok(Self {
            modulus,
            radius,
            padding,
            rotation,
            with_bunding_circle,
        })
    }
}
