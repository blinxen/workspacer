#include <vector>
#include <string>
#include <functional>

#include <ftxui/dom/elements.hpp>
#include <ftxui/screen/terminal.hpp>
#include "ftxui/component/component.hpp"
#include "ftxui/component/component_options.hpp"
#include "ftxui/component/screen_interactive.hpp"
#include "ftxui/component/event.hpp"

using namespace ftxui;
using namespace std;

namespace terminal {

    void draw(vector<string>* workspaces, int* selected_entry, function<void()> on_enter_callback) {
        Dimensions terminal_size = Terminal::Size();
        ScreenInteractive screen = ScreenInteractive::Fullscreen();

        MenuOption option = MenuOption::Vertical();
        option.on_enter = screen.WithRestoredIO(on_enter_callback);
        auto menu = Menu(
            workspaces,
            selected_entry,
            option
        );

        menu |= size(WIDTH, EQUAL, terminal_size.dimx / 2);
        menu |= size(HEIGHT, EQUAL, terminal_size.dimy / 2);
        menu |= border;

        screen.Loop(
            Container::Vertical({ menu }) | center
        );

    }

}
