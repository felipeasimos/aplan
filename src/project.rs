use crate::wsb::WSB;

pub struct Project {

    wsb: WSB,
    // burndown: Burndown
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            wsb: WSB::new(name)
        }
    }
}

#[cfg(test)]
mod tests {

}
