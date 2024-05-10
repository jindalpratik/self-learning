#include <iostream>
#include <map>
using namespace std;

int main()
{
    map<string, int> m;
    m["monkey"] = 4;
    m["banana"] = 3;
    m["harspsichord"] = 9;
    cout << m["banana"] << "\n";

    cout << m["aybabtu"] << "\n";       // Accessing a key with doesn't exists created it with a default value.
    cout << m.count("aybabtu") << endl; // This prints 1
    cout << m.count("hello") << endl;   // This prints 0

    for (auto x : m)
    {
        cout << x.first << " " << x.second << endl;
    }

    return 0;
}