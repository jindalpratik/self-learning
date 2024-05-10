#include <iostream>
using namespace std;

int main()
{
    string a = "hatti";
    string b = a+a;
    cout << b  << "\n";
    b[5] = 'v';
    cout << b << "\n";
    string c = b.substr(3,4);
    cout << c << "\n";
}