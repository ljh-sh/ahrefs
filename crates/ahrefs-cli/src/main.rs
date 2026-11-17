mod cli;
mod commands;
mod output;

use std::io::Write;
use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let cli = cli::Cli::parse();

    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(err) => {
            eprintln!(
                "{}",
                output::error("runtime_failed", &err.to_string(), None)
            );
            return ExitCode::FAILURE;
        }
    };

    let rendered = match runtime.block_on(commands::run(cli)) {
        Ok(rendered) => rendered,
        Err(err) => {
            eprintln!("{}", output::error(err.kind, &err.message, err.status));
            return ExitCode::FAILURE;
        }
    };

    let mut stdout = std::io::stdout().lock();
    if let Err(err) = writeln!(stdout, "{rendered}") {
        // A closed pipe means the reader stopped early, such as `| head`.
        if err.kind() != std::io::ErrorKind::BrokenPipe {
            eprintln!("{}", output::error("write_failed", &err.to_string(), None));
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
