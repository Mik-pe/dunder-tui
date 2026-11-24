// Check the continuous objective only when the two public APIs return different rectangles.
// The dual has a feasible all-zero starting point. Rational pivots give an exact minimum.
use num_bigint::BigInt;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, Zero};
use ratatui::layout::{Constraint, Direction, Flex, Rect};
use std::collections::BTreeMap;

fn int(v: i64) -> Q {
    Q::from_integer(BigInt::from(v))
}
fn frac(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}
type Terms = Vec<(usize, Q)>;

#[derive(Clone)]
struct Bound {
    lhs: Terms,
    rhs: Q,
    penalty: Option<i64>,
}
#[derive(Clone, Copy)]
enum Relation {
    Equal,
    Above,
    Below,
}

struct Problem {
    variables: usize,
    bounds: Vec<Bound>,
}

fn width(i: usize, scale: Q) -> Terms {
    vec![(i, -scale.clone()), (i + 1, scale)]
}

impl Problem {
    fn constrain(&mut self, lhs: Terms, op: Relation, rhs: Q, penalty: Option<i64>) {
        if matches!(op, Relation::Below | Relation::Equal) {
            self.bounds.push(Bound {
                lhs: lhs.iter().map(|(k, x)| (*k, -x)).collect(),
                rhs: -&rhs,
                penalty,
            });
        }
        if matches!(op, Relation::Above | Relation::Equal) {
            self.bounds.push(Bound { lhs, rhs, penalty });
        }
    }

    fn new(extent: i64, rules: &[Constraint], flex: Flex, gap: i32) -> Self {
        use Relation::{Above, Below, Equal};
        let n = rules.len();
        let mut p = Self {
            variables: 2 * n + 2,
            bounds: Vec::new(),
        };
        p.constrain(vec![(0, Q::one())], Equal, Q::zero(), None);
        p.constrain(vec![(2 * n + 1, Q::one())], Equal, int(extent), None);
        for i in 0..p.variables {
            p.constrain(vec![(i, Q::one())], Below, int(extent), None);
        }
        let mut fills = Vec::new();
        for (i, &r) in rules.iter().enumerate() {
            let w = width(2 * i + 1, Q::one());
            p.constrain(w.clone(), Above, Q::zero(), None);
            let (target, weight) = match r {
                Constraint::Length(v) => (int(v.into()), 10_000_000),
                Constraint::Percentage(v) => (frac(extent * i64::from(v), 100), 1_000_000),
                Constraint::Ratio(a, b) => (frac(extent * i64::from(a), b.max(1).into()), 100_000),
                Constraint::Max(v) => {
                    p.constrain(w.clone(), Below, int(v.into()), Some(100_000_000));
                    (int(v.into()), 10_000)
                }
                Constraint::Min(v) => {
                    p.constrain(
                        w.clone(),
                        Above,
                        int(i64::from(v as i16)),
                        Some(100_000_000),
                    );
                    if flex == Flex::Legacy {
                        (int(v.into()), 10_000)
                    } else {
                        fills.push((i, Q::one()));
                        (int(extent), 1000)
                    }
                }
                Constraint::Fill(v) => {
                    fills.push((
                        i,
                        if v == 0 {
                            frac(1, 1_000_000)
                        } else {
                            int(v.into())
                        },
                    ));
                    (int(extent), 1000)
                }
            };
            p.constrain(w, Equal, target, Some(weight));
            if i > 0 && flex != Flex::Legacy {
                p.compare(2 * i - 1, Q::one(), 2 * i + 1, Q::one(), 1);
            }
        }
        for (k, (i, a)) in fills.iter().enumerate() {
            for (j, b) in &fills[k + 1..] {
                p.compare(2 * i + 1, b.clone(), 2 * j + 1, a.clone(), 100);
            }
        }
        let border = Some(1_001_000_999);
        let spacer = 100_100_100;
        for i in 0..=n {
            let s = width(2 * i, Q::one());
            let outer = i == 0 || i == n;
            match flex {
                Flex::Start | Flex::End | Flex::Center | Flex::Legacy if !outer => {
                    p.constrain(s, Equal, int(gap.into()), Some(spacer))
                }
                Flex::Legacy => p.constrain(s, Equal, Q::zero(), border),
                Flex::Start if i == 0 => p.constrain(s, Equal, Q::zero(), border),
                Flex::End if i == n => p.constrain(s, Equal, Q::zero(), border),
                Flex::Start | Flex::End | Flex::Center => {
                    p.constrain(s, Equal, int(extent), Some(100))
                }
                Flex::SpaceBetween if outer => p.constrain(s, Equal, Q::zero(), border),
                _ => {
                    p.constrain(s.clone(), Above, int(gap.into()), Some(spacer));
                    p.constrain(s, Equal, int(extent), Some(10));
                }
            }
        }
        if flex == Flex::Center {
            p.compare(0, Q::one(), 2 * n, Q::one(), spacer);
        }
        let pairs = match flex {
            Flex::SpaceEvenly => 0..n + 1,
            Flex::SpaceAround if n < 2 => 0..n + 1,
            Flex::SpaceBetween | Flex::SpaceAround => 1..n,
            _ => 0..0,
        };
        for i in pairs.clone() {
            for j in i + 1..pairs.end {
                p.compare(2 * i, Q::one(), 2 * j, Q::one(), spacer);
            }
        }
        if flex == Flex::SpaceAround && n > 1 {
            for i in [0, 2 * n] {
                p.compare(2, Q::one(), i, int(2), spacer);
            }
        }
        p
    }
    fn compare(&mut self, a: usize, ka: Q, b: usize, kb: Q, weight: i64) {
        let mut terms = width(a, ka);
        terms.extend(width(b, -kb));
        self.constrain(terms, Relation::Equal, Q::zero(), Some(weight));
    }
    fn score(&self, bins: Option<&[u16]>) -> Option<Q> {
        let mut pixel_bounds = Vec::new();
        if let Some(bins) = bins {
            for (i, &k) in bins.iter().enumerate() {
                // Ratatui rounds twice. Keep the upper, open edge out of the witness.
                let lo = int(k.into()) - frac(505, 1000);
                let hi = int(k.into()) + frac(495, 1000) - frac(1, 1_000_000_000_000);
                pixel_bounds.push(Bound {
                    lhs: vec![(i, Q::one())],
                    rhs: lo,
                    penalty: None,
                });
                pixel_bounds.push(Bound {
                    lhs: vec![(i, -Q::one())],
                    rhs: -hi,
                    penalty: None,
                });
            }
        }
        let m = self.bounds.len() + pixel_bounds.len();
        let mut rows = vec![Row::default(); self.variables];
        let mut objective = Row::default();
        for (i, b) in self.bounds.iter().chain(&pixel_bounds).enumerate() {
            objective.add(i, b.rhs.clone());
            for (v, a) in &b.lhs {
                rows[*v].add(i, -a);
            }
            if let Some(cost) = b.penalty {
                let mut upper = Row {
                    value: int(cost),
                    ..Default::default()
                };
                upper.add(i, -Q::one());
                rows.push(upper);
            }
        }
        for (i, r) in rows.iter_mut().enumerate() {
            r.basic = m + i;
        }
        while let Some((&enter, _)) = objective.terms.iter().find(|(_, v)| v.is_positive()) {
            let mut leaving: Option<(usize, Q)> = None;
            for (i, row) in rows.iter().enumerate() {
                if let Some(c) = row.terms.get(&enter).filter(|v| v.is_negative()) {
                    let step = -&row.value / c;
                    if leaving.as_ref().is_none_or(|(old, k)| {
                        step < *k || step == *k && row.basic < rows[*old].basic
                    }) {
                        leaving = Some((i, step));
                    }
                }
            }
            let (k, _) = leaving?;
            let mut pivot = std::mem::take(&mut rows[k]);
            let inv = -Q::one() / pivot.terms.remove(&enter).unwrap();
            pivot.value *= &inv;
            for v in pivot.terms.values_mut() {
                *v *= &inv;
            }
            pivot.add(pivot.basic, -inv);
            pivot.basic = enter;
            for r in &mut rows {
                r.eliminate(enter, &pivot);
            }
            objective.eliminate(enter, &pivot);
            rows[k] = pivot;
        }
        Some(objective.value)
    }
}

