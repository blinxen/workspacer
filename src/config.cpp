#include <vector>
#include <string>
#include <string.h>
#include <fstream>
#include <iostream>
#include <filesystem>
#include <filesystem>
#include <system_error>

#include "config.h"
#include "utils.h"

using namespace std;

// We use `string str` here since we want a copy of the input parameter
string* split_line(string str) {
    string* splits = new string[2];

    // Key
    splits[0] = strtok(str.data(), "=");
    // Value
    splits[1] = strtok(NULL, "=");

    return splits;
}

Config read_config(string* config_directory) {
    Config config = {
        "/usr/bin/hx",
        *config_directory + "/workspaces"
    };

    string config_path = *config_directory + "/config";

    ifstream ws_file(config_path);

    if (!ws_file.is_open()) {
        cerr << "No config file was found" << endl;
        cerr << "Using default configuration" << endl;
        error_code (ec);
        filesystem::create_directories(*config_directory, ec);
        if (ec.value() == 0) {
            // Create default config file
            ofstream conf_file(config_path);
            conf_file << "editor=" + config.editor << endl;
            conf_file << "workspaces_file=" + config.workspaces_file << endl;
            conf_file.close();
        } else {
            cerr << "Could not create configuration directory" << endl;
            cerr << ec.message() << endl;
        }
    } else {
        string line;
        while (getline(ws_file, line)) {

            string* splits = split_line(line);
            if (splits[0] == "editor") {
                config.editor = splits[1];
            } else if (splits[0] == "workspaces_file") {
                config.workspaces_file = splits[1];
            }
        }

    }

    return config;
}
