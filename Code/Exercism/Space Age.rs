#[derive(Debug)]
pub struct Duration {
    s: u64
} 

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            s: u64
        }    
    }
}

const EARTH_PERIOD: f64 = 31557600.0;

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.s as f64  / Self::period()    
    }
    fn period() -> f64; 
}
