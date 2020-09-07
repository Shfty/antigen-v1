
use std::collections::{HashMap, HashSet};

trait Foo {}

struct Bar<'a> {
    foos: HashMap<i64, &'a mut dyn Foo>
}

fn bar<'a>() -> Bar<'a> {
    let foos: HashMap<i64, &'a mut dyn Foo> = HashMap::new();
    Bar {
        foos
    }
}

fn unboxed_main() {
    let bar = bar();
}