#[macro_export]
macro_rules! check {
    ( $e:expr ) => (
    match $e {
        Ok(_) => {},
    Err(e) => return Err(e)
        }
    );
    ( $e:expr, $v:ident) => (
        match $e {
            Ok(r) => $v = r,
            Err(e) => return Err(e)
        }
    )
}
