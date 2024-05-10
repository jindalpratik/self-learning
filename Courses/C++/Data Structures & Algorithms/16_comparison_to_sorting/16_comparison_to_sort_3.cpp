#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

int main()
{
    vector<int> a = {5, 2, 8, 9, 4, 5};
    vector<int> b = {3, 2, 9, 5, 5};
    sort(a.begin(), a.end());
    sort(b.begin(), b.end());
    auto ita = a.begin();
    auto itb = b.begin();
    int count = 0;
    while (ita != a.end() && itb != b.end())
    {
        if (*ita == *itb)
        {
            count++;
            ita++;
            itb++;
        }
        else if (*ita < *itb)
        {
            ita++;
        }
        else
        {
            itb++;
        }
    }
    cout << count;
}