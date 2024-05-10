#include <iostream>
#include <vector>
using namespace std;

int main()
{
    vector<int> v = {1, 2, 3, 4, 5, 6, 8, 24, 20, 56, 78};
    int to_find;
    cin >> to_find;
    int n = v.size();
    int a = 0, b = n - 1;
    while (a <= b)
    {
        int k = (a + b) / 2;
        if (v[k] == to_find)
        {
            cout << "Array found at index " << k << endl;
            return 0;
        }
        if (v[k] > to_find)
            b = k - 1;
        else
            a = k + 1;
    }
    cout << "Element not found in array" << endl;
    return 0;
}