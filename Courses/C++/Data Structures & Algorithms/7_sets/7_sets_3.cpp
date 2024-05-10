#include <iostream>
#include <set>
using namespace std;

int main()
{
    // If we want there to be multiple elements in a set we can use multiset.
    multiset<int> s;
    s.insert(5);
    s.insert(5);
    s.insert(5);
    cout << s.count(5) << "\n";
    s.erase(5); //Removes all instances of 5;
    cout << s.count(5) << "\n";

    s.insert(5);
    s.insert(5);
    s.insert(5);
    s.erase(s.find(5)); //This can be used if we only want to remove a single element.
    cout << s.count(5) << "\n";

}