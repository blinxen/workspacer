#include <vector>
#include <string>
#include <fstream>
#include <iostream>
#include <filesystem>

#include "workspaces.h"

using namespace std;

namespace workspaces {

    vector<workspaces::Workspace> read_workspaces(string* path) {
        vector<workspaces::Workspace> workspaces;
        ifstream ws_file(*path);

        if (!ws_file.is_open()) {
            cerr << "Could not open file that contains all workspaces" << endl;
            exit(1);
        }

        string line;
        // Get all workspaces from the file
        // Each line represents a single workspace
        while (getline(ws_file, line)) {
            filesystem::file_status ws_fs_status = filesystem::status(line);
            // Check whether the path is a directory and whether it actually exists
            // If it does not exist, then we just ignore it
            if (filesystem::exists(ws_fs_status) && filesystem::is_directory(ws_fs_status)) {
                filesystem::path ws_path = filesystem::path(line);
                try {
                    workspaces.push_back(
                        {
                            ws_path.filename(),
                            filesystem::canonical(ws_path).string()
                        }
                    );
                } catch(filesystem::filesystem_error const& ex) {
                    cerr << "Unexpected error occured" << ex.what() << endl;
                }
            } else {
                cerr << "Path does not exist or is not a directory: " << line << endl;
            }
        }

        return workspaces;
    }
}
