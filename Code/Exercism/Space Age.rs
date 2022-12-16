#[derive(Debug)]
pub struct Duration;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            s: u64
        }    
    }
}

const EARTH_PERIOD: f64 = 31557600.0;
