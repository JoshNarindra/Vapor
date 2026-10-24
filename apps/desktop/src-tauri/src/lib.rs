use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SystemMemoryReading {
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SystemMemoryFailure {
    kind: &'static str,
    message: String,
}

#[tauri::command]
async fn read_system_memory() -> Result<SystemMemoryReading, SystemMemoryFailure> {
    let snapshot = steam_os_hub_process_monitor::read_system_memory().map_err(|error| {
        SystemMemoryFailure {
            kind: if error.kind() == std::io::ErrorKind::Unsupported {
                "unsupported"
            } else {
                "readFailed"
            },
            message: error.to_string(),
        }
    })?;

    Ok(SystemMemoryReading {
        total_bytes: snapshot.total_bytes(),
        available_bytes: snapshot.available_bytes(),
        used_bytes: snapshot.used_bytes(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_system_memory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
