#include <iostream>
#include <vector>
using namespace std;

int main()
{
    vector<int> v;
    v.push_back(5);
    v.push_back(2);
    cout << v.back() << "\n";
    v.push_back(5);
    cout << v.back() << "\n";
}