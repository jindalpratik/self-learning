#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

struct P
{
    int x, y;
    bool operator<(const P &p)
    {
        if (x != p.x)
            return x < p.x;
        else
            return y < p.y;
    }
};

int main()
{
    P struct_a, struct_b;
    struct_a.x = 9;
    struct_b.x = 8;
    struct_a.y = 6;
    struct_b.y = 4;
    vector<P> vec_a = {struct_a, struct_b};
    sort(vec_a.begin(), vec_a.end());
    for (int i = 0; i < vec_a.size(); i++)
    {
        cout << vec_a[i].x << " " << vec_a[i].y << endl;
    }
    return 0;
}