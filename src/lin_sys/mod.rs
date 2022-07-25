use nalgebra::{Dim, Matrix, RawStorage};

pub mod err;
pub mod fmt;
pub mod numbering;
pub mod unknowns;
pub mod env;

pub struct LinSys<T, R, C, S, U>
where
    U: unknowns::Unknowns,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    /// Matrix representation of a [linear system]. For convenience, the field is of type [`nalgebra::Matrix`]
    ///
    /// [linear system]: https://en.wikipedia.org/wiki/System_of_linear_equations
    pub matrix: Matrix<T, R, C, S>,
    /// The unknowns over which the [linear system] is defined
    ///
    /// [linear system]: https://en.wikipedia.org/wiki/System_of_linear_equations
    pub unknowns: U,
}

impl<T, R, C, S, U> LinSys<T, R, C, S, U>
where
    U: unknowns::Unknowns,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    pub fn new(mrls: Matrix<T, R, C, S>, unknowns: U) -> Option<Self> {
        match unknowns.len() {
            Ok(len) => {
                if len != mrls.ncols().checked_sub(1)? {
                    return None;
                }
            }
            _ => (),
        };
        Some(LinSys {
            matrix: mrls,
            unknowns,
        })
    }
}
