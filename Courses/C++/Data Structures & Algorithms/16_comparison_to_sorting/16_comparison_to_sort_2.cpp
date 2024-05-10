#include <iostream>
#include <vector>
#include <unordered_set>
using namespace std;

/*
As it is not necessary for the elements in the set to be sorted we can use unordered set which will dicrease the time
complexity to O(n). This is also a drop in replacement.
*/
int main()
{
    vector<int> a = {5, 2, 8, 9, 4};
    vector<int> b = {3, 2, 9, 5};
    unordered_set<int> sa, sb;
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