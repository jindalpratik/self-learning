#include<iostream>
using namespace std;

int main() {
    int arr[] = {-1, 2, 4, -3, 5, 2, -5, 2};
    int best = 0;
    int n = sizeof(arr)/sizeof(int);
    for(int a = 0; a<n;a++)
    {
        int sum = 0;
        for(int b = a;b <n;b++)
        {
            sum += arr[b];
            best = max(best, sum);
        }
    }
    cout << best << "\n";
}