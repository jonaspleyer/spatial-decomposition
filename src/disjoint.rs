use std::num::NonZeroUsize;

use approx_derive::RelativeEq;
use num_traits::AsPrimitive;
use simba::scalar::RealField;

#[derive(Clone, Debug, PartialEq, RelativeEq)]
#[approx(epsilon_type = F)]
pub struct Cuboid<F, const D: usize> {
    #[approx(into_iter)]
    pub min: [F; D],
    #[approx(into_iter)]
    pub max: [F; D],
}

pub type Rectangle<F> = Cuboid<F, 2>;

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
        let A = rectangle.max[0] - rectangle.min[0];
        let B = rectangle.max[1] - rectangle.min[1];

        use Decomposition::*;
        match self {
            row(hrow) => {
                let hrow: F = *hrow;
                let dx_row = B / hrow;
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
                let dx_col1 = A / n_cols1;
                let dx_col2 = A / n_cols2;
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
                let dx_col = A / kcol;
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
                let dx_row1 = B / n_rows1;
                let dx_row2 = B / n_rows2;
                let n_rows1: usize = n_rows1.as_();
                let n_rows2: usize = n_rows2.as_();

                let rects1 =
                    create_rectangles(0..n_rows1, 0..n_cols1, dx_row1, dx_col, min[1], min[0]);
                let rects2 = create_rectangles(
                    n_rows1..n_rows1 + n_rows2,
                    0..n_cols2,
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

/// See paper:
/// https://scispace.com/pdf/the-decomposition-of-a-rectangle-into-rectangles-of-minimal-3whu99wjdy.pdf
#[allow(non_snake_case)]
pub fn kong_mount_roscoe<F>(
    rectangle: &Rectangle<F>,
    n_subdomains: NonZeroUsize,
) -> Vec<Rectangle<F>>
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
    let decomposition = Decomposition::figure_out::<F>(&kmr_values);

    println!(
        "{}",
        match decomposition {
            Some(Decomposition::h1_row) => "h1_row",
            Some(Decomposition::h2_row) => "h2_row",
            Some(Decomposition::k1_col) => "k1_col",
            Some(Decomposition::k2_col) => "k2_col",
            None => "none",
        }
    );

    decomposition
        .map(|d| d.generate_rectangles(&kmr_values))
        .unwrap()
        .into_iter()
        .collect()
}

#[test]
fn test_identity() {
    let rectangle = Rectangle {
        min: [0.0; 2],
        max: [10.0; 2],
    };
    let rects = kong_mount_roscoe(&rectangle, 1.try_into().unwrap());
    assert_eq!(rects.len(), 1);
    assert_eq!(rects[0], rectangle);
}

#[test]
fn test_very_wide() {
    use approx::assert_abs_diff_eq;
    let rectangle = Rectangle {
        min: [0.0; 2],
        max: [100.0, 10.0],
    };
    let rects = kong_mount_roscoe(&rectangle, 4.try_into().unwrap());
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
fn test_very_long() {
    use approx::assert_abs_diff_eq;
    let rectangle = Rectangle {
        min: [-10f32, -200f32],
        max: [10f32, 200f32],
    };
    let rects = kong_mount_roscoe(&rectangle, 10.try_into().unwrap());
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
fn test_5x3_in_7() {
    let rectangle = Rectangle {
        min: [0.0; 2],
        max: [5.0, 3.0],
    };
    let rects = kong_mount_roscoe(&rectangle, 7.try_into().unwrap());
    assert_eq!(rects.len(), 7);
    // Row 1
    assert!(rects.contains(&Rectangle {
        min: [0.; 2],
        max: [5. / 3., 1.],
    }));
    assert!(rects.contains(&Rectangle {
        min: [0., 1.],
        max: [5. / 3., 2.],
    }));
    assert!(rects.contains(&Rectangle {
        min: [0., 2.],
        max: [5. / 3., 3.],
    }));
    // Row 2
    assert!(rects.contains(&Rectangle {
        min: [5. / 3., 0.],
        max: [2. * 5. / 3., 1.5],
    }));
    assert!(rects.contains(&Rectangle {
        min: [5. / 3., 1.5],
        max: [2. * 5. / 3., 3.],
    }));
    // Row 3
    assert!(rects.contains(&Rectangle {
        min: [5. / 3. * 2., 0.],
        max: [5., 1.5],
    }));
    assert!(rects.contains(&Rectangle {
        min: [5. / 3. * 2., 1.5],
        max: [5., 3.],
    }));
}
