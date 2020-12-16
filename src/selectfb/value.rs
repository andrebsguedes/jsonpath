use flexbuffers::{Reader, MapReader};

#[derive(Debug, Clone)]
pub enum Number<'de> {
    Reader(Reader<'de>),
    F64(f64)
}

impl Number<'_> {
    #[inline]
    pub fn is_i64(&self) -> bool {
        use flexbuffers::FlexBufferType::*;
        match self {
            Number::Reader(reader) => {
                match reader.flexbuffer_type() {
                    Int | IndirectInt => true,
                    _ => false
                }
            }
            Number::F64(_) => false
        }
    }

    #[inline]
    pub fn is_u64(&self) -> bool {
        use flexbuffers::FlexBufferType::*;
        match self {
            Number::Reader(reader) => {
                match reader.flexbuffer_type() {
                    UInt | IndirectUInt => true,
                    _ => false
                }
            }
            Number::F64(_) => false
        }
    }

    #[inline]
    pub fn is_f64(&self) -> bool {
        use flexbuffers::FlexBufferType::*;
        match self {
            Number::Reader(reader) => {
                match reader.flexbuffer_type() {
                    Float | IndirectFloat => true,
                    _ => false
                }
            }
            Number::F64(_) => false
        }
    }

    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Number::Reader(reader) => {
                reader.get_i64().ok()
            }
            Number::F64(_) => None
        }
    }

    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Number::Reader(reader) => {
                reader.get_u64().ok()
            }
            Number::F64(_) => None
        }
    }

    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Number::Reader(reader) => {
                reader.get_f64().ok()
            }
            Number::F64(n) => Some(*n)
        }
    }

    pub fn from_f64(n: f64) -> Option<Self> {
        Some(Number::F64(n))
    }
}

impl PartialEq for Number<'_> {
    fn eq(&self, other: &Number) -> bool {
        if self.is_i64() && other.is_i64() {
            return self.as_i64() == self.as_i64()
        }
        if self.is_u64() && other.is_u64() {
            return self.as_u64() == self.as_u64()
        }
        if self.is_f64() && other.is_f64() {
            return self.as_f64() == self.as_f64()
        }
        return false
    }
}

#[derive(Debug, Clone)]
pub struct Map<'de> {
    reader: MapReader<'de>
}

impl<'de> Map<'de> {
    pub fn new(from: MapReader<'de>) -> Map<'de> {
        Map {
            reader: from
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.reader.index_key(key).is_some()
    }

    pub fn get(&self, key: &str) -> Option<FbValue<'de>> {
        self.reader.index(key).ok().map(Into::into)
    }
}

impl<'de> IntoIterator for Map<'de> {
    type Item = (&'de str, FbValue<'de>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let vec: Vec<_> = self.reader.iter_keys().zip(self.reader.iter_values().map(Into::into)).collect();
        vec.into_iter()
    }
}

#[derive(Debug, Clone)]
pub enum Value<'de> {
    Null,
    Bool(bool),
    Number(Number<'de>),
    String(String),
    Array(Vec<FbValue<'de>>),
    Object(Map<'de>)
}

#[derive(Default, Debug, Clone)]
pub struct FbValue<'de> {
    reader: Reader<'de>
}

impl PartialEq for FbValue<'_> {
    fn eq(&self, other: &FbValue) -> bool {
        self.reader.address() == other.reader.address()
    }
}

impl Eq for FbValue<'_> {}

impl std::hash::Hash for FbValue<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.reader.address().hash(state);
    }
}

impl FbValue<'_> {
    #[inline]
    pub fn is_array(&self) -> bool {
        use flexbuffers::FlexBufferType::*;
        match self.reader.flexbuffer_type() {
            Vector | VectorInt | VectorUInt | VectorFloat | VectorKey | VectorBool | VectorInt2 | VectorUInt2 | VectorFloat2 | VectorInt3 | VectorUInt3 | VectorFloat3 | VectorInt4 | VectorUInt4 | VectorFloat4 => {
                true
            }
            _ => false
        }
    }

    pub fn reader(&self) -> Reader {
        self.reader.clone()
    }
}

impl<'de> Into<Value<'de>> for FbValue<'de> {
    fn into(self) -> Value<'de> {
        use flexbuffers::FlexBufferType::*;

        match self.reader.flexbuffer_type() {
            Null => Value::Null,
            Bool => Value::Bool(self.reader.as_bool()),
            Int | UInt | Float | IndirectInt | IndirectUInt | IndirectFloat => Value::Number(Number::Reader(self.reader)),
            String | Key => Value::String(self.reader.as_str().to_string()),
            Vector | VectorInt | VectorUInt | VectorFloat | VectorKey | VectorBool | VectorInt2 | VectorUInt2 | VectorFloat2 | VectorInt3 | VectorUInt3 | VectorFloat3 | VectorInt4 | VectorUInt4 | VectorFloat4 => Value::Array(self.reader.as_vector().iter().map(|reader| FbValue { reader }).collect()),
            Map => Value::Object(self::Map::new(self.reader.as_map())),
            _ => { unimplemented!("unsuported type") }
        }
    }
}

impl<'de> Into<FbValue<'de>> for Reader<'de> {
    fn into(self) -> FbValue<'de> {
        FbValue { reader: self }
    }
}
