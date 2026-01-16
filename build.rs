use std::path::PathBuf;

fn main() {
    cmd_lib::run_cmd! {
        cd web;
        bun install;
        bun run build;
    }
    .unwrap();

    memory_serve::load_directory(PathBuf::from("web").join("out"));
}
