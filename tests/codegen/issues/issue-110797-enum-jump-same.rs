//@ compile-flags: -O
//@ min-llvm-version: 18

#![crate_type = "lib"]

pub enum K {
    A(Box<[i32]>),
    B(Box<[u8]>),
    C(Box<[String]>),
    D(Box<[u16]>),
}

// CHECK-LABEL: @get_len
// CHECK-NEXT: {{.*}}:
// CHECK-NEXT: getelementptr inbounds
// CHECK-NEXT: load
// CHECK-NEXT: ret i64
#[no_mangle]
pub fn get_len(arg: &K) -> usize {
    match arg {
        K::A(ref lst) => lst.len(),
        K::B(ref lst) => lst.len(),
        K::C(ref lst) => lst.len(),
        K::D(ref lst) => lst.len(),
    }
}
