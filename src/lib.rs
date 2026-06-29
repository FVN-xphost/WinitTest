slint::include_modules!();
use slint::{Timer, TimerMode};
mod utils;

use utils::config::Config;
#[allow(unused)]
use utils::constant::*;
#[cfg(not(target_os = "android"))]
use utils::path::set_config_local_dir;
// use std::fs;
// use std::path::PathBuf;
use std::time::Duration;
fn run() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;
    let mut config = Config::new();
    let _ = config.load();
    let settings_global = app.global::<Settings>();
    settings_global.set_volume(config.volume);
    settings_global.set_text_speed(config.text_speed);
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        settings_global.set_is_fullscreen(config.is_fullscreen);
        app.window().set_fullscreen(config.is_fullscreen);
    }
    let app_weak = app.as_weak();
    let timer = Timer::default();
    timer.start(
        TimerMode::SingleShot,
        Duration::from_millis(200),
        move || {
            if let Some(app) = app_weak.upgrade() {
                app.set_bg_opacity(1.0);
            }
        },
    );

    let app_weak = app.as_weak();
    let timer = Timer::default();
    timer.start(
        TimerMode::SingleShot,
        Duration::from_millis(3000),
        move || {
            if let Some(app) = app_weak.upgrade() {
                app.set_buttons_opacity(1.0);
            }
        },
    );

    let app_weak = app.as_weak();
    let start_timer = Timer::default();
    app.on_start_game_clicked(move || {
        if let Some(app) = app_weak.upgrade() {
            app.set_overlay_opacity(1.0);
            let app_weak = app.as_weak();
            start_timer.start(
                TimerMode::SingleShot,
                Duration::from_millis(2000),
                move || {
                    if let Some(_) = app_weak.upgrade() {
                        println!("开始加载！...");
                    }
                },
            );
        }
    });

    app.on_exit_clicked(move || {
        std::process::exit(0);
    });

    let app_weak = app.as_weak();
    app.on_settings_clicked(move || {
        if let Some(app) = app_weak.upgrade() {
            app.set_settings_opacity(1.0);
            app.set_settings_show(true);
        }
    });

    let app_weak = app.as_weak();
    let return_timer = Timer::default();
    app.on_return_clicked(move || {
        if let Some(app) = app_weak.upgrade() {
            app.set_settings_opacity(0.0);
            let app_weak = app.as_weak();
            return_timer.start(
                TimerMode::SingleShot,
                Duration::from_millis(1000),
                move || {
                    if let Some(_) = app_weak.upgrade() {
                        app.set_settings_show(false);
                    }
                },
            );
        }
    });

    let app_weak = app.as_weak();
    app.on_save_settings(move || {
        let app = app_weak.upgrade().unwrap();
        let settings_global = app.global::<Settings>();
        let mut current_config = Config {
            volume: settings_global.get_volume(),
            text_speed: settings_global.get_text_speed(),
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            is_fullscreen: settings_global.get_is_fullscreen(),
        };
        let _ = current_config.save();
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        app.window().set_fullscreen(current_config.is_fullscreen);
    });

    app.run()?;
    Ok(())
}

// Android 日志
#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("[RenRs]"),
    );
}
// 非 Android 日志
#[cfg(not(target_os = "android"))]
fn init_logging() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
}

// 桌面入口
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn desktop_main() {
    init_logging();
    set_config_local_dir().expect("Lifecycle run set config local dir is failed in Android!");
    run().expect("Lifecycle run started is failed in Desktop!");
}
// Android 入口
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: slint::android::android_activity::AndroidApp) {
    init_logging();
    if let Some(path) = app.internal_data_path() {
        utils::path::CONFIG_LOCAL_DIR
            .set(path)
            .expect("Lifecycle run set config local dir is failed in Android!");
    } else {
        panic!("Lifecycle run get internal data path is failed in Android");
    }
    slint::android::init(app).expect("Lifecycle run init is failed in Android!");
    run().expect("Lifecycle run started is failed in Android!");
}

// iOS 入口
#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn ios_main() {
    init_logging();
    set_config_local_dir().expect("Lifecycle run set config local dir is failed in Android!");
    run().expect("Lifecycle run started is failed in iOS!");
}
