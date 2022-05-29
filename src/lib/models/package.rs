use eframe::egui::{CollapsingResponse, Ui};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

use super::part::Part;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum State {
    Added,
    Started,
    Finished,
    Extracted,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub parts: Arc<Vec<Part>>,
    pub path: Arc<PathBuf>,
    pub size: u64,
    pub state: State,
}

impl Package {
    pub(crate) fn render(&self, ui: &mut Ui) -> CollapsingResponse<()> {
        ui.collapsing(&self.name, |ui| {
            for part in self.parts.iter() {
                part.render(ui);
            }
        })
    }

    pub(crate) fn get_progress(&self) -> f32 {
        let progress = self.parts.iter().fold(0.0, |acc, part| acc + part.progress);

        progress / self.parts.len() as f32
    }
}
