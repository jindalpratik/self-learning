#include <iostream>
#include <set>
using namespace std;

int main()
{   
    set<int> s = {2,3,4,5,6,1,8,90};
    int x = 6;
    auto it = s.lower_bound(x);
    if (it == s.begin())
    {
        cout << *it << "\n";
    }
    else if (it == s.end())
    {
        it--;
        cout << *it << "\n";
    }
    else
    {
        int a = *it;
        it--;
        int b = *it;
        if (x - b < a - x)
            cout << b << "\n";
        else
            cout << a << "\n";
    }
}