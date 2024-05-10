#include <iostream>
#include <set>
#include <algorithm>
using namespace std;

int main()
{
    set<int> s = {1, 5, 3, 4, 8, 6, 9};
    set<int>::iterator it = s.begin();
    // Can also be written as
    auto new_it = s.begin();
    // And to print out the element we can do
    cout << *new_it << endl;

    for (auto k_it = s.begin(); k_it != s.end(); k_it++)
    {
        cout << *k_it << " ";
    }
    cout << endl;

    auto lar_it = s.end();
    lar_it--;
    cout << *lar_it;

    return 0;
}