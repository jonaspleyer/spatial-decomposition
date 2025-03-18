use std::num::NonZeroUsize;

use approx_derive::RelativeEq;
use num_traits::AsPrimitive;
use simba::scalar::RealField;

use crate::Rectangle;

#[allow(non_camel_case_types)]
enum Decomposition<F> {
    row(F),
    col(F),
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, RelativeEq, PartialEq)]
struct KongMountRoscoeValues<F> {
    p: F,
    h1: F,
    h2: F,
    k1: F,
    k2: F,
    A: F,
    B: F,
    C: F,
    S: F,
}

impl<F> KongMountRoscoeValues<F>
where
    F: 'static + Copy + RealField,
    usize: num_traits::cast::AsPrimitive<F>,
{
    #[allow(non_snake_case)]
    fn calculate(A: F, B: F, n_subdomains: usize) -> Self {
        let p: F = n_subdomains.as_();

        let h1 = (A * p / B).sqrt().floor();
        let h2 = (A * p / B).sqrt().ceil();
        let k1 = (B * p / A).sqrt().floor();
        let k2 = (B * p / A).sqrt().ceil();

        let mut C = p;
        let mut S = F::zero();
        for k in 1..n_subdomains {
            for h in 1..n_subdomains {
                let k = k.as_();
                let h = h.as_();
                if (h - F::one()) * (k - F::one()) < p {
                    let ci = (A / h).max(B / k);
                    C = C.min(ci);
                }
                if p < (h + F::one()) * (k + F::one()) {
                    let si = (A / h).min(B / k);
                    S = S.max(si);
                }
            }
        }

        #[rustfmt::skip]
        return KongMountRoscoeValues {
            p,
            h1, h2, k1, k2,
            A,  B,  C,  S,
        };
    }
}

impl<F> Decomposition<F> {
    #[rustfmt::skip]
    fn figure_out(values: &KongMountRoscoeValues<F>) -> Option<Decomposition<F>>
    where
        F: RealField + Copy,
    {
        use Decomposition::*;
        let KongMountRoscoeValues {
            p,
            h1, h2, k1, k2,
            A,  B,  C,  S,
        } = values.clone();
        let h0 = (p / k2).floor();
        let h3 = (p / k1).ceil();
        let k0 = (p / h2).floor();
        let k3 = (p / h1).ceil();

        if h1==h2 && k1==k2 {return Some(col(h1));}

        //  Lemma 2.9
        if h0 < h1 && B / k0 <= S {return Some(row(h2));}
        if k3 > k2 && A / h0 >= C {return Some(row(h1));}
        if h0 < h1 && A / h0 <= S {return Some(col(k2));}
        if h3 > h2 && A / h3 >= C {return Some(col(k1));}

        // Lemma 2.10
        if h0 == h1 && A / h1 <= S && A / h2 >= C {return Some(col(k2));}
        if h3 == h2 && A / h1 <= S && A / h2 >= C {return Some(col(k1));}
        if k0 == k1 && B / k1 <= S && B / k2 >= C {return Some(row(h2));}
        if k3 == k2 && B / k1 <= S && B / k2 >= C {return Some(row(h1));}

        // TODO is this exhaustive?
        // -> probably not!

        None
    }

