use crate::shiksha::{Varna, Akshara};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// JSON Serialization Structure for Varna
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedVarna {
    pub hk: String,
    pub dev: String,
    pub unicode: String,
    pub swara: Option<String>,
    pub sama_svara: Option<String>,
    pub matra: Option<String>,
    pub sthanani: Option<String>,
    pub prayatna: Option<String>,
}

/// Converts a Varna to a Serializable JSON Struct
pub fn serialize_varna(varna: &Varna) -> SerializedVarna {
    SerializedVarna {
        hk: varna.hk.to_string(),
        dev: varna.dev.to_string(),
        unicode: varna.uni.to_string(),
        swara: varna.swara.clone(),
        sama_svara: varna.sama_svara.clone(),
        matra: varna.matra.clone(),
        sthanani: varna.sthanani.clone(),
        prayatna: varna.prayatna.clone(),
    }
}

/// Converts a SerializedVarna back to Varna
pub fn deserialize_varna(serialized: &SerializedVarna) -> Varna {
    Varna::new(
        &serialized.hk,
        &serialized.dev,
        &serialized.unicode,
        serialized.swara.clone(),
        serialized.sama_svara.clone(),
        serialized.matra.clone(),
        serialized.sthanani.clone(),
        serialized.prayatna.clone(),
    )
}

/// JSON Serialization Structure for Akshara
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedAkshara {
    pub varnas: Vec<SerializedVarna>,
    pub swara: Option<String>,
    pub sama_svara: Option<String>,
    pub matra: Option<String>,
}

/// Converts an Akshara to a Serializable JSON Struct
pub fn serialize_akshara(akshara: &Akshara) -> SerializedAkshara {
    SerializedAkshara {
        varnas: akshara.varnas.iter().map(serialize_varna).collect(),
        swara: akshara.swara.clone(),
        sama_svara: akshara.sama_svara.clone(),
        matra: akshara.matra.clone(),
    }
}

/// Converts a SerializedAkshara back to Akshara
pub fn deserialize_akshara(serialized: &SerializedAkshara) -> Option<Akshara> {
    let varnas: Vec<Varna> = serialized.varnas.iter().map(deserialize_varna).collect();
    Akshara::new(varnas)
}

/// JSON Serialization Structure for Pada
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedPada {
    pub aksharas: Vec<SerializedAkshara>,
}

/// Converts a SerializedPada back to Pada
pub fn deserialize_pada(serialized: &SerializedPada) -> Pada {
    let aksharas = serialized.aksharas.iter().filter_map(deserialize_akshara).collect();
    Pada::new(aksharas)
}

/// JSON Serialization Structure for Vaakya
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedVaakya {
    pub padas: Vec<SerializedPada>,
}

/// Converts a SerializedVaakya back to Vaakya
pub fn deserialize_vaakya(serialized: &SerializedVaakya) -> Vaakya {
    let padas = serialized.padas.iter().map(deserialize_pada).collect();
    Vaakya::new(padas)
}

/// JSON Serialization Structure for Sutra
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedSutra {
    pub varnas: HashMap<String, SerializedVarna>,
}

/// Converts a SerializedSutra back to Sutra
pub fn deserialize_sutra(serialized: &SerializedSutra) -> Sutra {
    let varnas = serialized.varnas.iter().map(|(_, v)| deserialize_varna(v)).collect();
    Sutra::new(varnas)
}
