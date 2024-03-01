#pragma once

#include <vector>
#include <string>

using namespace std;

namespace workspaces {

    struct Workspace {
        string name;
        string path;
    };

    // Read all stored workspaces from the specified data file
    vector<Workspace> read_workspaces(string* path);
}
