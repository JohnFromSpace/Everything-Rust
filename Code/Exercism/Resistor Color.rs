use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black = 0,
    Blue = 1,
    Brown = 2,
    Green = 3,
    Grey = 4,
    Orange = 5,
    Red = 6,
    Violet = 7,
    White = 8,
    Yellow = 9,
}
pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color as usize
}
pub fn value_to_color_string(value: u32) -> String {
    if let Ok(resistor) = ResistorColor::from_int(value) {
        format!("{:?}", resistor)
    } 
    else {
        "value is out of range".to_string()
    }    
}
pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
