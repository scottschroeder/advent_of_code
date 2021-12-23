use anyhow::Result;
use std::{collections::HashMap, fmt, hash::Hash, marker::PhantomData};

pub trait Class: Copy + Hash + Eq + fmt::Debug {}
impl<T> Class for T where T: Copy + Hash + Eq + fmt::Debug {}

pub trait Constraint {
    type D;
    fn apply(&self, xdomain: &mut Vec<Self::D>, ydomain: &[Self::D]) -> bool;
}

pub struct Rule<T>(Box<dyn Constraint<D = T>>);

struct Agenda<C> {
    inner: HashMap<(C, C), bool>,
}

impl<C> Default for Agenda<C> {
    fn default() -> Self {
        Agenda {
            inner: HashMap::new(),
        }
    }
}

impl<C: Class> Agenda<C> {
    fn pop(&mut self) -> Option<(C, C)> {
        self.inner
            .iter_mut()
            .filter(|(_, v)| **v)
            .map(|(k, v)| {
                *v = false;
                *k
            })
            .next()
    }
    fn extend(&mut self, iter: impl Iterator<Item = (C, C)>) {
        for k in iter {
            self.inner.insert(k, true);
        }
    }
    fn len(&self) -> usize {
        self.inner.iter().filter(|(_, v)| **v).count()
    }
}

#[derive(Default)]
pub struct Arcs<C: Class, T> {
    rules: HashMap<(C, C), MultiConstraint<T>>,
}

impl<C: Class, T> Arcs<C, T> {
    fn add_constraint(&mut self, x: C, y: C, rule: Rule<T>) {
        let rules = self.rules.entry((x, y)).or_default();
        rules.0.push(rule);
    }

    fn get(&self, x: C, y: C) -> Option<&MultiConstraint<T>> {
        self.rules.get(&(x, y))
    }
    fn all_arcs(&self) -> impl Iterator<Item = (C, C)> + '_ {
        self.rules.keys().cloned()
    }
    fn arcs_for_y(&self, q: C) -> impl Iterator<Item = (C, C)> + '_ {
        self.all_arcs().filter(move |(_, y)| q == *y)
    }
}

#[derive(Default)]
pub struct AC3<C: Class, T> {
    data: HashMap<C, Vec<T>>,
    arcs: Arcs<C, T>,
}
impl<C: Class, T: 'static + PartialEq> AC3<C, T> {
    pub fn apply_exclusivity(&mut self) {
        let keys = self.data.keys().cloned().collect::<Vec<_>>();
        for x in &keys {
            for y in &keys {
                if x == y {
                    continue;
                }
                self.add_constraint(*x, *y, MembershipExclusivity::rule())
            }
        }
    }
}

impl<C: Class, T: 'static> AC3<C, T> {
    pub fn add_domain<D: Into<Vec<T>>>(&mut self, class: C, domain: D) {
        let domain = domain.into();
        self.data.insert(class, domain);
    }

    pub fn add_constraint<R: Constraint<D = T> + 'static>(&mut self, x: C, y: C, rule: R) {
        let r = Rule(Box::new(rule));
        self.arcs.add_constraint(x, y, r)
    }

    pub fn solve(self) -> Result<HashMap<C, Vec<T>>> {
        let AC3 { mut data, arcs } = self;
        let mut agenda = Agenda::default();
        agenda.extend(arcs.all_arcs());

        while let Some((x, y)) = agenda.pop() {
            log::trace!("Arc({:?}, {:?}) q={}", x, y, agenda.len());
            let r = if let Some(r) = arcs.get(x, y) {
                r
            } else {
                continue;
            };
            let (xdomain, ydomain) = get_mut_pair(&mut data, &x, &y)
                .ok_or_else(|| anyhow::anyhow!("no domain found"))?;
            let revised = r.apply(xdomain, ydomain.as_slice());
            if revised {
                // TODO this will result in duplicates
                let before = agenda.len();
                agenda.extend(arcs.arcs_for_y(x));
                log::trace!("Q {} -> {}", before, agenda.len());
            }
        }
        Ok(data)
    }
}

