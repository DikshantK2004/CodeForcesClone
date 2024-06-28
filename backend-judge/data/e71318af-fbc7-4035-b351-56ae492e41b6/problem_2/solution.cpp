#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int main(int argc, char* argv[]) {
    if (argc < 2) {  // Check if the command line argument is provided
        cerr << "Usage: " << argv[0] << " <output_path>";
        return 1;  // Return non-zero to indicate error
    }

    string n, k,r;
    cin >> n >> k >> r;

    string output_path = argv[1];  // Directly use argv[1], no need to convert from char array

    ifstream output_file(output_path);
    if (!output_file.is_open()) {  // Check if the file is opened successfully
        cerr << "Failed to open file: " << output_path ;
        return 1;  // Return non-zero to indicate error
    }

    string no, ko, r0;
    output_file >> no >> ko >> r0;

    if (output_file.fail()) {  // Check if reading from file failed
        cerr << "Failed to read from file: " << output_path ;
        return 1;  // Return non-zero to indicate error
    }

    if (n == no && k == ko) {
        cout << "YES";
    } else {
        cout << "NO";
    }

    return 0;  // Success
}
