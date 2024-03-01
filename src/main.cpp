#include "terminal/terminal.h"
#include "workspaces.h"

#include <iostream>
#include <cstdlib>
#include <vector>
#include <string>
#include <format>
#include <functional>

using namespace std;

function<void()> enter_editor_closure(vector<workspaces::Workspace>* workspaces, int* selected_item) {
    return [=] {
        // Both lines do the same
        // cout << (*workspaces)[*selected_item] << endl;
        // cout << workspaces->at(*selected_item) << endl;
        system(format("nvim {}", workspaces->at(*selected_item).path).c_str());
    };
}

int main() {
    string data_file_path = "./workspaces";

    int selected = 0;
    vector<workspaces::Workspace> workspaces = workspaces::read_workspaces(&data_file_path);
    vector<string> ws_string_repr = {};

    for (workspaces::Workspace &ws : workspaces) {
        ws_string_repr.push_back(format("{} <{}>", ws.name, ws.path));
    }

    terminal::draw(&ws_string_repr, &selected, enter_editor_closure(&workspaces, &selected));

    return EXIT_SUCCESS;
}
