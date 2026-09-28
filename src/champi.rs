use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Champi {
    pub nom: String,
    pub r#type: Type,
    #[serde(default)]
    pub anneau: bool,
    #[serde(default)]
    pub volve: bool,
    #[serde(default)]
    pub lait: bool,
    #[serde(default)]
    pub chair: Option<Chair>,
    #[serde(default)]
    pub couleur_spores: Option<Couleur>,
    #[serde(default)]
    pub couleur_chapeau: Option<Couleur>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Type {
    Lames,
    Tubes,
    Aiguillons,
    Plis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Chair {
    Cassante,
    Fibrileuse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Couleur {
    Blanc,
    Brun,
    Beige,
    Orange,
}
