#include <iostream>
#include <vector>
using namespace std;

vector<int> column;
vector<int> diag1;
vector<int> diag2;
vector<int> position;
int n;
long count = 0;

void print_position()
{
    cout << "-------" << endl;
    int count_p = 0;
    string s = "";
    for (auto i : position)
    {
        count_p++;
        if(i == 0) {
            s = s+ ".";
        } else {
            s = s+"q";
        }
        if(count_p == n)
        {
            count_p = 0;
            cout << s << endl;
            s = "";
        }
    }
    cout << endl << "-------" << endl;
}

void n_queens(int y)
{
    if(y == n)
    {
        count++;
        // print_position();
        return;
    }
    else
    {
        for(int x = 0; x<n; x++)
        {
            if(column[x] || diag1[x+y] || diag2[x-y+n-1]) continue;
            column[x] = diag1[x+y] = diag2[x-y+n-1] = position[n*y+x] = 1;
            n_queens(y+1);
            column[x] = diag1[x+y] = diag2[x-y+n-1] = position[n*y+x] = 0;
        }
    }
}

int main()
{  
    cout << "Enter the size of the chess board: ";
    cin >> n;
    column.assign(n,0);
    diag1.assign(2*n-1,0);
    diag2.assign(2*n-1,0);
    position.assign(n*n,0);
    
    n_queens(0);

    cout << "Total number of solutions: " << count << endl;
    return 0;
}