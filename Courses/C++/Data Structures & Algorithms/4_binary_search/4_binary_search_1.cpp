#include <iostream>
#include <vector>
using namespace std;

// This is not an example of binary search but instead an general way to search for an element in an array.
int main()
{
    vector<int> v = {1, 2, 3, 4, 5, 6, 8, 24, 20, 56, 78};
    int to_find = 20;
    bool flag = true;
    for (int i = 0; i < v.size(); i++)
    {
        if (v[i] == to_find)
        {
            cout << "Element found at index : " << i << endl;
            flag = false;
        }
    }
    if (flag)
    {
        cout << "Element was not found" << endl;
    }
}