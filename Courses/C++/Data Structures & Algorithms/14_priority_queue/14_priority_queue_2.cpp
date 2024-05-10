#include <iostream>
#include <queue>
using namespace std;

int main()
{
    priority_queue<int, vector<int>, greater<int>> pq;
    pq.push(5);
    pq.push(2);
    pq.push(3);
    pq.push(9);
    cout << pq.top() << "\n"; // 9
    pq.pop();
    cout << pq.top() << "\n"; // 8
    pq.pop();
    pq.push(6);
    cout << pq.top() << "\n"; // 6
    pq.pop();
}