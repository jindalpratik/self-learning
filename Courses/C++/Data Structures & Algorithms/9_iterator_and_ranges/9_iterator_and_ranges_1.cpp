#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;

int main()
{
    vector<int> v = {3, 4, 6, 8, 12, 13, 14, 17, 9, 3, 8, 5, 4};
    sort(v.begin(), v.end());
    for (auto i : v)
    {
        cout << i << " ";
    }
    cout << endl;

    reverse(v.begin(), v.end());
    for (auto i : v)
    {
        cout << i << " ";
    }
    cout << endl;
}