    #[allow(non_snake_case)]
    fn generate_rectangles(
        &self,
        p: F,
        rectangle: &Rectangle<F>,
    ) -> impl IntoIterator<Item = Rectangle<F>> + use<F>
    where
        F: 'static + RealField + Copy,
        F: num_traits::cast::AsPrimitive<usize>,
        usize: num_traits::cast::AsPrimitive<F>,
    {
        let min = rectangle.min;
        let B = rectangle.max[0] - rectangle.min[0];
        let A = rectangle.max[1] - rectangle.min[1];

        use Decomposition::*;
        match self {
            row(hrow) => {
                let hrow: F = *hrow;
                let dx_row = A / hrow;
                let (n_rows1, n_rows2, n_cols1, n_cols2): (usize, usize, F, F) =
                    if (p / hrow).round() == p / hrow {
                        (hrow.as_(), 0, (p / hrow), F::zero())
                    } else {
                        (
                            (p - hrow * (p / hrow).floor()).round().as_(),
                            (hrow * (p / hrow).ceil() - p).round().as_(),
                            (p / hrow).ceil(),
                            (p / hrow).floor(),
                        )
                    };
                let dx_col1 = B / n_cols1;
                let dx_col2 = B / n_cols2;
                let n_cols1: usize = n_cols1.as_();
                let n_cols2: usize = n_cols2.as_();

                let rects1 =
                    create_rectangles(0..n_rows1, 0..n_cols1, dx_row, dx_col1, min[1], min[0]);
                let rects2 = create_rectangles(
                    n_rows1..n_rows1 + n_rows2,
                    0..n_cols2,
                    dx_row,
                    dx_col2,
                    min[1],
                    min[0],
                );
                rects1.into_iter().chain(rects2)
            }
            col(kcol) => {
                let kcol: F = *kcol;
                let dx_col = B / kcol;
                let (n_cols1, n_cols2, n_rows1, n_rows2): (usize, usize, F, F) =
                    if (p / kcol).round() == p / kcol {
                        (kcol.as_(), 0, p / kcol, F::zero())
                    } else {
                        (
                            (p - kcol * (p / kcol).floor()).round().as_(),
                            (kcol * (p / kcol).ceil() - p).round().as_(),
                            (p / kcol).ceil(),
                            (p / kcol).floor(),
                        )
                    };
                let dx_row1 = A / n_rows1;
                let dx_row2 = A / n_rows2;
                let n_rows1: usize = n_rows1.as_();
                let n_rows2: usize = n_rows2.as_();

                let rects1 =
                    create_rectangles(0..n_rows1, 0..n_cols1, dx_row1, dx_col, min[1], min[0]);
                let rects2 = create_rectangles(
                    0..n_rows2,
                    n_cols1..n_cols1 + n_cols2,
                    dx_row2,
                    dx_col,
                    min[1],
                    min[0],
                );
                rects1.into_iter().chain(rects2)
            }
        }
    }
}

fn create_rectangles<F>(
    n_rows_range: std::ops::Range<usize>,
    n_cols_range: std::ops::Range<usize>,
    dx_row: F,
    dx_col: F,
    x0_row: F,
    x0_col: F,
) -> impl IntoIterator<Item = Rectangle<F>>
where
    F: 'static + RealField + Copy,
    F: num_traits::cast::AsPrimitive<usize>,
    usize: num_traits::cast::AsPrimitive<F>,
{
    (n_rows_range).flat_map(move |n| {
        (n_cols_range).clone().map(move |m| {
            let n: F = n.as_();
            let m: F = m.as_();
            let min = [x0_col + m * dx_col, x0_row + n * dx_row];
            let max = [
                x0_col + (m + F::one()) * dx_col,
                x0_row + (n + F::one()) * dx_row,
            ];
            Rectangle { min, max }
        })
    })
}

