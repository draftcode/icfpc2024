#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
using namespace std;

const int DR[] = { -1, 0, 1, 0 };
const int DC[] = { 0, 1, 0, -1 };

// Breadth-first search until we reach a '.'.
vector<pair<int,int>> bfs(int sr, int sc, const vector<string>& maze) {
    vector<int> dir_order = { 0, 1, 2, 3 };

    vector<vector<int>> dist(maze.size(), vector<int>(maze[0].size(), -1));
    vector<vector<pair<int,int>>> prev(maze.size(), vector<pair<int,int>>(maze[0].size(), make_pair(-1,-1)));
    dist[sr][sc] = 0;
    vector<pair<int,int>> q = { {sr, sc} };
    int gr = -1, gc = -1;
    for (int i = 0; i < q.size(); i++) {
        int r = q[i].first;
        int c = q[i].second;
        random_shuffle(dir_order.begin(), dir_order.end());
        for (int d = 0; d < 4; d++) {
            int nr = r + DR[dir_order[d]];
            int nc = c + DC[dir_order[d]];
            if (nr >= 0 && nr < maze.size() && nc >= 0 && nc < maze[0].size() && maze[nr][nc] != '#' && dist[nr][nc] == -1) {
                dist[nr][nc] = dist[r][c] + 1;
                prev[nr][nc] = { r, c };
                q.push_back({ nr, nc });
                if (maze[nr][nc] == '.') {
                    gr = nr;
                    gc = nc;
                    break;
                }
            }
        }
        if (gr != -1) {
            break;
        }
    }
    // Reconstruct path
    vector<pair<int,int>> path;
    for (int r = gr, c = gc; r != sr || c != sc; ) {
        path.push_back({ r, c });
        int pr = prev[r][c].first;
        int pc = prev[r][c].second;
        r = pr;
        c = pc;
    }
    reverse(path.begin(), path.end());
    return path;
}

string solve(const vector<string>& input)
{
    vector<string> maze = input;
    int sr = 0; int sc = 0;
    int pills = 0;
    for (int r = 0; r < maze.size(); r++) {
        for (int c = 0; c < input[0].size(); c++)
        {
            if (input[r][c] == 'L') {
                sr = r;
                sc = c;
            }
            if (input[r][c] == '.') {
                pills++;
            }
        }
    }

    string moves;
    while (pills > 0)
    {
        // Find nearest pill
        vector<pair<int,int>> path = bfs(sr, sc, maze);
        
        // Move to pill
        for (int i = 0; i < path.size(); i++) {
            int r = path[i].first;
            int c = path[i].second;
            if (maze[r][c] == '.') {
                pills--;
            }
            maze[r][c] = ' ';
        }

        // Output moves
        path.insert(path.begin(), { sr, sc });
        for (int i = 1; i < path.size(); i++) {
            int r0 = path[i-1].first;
            int c0 = path[i-1].second;
            int r1 = path[i].first;
            int c1 = path[i].second;
            if (r1 == r0 - 1) {
                moves.push_back('U');
            } else if (c1 == c0 + 1) {
                moves.push_back('R');
            } else if (r1 == r0 + 1) {
                moves.push_back('D');
            } else if (c1 == c0 - 1) {
                moves.push_back('L');
            }
        }
        sr = path.back().first;
        sc = path.back().second;
    }
    return moves;
}

int main() {
    vector<string> input;
    string line;
    while (getline(cin, line)) {
        input.push_back(line);
    }
    for (int i = 0; i < input.size(); i++) {
        cerr << input[i] << endl;
    }

    string best_moves = solve(input);
    for (int i = 0; i < 1000; i++) {
        string moves = solve(input);
        if (moves.size() < best_moves.size()) {
            best_moves = moves;
        }
    }
    cout << best_moves << endl;
    cerr << "Moves: " << best_moves.size() << endl;
}
