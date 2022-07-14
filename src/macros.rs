/// A macro to construct a [`SelectOpt`](crate::select::SelectOpt).
#[macro_export] macro_rules! selopt {
    ( $x:expr, $y:expr, $z:expr ) => {
        SelectOpt::new($x, Some($y), $z)
    };

    ( $x:expr, $z: expr ) => {
        SelectOpt::new($x, None, $z)
    }
}