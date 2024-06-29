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

  viewportSpaceSize: number = 2;
  viewportTopLeftSpaceXY: [number, number] = [-1, -1];
  reqCheckPoints: [number, number][] = [];
  cursorCanvasXY?: [number, number];
  dragStartCursorCanvasXY?: [number, number];
  waypoints: Waypoint[] = [];
  futureWaypoints: Waypoint[] = [];

  constructor() {}

  setCheckPointsAndInitViewport(
    reqCheckPoints: [number, number][],
    waypoints?: Waypoint[],
  ) {
    this.reqCheckPoints = reqCheckPoints;
    let minX = -1;
    let minY = -1;
    let maxX = 1;
    let maxY = 1;
    for (const [x, y] of reqCheckPoints) {
      minX = Math.min(minX, x);
      minY = Math.min(minY, y);
      maxX = Math.max(maxX, x);
      maxY = Math.max(maxY, y);
    }
    if (waypoints) {
      for (const [x, y] of waypoints) {
        minX = Math.min(minX, x);
        minY = Math.min(minY, y);
        maxX = Math.max(maxX, x);
        maxY = Math.max(maxY, y);
      }
      this.waypoints = waypoints;
    }
    const dx = maxX - minX;
    const dy = maxY - minY;
    const size = Math.max(dx, dy);
    this.viewportSpaceSize = Math.max(size * 1.05, 2);
    this.viewportTopLeftSpaceXY = [minX - size * 0.025, maxY + size * 0.025];
    console.log(this.viewportSpaceSize);
    console.log(this.viewportTopLeftSpaceXY);
  }

  setWaypoints(waypoints: Waypoint[], futureWaypoints?: Waypoint[]) {
    this.waypoints = waypoints;
    this.futureWaypoints = futureWaypoints ?? [];
  }

  private convertSpaceXYToCanvasXY([x, y]: [number, number]): [number, number] {
    const [minX, maxY] = this.viewportTopLeftSpaceXY;
    const canvasX = (x - minX) / this.viewportSpaceSize;
    const canvasY = (maxY - y) / this.viewportSpaceSize;
    // ドラッグされている最中はこの座標がずれる。
    if (this.dragStartCursorCanvasXY && this.cursorCanvasXY) {
      const [dragStartX, dragStartY] = this.dragStartCursorCanvasXY;
      const [currentX, currentY] = this.cursorCanvasXY;
      const dx = currentX - dragStartX;
      const dy = currentY - dragStartY;
      return [canvasX + dx, canvasY + dy];
    }
    // この座標が0-1の範囲に入ると画面に表示される
    return [canvasX, canvasY];
  }

  public addEventListeners(canvas: HTMLCanvasElement): () => void {
    const mousedownEvent = (e: MouseEvent) => this.mousedownEvent(canvas, e);
    const mouseleaveEvent = () => this.mouseleaveEvent(canvas);
    const mousemoveEvent = (e: MouseEvent) => this.mousemoveEvent(canvas, e);
    const mouseupEvent = (e: MouseEvent) => this.mouseupEvent(canvas, e);
    const wheelEvent = (e: WheelEvent) => this.wheelEvent(canvas, e);
    canvas.addEventListener("mousedown", mousedownEvent);
    canvas.addEventListener("mouseleave", mouseleaveEvent);
    canvas.addEventListener("mousemove", mousemoveEvent);
    canvas.addEventListener("mouseup", mouseupEvent);
    canvas.addEventListener("wheel", wheelEvent);
    return () => {
      canvas.removeEventListener("mousedown", mousedownEvent);
      canvas.removeEventListener("mouseleave", mouseleaveEvent);
      canvas.removeEventListener("mousemove", mousemoveEvent);
      canvas.removeEventListener("mouseup", mouseupEvent);
      canvas.removeEventListener("wheel", wheelEvent);
    };
  }

  private getMouseCanvasXY(
    canvas: HTMLCanvasElement,
    e: MouseEvent,
  ): [number, number] {
    const c = canvas.getBoundingClientRect();
    const pixelCoord = [
      ((e.clientX - c.left) * canvas.width) / c.width,
      ((e.clientY - c.top) * canvas.height) / c.height,
    ];
    return [pixelCoord[0] / canvas.width, pixelCoord[1] / canvas.height];
  }

  private wheelEvent(canvas: HTMLCanvasElement, e: WheelEvent) {
    e.preventDefault();
    if (e.deltaY < 0) {
      this.zoomWithMousePos(0.8, this.getMouseCanvasXY(canvas, e));
    } else {
      this.zoomWithMousePos(1.2, this.getMouseCanvasXY(canvas, e));
    }
    return false;
  }

  public zoomWithMousePos(factor: number, canvasXY: [number, number]) {
    const [spaceX, spaceY] = [
      canvasXY[0] * this.viewportSpaceSize + this.viewportTopLeftSpaceXY[0],
      this.viewportTopLeftSpaceXY[1] - canvasXY[1] * this.viewportSpaceSize,
    ];
    this.viewportSpaceSize *= factor;
    this.viewportTopLeftSpaceXY = [
      spaceX - canvasXY[0] * this.viewportSpaceSize,
      spaceY + canvasXY[1] * this.viewportSpaceSize,
    ];
  }

  private mousedownEvent(canvas: HTMLCanvasElement, e: MouseEvent) {
    this.dragStartCursorCanvasXY = this.getMouseCanvasXY(canvas, e);
  }

  private mouseupEvent(canvas: HTMLCanvasElement, e: MouseEvent) {
    const current = this.getMouseCanvasXY(canvas, e);
    if (
      !this.dragStartCursorCanvasXY ||
      (this.dragStartCursorCanvasXY[0] == current[0] &&
        this.dragStartCursorCanvasXY[1] == current[1])
    ) {
      // 単体クリック
    }
    this.commitDragMove();
    this.dragStartCursorCanvasXY = undefined;
  }

  private commitDragMove() {
    if (this.cursorCanvasXY && this.dragStartCursorCanvasXY) {
      const [dx, dy] = [
        this.cursorCanvasXY[0] - this.dragStartCursorCanvasXY[0],
        this.cursorCanvasXY[1] - this.dragStartCursorCanvasXY[1],
      ];
      this.viewportTopLeftSpaceXY = [
        this.viewportTopLeftSpaceXY[0] - dx * this.viewportSpaceSize,
        this.viewportTopLeftSpaceXY[1] + dy * this.viewportSpaceSize,
      ];
    }
  }

  private mousemoveEvent(canvas: HTMLCanvasElement, e: MouseEvent) {
    this.cursorCanvasXY = this.getMouseCanvasXY(canvas, e);
  }

  private mouseleaveEvent(_: HTMLCanvasElement) {
    this.cursorCanvasXY = undefined;
  }

  plotBasis(ctx: CanvasRenderingContext2D) {
    const cw = ctx.canvas.width;
    const ch = ctx.canvas.height;

    // 座標軸を描画
    const spaceCenterCanvasXY = this.convertSpaceXYToCanvasXY([0, 0]);
    ctx.lineWidth = 2;
    ctx.strokeStyle = "black";
    ctx.moveTo(0, spaceCenterCanvasXY[1] * ch);
    ctx.lineTo(cw, spaceCenterCanvasXY[1] * ch);
    ctx.stroke();

    ctx.strokeStyle = "black";
    ctx.moveTo(spaceCenterCanvasXY[0] * cw, 0);
    ctx.lineTo(spaceCenterCanvasXY[0] * cw, ch);
    ctx.stroke();

    // ReqCheckPointsの点を描画
    const canvasReqCheckPoints = this.reqCheckPoints.map(([x, y]) =>
      this.convertSpaceXYToCanvasXY([x, y]),
    );
    for (const [canvasX, canvasY] of canvasReqCheckPoints) {
      ctx.beginPath();
      ctx.arc(canvasX * cw, canvasY * ch, 8, 0, 2 * Math.PI);
      ctx.fillStyle = "red";
      ctx.fill();
      ctx.lineWidth = 4;
      ctx.strokeStyle = "red";
      ctx.stroke();
    }

    // カーソル座標表示
    if (this.cursorCanvasXY) {
      const [x, y] = this.dragStartCursorCanvasXY ?? this.cursorCanvasXY;
      const [spaceX, spaceY] = [
        Math.round(x * this.viewportSpaceSize + this.viewportTopLeftSpaceXY[0]),
        Math.round(this.viewportTopLeftSpaceXY[1] - y * this.viewportSpaceSize),
      ];
      ctx.font = "64px monospace";
      const text = `(${spaceX}, ${spaceY})`;
      const m = ctx.measureText(text);
      const h = m.actualBoundingBoxAscent + m.actualBoundingBoxDescent;
      ctx.fillStyle = "white";
      ctx.fillRect(0, 0, m.width + 30, h + 30);
      ctx.fillStyle = "black";
      ctx.fillText(text, 0, h);

      // カーソル位置にcrosshairを描画
      const [roundedX, roundedY] = this.convertSpaceXYToCanvasXY([
        spaceX,
        spaceY,
      ]);
      const pixelX = roundedX * cw;
      const pixelY = roundedY * ch;

      ctx.lineWidth = 5;
      ctx.strokeStyle = "black";
      ctx.beginPath();
      ctx.moveTo(pixelX, pixelY - 50);
      ctx.lineTo(pixelX, pixelY + 50);
      ctx.stroke();
      ctx.beginPath();
      ctx.moveTo(pixelX - 50, pixelY);
      ctx.lineTo(pixelX + 50, pixelY);
      ctx.stroke();
    }
  }

  plotWaypoints(ctx: CanvasRenderingContext2D) {
    if (this.waypoints.length === 0) {
      return;
    }
    const cw = ctx.canvas.width;
    const ch = ctx.canvas.height;
    const canvasWaypoints1 = this.waypoints.map(([x, y]) =>
      this.convertSpaceXYToCanvasXY([x, y]),
    );
    const last = this.waypoints.length - 1;
    let futureWaypoints = this.futureWaypoints;
    if (futureWaypoints.length === 0) {
      futureWaypoints = [
        [
          this.waypoints[last][0] + this.waypoints[last][2],
          this.waypoints[last][1] + this.waypoints[last][3],
          0,
          0,
        ],
      ];
    }
    const canvasWaypoints2 = futureWaypoints.map(([x, y]) =>
      this.convertSpaceXYToCanvasXY([x, y]),
    );

    // Waypointsのパスを描画
    ctx.lineWidth = 3;
    ctx.moveTo(canvasWaypoints1[0][0] * cw, canvasWaypoints1[0][1] * ch);
    ctx.beginPath();
    for (const [canvasX, canvasY] of canvasWaypoints1) {
      ctx.lineTo(canvasX * cw, canvasY * ch);
    }
    ctx.strokeStyle = "blue";
    ctx.stroke();

    ctx.beginPath();
    ctx.moveTo(canvasWaypoints1[last][0] * cw, canvasWaypoints1[last][1] * ch);
    for (const [canvasX, canvasY] of canvasWaypoints2) {
      ctx.lineTo(canvasX * cw, canvasY * ch);
    }
    ctx.strokeStyle = "gray";
    ctx.setLineDash([10, 10]);
    ctx.stroke();
    ctx.setLineDash([]);

    // Waypointsの点を描画
    for (const [canvasX, canvasY] of canvasWaypoints1) {
      ctx.beginPath();
      ctx.arc(canvasX * cw, canvasY * ch, 10, 0, 2 * Math.PI);
      ctx.fillStyle = "blue";
      ctx.fill();
      ctx.lineWidth = 4;
      ctx.strokeStyle = "blue";
      ctx.stroke();
    }
    ctx.beginPath();
    ctx.arc(
      canvasWaypoints1[last][0] * cw,
      canvasWaypoints1[last][1] * ch,
      30,
      0,
      2 * Math.PI,
    );
    ctx.fillStyle = "blue";
    ctx.fill();
    ctx.lineWidth = 5;
    ctx.strokeStyle = "blue";
    ctx.stroke();
    for (const [canvasX, canvasY] of canvasWaypoints2) {
      ctx.beginPath();
      ctx.arc(canvasX * cw, canvasY * ch, 10, 0, 2 * Math.PI);
      ctx.fillStyle = "gray";
      ctx.fill();
      ctx.lineWidth = 4;
      ctx.strokeStyle = "gray";
      ctx.stroke();
    }
  }
}

