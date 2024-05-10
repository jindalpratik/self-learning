#include <iostream>
using namespace std;

int main()
{
    int arr[] = {-1, 2, 4, -3, 5, 2, -5, 2};
    int best = 0, sum = 0;
    int n = sizeof(arr)/sizeof(int);
    for (int k = 0; k < n; k++)
    {
        sum = max(arr[k], sum + arr[k]);
        best = max(best, sum);
    }
    cout << best << "\n";
}