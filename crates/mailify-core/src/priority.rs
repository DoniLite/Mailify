use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Critical,
    High,
    #[default]
    Normal,
    Low,
    Bulk,
}

impl Priority {
    /// Numeric weight — lower value = higher priority (apalis ordering).
    pub fn weight(&self) -> i32 {
        match self {
            Self::Critical => 0,
            Self::High => 10,
            Self::Normal => 50,
            Self::Low => 100,
            Self::Bulk => 200,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights_are_ordered() {
        assert!(Priority::Critical.weight() < Priority::High.weight());
        assert!(Priority::High.weight() < Priority::Normal.weight());
        assert!(Priority::Normal.weight() < Priority::Low.weight());
        assert!(Priority::Low.weight() < Priority::Bulk.weight());
    }

    #[test]
    fn default_is_normal() {
        assert_eq!(Priority::default(), Priority::Normal);
    }

    #[test]
    fn json_round_trip() {
        let p = Priority::Critical;
        let s = serde_json::to_string(&p).unwrap();
        assert_eq!(s, "\"critical\"");
        let back: Priority = serde_json::from_str(&s).unwrap();
        assert_eq!(back, p);
    }
}
