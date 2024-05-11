#include<iostream>
using namespace std;

class Node
{
public:
    int data;
    Node* left;
    Node* right;
    Node(int d)
    {
        data = d;
        left = NULL;
        right = NULL;
    }
};

void print(Node* r)
{
    if(r == NULL)
    {
        return;
    }
    cout << r->data;
    print(r->left);
    print(r->right);
}

int main()
{
    Node *root = new Node(1);
    root->left = new Node(2);
    root->right = new Node(3);
    print(root);
    return 0;
}