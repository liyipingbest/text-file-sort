/// Field type
#[derive(Clone, Debug)]
pub enum FieldType {
    /// String
    String,
    /// Signed 64 bit integer
    Integer,
    /// Signed 128 bit integer
    BigInt,
    /// 64 bit floating point number
    Number,
}
