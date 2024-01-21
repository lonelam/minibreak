use std::error::Error;

use windows::core::HSTRING;
use windows::Win32::System::Com::*;
use windows::Win32::UI::Shell::*;

pub async fn set_from_path(path: &str) -> Result<(), Box<dyn Error>> {
    unsafe {
        CoInitialize(None)?;
    }
    unsafe {
        let desktop_wallpaper: IDesktopWallpaper =
            CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;
        desktop_wallpaper.SetWallpaper(None, &HSTRING::from(path))?;
        // desktop_wallpaper
    }
    unsafe {
        CoUninitialize();
    }

    Ok(())
}

pub async fn show_desktop() -> Result<(), Box<dyn Error>> {
    unsafe {
        CoInitialize(None)?;
        let shell: IShellDispatch = CoCreateInstance(&Shell, None, CLSCTX_INPROC_SERVER)?;
        shell.MinimizeAll()?;
    }
    Ok(())
}
