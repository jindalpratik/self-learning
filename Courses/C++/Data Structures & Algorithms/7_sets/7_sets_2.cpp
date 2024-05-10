#include <iostream>
#include <set>
using namespace std;

int main()
{
    set<int> s = {2, 3, 4, 5, 6, 6};
    cout << s.count(6) << "\n"; // Is a 1 as the duplicate 6 is ignored.
    cout << s.size() << "\n";
    for (auto x : s)
    {
        cout << x << " ";
    }
}