/// Partitions a rectangle into multiple smaller rectangles
///
/// This algorithms follows the paper by [Kong, Mount and Roscoe](https://scispace.com/pdf/the-decomposition-of-a-rectangle-into-rectangles-of-minimal-3whu99wjdy.pdf)
///
/// This algorithms divides a given rectangle into multiple smaller rectangles and minimizes the
/// maximum rectangle perimeter.
///
/// ```
/// use spatial_decomposition::{kmr_decompose, Rectangle};
///
/// let domain = Rectangle {
///     min: [0., 40.],
///     max: [100., 240.],
/// };
///
/// let n_subdomains = 9;
/// let subdomains = kmr_decompose(&domain, n_subdomains.try_into().unwrap());
///
/// assert_eq!(subdomains.len(), 9);
///
/// for subdomain in subdomains {
/// }
/// ```
///
/// ## Examples
///
/// Cases of very long/wide rectangles satisfying n_subdomain < max(a/b, b/a)
/// will be split intuitively along the long axis.
///
/// ```text
///                    B = 90
///        ┌─────────┬─────────┬─────────┐
///        │         │         │         │
/// A = 20 │         │         │         │
///        │         │         │         │
///        └─────────┴─────────┴─────────┘
/// ```
///
/// ```
/// # use spatial_decomposition::{kmr_decompose, Rectangle};
/// # use approx::assert_abs_diff_eq;
/// let domain = Rectangle {
///     min: [0.0; 2],
///     max: [90.0, 20.0],
/// };
/// let subdomains = kmr_decompose(
///     &domain,
///     3.try_into().unwrap()
/// );
/// // ┌─────────┬─
/// // │         │
/// // │         │
/// // │         │
/// // └─────────┴─
/// assert_abs_diff_eq!(
///     subdomains[0],
///     &Rectangle {
///         min: [0.0; 2],
///         max: [30.0, 20.0],
/// });
///
/// //           ┬─────────┬─
/// //           │         │
/// //           │         │
/// //           │         │
/// //           ┴─────────┴─
/// assert_abs_diff_eq!(
///     subdomains[1],
///     &Rectangle {
///         min: [30.0, 0.0],
///         max: [60.0, 20.0],
/// });
///
/// //                     ┬─────────┐
/// //                     │         │
/// //                     │         │
/// //                     │         │
/// //                     ┴─────────┘
/// assert_abs_diff_eq!(
///     subdomains[2],
///     &Rectangle {
///         min: [60.0, 0.0],
///         max: [90.0, 20.0],
/// });
/// ```
#[allow(non_snake_case)]
pub fn kmr_decompose<F>(rectangle: &Rectangle<F>, n_subdomains: NonZeroUsize) -> DecomposedDomain<F>
where
    F: 'static + Copy + RealField,
    F: num_traits::cast::AsPrimitive<usize>,
    usize: num_traits::cast::AsPrimitive<F>,
{
    let n_subdomains = n_subdomains.get();
    if n_subdomains == 1 {
        return vec![rectangle.clone()];
    }

    // Cover the very wide/long cases where n_subdomain < max(a/b, b/a)
    //                  a
    //   ┌─────────┬─────────┬─────────┐
    //   │         │         │         │
    // b │         │         │         │
    //   │         │         │         │
    //   └─────────┴─────────┴─────────┘
    let n_subdomains_float: F = n_subdomains.as_();
    let B = rectangle.max[0] - rectangle.min[0];
    let A = rectangle.max[1] - rectangle.min[1];
    let ratio_max = (B / A).max(A / B);
    if n_subdomains_float <= ratio_max {
        if B >= A {
            let dx = B / n_subdomains_float;
            return (0..n_subdomains)
                .map(|n| Rectangle {
                    min: [rectangle.min[0] + n.as_() * dx, rectangle.min[1]],
                    max: [rectangle.min[0] + (n + 1).as_() * dx, rectangle.max[1]],
                })
                .collect();
        } else {
            let dx = A / n_subdomains_float;
            return (0..n_subdomains)
                .map(|n| Rectangle {
                    min: [rectangle.min[0], rectangle.min[1] + n.as_() * dx],
                    max: [rectangle.max[0], rectangle.min[1] + (n + 1).as_() * dx],
                })
                .collect();
        };
    }

    let kmr_values = KongMountRoscoeValues::calculate(A, B, n_subdomains);
    let decomposition = Decomposition::figure_out(&kmr_values);

    decomposition
        .map(|d| d.generate_rectangles(n_subdomains_float, rectangle))
        .unwrap()
        .into_iter()
        .collect()
}

/// Returned from decomposition methods.
pub type DecomposedDomain<F> = Vec<Rectangle<F>>;

/// Error variants of decomposition or digitization
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not find suitable subdomain for given digit")]
    Decompose,
    #[error("Could not assign index to subdomain")]
    Indexing,
}

/// Returned from digitization methods
pub type SortedDigits<F, I> = Vec<(Rectangle<F>, Vec<(I, Rectangle<F>)>)>;

pub type Result<T> = std::result::Result<T, Error>;

