use std::sync::Arc;

pub trait Random: Send + Sync {
    fn rand(&self) -> f64;
    fn rand_int_interval(&self, min: i64, max: i64) -> i64;
    fn rand_interval(&self, min: f64, max: f64) -> f64;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_new() -> Arc<dyn Random> {
    use crate::random::rand::RandRandom;

    Arc::new(RandRandom::new())
}

#[cfg(not(target_arch = "wasm32"))]
pub mod rand {
    use crate::Random;

    pub struct RandRandom {}

    impl RandRandom {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Random for RandRandom {
        fn rand(&self) -> f64 {
            rand::random()
        }

        fn rand_interval(&self, min: f64, max: f64) -> f64 {
            rand::random_range(min..max)
        }

        fn rand_int_interval(&self, min: i64, max: i64) -> i64 {
            rand::random_range(min..max)
        }
    }

    impl Default for RandRandom {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    pub mod test {
        use crate::Random;

        use super::RandRandom;

        #[test]
        fn test_rand() {
            let random = RandRandom::new();
            for _ in 0..1000 {
                let v = random.rand();
                assert!(v >= 0.0 && v < 1.0);
            }
        }

        #[test]
        fn rand_interval() {
            let random = RandRandom::new();
            for _ in 0..1000 {
                let v = random.rand_interval(4.2, 8.9);
                assert!(v >= 4.2 && v < 8.9);
            }
        }

        #[test]
        fn rand_int_interval() {
            let random = RandRandom::new();
            for _ in 0..1000 {
                let v = random.rand_int_interval(4, 42);
                assert!(v >= 4 && v < 42);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn random_new() -> Arc<dyn Random> {
    use crate::random::wasm::WasmRandom;

    Arc::new(WasmRandom::new())
}

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use crate::Random;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = Math)]
        fn random() -> f64;
    }

    pub struct WasmRandom {}

    impl WasmRandom {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Random for WasmRandom {
        fn rand(&self) -> f64 {
            random()
        }

        fn rand_interval(&self, min: f64, max: f64) -> f64 {
            let delta = max - min;
            (random() * delta) + min
        }

        fn rand_int_interval(&self, min: i64, max: i64) -> i64 {
            let delta = max - min + 1; // inclusive range
            (self.rand() * delta as f64).floor() as i64 + min
        }
    }

    impl Default for WasmRandom {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Mutex;

    use crate::Random;

    pub struct MockRandom {
        values: Vec<f64>,
        index: Mutex<usize>,
    }

    impl MockRandom {
        pub fn new(values: Vec<f64>) -> Self {
            MockRandom {
                values,
                index: Mutex::new(0),
            }
        }
    }

    impl Random for MockRandom {
        fn rand(&self) -> f64 {
            let mut idx = self.index.lock().unwrap();
            let val = self.values[*idx % self.values.len()];
            *idx += 1;
            val
        }

        fn rand_interval(&self, min: f64, max: f64) -> f64 {
            min + (max - min) * self.rand()
        }

        fn rand_int_interval(&self, min: i64, max: i64) -> i64 {
            let range = max - min;
            min + (self.rand() * range as f64) as i64
        }
    }
}
