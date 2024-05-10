#include <iostream>
#include <algorithm>
using namespace std;

int main()
{
    string s = "HelloHowARe";
    sort(s.begin(), s.end());
    cout << s << endl;
}