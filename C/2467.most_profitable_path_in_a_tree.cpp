// TODO: Revisit
class Solution {
public:
  int mostProfitablePath(vector<vector<int>> &edges, int bob,
                         vector<int> &amount) {
    int n = amount.size(), maxIncome = INT_MIN;
    tree.resize(n);
    visited.assign(n, false);
    queue<vector<int>> q;
    q.push({0, 0, 0});

    // Form tree with edges
    for (vector<int> edge : edges) {
      tree[edge[0]].push_back(edge[1]);
      tree[edge[1]].push_back(edge[0]);
    }

    dfs(bob, 0);

    visited.assign(n, false);
    while (!q.empty()) {
      int sourceNode = q.front()[0], time = q.front()[1],
          income = q.front()[2];

      // Alice reaches the node first
      if (bobPath.find(sourceNode) == bobPath.end() ||
          time < bobPath[sourceNode]) {
        income += amount[sourceNode];
      }

      // Alice and Bob reach the node at the same time
      else if (time == bobPath[sourceNode]) {
        income += (amount[sourceNode] / 2);
      }

      // Update max value if current node is a new leaf
      if (tree[sourceNode].size() == 1 && sourceNode != 0) {
        maxIncome = max(maxIncome, income);
      }
      // Explore adjacent unvisited vertices
      for (int adjacentNode : tree[sourceNode]) {
        if (!visited[adjacentNode]) {
          q.push({adjacentNode, time + 1, income});
        }
      }

      // Mark and remove current node
      visited[sourceNode] = true;
      q.pop();
    }
    return maxIncome;
  }

private:
  unordered_map<int, int> bobPath;
  vector<bool> visited;
  vector<vector<int>> tree;

  bool dfs(int sourceNode, int time) {
    // Mark and set time node is reached
    bobPath[sourceNode] = time;
    visited[sourceNode] = true;

    // Destination for Bob is found
    if (sourceNode == 0) {
      return true;
    }

    // Traverse through unvisited nodes
    for (auto adjacentNode : tree[sourceNode]) {
      if (!visited[adjacentNode]) {
        if (dfs(adjacentNode, time + 1)) {
          return true;
        }
      }
    }
    // If node 0 isn't reached, remove current node from path
    bobPath.erase(sourceNode);
    return false;
  }
};
