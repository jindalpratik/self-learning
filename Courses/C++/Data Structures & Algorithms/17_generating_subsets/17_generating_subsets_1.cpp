#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

vector<int> subset;

void process_subset()
{
    cout << "{";
    for (auto i : subset)
    {
        cout << i << " ";
    }
    cout << "}" << endl;
}

void search(int k, int n, vector<int> v)
{
    if (k == n)
    {
        process_subset();
    }
    else
    {
        search(k + 1, n, v);
        subset.push_back(v[k]);
        search(k+1,n,v);
        subset.pop_back();
    }
}

int main()
{
    vector<int> v = {2, 5, 6, 8};
    int n = v.size();
    search(0, n, v);
}