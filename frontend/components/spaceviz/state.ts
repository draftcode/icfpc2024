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
  reqCheckPoints: [number, number][];
  cursorCanvasXY?: [number, number];
  dragStartCursorCanvasXY?: [number, number];

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
    this.viewportSpaceSize = size * 1.05;
    this.viewportTopLeftSpaceXY = [minX - size * 0.025, maxY + size * 0.025];
    this.reqCheckPoints = reqCheckPoints;
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

  plotWaypoints(ctx: CanvasRenderingContext2D) {
    const cw = ctx.canvas.width;
    const ch = ctx.canvas.height;
    const canvasReqCheckPoints = this.reqCheckPoints.map(([x, y]) =>
      this.convertSpaceXYToCanvasXY([x, y]),
    );
    const spaceCenterCanvasXY = this.convertSpaceXYToCanvasXY([0, 0]);
    const canvasWaypoints = this.waypoints.map(([x, y]) =>
      this.convertSpaceXYToCanvasXY([x, y]),
    );

    ctx.lineWidth = 3;
    ctx.clearRect(0, 0, cw, ctx.canvas.height);

    // Waypointsのパスを描画
    ctx.strokeStyle = "blue";
    ctx.moveTo(canvasWaypoints[0][0] * cw, canvasWaypoints[0][1] * ch);
    ctx.beginPath();
    for (const [canvasX, canvasY] of canvasWaypoints) {
      ctx.lineTo(canvasX * cw, canvasY * ch);
    }
    ctx.stroke();

    // Waypointsの点を描画
    for (const [canvasX, canvasY] of canvasWaypoints) {
      ctx.beginPath();
      ctx.arc(canvasX * cw, canvasY * ch, 10, 0, 2 * Math.PI);
      ctx.fillStyle = "blue";
      ctx.fill();
      ctx.lineWidth = 4;
      ctx.strokeStyle = "blue";
      ctx.stroke();
    }

    // ReqCheckPointsの点を描画
    for (const [canvasX, canvasY] of canvasReqCheckPoints) {
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
    ctx.moveTo(0, spaceCenterCanvasXY[1] * ch);
    ctx.lineTo(cw, spaceCenterCanvasXY[1] * ch);
    ctx.stroke();

    ctx.strokeStyle = "black";
    ctx.moveTo(spaceCenterCanvasXY[0] * cw, 0);
    ctx.lineTo(spaceCenterCanvasXY[0] * cw, ch);
    ctx.stroke();

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
