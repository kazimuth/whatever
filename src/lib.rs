//! this is gonna wanna look fairly dynamic.
//! dw too much about the rust type system.
//! general pattern is an ordered collection of ECS diagrams, stretching from concrete values to system types and program shapes
//!
//! we define values to intersect to `!` (ie they are lattice atoms)
//! whereas fresh variables are defined to intersect to fresh(T1 | T2).
//! `{}` is included in every atom, and every atom is included in its type. Atoms are not necessarily included in
//! any invocation of fresh(T1 | T2) depending if more constraints are imposed on that invocation later.
//!
//! The general lattice you should be thinking about is:
//!        T
//!     a  b  c ...
//!        !
//! This can be instantiated and elaborated on for particular types.
//! For instance:
//!           (a Ord b)
//!   (a < b)  (a = b)  (a > b)
//!  (a <= b)  (a != b)  (a >= b)
//!               !
//! with the appropriate inclusions drawn in. (a <= b) and (a != b) = (a < b), etc.
//! ("and" means lattice meet, aka least upper bound, aka "product" in categorical language.)
//! (generally when we say "lattice" we mean "incomplete prelattice": a transitive relation.
//! elements may not be less than themselves and there may be globs of elements all equal to each other.
//! meet and join return bags, but this is often suppressed in the notation by coercing values to one-element bags.)
//!
//! Most compiler optimizations want to move monotonically down this lattice:
//! That is, use existing information to drive the state of other pieces of the lattice down,
//! until nothing more can be done.
//!
//! for now, we pursue a strategy similar to the "generalized elements" of 1-category theory
//! (1-lattice-theory) without pursuing n-category theory (n-lattice theory).
//! That is, you can think of a variable as being witnessed, at the end of any particular program run,
//! by a bag of values that variable has taken on, which are linked together in various ways.
//! (But of course, program runs may not have ended yet, and anyway you should really be thinking of all of them at once. Keep in mind they may overlap due to networking and sandboxing.)
//!
//! There is no notion of "identity" really, or notion of canonical pointers, just relation composition. Two equal values may not participate in the same relation; equality is simply the dynamic runtime equality we are familiar with, which does not take pointers or other identifying information into account -- even for collections.
//! In fact, we avoid any notion of "pointer" until very late in compilation, because in many compilation
//! targets, you don't really have them (dynamic langs, config langs, FPGAs, distributed systems...)
//!
//! Inclusions are stored as sparsely as possible -- we only store inclusions for shapes that are explicitly included as flat shapes in the program text. e.g. (a,b,c,d,e) and (a,b,x:(c,d,e))) will not be checked for inclusion against each other. However, programmatically defined routes between types can still be searched.
//!
//! Numerical, vector, tensor, and arithmetic operations are stored using a format inspired by "Graphical Linear Algebra" by Sobelciski (sic myself) and "Picturing Quantum Processes" by whoever wrote that.
//! A number looks like
//! ```bash
//!           -----------------
//! ------copy  ...n wires...  sum--------
//!           -----------------
//! ```
//! but of course these wires are virtual, not (necessarily) explicitly witnessed in the IR.
//! 1/n is stored as n going in the other direction. 1/0 is interpreted as a relation that requires a zero input
//! and produces "anything" as an output.
//! products of nonzero numbers are stored as bags,
//! products of possibly-zero numbers are stored as sequences of bags bridged by zeros.
//! naturals start from one. subtraction is defined as partial, none of that proof assistant 1-2=0 bs.
//!
//!
//!

use std::sync::{Arc, RwLock};
