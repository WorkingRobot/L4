use gdk4_win32::glib::clone::Downgrade;
use gtk::glib::{SignalHandlerId, WeakRef};

pub struct SignalHolder {
    object: WeakRef<gtk::Widget>,
    signal_handle: Option<SignalHandlerId>,
}

impl SignalHolder {
    pub fn new(object: &gtk::Widget, signal_handle: SignalHandlerId) -> Self {
        Self {
            object: object.downgrade(),
            signal_handle: Some(signal_handle),
        }
    }
}

impl Drop for SignalHolder {
    fn drop(&mut self) {
        if let Some(v) = self.object.upgrade() {
            gtk::glib::signal_handler_disconnect(&v, self.signal_handle.take().unwrap());
        }
    }
}

#[macro_export]
macro_rules! signal {
    ($object:ident, $name:ident, $($closure:tt)+) => {
        ::deps::utils::SignalHolder::new($object.upcast_ref::<gtk::Widget>(), $object.$name($($closure)+))
    };
}

pub use signal;
