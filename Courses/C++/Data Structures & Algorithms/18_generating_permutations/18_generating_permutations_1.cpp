#include <iostream>
#include <vector>

using namespace std;

vector<int> permutation;
vector<bool> chosen;
int n; // size of the permutation

void processPermutation() {
    // Process the permutation
    cout << "{ ";
    for (int i = 0; i < permutation.size(); ++i) {
        cout << permutation[i] << " ";
    }
    cout << "}\n";
}

void search() {
    if (permutation.size() == n) {
        processPermutation();
    } else {
        for(int i = 0; i<n;i++)
        {
            if(chosen[i] == true) continue;
            chosen[i] = true;
            permutation.push_back(i);
            search();
            chosen[i] = false;
            permutation.pop_back();
        }
    }
}

int main() {
    cout << "Enter the size of the permutation: ";
    cin >> n;
    
    // Initialize chosen vector with n elements all set to false
    chosen.assign(n, false);
    
    search();
    
    return 0;
}
