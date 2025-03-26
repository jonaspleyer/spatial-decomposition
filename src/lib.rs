//! This crate contains algorithms to decompose spatial sets into smaller subdomains.
//! To this date, only `1D` and `2D` algorithms are implemented.
//!
//! ## Overview
//! | Domain | Function | Reference |
//! |:--- | --- |:--- |
//! | [Rectangle] | [kmr_decompose] | [Kong, Mount and Roscoe](https://scispace.com/pdf/the-decomposition-of-a-rectangle-into-rectangles-of-minimal-3whu99wjdy.pdf) |
//! | [Rectangle] | [kmr_digitize_1] | [Kong, Mount and Roscoe](https://scispace.com/pdf/the-decomposition-of-a-rectangle-into-rectangles-of-minimal-3whu99wjdy.pdf) |
//! | [Rectangle] | [kmr_digitize_1_single] | [Kong, Mount and Roscoe](https://scispace.com/pdf/the-decomposition-of-a-rectangle-into-rectangles-of-minimal-3whu99wjdy.pdf) |

use approx::RelativeEq;

/// Generalized cuboid in `D` dimensions
///
/// ```
/// use spatial_decomposition::*;
/// use approx::assert_abs_diff_eq;
///
/// let c1 = Cuboid {
///     min: [1.; 4],
///     max: [15., 15., 10., 10.],
/// };
/// let c2 = Cuboid {
///     min: [1.1; 4],
///     max: [14.9, 14.99, 10.02, 9.97],
/// };
///
/// assert_abs_diff_eq!(c1, c2, epsilon = 0.11);
/// ```
#[derive(Clone, Debug, PartialEq, RelativeEq)]
#[approx(epsilon_type = F)]
pub struct Cuboid<F, const D: usize> {
    /// Lower bounds of cuboid
    #[approx(into_iter)]
    pub min: [F; D],
    /// Upper bounds of cuboid
    #[approx(into_iter)]
    pub max: [F; D],
}

unsafe impl<F, const D: usize> Send for Cuboid<F, D> {}
unsafe impl<F, const D: usize> Sync for Cuboid<F, D> {}

/// `2D` variant of the [Cuboid]
pub type Rectangle<F> = Cuboid<F, 2>;

/// `1D` variant of the [Cuboid]
pub type Line<F> = Cuboid<F, 1>;

mod kong_mount_roscoe;

pub use kong_mount_roscoe::*;
