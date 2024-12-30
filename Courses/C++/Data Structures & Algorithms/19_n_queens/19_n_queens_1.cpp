#include <iostream>
#include <vector>
using namespace std;

vector<int> column;
vector<int> diag1;
vector<int> diag2;
int n;
int count = 0;

void n_queens(int row)
{
    if (row == n)
    {
        count++;
        return;
    }
    else
    {
        for (int col = 0; col < n; col++)
        {
            if (column[col] || diag1[col + row] || diag2[col - row + n - 1])
                continue;
            
            column[col] = diag1[col + row] = diag2[col - row + n - 1] = 1;
            n_queens(row + 1);
            column[col] = diag1[col + row] = diag2[col - row + n - 1] = 0;
        }
    }
}

int main()
{
    cout << "Enter the size of the chess board: ";
    cin >> n;
    column.assign(n, 0);
    diag1.assign(2 * n - 1, 0);
    diag2.assign(2 * n - 1, 0);

    n_queens(0);

    cout << "Total number of solutions: " << count << endl;
    return 0;
}