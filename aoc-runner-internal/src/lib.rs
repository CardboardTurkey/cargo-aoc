extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::*;
use serde_json::Value;
use std::cmp::Ordering;
use std::error;
use std::fs;
use std::iter::FromIterator;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::sync::LazyLock;

pub static TARGET_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let metadata: Value = serde_json::from_slice(
        &Command::new("cargo")
            .arg("metadata")
            .output()
            .expect("Failed to find target directory")
            .stdout,
    )
    .expect("Failed to find target directory");
    let metadata = metadata
        .as_object()
        .expect("Failed to find target directory");
    PathBuf::from(
        metadata
            .get("target_directory")
            .expect("Failed to find target directory")
            .as_str()
            .expect("Failed to find target directory"),
    )
});

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Day(pub u8);

impl FromStr for Day {
    type Err = String;

    fn from_str(day: &str) -> Result<Self, Self::Err> {
        let slice = if day.len() < 4 || &day[..3] != "day" {
            day
        } else {
            &day[3..]
        };

        slice
            .parse()
            .map_err(|e| format!("Failed to parse {}: {:?}", day, e))
            .and_then(|d| {
                if d == 0 || d > 25 {
                    Err(format!("day {} is not between 0 and 25", d))
                } else {
                    Ok(Day(d))
                }
            })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Part(pub u8);

impl FromStr for Part {
    type Err = String;

    fn from_str(part: &str) -> Result<Self, Self::Err> {
        Ok(match part {
            "part1" | "1" => Part(1),
            "part2" | "2" => Part(2),
            _ => return Err(format!("Failed to parse part: {}", part)),
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct DayPart {
    pub day: Day,
    pub part: Part,
    pub name: Option<String>,
}

impl DayPart {
    pub fn without_name(&self) -> DayPart {
        DayPart {
            name: None,
            day: self.day,
            part: self.part,
        }
    }
}

impl PartialOrd for DayPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DayPart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.day
            .cmp(&other.day)
            .then(self.part.cmp(&other.part))
            .then(self.name.cmp(&other.name))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayParts {
    pub year: u32,
    parts: Vec<DayPart>,
}

impl DayParts {
    pub fn save(&self) -> Result<(), Box<dyn error::Error>> {
        fs::create_dir_all(TARGET_DIR.join("aoc"))?;
        let f = fs::File::create(TARGET_DIR.join("aoc/completed.json"))?;

        serde_json::to_writer_pretty(f, &self)?;

        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn error::Error>> {
        let f = fs::File::open(TARGET_DIR.join("aoc/completed.json"))?;

        Ok(serde_json::from_reader(f)?)
    }
}

impl Deref for DayParts {
    type Target = [DayPart];

    fn deref(&self) -> &[DayPart] {
        &self.parts
    }
}

impl DerefMut for DayParts {
    fn deref_mut(&mut self) -> &mut [DayPart] {
        &mut self.parts
    }
}

pub struct DayPartsBuilder {
    parts: Vec<DayPart>,
}

impl DayPartsBuilder {
    pub fn with_year(self, year: u32) -> DayParts {
        DayParts {
            year,
            parts: self.parts,
        }
    }
}

impl FromIterator<DayPart> for DayPartsBuilder {
    fn from_iter<T: IntoIterator<Item = DayPart>>(iter: T) -> Self {
        let parts = iter.into_iter().collect();
        DayPartsBuilder { parts }
    }
}
