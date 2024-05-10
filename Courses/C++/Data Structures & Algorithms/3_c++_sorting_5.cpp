#include <iostream>
#include <vector>
#include <tuple>
#include <algorithm>
using namespace std;

int main()
{
    vector<tuple<int, int, int>> v;
    v.push_back({1, 2, 3});
    v.push_back({2, 3, 4});
    v.push_back({1, 5, 4});
    sort(v.begin(), v.end());
    for (int i = 0; i < v.size(); i++)
    {
        cout << get<0>(v[i]) << " " << get<1>(v[i]) << " " << get<2>(v[i]) << endl;
    }
}