#[derive(Clone, Default)]
struct Row {
    basic: usize,
    value: Q,
    terms: BTreeMap<usize, Q>,
}
impl Row {
    fn add(&mut self, k: usize, v: Q) {
        if v.is_zero() {
            return;
        }
        let slot = self.terms.entry(k).or_default();
        *slot += v;
        if slot.is_zero() {
            self.terms.remove(&k);
        }
    }
    fn eliminate(&mut self, v: usize, pivot: &Row) {
        let Some(c) = self.terms.remove(&v) else {
            return;
        };
        self.value += &c * &pivot.value;
        for (&k, a) in &pivot.terms {
            self.add(k, &c * a);
        }
    }
}

pub fn optimal_projections(
    rules: &[Constraint],
    flex: Flex,
    gap: i32,
    inner: Rect,
    axis: Direction,
    reference: &[Rect],
    actual: &[Rect],
) -> (bool, bool) {
    let bounds = |r: Rect| match axis {
        Direction::Horizontal => (r.x, r.right()),
        Direction::Vertical => (r.y, r.bottom()),
    };
    let (origin, end) = bounds(inner);
    let n = rules.len();
    let problem = Problem::new(i64::from(end - origin), rules, flex, gap);
    let best = problem.score(None).expect("bounded layout");
    let optimal = |rects: &[Rect]| {
        let mut bins = vec![0];
        for &r in &rects[..n] {
            let (a, b) = bounds(r);
            if a < origin || b > end {
                return false;
            }
            bins.extend([a - origin, b - origin]);
        }
        bins.push(end - origin);
        for (i, pair) in bins.windows(2).enumerate() {
            let q = origin + pair[0];
            let size = pair[1].saturating_sub(pair[0]);
            let r = match axis {
                Direction::Horizontal => Rect::new(q, inner.y, size, inner.height),
                Direction::Vertical => Rect::new(inner.x, q, inner.width, size),
            };
            if r != rects[if i % 2 == 0 { n + i / 2 } else { i / 2 }] {
                return false;
            }
        }
        problem.score(Some(&bins)).as_ref() == Some(&best)
    };
    (optimal(reference), optimal(actual))
}
