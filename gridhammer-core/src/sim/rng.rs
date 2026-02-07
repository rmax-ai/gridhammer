use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Rng {
    seed: u64,
    rng: ChaCha8Rng,
}

impl Serialize for Rng {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.seed.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Rng {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seed = u64::deserialize(deserializer)?;
        Ok(Rng::new(seed))
    }
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng {
            seed,
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn generate<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        use rand::Rng;
        self.rng.r#gen()
    }
}
