#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

string dirs = "DRLU";

vector<int> cr = {1, 0, 0, -1};
vector<int> cc = {0, 1, -1, 0};
vector<vector<int>> maze;
vector<string> ans;
string current_path;
int n;

bool is_possible(int row, int col, vector<vector<int>> maze)
{
    return (row >= 0 && col >= 0 && row < n && col < n && maze[row][col]);
}

void rat_maze(int row, int col)
{
    if (col == n - 1 && row == n - 1)
    {
        ans.push_back(current_path);
        return;
    }
    else
    {
        maze[row][col] = 0;
        for (int i = 0; i < n; i++)
        {
            int next_row = row + cr[i];
            int next_col = col + cc[i];
            if (is_possible(next_row, next_col, maze))
            {
                current_path += dirs[i];
                rat_maze(next_row, next_col);
                current_path.pop_back();
            }
        }
        maze[row][col] = 1;
    }
}

int main()
{
    maze = {{1, 0, 0, 0},
            {1, 1, 0, 0},
            {0, 1, 1, 0},
            {1, 1, 1, 1}};
    int col, row;
    col = row = 0;
    n = maze.size();
    current_path = "";
    rat_maze(row, col);
    sort(ans.begin(), ans.end());
    for (auto i : ans)
    {
        cout << i << endl;
    }
}