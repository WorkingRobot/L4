use gtk::glib::{Continue, MainContext, Receiver, Sender};

pub struct Dispatcher<T> {
    sender: Sender<T>,
    receiver: Option<Receiver<T>>,
}

impl<T> Dispatcher<T> {
    // Create dispatcher with default main context
    pub fn new() -> Self {
        let (sender, receiver) = MainContext::channel(gtk::glib::PRIORITY_DEFAULT);
        Self {
            sender,
            receiver: Some(receiver),
        }
    }

    pub fn connect<F: FnMut(T) -> bool + 'static>(&mut self, func: F) {
        self.connect_to_context(&MainContext::ref_thread_default(), func);
    }

    pub fn connect_to_context<F: FnMut(T) -> bool + 'static>(
        &mut self,
        context: &MainContext,
        mut func: F,
    ) {
        self.receiver
            .take()
            .expect("receiver was already connected to")
            .attach(Some(context), move |d| Continue(func(d)));
    }

    pub fn emit(&self, data: T) {
        _ = self.sender.send(data);
    }
}

impl<T> Default for Dispatcher<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for Dispatcher<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: None,
        }
    }
}
