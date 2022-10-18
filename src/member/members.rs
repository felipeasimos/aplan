use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::prelude::Error;

use super::Member;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Members {
    #[serde_as(as="Vec<(_, _)>")]
    members: HashMap<String, Member>
}

impl Members {

    pub(crate) fn new() -> Self {
        Self {
            members: HashMap::new()
        }
    }

    pub fn get(&self, name: &str) -> Result<&Member, Error> {
        self.members.get(name)
            .ok_or_else(|| Error::MemberNotFound(name.to_string()))
    }

    pub fn members(&self) -> impl Iterator<Item=&Member> {
        self.members.values()
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub(crate) fn insert(&mut self, name: String) -> Result<(), Error> {
        self.members.insert(name.to_string(), Member::new(&name));
        Ok(())
    }

    pub(crate) fn remove(&mut self, name: &str) -> Result<Member, Error> {
        self.members.remove(&name[..])
            .ok_or_else(|| Error::MemberNotFound(name.to_string()))
    }

    pub(crate) fn get_mut(&mut self, name: &str) -> Result<&mut Member, Error> {
        self.members.get_mut(name)
            .ok_or_else(|| Error::MemberNotFound(name.to_string()))
    }
}
