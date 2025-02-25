use crate::shiksha::{Varna, Swara, SamaSvara, Matra, Sthanani, Prayatna};
use serde::{Serialize, Deserialize};

/// JSON Serialization Structure
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
        hk: varna.hk.to_string(),  // Ensure Harvard-Kyoto transliteration
        dev: varna.dev.to_string(),
        unicode: varna.uni.to_string(),
        swara: varna.swara.map(|s| format!("{:?}", s)), 
        sama_svara: varna.sama_svara.map(|s| format!("{:?}", s)),
        matra: varna.matra.map(|m| format!("{:?}", m)),
        sthanani: varna.sthanani.map(|s| format!("{:?}", s)),
        prayatna: varna.prayatna.map(|p| format!("{:?}", p)),
    }
}

/// Converts a SerializedVarna back to Varna
pub fn deserialize_varna(serialized: &SerializedVarna) -> Varna {
    Varna::new(
        &serialized.hk,
        &serialized.dev,
        &serialized.unicode,
        serialized.swara.as_deref().map(|s| match s {
            "Udaatta" => Swara::Udaatta,
            "Anudaatta" => Swara::Anudaatta,
            _ => Swara::Svarita,
        }),
        serialized.sama_svara.as_deref().map(|s| match s {
            "Sa" => SamaSvara::Sa,
            "Re" => SamaSvara::Re,
            "Ga" => SamaSvara::Ga,
            "Ma" => SamaSvara::Ma,
            "Pa" => SamaSvara::Pa,
            "Dha" => SamaSvara::Dha,
            "Ni" => SamaSvara::Ni,
            _ => SamaSvara::Sa,
        }),
        serialized.matra.as_deref().map(|m| match m {
            "Hrasva" => Matra::Hrasva,
            "Diirgha" => Matra::Diirgha,
            "Pluta" => Matra::Pluta,
            _ => Matra::Hrasva,
        }),
        serialized.sthanani.as_deref().map(|s| match s {
            "Kantha" => Sthanani::Kantha,
            "Talu" => Sthanani::Talu,
            "Murdha" => Sthanani::Murdha,
            "Jihvamula" => Sthanani::Jihvamula,
            "Danta" => Sthanani::Danta,
            "Nasika" => Sthanani::Nasika,
            "Oshtha" => Sthanani::Oshtha,
            _ => Sthanani::Kantha,
        }),
        serialized.prayatna.as_deref().map(|p| match p {
            "Sprishta" => Prayatna::Sprishta,
            "IshatSparsha" => Prayatna::IshatSparsha,
            "Vivrita" => Prayatna::Vivrita,
            "Samvruta" => Prayatna::Samvruta,
            "Alpaprana" => Prayatna::Alpaprana,
            "Mahaprana" => Prayatna::Mahaprana,
            "Nasika" => Prayatna::Nasika,
            "Anunasika" => Prayatna::Anunasika,
            _ => Prayatna::Vivrita,
        }),
    )
}
