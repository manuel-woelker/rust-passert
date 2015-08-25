
macro_rules! passert {
    ( $e1:expr, $e2:expr ) => {
        {
            let e1r = $e1;
            let e2r = $e2;
            let res = e1r == e2r;
            println!("{}", concat!($e1, "==", $e2));
            assert!(res, format!("{}", concat!($e1, "==", $e2)));
        }
    };
}

    macro_rules! passert_eq {
        ( $ left : expr , $ right : expr ) => (
            {
                match ( & ( $ left ) , & ( $ right ) ) {
                    ( left_val , right_val ) => {
                        if ! ( * left_val == * right_val ) {
                            panic ! (
                                "assertion failed: `{} == {}` ({}: `{:?}`, {}: `{:?}`)",
                                stringify!($ left), stringify!($ right), stringify!($ left), * left_val ,  stringify!($ right),* right_val )
                        }
                    }
                }
            }
        )
    }

#[test]
fn it_works() {
    let a = Some(32);
    let b = None;
    passert_eq!(a.map(|x| x+3), a);
//    passert!(a, a);
    passert_eq!(a, b);
//    passert!(a, b);
}