export function calculateSplitWaypoints(
  path1: string,
  path2: string,
): [Waypoint[], Waypoint[]] {
  const waypoints1: Waypoint[] = [];
  const waypoints2: Waypoint[] = [];
  let currentX = 0;
  let currentY = 0;
  let currentVX = 0;
  let currentVY = 0;
  waypoints1.push([currentX, currentY, currentVX, currentVY]);
  for (let i = 0; i < path1.length; i++) {
    const [dx, dy] = getDxDy(path1[i]);
    currentVX += dx;
    currentVY += dy;
    currentX += currentVX;
    currentY += currentVY;
    waypoints1.push([currentX, currentY, currentVX, currentVY]);
  }
  for (let i = 0; i < path2.length; i++) {
    const [dx, dy] = getDxDy(path2[i]);
    currentVX += dx;
    currentVY += dy;
    currentX += currentVX;
    currentY += currentVY;
    waypoints2.push([currentX, currentY, currentVX, currentVY]);
  }
  return [waypoints1, waypoints2];
}

export function calculateWaypoints(path: string): Waypoint[] {
  const waypoints: Waypoint[] = [];
  let currentX = 0;
  let currentY = 0;
  let currentVX = 0;
  let currentVY = 0;
  waypoints.push([currentX, currentY, currentVX, currentVY]);
  for (let i = 0; i < path.length; i++) {
    const [dx, dy] = getDxDy(path[i]);
    currentVX += dx;
    currentVY += dy;
    currentX += currentVX;
    currentY += currentVY;
    waypoints.push([currentX, currentY, currentVX, currentVY]);
  }
  return waypoints;
}

function getDxDy(c: string): [number, number] {
  switch (c) {
    case "1":
      return [-1, -1];
    case "2":
      return [0, -1];
    case "3":
      return [1, -1];
    case "4":
      return [-1, 0];
    case "5":
      return [0, 0];
    case "6":
      return [1, 0];
    case "7":
      return [-1, 1];
    case "8":
      return [0, 1];
    case "9":
      return [1, 1];
    default:
      return [-Infinity, -Infinity];
  }
}

export function parseReqPoints(reqPointsStr: string): [number, number][] {
  return reqPointsStr
    .split("\n")
    .map((line) => line.split(" ").map((s) => parseInt(s)))
    .filter((point) => point.length === 2) as [number, number][];
}
