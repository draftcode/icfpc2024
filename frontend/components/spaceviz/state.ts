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

  waypoints: Waypoint[];
  canvasWaypoints: [number, number][];
  reqCheckPoints: [number, number][];
  canvasReqCheckPoints: [number, number][];
  spaceCenterCanvasXY: [number, number];

  constructor(waypoints: Waypoint[], reqCheckPoints: [number, number][]) {
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
    for (const [x, y] of reqCheckPoints) {
      minX = Math.min(minX, x);
      minY = Math.min(minY, y);
      maxX = Math.max(maxX, x);
      maxY = Math.max(maxY, y);
    }
    const dx = maxX - minX;
    const dy = maxY - minY;
    const size = Math.max(dx, dy);
    const viewportSpaceSize = size * 1.05;
    const viewportTopLeftSpaceXY = [minX - size * 0.025, maxY + size * 0.025];
    const convertSpaceXYToCanvasXY = ([x, y]: [number, number]): [
      number,
      number,
    ] => {
      const [minX, maxY] = viewportTopLeftSpaceXY;
      const canvasX = (x - minX) / viewportSpaceSize;
      const canvasY = (maxY - y) / viewportSpaceSize;
      // この座標が0-1の範囲に入ると画面に表示される
      return [canvasX, canvasY];
    };
    this.canvasWaypoints = waypoints.map(([x, y]) =>
      convertSpaceXYToCanvasXY([x, y]),
    );
    this.reqCheckPoints = reqCheckPoints;
    this.canvasReqCheckPoints = reqCheckPoints.map(convertSpaceXYToCanvasXY);
    this.spaceCenterCanvasXY = convertSpaceXYToCanvasXY([0, 0]);
  }

  plotWaypoints(ctx: CanvasRenderingContext2D) {
    const cw = ctx.canvas.width;
    const ch = ctx.canvas.height;

    ctx.lineWidth = 3;
    ctx.clearRect(0, 0, cw, ctx.canvas.height);

    // Waypointsのパスを描画
    ctx.strokeStyle = "blue";
    ctx.moveTo(
      this.canvasWaypoints[0][0] * cw,
      this.canvasWaypoints[0][1] * ch,
    );
    ctx.beginPath();
    for (const [canvasX, canvasY] of this.canvasWaypoints) {
      ctx.lineTo(canvasX * cw, canvasY * ch);
    }
    ctx.stroke();

    // Waypointsの点を描画
    for (const [canvasX, canvasY] of this.canvasWaypoints) {
      ctx.beginPath();
      ctx.arc(canvasX * cw, canvasY * ch, 10, 0, 2 * Math.PI);
      ctx.fillStyle = "blue";
      ctx.fill();
      ctx.lineWidth = 4;
      ctx.strokeStyle = "blue";
      ctx.stroke();
    }

    // ReqCheckPointsの点を描画
    for (const [canvasX, canvasY] of this.canvasReqCheckPoints) {
      ctx.beginPath();
      ctx.arc(canvasX * cw, canvasY * ch, 8, 0, 2 * Math.PI);
      ctx.fillStyle = "red";
      ctx.fill();
      ctx.lineWidth = 4;
      ctx.strokeStyle = "red";
      ctx.stroke();
    }

    // 座標軸を描画
    ctx.lineWidth = 2;
    ctx.strokeStyle = "black";
    ctx.moveTo(0, this.spaceCenterCanvasXY[1] * ch);
    ctx.lineTo(cw, this.spaceCenterCanvasXY[1] * ch);
    ctx.stroke();

    ctx.strokeStyle = "black";
    ctx.moveTo(this.spaceCenterCanvasXY[0] * cw, 0);
    ctx.lineTo(this.spaceCenterCanvasXY[0] * cw, ch);
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

export function parseReqPoints(reqPointsStr: string): [number, number][] {
  return reqPointsStr
    .split("\n")
    .map((line) => line.split(" ").map((s) => parseInt(s)))
    .filter((point) => point.length === 2) as [number, number][];
}
