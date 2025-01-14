// This is part of a set of tests exploring the different ways a
// `#[structural_match]` ADT might try to hold a
// non-`#[structural_match]` in hidden manner that lets matches
// through that we had intended to reject.
//
// See discussion on rust-lang/rust#62307 and rust-lang/rust#62339

// run-pass

struct NoDerive(i32);

// This impl makes NoDerive irreflexive.
impl PartialEq for NoDerive { fn eq(&self, _: &Self) -> bool { false } }

impl Eq for NoDerive { }

#[derive(PartialEq, Eq)]
struct WrapParam<T>(T);

const WRAP_INDIRECT_PARAM: & &WrapParam<NoDerive> = & &WrapParam(NoDerive(0));

fn main() {
    match WRAP_INDIRECT_PARAM {
        WRAP_INDIRECT_PARAM => { panic!("WRAP_INDIRECT_PARAM matched itself"); }
        //~^ WARN must be annotated with `#[derive(PartialEq, Eq)]`
        //~| WARN will become a hard error in a future release
        _ => { println!("WRAP_INDIRECT_PARAM correctly did not match itself"); }
    }
}
