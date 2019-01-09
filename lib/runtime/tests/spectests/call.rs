// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/call.wast
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
      (type (;0;) (func (param i32 i32) (result i32)))
      (type (;1;) (func (result i32)))
      (type (;2;) (func (result i64)))
      (type (;3;) (func (result f32)))
      (type (;4;) (func (result f64)))
      (type (;5;) (func (param i32) (result i32)))
      (type (;6;) (func (param i64) (result i64)))
      (type (;7;) (func (param f32) (result f32)))
      (type (;8;) (func (param f64) (result f64)))
      (type (;9;) (func (param f32 i32) (result i32)))
      (type (;10;) (func (param i32 i64) (result i64)))
      (type (;11;) (func (param f64 f32) (result f32)))
      (type (;12;) (func (param i64 f64) (result f64)))
      (type (;13;) (func (param i64 i64) (result i64)))
      (type (;14;) (func (param i64) (result i32)))
      (type (;15;) (func))
      (func (;0;) (type 1) (result i32)
        i32.const 306)
      (func (;1;) (type 2) (result i64)
        i64.const 356)
      (func (;2;) (type 3) (result f32)
        f32.const 0x1.e64p+11 (;=3890;))
      (func (;3;) (type 4) (result f64)
        f64.const 0x1.ec8p+11 (;=3940;))
      (func (;4;) (type 5) (param i32) (result i32)
        get_local 0)
      (func (;5;) (type 6) (param i64) (result i64)
        get_local 0)
      (func (;6;) (type 7) (param f32) (result f32)
        get_local 0)
      (func (;7;) (type 8) (param f64) (result f64)
        get_local 0)
      (func (;8;) (type 9) (param f32 i32) (result i32)
        get_local 1)
      (func (;9;) (type 10) (param i32 i64) (result i64)
        get_local 1)
      (func (;10;) (type 11) (param f64 f32) (result f32)
        get_local 1)
      (func (;11;) (type 12) (param i64 f64) (result f64)
        get_local 1)
      (func (;12;) (type 1) (result i32)
        call 0)
      (func (;13;) (type 2) (result i64)
        call 1)
      (func (;14;) (type 3) (result f32)
        call 2)
      (func (;15;) (type 4) (result f64)
        call 3)
      (func (;16;) (type 1) (result i32)
        i32.const 32
        call 4)
      (func (;17;) (type 2) (result i64)
        i64.const 64
        call 5)
      (func (;18;) (type 3) (result f32)
        f32.const 0x1.51eb86p+0 (;=1.32;)
        call 6)
      (func (;19;) (type 4) (result f64)
        f64.const 0x1.a3d70a3d70a3dp+0 (;=1.64;)
        call 7)
      (func (;20;) (type 1) (result i32)
        f32.const 0x1.00ccccp+5 (;=32.1;)
        i32.const 32
        call 8)
      (func (;21;) (type 2) (result i64)
        i32.const 32
        i64.const 64
        call 9)
      (func (;22;) (type 3) (result f32)
        f64.const 0x1p+6 (;=64;)
        f32.const 0x1p+5 (;=32;)
        call 10)
      (func (;23;) (type 4) (result f64)
        i64.const 64
        f64.const 0x1.0066666666666p+6 (;=64.1;)
        call 11)
      (func (;24;) (type 6) (param i64) (result i64)
        get_local 0
        i64.eqz
        if (result i64)  ;; label = @1
          i64.const 1
        else
          get_local 0
          get_local 0
          i64.const 1
          i64.sub
          call 24
          i64.mul
        end)
      (func (;25;) (type 13) (param i64 i64) (result i64)
        get_local 0
        i64.eqz
        if (result i64)  ;; label = @1
          get_local 1
        else
          get_local 0
          i64.const 1
          i64.sub
          get_local 0
          get_local 1
          i64.mul
          call 25
        end)
      (func (;26;) (type 6) (param i64) (result i64)
        get_local 0
        i64.const 1
        i64.le_u
        if (result i64)  ;; label = @1
          i64.const 1
        else
          get_local 0
          i64.const 2
          i64.sub
          call 26
          get_local 0
          i64.const 1
          i64.sub
          call 26
          i64.add
        end)
      (func (;27;) (type 14) (param i64) (result i32)
        get_local 0
        i64.eqz
        if (result i32)  ;; label = @1
          i32.const 44
        else
          get_local 0
          i64.const 1
          i64.sub
          call 28
        end)
      (func (;28;) (type 14) (param i64) (result i32)
        get_local 0
        i64.eqz
        if (result i32)  ;; label = @1
          i32.const 99
        else
          get_local 0
          i64.const 1
          i64.sub
          call 27
        end)
      (func (;29;) (type 15)
        call 29)
      (func (;30;) (type 15)
        call 31)
      (func (;31;) (type 15)
        call 30)
      (func (;32;) (type 1) (result i32)
        call 0
        i32.const 2
        i32.const 3
        select)
      (func (;33;) (type 1) (result i32)
        i32.const 2
        call 0
        i32.const 3
        select)
      (func (;34;) (type 1) (result i32)
        i32.const 2
        i32.const 3
        call 0
        select)
      (func (;35;) (type 1) (result i32)
        call 0
        if (result i32)  ;; label = @1
          i32.const 1
        else
          i32.const 2
        end)
      (func (;36;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          call 0
          i32.const 2
          br_if 0 (;@1;)
        end)
      (func (;37;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 2
          call 0
          br_if 0 (;@1;)
        end)
      (func (;38;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          call 0
          i32.const 2
          br_table 0 (;@1;) 0 (;@1;)
        end)
      (func (;39;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 2
          call 0
          br_table 0 (;@1;) 0 (;@1;)
        end)
      (func (;40;) (type 0) (param i32 i32) (result i32)
        get_local 0)
      (func (;41;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          call 0
          i32.const 2
          i32.const 0
          call_indirect (type 0)
        end)
      (func (;42;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 2
          call 0
          i32.const 0
          call_indirect (type 0)
        end)
      (func (;43;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          i32.const 2
          call 0
          call_indirect (type 0)
        end)
      (func (;44;) (type 15)
        call 0
        i32.const 1
        i32.store)
      (func (;45;) (type 15)
        i32.const 10
        call 0
        i32.store)
      (func (;46;) (type 1) (result i32)
        call 0
        memory.grow)
      (func (;47;) (type 1) (result i32)
        call 0
        return)
      (func (;48;) (type 15)
        call 0
        drop)
      (func (;49;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          call 0
          br 0 (;@1;)
        end)
      (func (;50;) (type 1) (result i32)
        (local i32)
        call 0
        set_local 0
        get_local 0)
      (func (;51;) (type 1) (result i32)
        (local i32)
        call 0
        tee_local 0)
      (func (;52;) (type 1) (result i32)
        call 0
        set_global 0
        get_global 0)
      (func (;53;) (type 1) (result i32)
        call 0
        i32.load)
      (table (;0;) 1 1 anyfunc)
      (memory (;0;) 1)
      (global (;0;) (mut i32) (i32.const 10))
      (export \"type-i32\" (func 12))
      (export \"type-i64\" (func 13))
      (export \"type-f32\" (func 14))
      (export \"type-f64\" (func 15))
      (export \"type-first-i32\" (func 16))
      (export \"type-first-i64\" (func 17))
      (export \"type-first-f32\" (func 18))
      (export \"type-first-f64\" (func 19))
      (export \"type-second-i32\" (func 20))
      (export \"type-second-i64\" (func 21))
      (export \"type-second-f32\" (func 22))
      (export \"type-second-f64\" (func 23))
      (export \"fac\" (func 24))
      (export \"fac-acc\" (func 25))
      (export \"fib\" (func 26))
      (export \"even\" (func 27))
      (export \"odd\" (func 28))
      (export \"runaway\" (func 29))
      (export \"mutual-runaway\" (func 30))
      (export \"as-select-first\" (func 32))
      (export \"as-select-mid\" (func 33))
      (export \"as-select-last\" (func 34))
      (export \"as-if-condition\" (func 35))
      (export \"as-br_if-first\" (func 36))
      (export \"as-br_if-last\" (func 37))
      (export \"as-br_table-first\" (func 38))
      (export \"as-br_table-last\" (func 39))
      (export \"as-call_indirect-first\" (func 41))
      (export \"as-call_indirect-mid\" (func 42))
      (export \"as-call_indirect-last\" (func 43))
      (export \"as-store-first\" (func 44))
      (export \"as-store-last\" (func 45))
      (export \"as-memory.grow-value\" (func 46))
      (export \"as-return-value\" (func 47))
      (export \"as-drop-operand\" (func 48))
      (export \"as-br-value\" (func 49))
      (export \"as-set_local-value\" (func 50))
      (export \"as-tee_local-value\" (func 51))
      (export \"as-set_global-value\" (func 52))
      (export \"as-load-operand\" (func 53))
      (elem (;0;) (i32.const 0) 40))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(&spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 202
fn c1_l202_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c1_l202_action_invoke");
    let result = instance.call("type-i32", &[]);
    assert_eq!(result, Ok(Some(Value::I32(306 as i32))));
    result.map(|_| ())
}

// Line 203
fn c2_l203_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c2_l203_action_invoke");
    let result = instance.call("type-i64", &[]);
    assert_eq!(result, Ok(Some(Value::I64(356 as i64))));
    result.map(|_| ())
}

// Line 204
fn c3_l204_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c3_l204_action_invoke");
    let result = instance.call("type-f32", &[]);
    assert_eq!(result, Ok(Some(Value::F32((3890.0f32).to_bits()))));
    result.map(|_| ())
}

// Line 205
fn c4_l205_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c4_l205_action_invoke");
    let result = instance.call("type-f64", &[]);
    assert_eq!(result, Ok(Some(Value::F64((3940.0f64).to_bits()))));
    result.map(|_| ())
}

// Line 207
fn c5_l207_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c5_l207_action_invoke");
    let result = instance.call("type-first-i32", &[]);
    assert_eq!(result, Ok(Some(Value::I32(32 as i32))));
    result.map(|_| ())
}

// Line 208
fn c6_l208_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c6_l208_action_invoke");
    let result = instance.call("type-first-i64", &[]);
    assert_eq!(result, Ok(Some(Value::I64(64 as i64))));
    result.map(|_| ())
}

// Line 209
fn c7_l209_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c7_l209_action_invoke");
    let result = instance.call("type-first-f32", &[]);
    assert_eq!(result, Ok(Some(Value::F32((1.32f32).to_bits()))));
    result.map(|_| ())
}

// Line 210
fn c8_l210_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c8_l210_action_invoke");
    let result = instance.call("type-first-f64", &[]);
    assert_eq!(result, Ok(Some(Value::F64((1.64f64).to_bits()))));
    result.map(|_| ())
}

// Line 212
fn c9_l212_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c9_l212_action_invoke");
    let result = instance.call("type-second-i32", &[]);
    assert_eq!(result, Ok(Some(Value::I32(32 as i32))));
    result.map(|_| ())
}

// Line 213
fn c10_l213_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c10_l213_action_invoke");
    let result = instance.call("type-second-i64", &[]);
    assert_eq!(result, Ok(Some(Value::I64(64 as i64))));
    result.map(|_| ())
}

// Line 214
fn c11_l214_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c11_l214_action_invoke");
    let result = instance.call("type-second-f32", &[]);
    assert_eq!(result, Ok(Some(Value::F32((32.0f32).to_bits()))));
    result.map(|_| ())
}

// Line 215
fn c12_l215_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c12_l215_action_invoke");
    let result = instance.call("type-second-f64", &[]);
    assert_eq!(result, Ok(Some(Value::F64((64.1f64).to_bits()))));
    result.map(|_| ())
}

// Line 217
fn c13_l217_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c13_l217_action_invoke");
    let result = instance.call("fac", &[Value::I64(0 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(1 as i64))));
    result.map(|_| ())
}

// Line 218
fn c14_l218_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c14_l218_action_invoke");
    let result = instance.call("fac", &[Value::I64(1 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(1 as i64))));
    result.map(|_| ())
}

// Line 219
fn c15_l219_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c15_l219_action_invoke");
    let result = instance.call("fac", &[Value::I64(5 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(120 as i64))));
    result.map(|_| ())
}

// Line 220
fn c16_l220_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c16_l220_action_invoke");
    let result = instance.call("fac", &[Value::I64(25 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(7034535277573963776 as i64))));
    result.map(|_| ())
}

// Line 221
fn c17_l221_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c17_l221_action_invoke");
    let result = instance.call("fac-acc", &[Value::I64(0 as i64), Value::I64(1 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(1 as i64))));
    result.map(|_| ())
}

// Line 222
fn c18_l222_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c18_l222_action_invoke");
    let result = instance.call("fac-acc", &[Value::I64(1 as i64), Value::I64(1 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(1 as i64))));
    result.map(|_| ())
}

// Line 223
fn c19_l223_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c19_l223_action_invoke");
    let result = instance.call("fac-acc", &[Value::I64(5 as i64), Value::I64(1 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(120 as i64))));
    result.map(|_| ())
}

// Line 225
fn c20_l225_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c20_l225_action_invoke");
    let result = instance.call("fac-acc", &[Value::I64(25 as i64), Value::I64(1 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(7034535277573963776 as i64))));
    result.map(|_| ())
}

// Line 229
fn c21_l229_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c21_l229_action_invoke");
    let result = instance.call("fib", &[Value::I64(0 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(1 as i64))));
    result.map(|_| ())
}

// Line 230
fn c22_l230_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c22_l230_action_invoke");
    let result = instance.call("fib", &[Value::I64(1 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(1 as i64))));
    result.map(|_| ())
}

// Line 231
fn c23_l231_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c23_l231_action_invoke");
    let result = instance.call("fib", &[Value::I64(2 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(2 as i64))));
    result.map(|_| ())
}

// Line 232
fn c24_l232_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c24_l232_action_invoke");
    let result = instance.call("fib", &[Value::I64(5 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(8 as i64))));
    result.map(|_| ())
}

// Line 233
fn c25_l233_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c25_l233_action_invoke");
    let result = instance.call("fib", &[Value::I64(20 as i64)]);
    assert_eq!(result, Ok(Some(Value::I64(10946 as i64))));
    result.map(|_| ())
}

// Line 235
fn c26_l235_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c26_l235_action_invoke");
    let result = instance.call("even", &[Value::I64(0 as i64)]);
    assert_eq!(result, Ok(Some(Value::I32(44 as i32))));
    result.map(|_| ())
}

// Line 236
fn c27_l236_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c27_l236_action_invoke");
    let result = instance.call("even", &[Value::I64(1 as i64)]);
    assert_eq!(result, Ok(Some(Value::I32(99 as i32))));
    result.map(|_| ())
}

// Line 237
fn c28_l237_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c28_l237_action_invoke");
    let result = instance.call("even", &[Value::I64(100 as i64)]);
    assert_eq!(result, Ok(Some(Value::I32(44 as i32))));
    result.map(|_| ())
}

// Line 238
fn c29_l238_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c29_l238_action_invoke");
    let result = instance.call("even", &[Value::I64(77 as i64)]);
    assert_eq!(result, Ok(Some(Value::I32(99 as i32))));
    result.map(|_| ())
}

// Line 239
fn c30_l239_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c30_l239_action_invoke");
    let result = instance.call("odd", &[Value::I64(0 as i64)]);
    assert_eq!(result, Ok(Some(Value::I32(99 as i32))));
    result.map(|_| ())
}

// Line 240
fn c31_l240_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c31_l240_action_invoke");
    let result = instance.call("odd", &[Value::I64(1 as i64)]);
    assert_eq!(result, Ok(Some(Value::I32(44 as i32))));
    result.map(|_| ())
}

// Line 241
fn c32_l241_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c32_l241_action_invoke");
    let result = instance.call("odd", &[Value::I64(200 as i64)]);
    assert_eq!(result, Ok(Some(Value::I32(99 as i32))));
    result.map(|_| ())
}

// Line 242
fn c33_l242_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c33_l242_action_invoke");
    let result = instance.call("odd", &[Value::I64(77 as i64)]);
    assert_eq!(result, Ok(Some(Value::I32(44 as i32))));
    result.map(|_| ())
}

// Line 244

// Line 245

// Line 247
fn c36_l247_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c36_l247_action_invoke");
    let result = instance.call("as-select-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(306 as i32))));
    result.map(|_| ())
}

// Line 248
fn c37_l248_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c37_l248_action_invoke");
    let result = instance.call("as-select-mid", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 249
fn c38_l249_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c38_l249_action_invoke");
    let result = instance.call("as-select-last", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 251
fn c39_l251_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c39_l251_action_invoke");
    let result = instance.call("as-if-condition", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 253
fn c40_l253_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c40_l253_action_invoke");
    let result = instance.call("as-br_if-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(306 as i32))));
    result.map(|_| ())
}

// Line 254
fn c41_l254_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c41_l254_action_invoke");
    let result = instance.call("as-br_if-last", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 256
fn c42_l256_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c42_l256_action_invoke");
    let result = instance.call("as-br_table-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(306 as i32))));
    result.map(|_| ())
}

// Line 257
fn c43_l257_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c43_l257_action_invoke");
    let result = instance.call("as-br_table-last", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 259
fn c44_l259_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c44_l259_action_invoke");
    let result = instance.call("as-call_indirect-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(306 as i32))));
    result.map(|_| ())
}

// Line 260
fn c45_l260_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c45_l260_action_invoke");
    let result = instance.call("as-call_indirect-mid", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 261
fn c46_l261_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c46_l261_action_invoke");
    let result = instance.call("as-call_indirect-last", &[]);
    
    result.map(|_| ())
}

#[test]
fn c46_l261_assert_trap() {
    let mut instance = create_module_1();
    let result = c46_l261_action_invoke(&mut*instance);
    assert!(result.is_err());
}

// Line 263
fn c47_l263_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c47_l263_action_invoke");
    let result = instance.call("as-store-first", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 264
fn c48_l264_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c48_l264_action_invoke");
    let result = instance.call("as-store-last", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 266
fn c49_l266_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c49_l266_action_invoke");
    let result = instance.call("as-memory.grow-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 267
fn c50_l267_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c50_l267_action_invoke");
    let result = instance.call("as-return-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(306 as i32))));
    result.map(|_| ())
}

// Line 268
fn c51_l268_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c51_l268_action_invoke");
    let result = instance.call("as-drop-operand", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 269
fn c52_l269_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c52_l269_action_invoke");
    let result = instance.call("as-br-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(306 as i32))));
    result.map(|_| ())
}

// Line 270
fn c53_l270_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c53_l270_action_invoke");
    let result = instance.call("as-set_local-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(306 as i32))));
    result.map(|_| ())
}

// Line 271
fn c54_l271_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c54_l271_action_invoke");
    let result = instance.call("as-tee_local-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(306 as i32))));
    result.map(|_| ())
}

// Line 272
fn c55_l272_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c55_l272_action_invoke");
    let result = instance.call("as-set_global-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(306 as i32))));
    result.map(|_| ())
}

// Line 273
fn c56_l273_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c56_l273_action_invoke");
    let result = instance.call("as-load-operand", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 278
#[test]
fn c57_l278_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 3, 2, 0, 0, 10, 10, 2, 5, 0, 16, 1, 69, 11, 2, 0, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 285
#[test]
fn c58_l285_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 8, 2, 96, 0, 0, 96, 0, 1, 126, 3, 3, 2, 0, 1, 10, 12, 2, 5, 0, 16, 1, 69, 11, 4, 0, 66, 1, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 293
#[test]
fn c59_l293_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 8, 2, 96, 0, 0, 96, 1, 127, 0, 3, 3, 2, 0, 1, 10, 9, 2, 4, 0, 16, 1, 11, 2, 0, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 300
#[test]
fn c60_l300_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 2, 124, 127, 0, 3, 3, 2, 0, 1, 10, 9, 2, 4, 0, 16, 1, 11, 2, 0, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 307
#[test]
fn c61_l307_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 3, 2, 0, 0, 10, 11, 2, 6, 0, 65, 1, 16, 1, 11, 2, 0, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 314
#[test]
fn c62_l314_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 3, 2, 0, 0, 10, 20, 2, 15, 0, 68, 0, 0, 0, 0, 0, 0, 0, 64, 65, 1, 16, 1, 11, 2, 0, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 322
#[test]
fn c63_l322_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 2, 127, 127, 0, 3, 3, 2, 0, 1, 10, 12, 2, 7, 0, 1, 65, 1, 16, 1, 11, 2, 0, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 329
#[test]
fn c64_l329_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 2, 127, 127, 0, 3, 3, 2, 0, 1, 10, 12, 2, 7, 0, 65, 1, 1, 16, 1, 11, 2, 0, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 336
#[test]
fn c65_l336_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 2, 127, 124, 0, 3, 3, 2, 0, 1, 10, 20, 2, 15, 0, 68, 0, 0, 0, 0, 0, 0, 240, 63, 65, 1, 16, 1, 11, 2, 0, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 343
#[test]
fn c66_l343_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 9, 2, 96, 0, 0, 96, 2, 124, 127, 0, 3, 3, 2, 0, 1, 10, 20, 2, 15, 0, 65, 1, 68, 0, 0, 0, 0, 0, 0, 240, 63, 16, 1, 11, 2, 0, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 354
#[test]
fn c67_l354_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 6, 1, 4, 0, 16, 1, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 358
#[test]
fn c68_l358_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 10, 1, 8, 0, 16, 148, 152, 219, 226, 3, 11];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
    c1_l202_action_invoke(&mut instance);
    c2_l203_action_invoke(&mut instance);
    c3_l204_action_invoke(&mut instance);
    c4_l205_action_invoke(&mut instance);
    c5_l207_action_invoke(&mut instance);
    c6_l208_action_invoke(&mut instance);
    c7_l209_action_invoke(&mut instance);
    c8_l210_action_invoke(&mut instance);
    c9_l212_action_invoke(&mut instance);
    c10_l213_action_invoke(&mut instance);
    c11_l214_action_invoke(&mut instance);
    c12_l215_action_invoke(&mut instance);
    c13_l217_action_invoke(&mut instance);
    c14_l218_action_invoke(&mut instance);
    c15_l219_action_invoke(&mut instance);
    c16_l220_action_invoke(&mut instance);
    c17_l221_action_invoke(&mut instance);
    c18_l222_action_invoke(&mut instance);
    c19_l223_action_invoke(&mut instance);
    c20_l225_action_invoke(&mut instance);
    c21_l229_action_invoke(&mut instance);
    c22_l230_action_invoke(&mut instance);
    c23_l231_action_invoke(&mut instance);
    c24_l232_action_invoke(&mut instance);
    c25_l233_action_invoke(&mut instance);
    c26_l235_action_invoke(&mut instance);
    c27_l236_action_invoke(&mut instance);
    c28_l237_action_invoke(&mut instance);
    c29_l238_action_invoke(&mut instance);
    c30_l239_action_invoke(&mut instance);
    c31_l240_action_invoke(&mut instance);
    c32_l241_action_invoke(&mut instance);
    c33_l242_action_invoke(&mut instance);
    c36_l247_action_invoke(&mut instance);
    c37_l248_action_invoke(&mut instance);
    c38_l249_action_invoke(&mut instance);
    c39_l251_action_invoke(&mut instance);
    c40_l253_action_invoke(&mut instance);
    c41_l254_action_invoke(&mut instance);
    c42_l256_action_invoke(&mut instance);
    c43_l257_action_invoke(&mut instance);
    c44_l259_action_invoke(&mut instance);
    c45_l260_action_invoke(&mut instance);
    c47_l263_action_invoke(&mut instance);
    c48_l264_action_invoke(&mut instance);
    c49_l266_action_invoke(&mut instance);
    c50_l267_action_invoke(&mut instance);
    c51_l268_action_invoke(&mut instance);
    c52_l269_action_invoke(&mut instance);
    c53_l270_action_invoke(&mut instance);
    c54_l271_action_invoke(&mut instance);
    c55_l272_action_invoke(&mut instance);
    c56_l273_action_invoke(&mut instance);
}
