slint::include_modules!();
use slint::{Timer, TimerMode};
mod utils;

use utils::config::Config;
use utils::constant::*;
// use std::fs;
// use std::path::PathBuf;
use std::time::Duration;
fn run() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;
    // let config = Config::new();
    // if let Some(config) = config {}
    // let settings_global = app.global::<Settings>();
    // settings_global.set_volume(config.volume);
    // settings_global.set_text_speed(config.text_speed);
    // settings_global.set_fullscreen(config.fullscreen);

    // app.window().set_fullscreen(config.fullscreen)?;

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

    // let app_weak = app.as_weak();
    // app.on_save_settings(move || {
    //     let app = app_weak.upgrade().unwrap();
    //     let settings_global = app.global::<Settings>();

    //     let current_config = Config {
    //         volume: settings_global.get_volume(),
    //         text_speed: settings_global.get_text_speed(),
    //         fullscreen: settings_global.get_fullscreen(),
    //     };

    //     current_config.save();
    //     app.window().set_fullscreen(current_config.fullscreen)?;

    //     Ok::<(), slint::PlatformError>(())
    // })
    // .unwrap();

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
#[cfg(not(target_os = "android"))]
pub fn desktop_main() {
    init_logging();
    run().expect("Lifecycle run started is failed in Desktop!");
    // let event_loop = winit::event_loop::EventLoop::<AppEvent>::with_user_event()
    //     .build()
    //     .unwrap();
    // let event_proxy = event_loop.create_proxy();
    // std::thread::spawn(move || {
    //     loop {
    //         std::thread::sleep(std::time::Duration::from_millis(1000 / WINDOW_TICK));
    //         if event_proxy.send_event(AppEvent::FrameTick).is_err() {
    //             break;
    //         }
    //     }
    // });
    // let mut app = App::new();
    // event_loop.run_app(&mut app).unwrap();
}
// Android 入口
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: AndroidApp) {
    init_logging();
    run().expect("Lifecycle run started is failed in Android!");
}

// iOS 入口
#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn ios_main() {
    init_logging();
    run().expect("Lifecycle run started is failed in iOS!");
}
