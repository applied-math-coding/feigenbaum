#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod feigenbaum;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![plot_feigenbaum_diag])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn plot_feigenbaum_diag(a: f64, b: f64, dark_iters: usize, n_r: usize) -> (Vec<f64>, Vec<f64>) {
    let delta = (b - a) / n_r as f64;
    feigenbaum::plot(a, b, dark_iters, 1000, delta, 1)
    // feigenbaum::plot(a, b, 100, 100, 0.1,1)
}
