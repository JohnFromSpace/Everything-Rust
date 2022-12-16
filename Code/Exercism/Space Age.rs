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

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;
