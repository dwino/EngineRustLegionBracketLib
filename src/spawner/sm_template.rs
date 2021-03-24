use crate::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Produce,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct SmTemplate {
    pub entity_type: EntityType,
    pub name: String,
    pub glyph: char,
    pub color: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SmTemplates {
    pub entities: Vec<SmTemplate>,
}

impl SmTemplates {
    pub fn load() -> Self {
        let file = File::open("resources/sm_template.ron").expect("Failed opening file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();

        self.entities.iter().for_each(|t| {
            available_entities.push(t);
        });

        let mut commands = legion::systems::CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(*pt, entity, &mut commands, rng);
            }
        });
        commands.flush(ecs);
    }
    fn spawn_entity(
        &self,
        pt: Point,
        template: &SmTemplate,
        commands: &mut legion::systems::CommandBuffer,
        rng: &mut RandomNumberGenerator,
    ) {
        let color_string = RGB::from_hex(template.color.clone()).expect("Bad Hex");
        let entity = commands.push((
            pt,
            Render {
                color: ColorPair::new(color_string, RGB::from_hex("#D7E7D0").unwrap()),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));
        match template.entity_type {
            EntityType::Produce => commands.add_component(entity, Item {}),
        }
    }
}
