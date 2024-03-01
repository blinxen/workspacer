#include <iostream>
#include <cstdlib>
#include <vector>
#include <string>
#include <format>
#include <functional>

#include "config.h"
#include "utils.h"
#include "terminal/terminal.h"
#include "workspaces.h"

using namespace std;

function<void()> enter_editor_closure(string* editor, vector<workspaces::Workspace>* workspaces, int* selected_item) {
    return [=] {
        // Both lines do the same
        // cout << (*workspaces)[*selected_item] << endl;
        // cout << workspaces->at(*selected_item) << endl;
        system(format("{} {}", *editor, workspaces->at(*selected_item).path).c_str());
    };
}

int main() {
    string config_path = format("{}{}", user_home(), "/.config/workspacer");
    Config config = read_config(&config_path);

    int selected = 0;
    vector<workspaces::Workspace> workspaces = workspaces::read_workspaces(&config.workspaces_file);
    vector<string> ws_string_repr = {};

    for (workspaces::Workspace &ws : workspaces) {
        ws_string_repr.push_back(format("{} <{}>", ws.name, ws.path));
    }

    terminal::draw(&ws_string_repr, &selected, enter_editor_closure(&config.editor, &workspaces, &selected));

    return EXIT_SUCCESS;
}
