#include <iostream>
#include <vector>
#include <queue>
#include <algorithm>
#include <bitset>
using namespace std;

constexpr int MIN_V = -40;
constexpr int MAX_V = 40;
constexpr int NUM_V = MAX_V - MIN_V + 1;
constexpr int MIN_D = -4000;
constexpr int MAX_D = 4000;
constexpr int NUM_D = MAX_D - MIN_D + 1;
constexpr int BEAM_WIDTH = 100;

constexpr int AX[] = {-1, 0, 1};

/*
 * min_steps[vx1][vx2][dx]: minimum steps to move dx with initial velocity vx1 and terminal velocity vx2.
 *   this is one dimension.
 * vector<(steps, terminal velocity)> get_min_steps(vx, dx): return the minimum steps to move dx with initial velocity vx. The terminal velocity is unspecified.
 * max(get_min_steps(vx, dx).first, get_min_steps(vy, dy).first): lower bound of the steps to move (dx, dy) with initial velocity (vx, vy).
 * get_moves(vx, dx, t): return the path to move dx with initial velocity vx in exactly t steps.
 *   - Going through terminal velocity and check if min_steps is t.
 *   - Divide vx into dx1 + dx2, and check if min_steps[vx1][*][dx1] + min_steps[*][**][dx2] is t.
 *   - Make random 9-pad moves, and check if remaining min_steps is t-1.
 * Beam search for visiting order?
 *   - Penalties
 *     - Moves made
 *     - Bounding box size(area?) of remaining points.
 *     - Current velocity (slower is better?)
 * Or, SA
 *   - Moves
 *       - Swap two points in the order.
 *
 */


struct Vec2d { int x, y; Vec2d(int x, int y) : x(x), y(y) {} };
struct MovePack {
    int steps, terminal_velocity;
    MovePack(int steps, int terminal_velocity) : steps(steps), terminal_velocity(terminal_velocity) {}
    MovePack() : steps(1e9), terminal_velocity(0) {}
};
struct QueueState {
    int v0, v, pos, steps;
    QueueState(int v0, int v, int pos, int steps) : v0(v0), v(v), pos(pos), steps(steps) {}
};
struct BeamState {
    int moves, vx, vy;
    BeamState* prev = nullptr;
};


// Find the minimum steps from x1 to x2 with initial velocity v (terminal velocity is free)
vector<MovePack> get_moves(int v, int dx, int max_results, const vector<vector<vector<int>>>& min_steps) {
    vector<MovePack> moves;
    for (int ve = MIN_V; ve < MAX_V; ve++) {
        int steps = min_steps[v-MIN_V][ve-MIN_V][dx-MIN_D];
        if (steps == 1e9) continue;
        moves.emplace_back(steps, ve);
    }
    sort(moves.begin(), moves.end(), [](const MovePack& a, const MovePack& b) {
        return a.steps < b.steps || (a.steps == b.steps && abs(a.terminal_velocity) < abs(b.terminal_velocity));
    });
    if (moves.size() > max_results) {
        moves.resize(max_results);
    }
    return moves;
}

