use slint::ComponentHandle;
use std::{path::PathBuf, time::Duration};

fn snapshot(window: &SpikeWindow, language: &str) {
    let pixels = window.window().take_snapshot().expect("capture rendered Wayland window");
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/captures");
    std::fs::create_dir_all(&directory).expect("create capture directory");
    let path = directory.join(format!("{language}.png"));
    image::save_buffer(&path, pixels.as_bytes(), pixels.width(), pixels.height(), image::ColorType::Rgba8)
        .expect("encode screenshot");
    println!("Rendered {language}: {}", path.display());
}

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let window = SpikeWindow::new()?;
    // Start deterministically; subsequent switches use the bundled gettext catalog.
    slint::select_bundled_translation("en").expect("English is the source language");
    let weak = window.as_weak();
    window.on_switch_language(move || {
        if let Some(window) = weak.upgrade() {
            let spanish = !window.get_spanish();
            if let Err(error) = slint::select_bundled_translation(if spanish { "es" } else { "en" }) {
                eprintln!("Could not select translation: {error}");
                return;
            }
            window.set_spanish(spanish);
            window.set_prompt_active(!window.get_prompt_active());
        }
    });
    // Self-driven visual experiment, without injecting input into the desktop.
    if std::env::args().any(|argument| argument == "--capture") {
        let weak = window.as_weak();
        slint::Timer::single_shot(Duration::from_millis(250), move || {
            let window = weak.upgrade().expect("window alive");
            snapshot(&window, "en");
            window.invoke_switch_language();
            slint::Timer::single_shot(Duration::from_millis(250), move || {
                snapshot(&window, "es");
                slint::quit_event_loop().expect("stop capture run");
            });
        });
    }
    window.run()
}
