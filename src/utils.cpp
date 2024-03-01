#include <cstdlib>
#include <pwd.h>
#include <iostream>

using namespace std;

char* user_home() {
    char* home = NULL;

    if ((home = getenv("HOME")) == NULL) {
        home = getpwnam(home)->pw_dir;
    }

    if (home == NULL) {
        cerr << "Could not determine the HOME directory of the current user" << endl;
        exit(1);
    }

    return home;
}
