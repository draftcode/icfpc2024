export interface CellValue {
  coord: [number, number];
  value: string;
}

export function parseStateString(stateString: string): Map<string, CellValue> {
  const state = new Map<string, CellValue>();
  const lines = stateString.split("\n");
  for (let y = 0; y < lines.length; y++) {
    const line = lines[y];
    const vs = line.split(" ").filter((value) => value !== "");
    for (let x = 0; x < vs.length; x++) {
      const value = vs[x];
      if (value !== ".") {
        state.set(`${x},${y}`, { coord: [x, y], value });
      }
    }
  }
  return state;
}

export function serializeState(state: Map<string, CellValue>): string {
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (const cell of Array.from(state.values())) {
    minX = Math.min(minX, cell.coord[0]);
    minY = Math.min(minY, cell.coord[1]);
    maxX = Math.max(maxX, cell.coord[0]);
    maxY = Math.max(maxY, cell.coord[1]);
  }
  const lines = [];
  for (let y = minY; y <= maxY; y++) {
    const line = [];
    for (let x = minX; x <= maxX; x++) {
      const cell = state.get(`${x},${y}`);
      if (cell) {
        line.push(`${cell.value}`);
      } else {
        line.push(".");
      }
    }
    lines.push(line.join(" "));
  }
  return lines.join("\n");
}
