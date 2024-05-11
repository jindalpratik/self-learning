#include <iostream>
#include <vector>
using namespace std;

int main()
{
    vector<int> v = {{-8, 2, 3, -6, 10}};
    int k = 2;
    for (int i = 0; i < v.size() - k + 1; i++)
    {
        bool flag = true;
        for (int j = 0; j < k; j++)
        {
            if(v[i+j] < 0)
            {
                cout << v[i+j] << " ";
                flag = false;
                break;
            }
        }
        if(flag)
        {
            cout << "0 ";
        }
    }
}