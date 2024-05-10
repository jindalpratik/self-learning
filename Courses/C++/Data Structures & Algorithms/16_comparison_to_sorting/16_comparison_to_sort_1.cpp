#include <iostream>
#include <vector>
#include <set>
using namespace std;

// To find the number of elements common in two arrays we can use sets and the operations will be O(nlogn) time complexity.
int main()
{
    vector<int> a = {5, 2, 8, 9, 4};
    vector<int> b = {3, 2, 9, 5};
    set<int> sa, sb;
    for(auto i: a)
    {
        sa.insert(i);
    }
    for(auto i : b)
    {
        sb.insert(i);
    }
    int count = 0;
    for(auto i : sb)
    {
        if(sa.count(i))
        {
            count += 1;
        }
    }
    cout << count << endl;
}