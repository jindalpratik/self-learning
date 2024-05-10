#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

bool comp(string a, string b)
{
    if (a.size() != b.size())
        return a.size() < b.size();
    return a < b;
}

int main()
{
    string a = "helloksj";
    string b = "mellows";
    vector<string> str_vec = {a,b};
    sort(str_vec.begin(), str_vec.end(), comp);
    cout << str_vec[0] << " " <<  str_vec[1] << endl;
    return 0;
}