/// Assign given digits to a number of subdomains generated by the [kmr_decompose] method.
///
/// It is the first algorithm presented in their paper.
/// This particular implementation is `O(n*p)` where `n` is the number of digits and `p` is the
/// number of subdomains generated.
pub fn kmr_digitize_1<F, I>(
    rectangle: &Rectangle<F>,
    n_subdomains: NonZeroUsize,
    digits: impl IntoIterator<Item = (I, Rectangle<F>)>,
) -> Result<SortedDigits<F, I>>
where
    F: 'static + Copy + RealField,
    F: num_traits::cast::AsPrimitive<usize>,
    usize: num_traits::cast::AsPrimitive<F>,
    I: 'static,
{
    let subdomains = kmr_decompose(rectangle, n_subdomains);
    let mut res: Vec<_> = subdomains
        .iter()
        .map(|subdomain| (subdomain.clone(), Vec::new()))
        .collect();
    for (digit, rect) in digits.into_iter() {
        let index = kmr_digitize_1_single(&subdomains, &rect)?;
        res[index].1.push((digit, rect));
    }
    Ok(res)
}

pub fn kmr_digitize_1_single<F>(
    decomposed_domain: &DecomposedDomain<F>,
    subspace: &Rectangle<F>,
) -> Result<usize>
where
    F: 'static + Copy + RealField,
    F: num_traits::cast::AsPrimitive<usize>,
    usize: num_traits::cast::AsPrimitive<F>,
{
    for (n_subdomain, subdomain) in decomposed_domain.iter().enumerate() {
        let middle = [
            (subspace.min[0] + subspace.max[0]) / (F::one() + F::one()),
            (subspace.min[1] + subspace.max[1]) / (F::one() + F::one()),
        ];
        if middle[0] <= subdomain.max[0]
            && middle[1] <= subdomain.max[1]
            && middle[0] >= subdomain.min[0]
            && middle[1] >= subdomain.min[1]
        {
            return Ok(n_subdomain);
        }
    }
    Err(Error::Indexing)
}

#[test]
fn kmr_digitize_square_2x2_in_4() {
    let domain = Rectangle {
        min: [0.0; 2],
        max: [100.0; 2],
    };
    let digits = kmr_decompose(&domain, 16.try_into().unwrap())
        .into_iter()
        .enumerate();
    let sorted = kmr_digitize_1(&domain, 4.try_into().unwrap(), digits).unwrap();
    for (_, voxels) in sorted.iter() {
        assert_eq!(voxels.len(), 4);
    }
}

#[test]
fn kmr_decompose_identity() {
    let rectangle = Rectangle {
        min: [0.0; 2],
        max: [10.0; 2],
    };
    let rects = kmr_decompose(&rectangle, 1.try_into().unwrap());
    assert_eq!(rects.len(), 1);
    assert_eq!(rects[0], rectangle);
}

#[test]
fn kmr_decompose_very_wide() {
    use approx::assert_abs_diff_eq;
    let rectangle = Rectangle {
        min: [0.0; 2],
        max: [100.0, 10.0],
    };
    let rects = kmr_decompose(&rectangle, 4.try_into().unwrap());
    assert_eq!(rects.len(), 4);
    assert_abs_diff_eq!(
        rects[0],
        Rectangle {
            min: [0.0; 2],
            max: [25.0, 10.0],
        }
    );
    assert_abs_diff_eq!(
        rects[1],
        Rectangle {
            min: [25.0, 0.0],
            max: [50.0, 10.0],
        }
    );
    assert_abs_diff_eq!(
        rects[2],
        Rectangle {
            min: [50.0, 0.0],
            max: [75.0, 10.0],
        }
    );
    assert_abs_diff_eq!(
        rects[3],
        Rectangle {
            min: [75.0, 0.0],
            max: [100.0, 10.0],
        }
    );
}

#[test]
fn kmr_decompose_very_long() {
    use approx::assert_abs_diff_eq;
    let rectangle = Rectangle {
        min: [-10f32, -200f32],
        max: [10f32, 200f32],
    };
    let rects = kmr_decompose(&rectangle, 10.try_into().unwrap());
    let dx = (rectangle.max[1] - rectangle.min[1]) / 10f32;
    for (n, rect) in rects.into_iter().enumerate() {
        assert_abs_diff_eq!(
            rect,
            Rectangle {
                min: [rectangle.min[0], rectangle.min[1] + n as f32 * dx],
                max: [rectangle.max[0], rectangle.min[1] + (n + 1) as f32 * dx],
            }
        );
    }
}

