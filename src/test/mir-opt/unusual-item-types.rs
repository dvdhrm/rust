// Test that we don't ICE when trying to dump MIR for unusual item types and
// that we don't create filenames containing `<` and `>`

// EMIT_MIR_FOR_EACH_BIT_WIDTH

struct A;

// EMIT_MIR rustc.{{impl}}-ASSOCIATED_CONSTANT.mir_map.0.mir
impl A {
    const ASSOCIATED_CONSTANT: i32 = 2;
}

// See #59021
// EMIT_MIR rustc.Test-X-{{constructor}}.mir_map.0.mir
enum Test {
    X(usize),
    Y { a: usize },
}

// EMIT_MIR rustc.E-V-{{constant}}.mir_map.0.mir
enum E {
    V = 5,
}

fn main() {
    let f = Test::X as fn(usize) -> Test;
// EMIT_MIR rustc.ptr-drop_in_place.std__vec__Vec_i32_.AddMovesForPackedDrops.before.mir
    let v = Vec::<i32>::new();
}
