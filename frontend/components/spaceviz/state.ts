type Waypoint = [number, number, number, number]; // [x, y, vx, vy]

export class WaypointVizState {
  // Space座標系
  //
  //           +y
  //           |
  //           |
  //  -x ------+------ +x
  //           |
  //           |
  //           -y
  //
  // Canvas座標系
  //
  //     -y
  //  -x +------------ +x
  //     |
  //     |
  //     |
  //     |
  //     +y

  viewportSpaceSize: number;
  viewportTopLeftSpaceXY: [number, number];
  waypoints: Waypoint[];

  constructor(waypoints: Waypoint[]) {
    this.waypoints = waypoints;
    let minX = Infinity;
    let minY = Infinity;
    let maxX = -Infinity;
    let maxY = -Infinity;
    for (const [x, y] of waypoints) {
      minX = Math.min(minX, x);
      minY = Math.min(minY, y);
      maxX = Math.max(maxX, x);
      maxY = Math.max(maxY, y);
    }
    const dx = maxX - minX;
    const dy = maxY - minY;
    this.viewportSpaceSize = Math.max(dx, dy);
    this.viewportTopLeftSpaceXY = [minX, maxY];
  }

  convertWaypointsToCanvasXY(waypoints: Waypoint[]): [number, number][] {
    return waypoints.map(([x, y]) => {
      const [minX, maxY] = this.viewportTopLeftSpaceXY;
      const canvasX = (x - minX) / this.viewportSpaceSize;
      const canvasY = (maxY - y) / this.viewportSpaceSize;
      // この座標が0-1の範囲に入ると画面に表示される
      return [canvasX, canvasY];
    });
  }

  plotWaypoints(ctx: CanvasRenderingContext2D) {
    const canvasWaypoints = this.convertWaypointsToCanvasXY(this.waypoints);
    ctx.lineWidth = 3;
    ctx.beginPath();
    ctx.moveTo(...canvasWaypoints[0]);
    for (const [canvasX, canvasY] of canvasWaypoints) {
      ctx.lineTo(canvasX * ctx.canvas.width, canvasY * ctx.canvas.height);
    }
    ctx.stroke();
  }
}

export function calculateWaypoints(path: string): Waypoint[] {
  const waypoints: Waypoint[] = [];
  let currentX = 0;
  let currentY = 0;
  let currentVX = 0;
  let currentVY = 0;
  waypoints.push([currentX, currentY, currentVX, currentVY]);
  for (let i = 0; i < path.length; i++) {
    switch (path[i]) {
      case "1":
        currentVX -= 1;
        currentVY -= 1;
        break;
      case "2":
        currentVY -= 1;
        break;
      case "3":
        currentVX += 1;
        currentVY -= 1;
        break;
      case "4":
        currentVX -= 1;
        break;
      case "5":
        break;
      case "6":
        currentVX += 1;
        break;
      case "7":
        currentVX -= 1;
        currentVY += 1;
        break;
      case "8":
        currentVY += 1;
        break;
      case "9":
        currentVX += 1;
        currentVY += 1;
        break;
    }
    currentX += currentVX;
    currentY += currentVY;
    waypoints.push([currentX, currentY, currentVX, currentVY]);
  }
  return waypoints;
}
