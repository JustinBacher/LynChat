fn main() {
    // This ensures the OUT_DIR environment variable is set for Tauri's generate_context! macro
    tauri_build::build();
}