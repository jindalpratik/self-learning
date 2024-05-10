#include <iostream>
#include <algorithm>
using namespace std;

int main()
{
    int arr[] = {5, 6, 7, 2, 4, 8, 9, 3, 64};
    int n = sizeof(arr) / sizeof(int);
    sort(arr, arr + n);
    for (int i; i < n; i++)
    {
        cout << arr[i] << " ";
    }
}