use deno_runtime::{permissions::Permissions, worker::MainWorker};
use deno_core::{resolve_url_or_path, ModuleSpecifier};

use deno::program_state::ProgramState;
use deno::create_main_worker;
use deno::file_fetcher::File;
use deno::media_type::MediaType;

use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use anyhow::{Result, Error};
use futures::FutureExt;
use std::sync::Arc;
use deno_core::error::AnyError;

pub enum Flavor { JS, JSX, TS, TSX }

pub struct Runner {
    runtime: Runtime,
    state: Arc<ProgramState>,
    permissions: Permissions,
    specifier: ModuleSpecifier,
}

impl Runner {
    async fn eval(&self, worker: &mut MainWorker, specifier: ModuleSpecifier) -> Result<(), AnyError> {
        worker.execute_module(&specifier).await?;
        worker.execute("window.dispatchEvent(new Event('load'))")?;
        worker.run_event_loop(false).await?;
        worker.execute("window.dispatchEvent(new Event('unload'))")?;

        Ok(())
    }

    pub fn run(&self, source: String, flavor: Flavor, _timeout: Duration) -> Result<String, Error> {
        let mut worker: MainWorker = create_main_worker(
            &self.state,
            self.specifier.clone(),
            self.permissions.clone(),
            false
        );

        self.state.file_fetcher.insert_cached(File {
            local: self.specifier.clone().to_file_path().unwrap(),
            maybe_types: None,
            media_type: match flavor {
                Flavor::JS => MediaType::JavaScript,
                Flavor::JSX => MediaType::Jsx,
                Flavor::TS => MediaType::TypeScript,
                Flavor::TSX => MediaType::Tsx
            },
            source,
            specifier: self.specifier.clone()
        });

        let future = self.eval(&mut worker, self.specifier.clone());

        let _result: Result<(), AnyError> = self.runtime.block_on(future.boxed_local());

        Result::Ok("".to_string())
    }

    pub fn new() -> Runner {
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

        Runner { runtime, state, permissions, specifier }
    }
}