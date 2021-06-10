use crate::deno::{
    Flavor,
    Runner
};

use std::time::Duration;

use ext_php_rs::{
    info_table_end,
    info_table_row,
    info_table_start,
    parse_args,
    php::{
        args::Arg,
        class::{
            ClassBuilder,
            ClassEntry
        },
        enums::DataType,
        exceptions::throw,
        execution_data::ExecutionData,
        flags::MethodFlags,
        function::FunctionBuilder,
        module::{
            ModuleBuilder,
            ModuleEntry
        },
        types::{
            long::ZendLong,
            zval::Zval,
            object::{
                ZendObject,
                ZendClassObject,
                ZendObjectHandlers,
                ZendObjectOverride
            }
        }
    }
};

struct Deno {
    runner: Runner
}

static mut DENO_HANDLERS: Option<*mut ZendObjectHandlers> = None;

impl ZendObjectOverride for Deno {
    extern "C" fn create_object(ce: *mut ClassEntry) -> *mut ZendObject {
        unsafe {
            if DENO_HANDLERS.is_none() {
                DENO_HANDLERS = Some(ZendObjectHandlers::init::<Deno>());
            }

            ZendClassObject::<Deno>::new_ptr(ce, DENO_HANDLERS.unwrap())
        }
    }
}

impl Deno {
    pub extern "C" fn constructor(_execute_data: &mut ExecutionData, _retval: &mut Zval) {}

    pub extern "C" fn execute(execute_data: &mut ExecutionData, retval: &mut Zval) {
        let mut source = Arg::new("source", DataType::String);
        let mut flavor = Arg::new("flavor", DataType::Long).default(3_i64);
        let mut timeout = Arg::new("timeout", DataType::Long).default(0_i64);

        parse_args!(execute_data, source; flavor, timeout);

        let source = String::from(source.val::<String>().unwrap());
        let flavor: Flavor = match i64::from(flavor.val::<ZendLong>().unwrap()) {
            0_i64 => Flavor::JSX,
            1_i64 => Flavor::TSX,
            2_i64 => Flavor::JS,
            _ => Flavor::TS,
        };
        let timeout: Duration = Duration::from_secs(i64::from(timeout.val::<ZendLong>().unwrap()) as u64);

        let object: &mut ZendClassObject<Deno> = ZendClassObject::<Deno>::get(execute_data).unwrap();

        let result = object.runner.run(source, flavor, timeout).unwrap_or_else(|error| {
            throw(
                ClassEntry::error_exception(),
                &error.to_string(),
            );

            "".to_string()
        });

        retval.set_string(result);
    }
}

impl Default for Deno {
    fn default() -> Self {
        Deno {
            runner: Runner::new()
        }
    }
}

#[no_mangle]
pub extern "C" fn module_init(_type: i32, _module_number: i32) -> i32 {
    ClassBuilder::new("Deno")
        .method(
            FunctionBuilder::constructor(Deno::constructor).build(),
            MethodFlags::Public
        )
        .method(
            FunctionBuilder::new("execute", Deno::execute)
                .arg(Arg::new("source", DataType::String))
                .not_required()
                .arg(Arg::new("flavor", DataType::Long).default(3_i64))
                .arg(Arg::new("timeout", DataType::Long).default(0_i64))
                .returns(DataType::String, false, false)
                .build(),
            MethodFlags::Public
        )
        .constant("JSX", 0_i64)
        .constant("TSX", 1_i64)
        .constant("JS", 2_i64)
        .constant("TS", 3_i64)
        .object_override::<Deno>()
        .build();

    0
}

#[no_mangle]
pub extern "C" fn get_module() -> *mut ext_php_rs::php::module::ModuleEntry {
    ModuleBuilder::new("ext-deno", "1.11.0")
        .info_function(php_module_info)
        .startup_function(module_init)
        .build()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("deno extension", "enabled");
    info_table_end!();
}