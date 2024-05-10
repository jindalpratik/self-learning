#include <iostream>
#include <vector>
using namespace std;

int main()
{   
    // We can initialize the vector v with default values as follows
    vector<int> v = {2,4,2,5,1};

    for (auto i : v)
    {
        cout << i << " ";
    }
    cout << endl;

    vector<int> k(10, 5);
    for(auto i : k)
    {
        cout << i << " ";
    }
    cout << "\n";
}