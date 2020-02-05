use crate::*;
use std::fmt::Display;

pub trait Problem: Display + Debug {}

impl<E> Problem for E where E: Display + Debug {}

#[derive(Debug)]
pub struct State<'a> {
    problems: Vec<(Box<dyn Problem + 'a>, Location<'a>)>,
    panic: bool,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        State {
            problems: vec![],
            panic: false,
        }
    }
}

impl<'a> State<'a> {
    pub fn raise<P: 'a + Problem>(&mut self, problem: P, location: Location<'a>) {
        if !self.panic {
            self.problems.push((Box::new(problem), location));
            self.panic = true;
        }
    }

    /*
    pub fn problems(&self) -> &[(dyn Problem, Location<'a>)] {
        self.problems.as_slice()
    }
    */

    //pub fn fuse_panic(&mut self) {
    //self.panic = false;
    //}
}

impl<'a> Into<Vec<(Box<dyn Problem + 'a>, Location<'a>)>> for State<'a> {
    fn into(self) -> Vec<(Box<dyn Problem + 'a>, Location<'a>)> {
        self.problems
    }
}
