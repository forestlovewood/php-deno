use tokio::runtime::{Builder, Runtime};
use anyhow::{Result, Error};
use std::time::Duration;
use futures::FutureExt;
use std::sync::Arc;

use deno::{program_state::ProgramState, create_main_worker, file_fetcher::File, media_type::MediaType};
use deno_core::{resolve_url_or_path, ModuleSpecifier, error::AnyError};
use deno_runtime::{permissions::Permissions, worker::MainWorker};
use std::os::raw::{c_char, c_int, c_double, c_long};
use std::ffi::CString;

#[repr(C)]
pub enum Type { JS, JSX, TS, TSX }

pub struct Deno {
    runtime: Runtime,
    state: Arc<ProgramState>,
    permissions: Permissions,
    specifier: ModuleSpecifier,
}

impl Deno {
    async fn eval(&self, worker: &mut MainWorker, specifier: ModuleSpecifier) -> Result<(), AnyError> {
        worker.execute_module(&specifier).await?;
        worker.execute("window.dispatchEvent(new Event('load'))")?;
        worker.run_event_loop(false).await?;
        worker.execute("window.dispatchEvent(new Event('unload'))")?;

        Ok(())
    }

    pub fn run(&self, source: String, media_type: Type, _timeout: Duration) -> Result<String, Error> {
        let mut worker: MainWorker = create_main_worker(
            &self.state,
            self.specifier.clone(),
            self.permissions.clone(),
            false
        );

        self.state.file_fetcher.insert_cached(File {
            local: self.specifier.clone().to_file_path().unwrap(),
            maybe_types: None,
            media_type: match media_type {
                Type::JS => MediaType::JavaScript,
                Type::JSX => MediaType::Jsx,
                Type::TS => MediaType::TypeScript,
                Type::TSX => MediaType::Tsx
            },
            source,
            specifier: self.specifier.clone()
        });

        let future = self.eval(&mut worker, self.specifier.clone());

        let _result: Result<(), AnyError> = self.runtime.block_on(future.boxed_local());

        Result::Ok("".to_string())
    }

    pub fn new() -> Deno {
        let runtime = Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .max_blocking_threads(32)
            .build()
            .unwrap();

        let state: Arc<ProgramState> = runtime.block_on(async {
            ProgramState::build(Default::default()).await
        }.boxed_local()).unwrap();

        let permissions = Permissions::allow_all();
        let specifier = resolve_url_or_path("./$deno$stdin.ts").unwrap();

        Deno { runtime, state, permissions, specifier }
    }
}

#[no_mangle]
pub extern "C" fn execute(source: *const c_char, media_type: *const Type, timeout: *const c_long) -> *const c_char {
    let result = CString::new("execute").expect("CString::new failed");
    result.as_ptr()
}

#[no_mangle]
pub extern "C" fn execute_file(path: *const c_char, media_type: *const Type, timeout: *const c_long) -> *const c_char {
    let result = CString::new("execute_file").expect("CString::new failed");
    result.as_ptr()
}