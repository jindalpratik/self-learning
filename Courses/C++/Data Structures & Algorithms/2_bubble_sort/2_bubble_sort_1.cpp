#include <iostream>
using namespace std;

int main()
{
    int arr[] = {1, 3, 8, 2, 9, 2, 5, 6};
    int n = sizeof(arr) / sizeof(int);
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n - 1; j++)
        {
            if (arr[j] > arr[j + 1])
            {
                swap(arr[j], arr[j + 1]);
            }
        }
    }
}