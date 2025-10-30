use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::sync::Arc;

use clap::{ArgAction, Parser};
use comfy_table::{presets::UTF8_FULL, Cell, CellAlignment, Table};
use humansize::{format_size, BINARY};
use tokio::task;
use tokio::sync::{mpsc, Semaphore};
use ignore::{WalkBuilder, WalkState};

#[derive(Parser, Debug)]
#[command(name = "rarscanner", version, about = "Skanna filsystemet efter .rar-filer")]
struct Cli {
    /// Rotkatalog(er) att skanna
    #[arg(value_name = "PATH", default_values_t = vec![".".to_string()])]
    roots: Vec<String>,

    /// Max antal samtidiga I/O-uppgifter
    #[arg(short = 'c', long = "concurrency", default_value_t = num_cpus::get() * 4)]
    concurrency: usize,

    /// Följ symboliska länkar
    #[arg(long = "follow-symlinks", action = ArgAction::SetTrue)]
    follow_symlinks: bool,
}

#[derive(Debug, Clone)]
struct FoundFile {
    path: PathBuf,
    size: u64,
    modified: Option<SystemTime>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let semaphore = Arc::new(Semaphore::new(cli.concurrency.max(1)));
    let (tx, mut rx) = mpsc::unbounded_channel::<FoundFile>();

    // Använd ignore::WalkBuilder (parallell walker) i blocking tråd per rot
    for root in &cli.roots {
        let root_path = PathBuf::from(root);
        let tx_clone = tx.clone();
        let follow = cli.follow_symlinks;
        let sem_clone = semaphore.clone();
        task::spawn_blocking(move || {
            let mut walker = WalkBuilder::new(&root_path);
            walker.follow_links(follow);
            walker.hidden(false);
            walker.git_ignore(true).git_global(true).git_exclude(true);

            // Parallell bearbetning via walker.build_parallel
            let tx_inner = tx_clone;
            let sem_arc = sem_clone;
            walker.build_parallel().run(|| {
                let tx_thread = tx_inner.clone();
                let sem_thread = sem_arc.clone();
                Box::new(move |result| {
                    match result {
                        Ok(entry) => {
                            if let Some(ft) = entry.file_type() {
                                if ft.is_file() {
                                    let path = entry.path().to_path_buf();
                                    if has_rar_extension(&path) {
                                        // throttla metadata med semafor för att inte översvämma I/O
                                        let _ = sem_thread.clone().try_acquire_owned();
                                        if let Ok(md) = std::fs::metadata(&path) {
                                            let _ = tx_thread.send(FoundFile {
                                                path: path.clone(),
                                                size: md.len(),
                                                modified: md.modified().ok(),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        Err(_err) => {
                            // ignorerar fel på enskilda entries
                        }
                    }
                    WalkState::Continue
                })
            });
        });
    }
    drop(tx);

    let mut found: Vec<FoundFile> = Vec::new();
    while let Some(file) = rx.recv().await {
        found.push(file);
    }

    found.sort_by(|a, b| a.path.cmp(&b.path));
    print_table(&found).await;

    Ok(())
}

// rekursiv scan_dir togs bort till förmån för ignore::WalkBuilder

fn has_rar_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("rar"))
        .unwrap_or(false)
}

async fn print_table(found: &[FoundFile]) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![
        Cell::new("Fil").set_alignment(CellAlignment::Left),
        Cell::new("Storlek").set_alignment(CellAlignment::Right),
        Cell::new("Ändrad").set_alignment(CellAlignment::Center),
    ]);

    for f in found {
        let size = format_size(f.size, BINARY);
        let modified_str = match f.modified {
            Some(st) => match chrono::DateTime::<chrono::Local>::from(st).format("%Y-%m-%d %H:%M:%S").to_string() {
                s => s,
            },
            None => "-".to_string(),
        };
        table.add_row(vec![
            Cell::new(f.path.display().to_string()).set_alignment(CellAlignment::Left),
            Cell::new(size).set_alignment(CellAlignment::Right),
            Cell::new(modified_str).set_alignment(CellAlignment::Center),
        ]);
    }

    println!("{}", table);
}
