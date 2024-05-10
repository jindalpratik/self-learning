#include <iostream>
#include <set>
using namespace std;

int main()
{
    set<int> s;
    s.insert(3);
    s.insert(4);
    s.insert(2);
    // Will not add a new element as there is already a 3 present and all the elements of a set are distinct.
    // This also results in count either resulting in a 1 or a 0.
    s.insert(3);

    cout << s.count(3) << endl;
    cout << s.count(5) << endl;

    s.erase(3);
    s.insert(5);

    cout << s.count(3) << endl;
    cout << s.count(5) << endl;
}