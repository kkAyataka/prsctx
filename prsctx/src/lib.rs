// Copyright (C) 2024 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use std::cell::RefCell;


#[derive(Clone)]
#[derive(Debug)]
pub struct Context {
    pub name: String,
    pub file_name: String,
    pub line_no: u32,
    pub  module_path: String,
}

thread_local!(static CONTEXT_STACK:RefCell<Vec<Context>> = RefCell::<Vec<Context>>::new(vec![]));

pub struct Mark {
}

impl Mark {
    pub fn new(
        name: &str,
        file_name: &str,
        line_no: u32,
        module_path: &str,
    ) -> Mark {
        CONTEXT_STACK.with(|c| {
            c.borrow_mut().push(Context {
                name: String::from(name),
                file_name: String::from(file_name),
                line_no,
                module_path: String::from(module_path),
            });
        });

        Mark{}
    }
}

impl Drop for Mark {
    fn drop(&mut self) {
        CONTEXT_STACK.with(|s| {
            s.borrow_mut().pop();
        });
    }
}

pub fn get_context_stack() -> Vec<Context> {
    let mut contexts = Vec::<Context>::new();

    CONTEXT_STACK.with(|s| {
        for c in s.borrow().iter() {
            contexts.push(c.clone());
        }
    });

    contexts
}

pub fn get_context_stack_chained_string(chain: Option<&str>) -> String {
    let mut str = String::new();

    CONTEXT_STACK.with(|s| {
        let chain = if let Some(s) = chain { s } else { ">" };
        for c in s.borrow().iter() {
            str.push_str(chain);
            str.push_str(c.name.as_str());
        }
    });

    str
}

pub fn print_context_stack_chaind_string() {
    println!("{}", get_context_stack_chained_string(None));
}

pub fn print_context_stack() {
    CONTEXT_STACK.with(|s| {
        for c in s.borrow().iter() {
            println!("{:?}", &c);
        }
    });
}

#[macro_export]
macro_rules! mark {
    ( $name:expr ) => {
        let _prsctx = $crate::Mark::new($name, file!(), line!(), module_path!());
    };
}


//------------------------------------------------------------------------------
// tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn new_mark() {
        let _m = Mark::new("name", file!(), line!(), module_path!());
        let c = &(get_context_stack())[0];
        assert_eq!("name", c.name);
        assert_eq!("prsctx/src/lib.rs", c.file_name);
        assert_ne!(0, c.line_no);
        assert_eq!("prsctx::tests", c.module_path);
    }

    #[test]
    fn ctx_is_stacked_and_droped() {
        mark!("root");
        assert_eq!(">root", get_context_stack_chained_string(Some(">")));

        fn fn1() {
            mark!("fn1");
            assert_eq!(">root>fn1", get_context_stack_chained_string(Some(">")));
        }

        fn1();
        assert_eq!(">root", get_context_stack_chained_string(Some(">")));
    }

    #[test]
    fn check_ctx_data() {
        mark!("root"); let l0 = line!();
        let s = get_context_stack();
        assert_eq!(1, s.len());

        {
            mark!("b1"); let l1 = line!();
            let s = get_context_stack();
            assert_eq!(2, s.len());

            {
                mark!("b2"); let l2 = line!();
                let s = get_context_stack();
                assert_eq!(3, s.len());

                // 0
                assert_eq!("root", s[0].name);
                assert_eq!("prsctx/src/lib.rs", &s[0].file_name);
                assert_eq!(l0, s[0].line_no);
                assert_eq!("prsctx::tests", &s[0].module_path);

                // 1
                assert_eq!("b1", s[1].name);
                assert_eq!("prsctx/src/lib.rs", &s[1].file_name);
                assert_eq!(l1, s[1].line_no);
                assert_eq!("prsctx::tests", &s[1].module_path);

                // 2
                assert_eq!("b2", s[2].name);
                assert_eq!("prsctx/src/lib.rs", &s[2].file_name);
                assert_eq!(l2, s[2].line_no);
                assert_eq!("prsctx::tests", &s[2].module_path);
            }

            assert_eq!(2, s.len());
        }

        assert_eq!(1, s.len());
    }

    #[test]
    fn stacked_per_thread() {
        mark!("root");

        let th1 = thread::spawn(|| {
            mark!("th1");
            thread::sleep(Duration::from_millis(10));

            {
                mark!("b1");
                thread::sleep(Duration::from_millis(10));

                let s = get_context_stack();
                assert_eq!(2, s.len());
                assert_eq!("th1", s[0].name);
                assert_eq!("b1", s[1].name);
            }
        });

        let th2 = thread::spawn(|| {
            mark!("th2");
            thread::sleep(Duration::from_millis(10));

            {
                mark!("b2");
                thread::sleep(Duration::from_millis(10));

                let s = get_context_stack();
                assert_eq!(2, s.len());
                assert_eq!("th2", s[0].name);
                assert_eq!("b2", s[1].name);
            }
        });

        let s = get_context_stack();
        assert_eq!(1, s.len());
        assert_eq!("root", s[0].name);

        th1.join().unwrap();
        th2.join().unwrap();
    }
}
