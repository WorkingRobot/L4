use gdk4_win32::prelude::*;
use gdk4_win32::Win32Surface;
use gtk::{traits::NativeExt, Native};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::HiDpi::GetDpiForWindow;

pub trait UsesDpi: IsA<Native> {
    fn get_dpi(&self) -> Option<u32>;
}

impl<T> UsesDpi for T
where
    T: IsA<Native>,
{
    fn get_dpi(&self) -> Option<u32> {
        let hwnd = HWND(self.surface().downcast_ref::<Win32Surface>()?.handle().0);
        unsafe { Some(GetDpiForWindow(hwnd)) }
    }
}
