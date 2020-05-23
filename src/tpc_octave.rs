use tonality::{Interval, Tpc};

#[derive(Clone, Debug, PartialEq)]
pub struct TpcOctave(pub Tpc, pub i8);

impl std::ops::Add<Interval> for TpcOctave {
    type Output = Option<TpcOctave>;

    fn add(self, rhs: Interval) -> Self::Output {
        let tmp = self.0 + rhs;
        tmp.map(|tmp| {
            if (tmp.step() as i8) < (self.0.step() as i8) {
                Self(tmp, self.1 + 1)
            } else {
                Self(tmp, self.1)
            }
        })
    }
}
