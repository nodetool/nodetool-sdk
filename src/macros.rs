#[macro_export]
macro_rules! extract_inputs {
    (
        $params:expr,
        $($variant:ident),+
    ) => {{
        let mut counter = 0;
        let mut gen = || {
            counter += 1;
            counter
        };
        (
            $(
                if let NodeParameter::$variant(x) = $params[gen()].as_ref().unwrap() { *x } else {
                    panic!("invalid parameter type")
                },
            )+
        )
    }};
}
