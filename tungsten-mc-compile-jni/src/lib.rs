//! JNI exports for the Java class `tungsten.compiler.NativeCompiler`. See `docs/jni-api.md`.
//!
//! Every export runs inside [`guard`]: a panic or a failed JNI call throws
//! `tungsten.compiler.TungstenCompilerException` and the export returns a neutral value (0 / null).
//! Nothing unwinds into the JVM.

use std::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind};

use jni::JNIEnv;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jlong, jstring};
use tungsten_mc_compile::{CompilerSession, compiler_version};

pub mod json;

const EXCEPTION_CLASS: &str = "tungsten/compiler/TungstenCompilerException";

/// Runs `f`, turning a panic into its message.
pub fn catch_panic<R>(f: impl FnOnce() -> R) -> Result<R, String> {
    catch_unwind(AssertUnwindSafe(f)).map_err(|payload| panic_message(payload.as_ref()))
}

fn panic_message(payload: &(dyn Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        format!("compiler panicked: {}", s)
    } else if let Some(s) = payload.downcast_ref::<String>() {
        format!("compiler panicked: {}", s)
    } else {
        "compiler panicked".to_string()
    }
}

/// Runs an export's body. An `Err` or a panic throws `TungstenCompilerException` (unless a Java
/// exception is already pending) and returns `neutral`.
fn guard<'local, R>(
    env: &mut JNIEnv<'local>,
    neutral: R,
    f: impl FnOnce(&mut JNIEnv<'local>) -> Result<R, String>,
) -> R {
    match catch_panic(|| f(env)).and_then(|r| r) {
        Ok(value) => value,
        Err(message) => {
            if !env.exception_check().unwrap_or(true) {
                // If even this fails there is nothing left to report to.
                let _ = env.throw_new(EXCEPTION_CLASS, message);
            }
            neutral
        }
    }
}

fn jni_error(e: jni::errors::Error) -> String {
    format!("JNI call failed: {}", e)
}

/// The session behind a handle from `create`.
///
/// # Safety
///
/// `handle` must come from `create` and not have been passed to `dispose`, and the Java side must
/// not use it from two threads at once.
unsafe fn session<'a>(handle: jlong) -> Result<&'a mut CompilerSession, String> {
    if handle == 0 {
        return Err("invalid compiler handle 0".into());
    }
    Ok(unsafe { &mut *(handle as *mut CompilerSession) })
}

fn get_string(env: &mut JNIEnv, s: &JString) -> Result<String, String> {
    Ok(env.get_string(s).map_err(jni_error)?.into())
}

fn new_string(env: &mut JNIEnv, s: &str) -> Result<jstring, String> {
    Ok(env.new_string(s).map_err(jni_error)?.into_raw())
}

/// `static native long create()`
#[unsafe(no_mangle)]
pub extern "system" fn Java_tungsten_compiler_NativeCompiler_create<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jlong {
    guard(&mut env, 0, |_| Ok(Box::into_raw(Box::new(CompilerSession::new())) as jlong))
}

/// `static native void loadResource(long handle, String id, byte[] bytes)`
#[unsafe(no_mangle)]
pub extern "system" fn Java_tungsten_compiler_NativeCompiler_loadResource<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    id: JString<'local>,
    bytes: JByteArray<'local>,
) {
    guard(&mut env, (), |env| {
        let session = unsafe { session(handle) }?;
        let id = get_string(env, &id)?;
        let bytes = env.convert_byte_array(&bytes).map_err(jni_error)?;
        session.load_resource(&id, &bytes);
        Ok(())
    })
}

/// `static native void setRoot(long handle, String id)`
#[unsafe(no_mangle)]
pub extern "system" fn Java_tungsten_compiler_NativeCompiler_setRoot<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    id: JString<'local>,
) {
    guard(&mut env, (), |env| {
        let session = unsafe { session(handle) }?;
        let id = get_string(env, &id)?;
        session.set_root(&id);
        Ok(())
    })
}

/// `static native String compile(long handle)`: the result document (see [`json`]).
#[unsafe(no_mangle)]
pub extern "system" fn Java_tungsten_compiler_NativeCompiler_compile<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
) -> jstring {
    guard(&mut env, std::ptr::null_mut(), |env| {
        let session = unsafe { session(handle) }?;
        let document = json::result_json(&session.compile());
        new_string(env, &document)
    })
}

/// `static native String version()`
#[unsafe(no_mangle)]
pub extern "system" fn Java_tungsten_compiler_NativeCompiler_version<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    guard(&mut env, std::ptr::null_mut(), |env| new_string(env, compiler_version()))
}

/// `static native void dispose(long handle)`; 0 is ignored.
#[unsafe(no_mangle)]
pub extern "system" fn Java_tungsten_compiler_NativeCompiler_dispose<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
) {
    guard(&mut env, (), |_| {
        if handle != 0 {
            drop(unsafe { Box::from_raw(handle as *mut CompilerSession) });
        }
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catches_a_forced_panic() {
        let result: Result<(), String> = catch_panic(|| panic!("boom"));
        assert_eq!(result, Err("compiler panicked: boom".to_string()));

        let n = 3;
        let result: Result<(), String> = catch_panic(|| panic!("formatted {}", n));
        assert_eq!(result, Err("compiler panicked: formatted 3".to_string()));
    }

    #[test]
    fn passes_values_through() {
        assert_eq!(catch_panic(|| 42), Ok(42));
    }
}
