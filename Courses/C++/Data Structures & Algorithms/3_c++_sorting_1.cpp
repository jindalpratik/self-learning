#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;

int main()
{
    vector<int> v = {4, 2, 5, 6, 7, 8, 9, 1, 5, 6};
    cout << "Hello world" << endl;
    sort(v.begin(), v.end());
    for (int &i : v)
    {
        cout << i << " ";
    }
}