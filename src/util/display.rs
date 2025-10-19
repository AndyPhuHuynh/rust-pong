use glutin::display::DisplayApiPreference;
use raw_window_handle::RawWindowHandle;

#[cfg(target_os = "windows")]
pub const DISPLAY_PREF: fn(RawWindowHandle) -> DisplayApiPreference =
    |raw_window_handle| DisplayApiPreference::EglThenWgl(Some(raw_window_handle));

#[cfg(target_os = "linux")]
pub const DISPLAY_PREF: fn(RawWindowHandle) -> DisplayApiPreference =
    |raw_window_handle| DisplayApiPreference::Egl(Some(raw_window_handle));

#[cfg(target_os = "macos")]
pub const DISPLAY_PREF: fn(RawWindowHandle) -> DisplayApiPreference =
    |raw_window_handle| DisplayApiPreference::Cgl(Some(raw_window_handle));