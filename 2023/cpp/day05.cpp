#include "/home/origami/Dev/projects/dotfiles/snippets/debug.hpp"

#define println(x) cout << x << "\n"
#define ff first
#define ss second
#define all(x) x.begin(), x.end()
#define rall(x) x.rbegin(), x.rend()

#define int ll
typedef long long ll;

void solution_part1();

signed main(signed args, char const *argv[]) {
  ios_base::sync_with_stdio(false), cin.tie(nullptr), cout.tie(nullptr);
  cin.exceptions(cin.failbit);
  solution_part1();
  return 0;
}

struct Range {
  int dest, src, len;
  friend ostream &operator<<(std::ostream &ost, const Range range) {
    ost << make_tuple(range.dest, range.src, range.len);
    return ost;
  }
};

int get_loc_val(vector<vector<Range>> &maps, int seed) {
  for (auto &ranges : maps) {
    for (auto &range : ranges) {
      if (range.src <= seed and seed < (range.src + range.len)) {
        seed = (seed - range.src) + range.dest;
        break;
      }
    }
  }
  return seed;
}

void solution_part1() {
  // parse the seeds
  string line;
  getline(cin, line);
  auto const colon(line.find(':'));
  stringstream line_ss(line.substr(colon + 1));
  vector<int> seeds;
  int seed;
  while (line_ss >> seed)
    seeds.push_back(seed);

  vector<vector<Range>> maps;
  while (!cin.eof() and getline(cin, line)) {
    if (line.empty()) {
      getline(cin, line); // eat a line
      maps.push_back(vector<Range>());
    } else {
      line_ss = stringstream(line);
      int dest, src, len;
      line_ss >> dest >> src >> len;
      maps.back().emplace_back(dest, src, len);
    }
  }
  dbg(seeds);
  dbg(maps);

  int minimum = numeric_limits<int>::max();
  for (int seed : seeds) {
    auto location = get_loc_val(maps, seed);
    dbg(location);
    minimum = min(minimum, location);
  }
  println(minimum);

  // Part 2

}
