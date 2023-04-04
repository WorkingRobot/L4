use gtk::glib::{SignalHandlerId, WeakRef};

pub struct SignalHolder {
    object: WeakRef<gtk::Widget>,
    signal_handle: Option<SignalHandlerId>,
}

impl SignalHolder {
    pub fn new(object: &gtk::Widget, signal_handle: SignalHandlerId) -> Self {
        let weak = WeakRef::<gtk::Widget>::new();
        weak.set(Some(object));
        Self {
            object: weak,
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

macro_rules! signal {
    ($object:ident, $name:ident, $($closure:tt)+) => {
        SignalHolder::new($object.upcast_ref::<gtk::Widget>(), $object.$name($($closure)+))
    };
}

pub(crate) use signal;
