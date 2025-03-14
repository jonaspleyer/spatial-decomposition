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
enum Decomposition {
    h1_row,
    h2_row,
    k1_col,
    k2_col,
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
        let mut S = p;
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

impl Decomposition {
    #[rustfmt::skip]
    fn figure_out<F>(values: &KongMountRoscoeValues<F>) -> Option<Decomposition>
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

        // println!("{h1} {h2} {k1} {k2}");
        // println!("{h0} {h3} {k0} {k3}");
        // println!("{A} {B} {C} {S}");

        //  Lemma 2.9
        if h0 < h1 && B / k0 <= S {return Some(h2_row);}
        if k3 > k2 && A / h0 >= C {return Some(h1_row);}
        if h0 < h1 && A / h0 <= S {return Some(k2_col);}
        if h3 > h2 && A / h3 >= C {return Some(k1_col);}

        // Lemma 2.10
        if h0 == h1 && A / h1 <= S && A / h2 >= C {return Some(k2_col);}
        if h3 == h2 && A / h1 <= S && A / h2 >= C {return Some(k1_col);}
        if k0 == k1 && B / k1 <= S && B / k2 >= C {return Some(h2_row);}
        if k3 == k2 && B / k1 <= S && B / k2 >= C {return Some(h1_row);}

        // TODO is this exhaustive?
        // -> probably not!

        None
    }

    fn generate_rectangles<F>(
        &self,
        kmr_values: &KongMountRoscoeValues<F>,
    ) -> impl IntoIterator<Item = Rectangle<F>> + use<F>
    where
        F: 'static + RealField + Copy,
        F: num_traits::cast::AsPrimitive<usize>,
        usize: num_traits::cast::AsPrimitive<F>,
    {
        use Decomposition::*;
        #[rustfmt::skip]
        let KongMountRoscoeValues {
            p,
            h1, h2, k1, k2,
            A,  B,  C,  S,
        } = kmr_values.clone();
        match self {
            h1_row => todo!(),
            h2_row => {
                let dx_row = A / h1;
                let n_rows1: usize = (p - h1 * (p / h1).floor()).round().as_();
                let n_rows2: usize = (h1 * (p / h1).ceil() - p).round().as_();
                let n_cols1 = (p / h1).ceil();
                let n_cols2 = (p / h1).floor();
                let dx_col1 = B / n_cols1;
                let dx_col2 = B / n_cols2;
                let n_cols1: usize = n_cols1.as_();
                let n_cols2: usize = n_cols2.as_();

                let mut rects1: Vec<_> = (0..n_rows1)
                    .flat_map(|n| {
                        (0..n_cols1).map(move |m| {
                            let n: F = n.as_();
                            let m: F = m.as_();
                            Rectangle {
                                min: [n * dx_row, m * dx_col1],
                                max: [(n + F::one()) * dx_row, (m + F::one()) * dx_col1],
                            }
                        })
                    })
                    .collect();
                let rects2 = (0..n_rows2).flat_map(|n| {
                    (0..n_cols2).map(move |m| Rectangle {
                        min: [n_rows1.as_() * dx_row + n.as_() * dx_row, m.as_() * dx_col2],
                        max: [
                            n_rows1.as_() * dx_row + (n + 1).as_() * dx_row,
                            (m + 1).as_() * dx_col2,
                        ],
                    })
                });
                rects1.extend(rects2);
                rects1
            }
            _ => unimplemented!(),
        }
    }
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
    let A = rectangle.max[0] - rectangle.min[0];
    let B = rectangle.max[1] - rectangle.min[1];
    let ratio_max = (A / B).max(B / A);
    if n_subdomains_float <= ratio_max {
        if A >= B {
            let dx = A / n_subdomains_float;
            return (0..n_subdomains)
                .map(|n| Rectangle {
                    min: [rectangle.min[0] + n.as_() * dx, rectangle.min[1]],
                    max: [rectangle.min[0] + (n + 1).as_() * dx, rectangle.max[1]],
                })
                .collect();
        } else {
            let dx = B / n_subdomains_float;
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
