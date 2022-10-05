use std::collections::HashMap;

use serde::{Serialize, de::{Visitor, self}, Deserialize, ser::SerializeMap};

use super::Member;

#[derive(Debug, Clone)]
pub struct Members(HashMap<String, Member>);

impl Serialize for Members {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut map = serializer.serialize_map(Some(self.0.len()))?;
            for (k, v) in &self.0 {
                map.serialize_entry(&k.to_string(), &v)?;
            }
            map.end()
    }
}

struct MembersVisitor;

impl<'de> Visitor<'de> for MembersVisitor {
    type Value = Members;

    fn expecting(&self, formatter: &mut serde::__private::fmt::Formatter) -> serde::__private::fmt::Result {
        formatter.write_str("HashMap with String as key and Member as value")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut members : HashMap<String, Member> = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry()? {
            members.insert(key, value);
        }

        Ok(Members(members))
    }
}

impl<'de> Deserialize<'de> for Members {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {

        deserializer.deserialize_map(MembersVisitor {})
    }
}

impl Members {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_member(&mut self, name: &str) {
        let member = Member::new(name);
        self.0.insert(name.to_string(), member);
    }
}
