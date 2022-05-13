pub type Int = i32;
pub type Long = i64;
pub type Float = f32;
pub type Double = f64;
pub type Byte = u8;
pub type Str = String;

#[derive(Clone)]
pub enum Types {
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Byte(Byte),
    Str(Str),
}
