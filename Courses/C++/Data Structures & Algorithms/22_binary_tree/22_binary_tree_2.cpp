#include <iostream>
#include <vector>
#include <queue>
using namespace std;

class Node
{
public:
    int data;
    Node *left;
    Node *right;
    Node(int val)
    {
        data = val;
        left = NULL;
        right = NULL;
    }
};

void preorder(Node *node)
{
    if (node == NULL)
    {
        return;
    }
    cout << node->data << " ";
    preorder(node->left);
    preorder(node->right);
}

int main()
{
    int n;
    cin >> n;
    vector<int> arr(n);
    for (int i = 0; i < n; i++)
    {
        cin >> arr[i];
    }
    queue<Node *> q;
    int index = 1;
    Node *root = new Node(arr[0]);
    q.push(root);
    while (index < n && !q.empty())
    {
        Node *front = q.front();
        q.pop();
        if (arr[index] != -1)
        {
            Node *l = new Node(arr[index]);
            front->left = l;
            q.push(l);
        }
        index++;

        if (arr[index] != -1)
        {
            Node *r = new Node(arr[index]);
            front->right = r;
            q.push(r);
        }
        index++;
    }

    preorder(root);
}