#[test]
fn kmr_decompose_5x3_in_7() {
    let rectangle = Rectangle {
        min: [0.0; 2],
        max: [5.0, 3.0],
    };
    let rects = kmr_decompose(&rectangle, 7.try_into().unwrap());
    assert_eq!(rects.len(), 7);

    for i in 0..4 {
        let dx = (rectangle.max[0] - rectangle.min[0]) / 4.;
        let i = i as f64;
        assert!(rects.contains(&Rectangle {
            min: [i * dx, 0.],
            max: [(i + 1.) * dx, 1.5],
        }));
    }
    for i in 0..3 {
        let dx = (rectangle.max[0] - rectangle.min[0]) / 3.;
        let i = i as f64;
        assert!(rects.contains(&Rectangle {
            min: [i * dx, 1.5],
            max: [(i + 1.) * dx, 3.]
        }))
    }
}

#[test]
fn kmr_decompose_6x6_in_14() {
    let rectangle = Rectangle {
        min: [-60., 0.],
        max: [0., 60.],
    };
    let rects = kmr_decompose(&rectangle, 14.try_into().unwrap());
    assert_eq!(rects.len(), 14);
    for i in 0..5 {
        for j in 0..2 {
            let dx = 60. / 5.;
            let i = i as f64;
            let j = j as f64;
            let dy = 60. / 3.;
            let r = Rectangle {
                min: [i * dx - 60., j * dy],
                max: [(i + 1.) * dx - 60., (j + 1.) * dy],
            };
            assert!(rects.contains(&r));
        }
    }
    for i in 0..4 {
        let i = i as f64;
        let j = 2.;
        let dx = 60. / 4.;
        let dy = 60. / 3.;
        let r = Rectangle {
            min: [i * dx - 60., j * dy],
            max: [(i + 1.) * dx - 60., (j + 1.) * dy],
        };
        assert!(rects.contains(&r));
    }
}

#[test]
fn kmr_decompose_square_into_4() {
    let rectangle = Rectangle {
        min: [-40.0; 2],
        max: [-20.0; 2],
    };
    let rects = kmr_decompose(&rectangle, 4.try_into().unwrap());
    assert_eq!(rects.len(), 4);
    assert!(rects.contains(&Rectangle {
        min: [-40.; 2],
        max: [-30.; 2]
    }));
    assert!(rects.contains(&Rectangle {
        min: [-40.0, -30.0],
        max: [-30.0, -20.0]
    }));
    assert!(rects.contains(&Rectangle {
        min: [-30.0, -40.0],
        max: [-20.0, -30.0]
    }));
    assert!(rects.contains(&Rectangle {
        min: [-30.0; 2],
        max: [-20.0; 2]
    }));
}

#[test]
fn kmr_decompose_square_into_5() {
    let rectangle = Rectangle {
        min: [0.0; 2],
        max: [100.0; 2],
    };
    let subdomains = kmr_decompose(&rectangle, 5.try_into().unwrap());
    assert_eq!(subdomains.len(), 5);
    for s in subdomains.iter() {
        println!("{s:7.2?}");
    }
    assert!(subdomains.contains(&Rectangle {
        min: [0.0; 2],
        max: [50.0, 100.0 / 3.],
    }));
    assert!(subdomains.contains(&Rectangle {
        min: [0.0, 100. / 3.],
        max: [50., 100. / 3. * 2.]
    }));
    assert!(subdomains.contains(&Rectangle {
        min: [0.0, 100. / 3. * 2.],
        max: [50., 100.]
    }));
    assert!(subdomains.contains(&Rectangle {
        min: [50.0, 0.],
        max: [100., 50.]
    }));
    assert!(subdomains.contains(&Rectangle {
        min: [50.0, 50.],
        max: [100., 100.]
    }));
}
