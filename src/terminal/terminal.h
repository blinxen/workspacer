#pragma once

#include <vector>
#include <string>
#include <functional>

using namespace std;

namespace terminal {
    void draw(vector<string>* workspaces, int* selected_entry, function<void()> on_enter_callback);
}
