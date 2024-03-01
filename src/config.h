#pragma once

#include <vector>
#include <string>

using namespace std;

struct Config {
    string editor;
    string workspaces_file;
};

vector<string> string_split(string str);
// Read config file
Config read_config(string* path);
