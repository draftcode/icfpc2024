export enum CellState {
  Wall = "#",
  Pill = ".",
  Lambda = "L",
  Done = "",
}

const D = {
  U: [-1, 0],
  D: [1, 0],
  L: [0, -1],
  R: [0, 1],
};

export class LambdamanMap {
  width: number;
  height: number;
  chars: CellState[][];
  x = 0;
  y = 0;
  remaining = 0;

  constructor(raw: string) {
    const lines = raw.trim().split("\n");
    this.width = lines[0].length;
    this.height = lines.length;
    this.chars = lines.map((line) =>
      line
        .trim()
        .split("")
        .map((x) => x as CellState)
    );

    for (let x = 0; x < this.height; x++) {
      for (let y = 0; y < this.width; y++) {
        if (this.chars[x][y] === CellState.Lambda) {
          this.x = x;
          this.y = y;
        }
        if (this.chars[x][y] === CellState.Pill) {
          this.remaining++;
        }
      }
    }
  }

  clone(): LambdamanMap {
    const m = new LambdamanMap("");
    m.width = this.width;
    m.height = this.height;
    m.chars = this.chars.map((line) => line.slice());
    m.x = this.x;
    m.y = this.y;
    m.remaining = this.remaining;
    return m;
  }

  walk(dirs: string) {
    for (const x of dirs) {
      const dxdy = D[x as "U" | "D" | "L" | "R"];
      if (!dxdy) continue;
      const [dx, dy] = dxdy;
      const [nx, ny] = [this.x + dx, this.y + dy];
      if (nx < 0 || nx >= this.height || ny < 0 || ny >= this.width) {
        continue;
      }
      if (this.chars[nx][ny] === CellState.Wall) {
        continue;
      }
      this.chars[this.x][this.y] = CellState.Done;
      if (this.chars[nx][ny] === CellState.Pill) {
        this.remaining--;
      }
      this.chars[nx][ny] = CellState.Lambda;
      this.x = nx;
      this.y = ny;
    }
  }
}
