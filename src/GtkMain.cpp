#include <gtkmm.h>

class MyWindow : public Gtk::Window
{
public:
    MyWindow();
};

MyWindow::MyWindow()
{
    set_title("Basic application");
    set_default_size(200, 200);
}

#include "utils/formatters/Stopwatch.h"
#include "utils/Log.h"
#include "utils/Stopwatch.h"

namespace L4
{
    void Main()
    {
        LogSetup();
        // *(int*)0xdeadbeef = 5;
        // __builtin_trap();
        // asm("hlt");
        Ensure<LogLevel::Critical>(false, "hello {:s}", [] { return std::make_format_args("world"); });
    }
}

int main(int argc, char* argv[])
{
    L4::Main();
    return 0;
    auto app = Gtk::Application::create("org.gtkmm.examples.base");

    return app->make_window_and_run<MyWindow>(argc, argv);

    return 0;
}