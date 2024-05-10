#include <iostream>
#include <vector>
using namespace std;

int main()
{
    vector<int> v;
    v.push_back(3);
    v.push_back(2);
    v.push_back(5);
    cout << v[0] << "\n";
    cout << v[1] << "\n";
    cout << v[2] << "\n";

    // Standard way to iterate through vectors
    for (int i = 0; i < v.size(); i++)
    {
        cout << v[i] << "\n";
    }

    // Another easier way to iterate through vectors
    for (auto i : v)
    {
        cout << v[i] << "\n";
    }
}