use kv_log_macro as log;

fn main() {
    let filter = femme::Builder::new()
        .filter_module("filter", femme::LevelFilter::Info)
        .build();
    femme::with_filter(filter);
    log::error!("Buffer has to be 16 bytes in length");
    log::warn!("Unauthorized access attempt", { route: "/login", user_id: "827756627", });
    log::info!("Server listening", { port: "8080" });
    log::info!("Request handled", { method: "GET", path: "/foo/bar", status: 200, elapsed: "4ms" });
    log::debug!("Getting String as bson value type");
    log::trace!("Task spawned", {task_id: "567", thread_id: "12"});
    log::info!(r#"raw " fun with JSON"#);
    log::info!("n\ne\nw\nl\ni\nn\ne\n");
}
