#include <iostream>
#include <vector>
using namespace std;

int main()
{
    vector<int> v = {1, 2, 3, 4, 5, 6, 8, 24, 20, 56, 78};
    int to_find;
    cin >> to_find;
    int k = 0;
    int n = v.size();
    for (int b = n / 2; b >= 1; b /= 2)
    {
        while (k + b < n && v[k + b] <= to_find)
        {
            k += b;
        }
    }
    if (v[k] == to_find)
    {
        cout << "Element found at index " << k << endl;
    }
    else
    {
        cout << "Element not found" << endl;
    }
    return 0;
}