fn get_mut_pair<'a, K, V>(
    conns: &'a mut HashMap<K, V>,
    a: &K,
    b: &K,
) -> Option<(&'a mut V, &'a mut V)>
where
    K: Eq + std::hash::Hash,
{
    unsafe {
        let a = conns.get_mut(a)? as *mut _;
        let b = conns.get_mut(b)? as *mut _;
        assert_ne!(a, b, "The two keys must not resolve to the same value");
        Some((&mut *a, &mut *b))
    }
}

pub struct MultiConstraint<T>(Vec<Rule<T>>);

impl<T> Default for MultiConstraint<T> {
    fn default() -> Self {
        MultiConstraint(Vec::new())
    }
}

impl<T: 'static> Constraint for MultiConstraint<T> {
    type D = T;

    fn apply(&self, xdomain: &mut Vec<Self::D>, ydomain: &[Self::D]) -> bool {
        let mut modified = false;
        for r in &self.0 {
            modified = r.0.apply(xdomain, ydomain) || modified;
        }
        log::trace!("multi_rule modified: {:?}", modified);
        modified
    }
}

pub struct PairwiseConstraint<T>(Box<dyn Fn(&T, &T) -> bool>);

impl<T: 'static + fmt::Debug> PairwiseConstraint<T> {
    pub fn rule<F: 'static + Fn(&T, &T) -> bool>(f: F) -> PairwiseConstraint<T> {
        PairwiseConstraint(Box::new(f))
    }
}

impl<T: fmt::Debug> Constraint for PairwiseConstraint<T> {
    type D = T;

    fn apply(&self, xdomain: &mut Vec<Self::D>, ydomain: &[Self::D]) -> bool {
        let mut modified = false;
        xdomain.retain(|x_value| {
            for y_value in ydomain {
                let sat = (self.0)(x_value, y_value);
                log::trace!("x: {:?}, y: {:?} -> {:?}", x_value, y_value, sat);
                if sat {
                    return true;
                }
            }
            log::trace!("x: {:?} no possible values in y", x_value);
            modified = true;
            false
        });
        log::trace!("modified: {:?}", modified);
        modified
    }
}

pub struct MembershipExclusivity<T>(PhantomData<T>);

impl<T> MembershipExclusivity<T> {
    fn rule() -> MembershipExclusivity<T> {
        MembershipExclusivity(PhantomData::default())
    }
}

impl<T: PartialEq> Constraint for MembershipExclusivity<T> {
    type D = T;

    fn apply(&self, xdomain: &mut Vec<Self::D>, ydomain: &[Self::D]) -> bool {
        if ydomain.len() != 1 {
            return false;
        }
        let exclude = &ydomain[0];
        let mut modified = false;
        xdomain.retain(|x| {
            if x == exclude {
                modified = true;
                false
            } else {
                true
            }
        });
        modified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ac3_classic_test() {
        let mut ac3 = AC3::default();
        ac3.add_domain("A", vec![1, 2, 3]);
        ac3.add_domain("B", vec![1, 2, 3]);
        ac3.add_domain("C", vec![1, 2, 3]);
        ac3.add_constraint("A", "B", PairwiseConstraint::rule(|a, b| a > b));
        ac3.add_constraint("B", "A", PairwiseConstraint::rule(|b, a| b < a));
        ac3.add_constraint("B", "C", PairwiseConstraint::rule(|b, c| b == c));
        ac3.add_constraint("C", "B", PairwiseConstraint::rule(|c, b| c == b));
        let data = ac3.solve().unwrap();
        assert_eq!(
            data,
            vec![("A", vec![2, 3]), ("B", vec![1, 2]), ("C", vec![1, 2]),]
                .into_iter()
                .collect()
        )
    }
}
