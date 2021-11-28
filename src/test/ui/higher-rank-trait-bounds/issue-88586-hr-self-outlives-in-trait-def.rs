// Regression test for #88586: a higher-ranked outlives bound on Self in a trait
// definition caused an ICE when debug_assertions were enabled.
//
// The error output is incidentally unhelpful; this should be improved.

trait A where for<'a> Self: 'a
//~^ ERROR the parameter type `Self` may not live long enough
{
}

fn main() {}