vector<MovePack> get_moves_with_steps(int v, int dx, int steps, int max_results, const vector<vector<vector<int>>>& min_steps) {
    vector<MovePack> moves;
    for (int ve = MIN_V; ve < MAX_V; ve++) {
        int steps1 = min_steps[v-MIN_V][ve-MIN_V][dx-MIN_D];
        if (steps1 != steps) continue;
        moves.emplace_back(steps1, ve);
    }
    if (steps >= 1) {
        for (int k = 0; k < 3; k++) {
            int nv = v + AX[k];
            int ndx = dx - nv;
            if (nv < MIN_V || nv >= MAX_V) continue;
            if (ndx < MIN_D || ndx >= MAX_D) continue;
            for (int ve = MIN_V; ve < MAX_V; ve++) {
                int steps1 = min_steps[nv-MIN_V][ve-MIN_V][ndx-MIN_D];
                if (steps1 == steps - 1) {
                    moves.emplace_back(steps, ve);
                }
            }
            //vector<MovePack> moves1 = get_moves_with_steps(nv, ndx, steps - 1, max_results, min_steps);
            //for (int i = 0; i < moves1.size(); i++) {
            //    moves.emplace_back(steps, moves1[i].terminal_velocity);
            //}
        }
    }
    if (steps >= 2) {
        for (int a0 = -1; a0 <= 1; a0++) {
            for (int a1 = -1; a1 <= 1; a1++) {
                int nv1 = v + a0;
                int ndx1 = dx - nv1;
                int nv2 = nv1 + a1;
                int ndx2 = ndx1 - nv2;
                if (nv2 < MIN_V || nv2 >= MAX_V) continue;
                if (ndx2 < MIN_D || ndx2 >= MAX_D) continue;
                for (int ve = MIN_V; ve < MAX_V; ve++) {
                    int steps1 = min_steps[nv2-MIN_V][ve-MIN_V][ndx2-MIN_D];
                    if (steps1 == steps - 2) {
                        moves.emplace_back(steps, ve);
                    }
                }
            }
        }
    }

    /*
    if (steps == 1) {
        for (int i=0; i<moves.size(); i++) {
            cerr << "steps: " << moves[i].steps << " vel: " << moves[i].terminal_velocity << endl;
        }
    }
     */
    if (moves.empty()) {
        //cerr << "get_moves_with_steps(v: " << v << ", dx: " << dx << ", steps: " << steps << ") -  Moves are empty" << endl;
        return moves;
    }

    sort(moves.begin(), moves.end(), [](const MovePack& a, const MovePack& b) {
        return abs(a.terminal_velocity) < abs(b.terminal_velocity);
    });
    if (moves.size() > max_results) {
        moves.resize(max_results);
    }
    //cerr << "get_moves_with_steps(v: " << v << ", dx: " << dx << ", steps: " << steps << ") -  Found " << moves.size() << " moves" << endl;
    return moves;
}

bool can_reach(int vs, int ve, int dx, int steps, const vector<vector<vector<int>>>& min_steps) {
    if (min_steps[vs-MIN_V][ve-MIN_V][dx-MIN_D] == steps) {
        return true;
    }
    if (steps >= 1) {
        for (int k = 0; k < 3; k++) {
            int nv = vs + AX[k];
            int ndx = dx - nv;
            if (nv < MIN_V || nv >= MAX_V) continue;
            if (ndx < MIN_D || ndx >= MAX_D) continue;
            if (min_steps[nv-MIN_V][ve-MIN_V][ndx-MIN_D] == steps - 1) {
                return true;
            }
        }
    }
    if (steps >= 2) {
        for (int a0 = -1; a0 <= 1; a0++) {
            for (int a1 = -1; a1 <= 1; a1++) {
                int nv1 = vs + a0;
                int ndx1 = dx - nv1;
                int nv2 = nv1 + a1;
                int ndx2 = ndx1 - nv2;
                if (nv2 < MIN_V || nv2 >= MAX_V) continue;
                if (ndx2 < MIN_D || ndx2 >= MAX_D) continue;
                if (min_steps[nv2-MIN_V][ve-MIN_V][ndx2-MIN_D] == steps - 2) {
                    return true;
                }
            }
        }
    }
    return false;
}

vector<int> reconstruct_steps(int dx, int vs, int ve, int steps, const vector<vector<vector<int>>>& min_steps) {
    vector<int> result;
    int v = vs;
    for (int i = 0; i < steps; i++) {
        for (int k = 0; k < 3; k++) {
            int nv = v + AX[k];
            int ndx = dx - nv;
            if (nv < MIN_V || nv >= MAX_V) continue;
            if (ndx < MIN_D || ndx >= MAX_D) continue;
            if (can_reach(nv, ve, ndx, steps - i - 1, min_steps)) {
                v = nv;
                dx = ndx;
                result.push_back(AX[k]);
                break;
            }
        }
    }
    return result;
}

