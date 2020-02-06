mod map;
pub use map::*;

mod and;
pub use and::*;

// Also backtracking?
/*
mod opt {
    use crate::*;

    pub fn opt<'a, Output>(p: impl Parser<'a, Output = Output>) -> impl Parser<'a, Output = Option<Output>> {
        move |i: Input<'a>, state: &mut State<'a>| {
            let res = p.parse_state(i, state);
            todo!()
            /*
            match res {
                Ok((t, r)) => Ok((Some(t), r)),
                _ => Ok((None, i)),
            }
            */
        }
    }
}
pub use opt::*;
*/

//Backtracking
//mod or;
//pub use or::*;

