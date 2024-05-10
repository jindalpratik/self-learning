#include <iostream>
#include <algorithm>
using namespace std;

int main()
{
    int arr[] = {1, 2, 3, 4, 5};
    int n = sizeof(arr) / sizeof(int);
    int to_find;
    cin >> to_find;
    auto k = lower_bound(arr, arr + n, to_find) - arr;
    if (k < n && arr[k] == to_find)
    {
        cout << "Element found at position " << k << endl;
    }
    else
    {
        cout << "Elemtent not found" << endl;
    }

    auto a = lower_bound(arr, arr + n, to_find);
    auto b = upper_bound(arr, arr + n, to_find);
    cout << b - a << "\n";

    auto r = equal_range(arr, arr + n, to_find);
    cout << r.second - r.first << endl;
}