int main() {
    vector<Vec2d> nodes;
    {
        int x, y;
        while (cin >> x >> y) {
            nodes.push_back(Vec2d(x, y));
        }
    }

    vector<vector<vector<int>>> min_steps(NUM_V, vector<vector<int>>(NUM_V, vector<int>(NUM_D, 1e9)));
    queue<QueueState> q;
    for (int v = MIN_V; v <= MAX_V; v++) {
        min_steps[v-MIN_V][v-MIN_V][0-MIN_D] = 0;
        q.push(QueueState(v, v, 0, 0));
    }

    while (!q.empty()) {
        auto s = q.front();
        q.pop();
        if (min_steps[s.v0-MIN_V][s.v-MIN_V][s.pos-MIN_D] < s.steps) continue;
        for (int i = 0; i < 3; i++) {
            int nv = s.v + AX[i];
            int npos = s.pos + nv;
            if (nv < MIN_V || nv >= MAX_V) continue;
            if (npos < MIN_D || npos >= MAX_D) continue;
            if (min_steps[s.v0-MIN_V][nv-MIN_V][npos-MIN_D] <= s.steps + 1) continue;

            min_steps[s.v0-MIN_V][nv-MIN_V][npos-MIN_D] = s.steps + 1;
            q.push(QueueState(s.v0, nv, npos, s.steps + 1));
        }
    }

    int x = 0, y = 0;
    vector<vector<BeamState>> beams(nodes.size() + 1);
    beams[0].push_back({0, 0, 0, nullptr});
    for (int i = 0; i < nodes.size(); i++) {
        //cerr << "(" << x << ", " << y << ") to (" << nodes[i].x << ", " << nodes[i].y << ")" << endl;
        for (int j = 0; j < beams[i].size(); j++) {
            int cur_vx = beams[i][j].vx;
            int cur_vy = beams[i][j].vy;
            int cur_moves = beams[i][j].moves;

            // Find the minimum steps to move to the node in X/Y axis
            vector <MovePack> moves_x = get_moves(cur_vx, nodes[i].x - x, 10, min_steps);

            /*
            if (x == -28 && y == 106) {
                cerr << "Moves X: " << moves_x.size() << " / " << "cur_vx: " << cur_vx << " dx: " << (nodes[i].x - x)
                     << endl;
                for (int k = 0; k < moves_x.size(); k++) {
                    cerr << "steps: " << moves_x[k].steps << " vel: " << moves_x[k].terminal_velocity << endl;
                }
            }
             */

            vector <MovePack> moves_y = get_moves(cur_vy, nodes[i].y - y, 10, min_steps);
            /*
            if (x == -28 && y == 106) {
                cerr << "Moves Y: " << moves_y.size() << " / " << "cur_vy: " << cur_vy << " dy: " << (nodes[i].y - y)
                     << endl;
                for (int k = 0; k < moves_y.size(); k++) {
                    cerr << "steps: " << moves_y[k].steps << " vel: " << moves_y[k].terminal_velocity << endl;
                }
            }
             */
            if (moves_x.empty() || moves_y.empty()) continue;

            // Find the minimum steps to move to the node in X/Y axis with the same steps
            int lb_steps = std::max(moves_x[0].steps, moves_y[0].steps);
            for (int s = lb_steps; s <= lb_steps + 10; s++) {
                vector<MovePack> moves_x = get_moves_with_steps(cur_vx, nodes[i].x - x, s, 10, min_steps);
                vector<MovePack> moves_y = get_moves_with_steps(cur_vy, nodes[i].y - y, s, 10, min_steps);
                if (moves_x.empty() || moves_y.empty()) continue;

                for (int k = 0; k < moves_x.size(); k++) {
                    for (int l = 0; l < moves_y.size(); l++) {
                        beams[i+1].push_back({cur_moves + s, moves_x[k].terminal_velocity, moves_y[l].terminal_velocity, &beams[i][j]});
                    }
                }
            }
        }

        sort(beams[i+1].begin(), beams[i+1].end(), [](const BeamState& a, const BeamState& b) {
            return a.moves < b.moves || (a.moves == b.moves && (abs(a.vx) + abs(a.vy) < abs(b.vx) + abs(b.vy))) ||
                    (a.moves == b.moves && (abs(a.vx) + abs(a.vy) == abs(b.vx) + abs(b.vy)) && a.vx < b.vx) ||
                    (a.moves == b.moves && (abs(a.vx) + abs(a.vy) == abs(b.vx) + abs(b.vy)) && a.vx == b.vx && a.vy < b.vy);
        });
        unique(beams[i+1].begin(), beams[i+1].end(), [](const BeamState& a, const BeamState& b) {
            return a.vx == b.vx && a.vy == b.vy && a.moves == b.moves;
        });

        if (beams[i+1].size() > BEAM_WIDTH) {
            beams[i+1].resize(BEAM_WIDTH);
        }
        /*
        cerr << "------------------" << endl;
        for (int j = 0; j < beams[i+1].size(); j++) {
            cerr << beams[i+1][j].moves << " " << beams[i+1][j].vx << " " << beams[i+1][j].vy << endl;
        }
         */
        //if (i == 1) return 1;
        x = nodes[i].x;
        y = nodes[i].y;
    }

    std::cerr << "Visited nodes: " << nodes.size() << std::endl;
    std::cerr << "Best moves: " << beams[nodes.size()][0].moves << std::endl;

    // Reconstruct the steps.
    vector<int> vxs;
    vector<int> vys;
    vector<int> steps_history;
    for (auto* p = &beams[nodes.size()][0]; p != nullptr; p = p->prev) {
        vxs.push_back(p->vx);
        vys.push_back(p->vy);
        if (p->prev != nullptr) {
            steps_history.push_back(p->moves - p->prev->moves);
        }
    }
    reverse(vxs.begin(), vxs.end());
    reverse(vys.begin(), vys.end());
    reverse(steps_history.begin(), steps_history.end());


    string ans;
    int cx = 0;
    int cy = 0;
    int cvx = 0;
    int cvy = 0;
    for (int i=0; i<nodes.size(); i++) {
        int nx = nodes[i].x;
        int ny = nodes[i].y;
        int nvx = vxs[i+1];
        int nvy = vys[i+1];
        vector<int> steps_x = reconstruct_steps(nx - cx, cvx, nvx, steps_history[i], min_steps);
        vector<int> steps_y = reconstruct_steps(ny - cy, cvy, nvy, steps_history[i], min_steps);
        /*
        for (int i=0; i<steps_x.size(); i++) {
            cout << "(" << steps_x[i] << ", " << steps_y[i] << ")" << " ";
        }
        cout << endl;
        */
        for (int j=0; j<steps_x.size(); j++) {
            if (steps_x[j] == -1 && steps_y[j] == -1) ans += '1';
            if (steps_x[j] == -1 && steps_y[j] == 0) ans += '4';
            if (steps_x[j] == -1 && steps_y[j] == 1) ans += '7';
            if (steps_x[j] == 0 && steps_y[j] == -1) ans += '2';
            if (steps_x[j] == 0 && steps_y[j] == 0) ans += '5';
            if (steps_x[j] == 0 && steps_y[j] == 1) ans += '8';
            if (steps_x[j] == 1 && steps_y[j] == -1) ans += '3';
            if (steps_x[j] == 1 && steps_y[j] == 0) ans += '6';
            if (steps_x[j] == 1 && steps_y[j] == 1) ans += '9';
        }


        cx = nx;
        cy = ny;
        cvx = nvx;
        cvy = nvy;
    }

    cout << ans << endl;

    return 0;
}
