// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/unwind.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;
use std::{f32, f64};

use wasmer_runtime::types::Value;
use wasmer_runtime::{Instance, Module};
use wasmer_clif_backend::CraneliftCompiler;

use crate::spectests::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 3
fn create_module_1() -> Box<Instance> {
    let module_str = "(module
      (type (;0;) (func))
      (type (;1;) (func (result i32)))
      (func (;0;) (type 0)
        i32.const 3
        i64.const 1
        unreachable)
      (func (;1;) (type 0)
        i32.const 3
        i64.const 1
        br 0 (;@0;))
      (func (;2;) (type 1) (result i32)
        i32.const 3
        i64.const 1
        i32.const 9
        br 0 (;@0;))
      (func (;3;) (type 0)
        i32.const 3
        i64.const 1
        i32.const 1
        br_if 0 (;@0;)
        drop
        drop)
      (func (;4;) (type 1) (result i32)
        i32.const 3
        i64.const 1
        i32.const 9
        i32.const 1
        br_if 0 (;@0;)
        drop
        drop)
      (func (;5;) (type 0)
        i32.const 3
        i64.const 1
        i32.const 0
        br_table 0 (;@0;))
      (func (;6;) (type 1) (result i32)
        i32.const 3
        i64.const 1
        i32.const 9
        i32.const 0
        br_table 0 (;@0;))
      (func (;7;) (type 1) (result i32)
        i32.const 3
        i64.const 1
        i32.const 9
        return)
      (func (;8;) (type 0)
        block  ;; label = @1
          i32.const 3
          i64.const 1
          unreachable
        end)
      (func (;9;) (type 1) (result i32)
        block  ;; label = @1
          i32.const 3
          i64.const 1
          br 0 (;@1;)
        end
        i32.const 9)
      (func (;10;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 3
          i64.const 1
          i32.const 9
          br 0 (;@1;)
        end)
      (func (;11;) (type 1) (result i32)
        block  ;; label = @1
          i32.const 3
          i64.const 1
          i32.const 1
          br_if 0 (;@1;)
          drop
          drop
        end
        i32.const 9)
      (func (;12;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 3
          i64.const 1
          i32.const 9
          i32.const 1
          br_if 0 (;@1;)
          drop
          drop
        end)
      (func (;13;) (type 1) (result i32)
        block  ;; label = @1
          i32.const 3
          i64.const 1
          i32.const 0
          br_table 0 (;@1;)
        end
        i32.const 9)
      (func (;14;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 3
          i64.const 1
          i32.const 9
          i32.const 0
          br_table 0 (;@1;)
        end)
      (func (;15;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 3
          i64.const 1
          i32.const 9
          return
        end)
      (func (;16;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 3
          block  ;; label = @2
            i64.const 1
            unreachable
          end
        end)
      (func (;17;) (type 1) (result i32)
        block  ;; label = @1
          i32.const 3
          block  ;; label = @2
            i64.const 1
            br 1 (;@1;)
          end
          drop
        end
        i32.const 9)
      (func (;18;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 3
          block  ;; label = @2
            i64.const 1
            i32.const 9
            br 1 (;@1;)
          end
        end)
      (func (;19;) (type 1) (result i32)
        block  ;; label = @1
          i32.const 3
          block  ;; label = @2
            i64.const 1
            i32.const 1
            br_if 1 (;@1;)
            drop
          end
          drop
        end
        i32.const 9)
      (func (;20;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 3
          block  ;; label = @2
            i64.const 1
            i32.const 9
            i32.const 1
            br_if 1 (;@1;)
            drop
            drop
          end
        end)
      (func (;21;) (type 1) (result i32)
        block  ;; label = @1
          i32.const 3
          block  ;; label = @2
            i64.const 1
            i32.const 1
            br_table 1 (;@1;)
          end
          drop
        end
        i32.const 9)
      (func (;22;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 3
          block  ;; label = @2
            i64.const 1
            i32.const 9
            i32.const 1
            br_table 1 (;@1;)
          end
        end)
      (func (;23;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 3
          block  ;; label = @2
            i64.const 1
            i32.const 9
            return
          end
        end)
      (func (;24;) (type 1) (result i32)
        f32.const 0x0p+0 (;=0;)
        unreachable
        i64.eqz)
      (func (;25;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          i32.const 9
          br 0 (;@1;)
          i64.eqz
        end)
      (func (;26;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i64.const 0
          i32.const 9
          i32.const 1
          br_if 0 (;@1;)
          drop
          i64.eqz
        end)
      (func (;27;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          i32.const 9
          i32.const 0
          br_table 0 (;@1;) 0 (;@1;)
          i64.eqz
        end)
      (func (;28;) (type 1) (result i32)
        f32.const 0x0p+0 (;=0;)
        i32.const 9
        return
        i64.eqz)
      (func (;29;) (type 1) (result i32)
        f32.const 0x0p+0 (;=0;)
        f64.const 0x1p+0 (;=1;)
        unreachable
        i64.eq)
      (func (;30;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          f64.const 0x1p+0 (;=1;)
          i32.const 9
          br 0 (;@1;)
          i64.eq
        end)
      (func (;31;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i64.const 0
          i64.const 1
          i32.const 9
          i32.const 1
          br_if 0 (;@1;)
          drop
          i64.eq
        end)
      (func (;32;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          f64.const 0x1p+0 (;=1;)
          i32.const 9
          i32.const 0
          br_table 0 (;@1;)
          i64.eq
        end)
      (func (;33;) (type 1) (result i32)
        f32.const 0x0p+0 (;=0;)
        f64.const 0x1p+0 (;=1;)
        i32.const 9
        return
        i64.eq)
      (func (;34;) (type 1) (result i32)
        f32.const 0x0p+0 (;=0;)
        f64.const 0x1p+0 (;=1;)
        i64.const 0
        unreachable
        select)
      (func (;35;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          f64.const 0x1p+0 (;=1;)
          i64.const 0
          i32.const 9
          br 0 (;@1;)
          select
        end)
      (func (;36;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 0
          i32.const 1
          i32.const 0
          i32.const 9
          i32.const 1
          br_if 0 (;@1;)
          drop
          select
        end)
      (func (;37;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          f64.const 0x1p+0 (;=1;)
          i64.const 0
          i32.const 9
          i32.const 0
          br_table 0 (;@1;)
          select
        end)
      (func (;38;) (type 1) (result i32)
        f32.const 0x0p+0 (;=0;)
        f64.const 0x1p+0 (;=1;)
        i64.const 1
        i32.const 9
        return
        select)
      (func (;39;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          unreachable
        end)
      (func (;40;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          i32.const 9
          br 0 (;@1;)
        end)
      (func (;41;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 0
          i32.const 9
          i32.const 1
          br_if 0 (;@1;)
          drop
        end)
      (func (;42;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          i32.const 9
          i32.const 0
          br_table 0 (;@1;) 0 (;@1;)
        end)
      (func (;43;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          i32.const 9
          return
        end)
      (func (;44;) (type 1) (result i32)
        loop (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          unreachable
        end)
      (func (;45;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            f32.const 0x0p+0 (;=0;)
            i32.const 9
            br 1 (;@1;)
          end
        end)
      (func (;46;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            i32.const 0
            i32.const 9
            i32.const 1
            br_if 1 (;@1;)
            drop
          end
        end)
      (func (;47;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            f32.const 0x0p+0 (;=0;)
            i32.const 9
            i32.const 0
            br_table 1 (;@1;) 1 (;@1;)
          end
        end)
      (func (;48;) (type 1) (result i32)
        loop (result i32)  ;; label = @1
          f32.const 0x0p+0 (;=0;)
          i32.const 9
          return
        end)
      (export \"func-unwind-by-unreachable\" (func 0))
      (export \"func-unwind-by-br\" (func 1))
      (export \"func-unwind-by-br-value\" (func 2))
      (export \"func-unwind-by-br_if\" (func 3))
      (export \"func-unwind-by-br_if-value\" (func 4))
      (export \"func-unwind-by-br_table\" (func 5))
      (export \"func-unwind-by-br_table-value\" (func 6))
      (export \"func-unwind-by-return\" (func 7))
      (export \"block-unwind-by-unreachable\" (func 8))
      (export \"block-unwind-by-br\" (func 9))
      (export \"block-unwind-by-br-value\" (func 10))
      (export \"block-unwind-by-br_if\" (func 11))
      (export \"block-unwind-by-br_if-value\" (func 12))
      (export \"block-unwind-by-br_table\" (func 13))
      (export \"block-unwind-by-br_table-value\" (func 14))
      (export \"block-unwind-by-return\" (func 15))
      (export \"block-nested-unwind-by-unreachable\" (func 16))
      (export \"block-nested-unwind-by-br\" (func 17))
      (export \"block-nested-unwind-by-br-value\" (func 18))
      (export \"block-nested-unwind-by-br_if\" (func 19))
      (export \"block-nested-unwind-by-br_if-value\" (func 20))
      (export \"block-nested-unwind-by-br_table\" (func 21))
      (export \"block-nested-unwind-by-br_table-value\" (func 22))
      (export \"block-nested-unwind-by-return\" (func 23))
      (export \"unary-after-unreachable\" (func 24))
      (export \"unary-after-br\" (func 25))
      (export \"unary-after-br_if\" (func 26))
      (export \"unary-after-br_table\" (func 27))
      (export \"unary-after-return\" (func 28))
      (export \"binary-after-unreachable\" (func 29))
      (export \"binary-after-br\" (func 30))
      (export \"binary-after-br_if\" (func 31))
      (export \"binary-after-br_table\" (func 32))
      (export \"binary-after-return\" (func 33))
      (export \"select-after-unreachable\" (func 34))
      (export \"select-after-br\" (func 35))
      (export \"select-after-br_if\" (func 36))
      (export \"select-after-br_table\" (func 37))
      (export \"select-after-return\" (func 38))
      (export \"block-value-after-unreachable\" (func 39))
      (export \"block-value-after-br\" (func 40))
      (export \"block-value-after-br_if\" (func 41))
      (export \"block-value-after-br_table\" (func 42))
      (export \"block-value-after-return\" (func 43))
      (export \"loop-value-after-unreachable\" (func 44))
      (export \"loop-value-after-br\" (func 45))
      (export \"loop-value-after-br_if\" (func 46))
      (export \"loop-value-after-br_table\" (func 47))
      (export \"loop-value-after-return\" (func 48)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 212
fn c1_l212_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c1_l212_action_invoke");
    let result = instance.call("func-unwind-by-unreachable", &[]);
    
    result.map(|_| ())
}

#[test]
fn c1_l212_assert_trap() {
    let mut instance = create_module_1();
    let result = c1_l212_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 213
fn c2_l213_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c2_l213_action_invoke");
    let result = instance.call("func-unwind-by-br", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 214
fn c3_l214_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c3_l214_action_invoke");
    let result = instance.call("func-unwind-by-br-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 215
fn c4_l215_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c4_l215_action_invoke");
    let result = instance.call("func-unwind-by-br_if", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 216
fn c5_l216_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c5_l216_action_invoke");
    let result = instance.call("func-unwind-by-br_if-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 217
fn c6_l217_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c6_l217_action_invoke");
    let result = instance.call("func-unwind-by-br_table", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 218
fn c7_l218_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c7_l218_action_invoke");
    let result = instance.call("func-unwind-by-br_table-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 219
fn c8_l219_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c8_l219_action_invoke");
    let result = instance.call("func-unwind-by-return", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 221
fn c9_l221_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c9_l221_action_invoke");
    let result = instance.call("block-unwind-by-unreachable", &[]);
    
    result.map(|_| ())
}

#[test]
fn c9_l221_assert_trap() {
    let mut instance = create_module_1();
    let result = c9_l221_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 222
fn c10_l222_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c10_l222_action_invoke");
    let result = instance.call("block-unwind-by-br", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 223
fn c11_l223_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c11_l223_action_invoke");
    let result = instance.call("block-unwind-by-br-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 224
fn c12_l224_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c12_l224_action_invoke");
    let result = instance.call("block-unwind-by-br_if", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 225
fn c13_l225_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c13_l225_action_invoke");
    let result = instance.call("block-unwind-by-br_if-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 226
fn c14_l226_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c14_l226_action_invoke");
    let result = instance.call("block-unwind-by-br_table", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 227
fn c15_l227_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c15_l227_action_invoke");
    let result = instance.call("block-unwind-by-br_table-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 228
fn c16_l228_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c16_l228_action_invoke");
    let result = instance.call("block-unwind-by-return", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 230
fn c17_l230_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c17_l230_action_invoke");
    let result = instance.call("block-nested-unwind-by-unreachable", &[]);
    
    result.map(|_| ())
}

#[test]
fn c17_l230_assert_trap() {
    let mut instance = create_module_1();
    let result = c17_l230_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 231
fn c18_l231_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c18_l231_action_invoke");
    let result = instance.call("block-nested-unwind-by-br", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 232
fn c19_l232_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c19_l232_action_invoke");
    let result = instance.call("block-nested-unwind-by-br-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 233
fn c20_l233_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c20_l233_action_invoke");
    let result = instance.call("block-nested-unwind-by-br_if", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 234
fn c21_l234_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c21_l234_action_invoke");
    let result = instance.call("block-nested-unwind-by-br_if-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 235
fn c22_l235_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c22_l235_action_invoke");
    let result = instance.call("block-nested-unwind-by-br_table", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 236
fn c23_l236_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c23_l236_action_invoke");
    let result = instance.call("block-nested-unwind-by-br_table-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 237
fn c24_l237_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c24_l237_action_invoke");
    let result = instance.call("block-nested-unwind-by-return", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 239
fn c25_l239_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c25_l239_action_invoke");
    let result = instance.call("unary-after-unreachable", &[]);
    
    result.map(|_| ())
}

#[test]
fn c25_l239_assert_trap() {
    let mut instance = create_module_1();
    let result = c25_l239_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 240
fn c26_l240_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c26_l240_action_invoke");
    let result = instance.call("unary-after-br", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 241
fn c27_l241_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c27_l241_action_invoke");
    let result = instance.call("unary-after-br_if", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 242
fn c28_l242_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c28_l242_action_invoke");
    let result = instance.call("unary-after-br_table", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 243
fn c29_l243_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c29_l243_action_invoke");
    let result = instance.call("unary-after-return", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 245
fn c30_l245_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c30_l245_action_invoke");
    let result = instance.call("binary-after-unreachable", &[]);
    
    result.map(|_| ())
}

#[test]
fn c30_l245_assert_trap() {
    let mut instance = create_module_1();
    let result = c30_l245_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 246
fn c31_l246_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c31_l246_action_invoke");
    let result = instance.call("binary-after-br", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 247
fn c32_l247_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c32_l247_action_invoke");
    let result = instance.call("binary-after-br_if", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 248
fn c33_l248_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c33_l248_action_invoke");
    let result = instance.call("binary-after-br_table", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 249
fn c34_l249_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c34_l249_action_invoke");
    let result = instance.call("binary-after-return", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 251
fn c35_l251_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c35_l251_action_invoke");
    let result = instance.call("select-after-unreachable", &[]);
    
    result.map(|_| ())
}

#[test]
fn c35_l251_assert_trap() {
    let mut instance = create_module_1();
    let result = c35_l251_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 252
fn c36_l252_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c36_l252_action_invoke");
    let result = instance.call("select-after-br", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 253
fn c37_l253_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c37_l253_action_invoke");
    let result = instance.call("select-after-br_if", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 254
fn c38_l254_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c38_l254_action_invoke");
    let result = instance.call("select-after-br_table", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 255
fn c39_l255_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c39_l255_action_invoke");
    let result = instance.call("select-after-return", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 257
fn c40_l257_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c40_l257_action_invoke");
    let result = instance.call("block-value-after-unreachable", &[]);
    
    result.map(|_| ())
}

#[test]
fn c40_l257_assert_trap() {
    let mut instance = create_module_1();
    let result = c40_l257_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 258
fn c41_l258_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c41_l258_action_invoke");
    let result = instance.call("block-value-after-br", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 259
fn c42_l259_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c42_l259_action_invoke");
    let result = instance.call("block-value-after-br_if", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 260
fn c43_l260_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c43_l260_action_invoke");
    let result = instance.call("block-value-after-br_table", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 261
fn c44_l261_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c44_l261_action_invoke");
    let result = instance.call("block-value-after-return", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 263
fn c45_l263_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c45_l263_action_invoke");
    let result = instance.call("loop-value-after-unreachable", &[]);
    
    result.map(|_| ())
}

#[test]
fn c45_l263_assert_trap() {
    let mut instance = create_module_1();
    let result = c45_l263_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 264
fn c46_l264_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c46_l264_action_invoke");
    let result = instance.call("loop-value-after-br", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 265
fn c47_l265_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c47_l265_action_invoke");
    let result = instance.call("loop-value-after-br_if", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 266
fn c48_l266_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c48_l266_action_invoke");
    let result = instance.call("loop-value-after-br_table", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 267
fn c49_l267_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c49_l267_action_invoke");
    let result = instance.call("loop-value-after-return", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
    c2_l213_action_invoke(&mut instance);
    c3_l214_action_invoke(&mut instance);
    c4_l215_action_invoke(&mut instance);
    c5_l216_action_invoke(&mut instance);
    c6_l217_action_invoke(&mut instance);
    c7_l218_action_invoke(&mut instance);
    c8_l219_action_invoke(&mut instance);
    c10_l222_action_invoke(&mut instance);
    c11_l223_action_invoke(&mut instance);
    c12_l224_action_invoke(&mut instance);
    c13_l225_action_invoke(&mut instance);
    c14_l226_action_invoke(&mut instance);
    c15_l227_action_invoke(&mut instance);
    c16_l228_action_invoke(&mut instance);
    c18_l231_action_invoke(&mut instance);
    c19_l232_action_invoke(&mut instance);
    c20_l233_action_invoke(&mut instance);
    c21_l234_action_invoke(&mut instance);
    c22_l235_action_invoke(&mut instance);
    c23_l236_action_invoke(&mut instance);
    c24_l237_action_invoke(&mut instance);
    c26_l240_action_invoke(&mut instance);
    c27_l241_action_invoke(&mut instance);
    c28_l242_action_invoke(&mut instance);
    c29_l243_action_invoke(&mut instance);
    c31_l246_action_invoke(&mut instance);
    c32_l247_action_invoke(&mut instance);
    c33_l248_action_invoke(&mut instance);
    c34_l249_action_invoke(&mut instance);
    c36_l252_action_invoke(&mut instance);
    c37_l253_action_invoke(&mut instance);
    c38_l254_action_invoke(&mut instance);
    c39_l255_action_invoke(&mut instance);
    c41_l258_action_invoke(&mut instance);
    c42_l259_action_invoke(&mut instance);
    c43_l260_action_invoke(&mut instance);
    c44_l261_action_invoke(&mut instance);
    c46_l264_action_invoke(&mut instance);
    c47_l265_action_invoke(&mut instance);
    c48_l266_action_invoke(&mut instance);
    c49_l267_action_invoke(&mut instance);
}
