"use client";

import {
  calculateWaypoints,
  parseReqPoints,
  WaypointVizState,
} from "@/components/spaceviz/state";
import { useEffect, useMemo, useRef, useState } from "react";

export default function Page() {
  const [path, setPath] = useState("236659");
  const [reqPointsStr, setReqPointStr] = useState("");
  const [vizPath, setVizPath] = useState("236659");

  return (
    <div>
      <form
        className="flex"
        onSubmit={(e) => {
          e.preventDefault();
          setVizPath(path);
        }}
      >
        <input
          type="text"
          value={path}
          onChange={(e) => setPath(e.target.value)}
          className="input input-bordered w-full max-w-xs"
        />
        <textarea
          className="textarea textarea-bordered"
          value={reqPointsStr}
          onChange={(e) => setReqPointStr(e.target.value)}
        />
      </form>
      <Visualizer
        key={vizPath + reqPointsStr}
        path={vizPath}
        reqPointsStr={reqPointsStr}
      />
    </div>
  );
}

const CANVAS_SIZE = 4000;

function Visualizer({
  path,
  reqPointsStr,
}: {
  path: string;
  reqPointsStr: string;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const waypoints = useMemo(() => calculateWaypoints(path), [path]);
  const reqPoints = useMemo(() => parseReqPoints(reqPointsStr), [reqPointsStr]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) {
      return;
    }

    const state = new WaypointVizState(waypoints, reqPoints);
    let animationFrameId: number = 0;
    const render = () => {
      const ctx = canvas.getContext("2d");
      if (ctx) {
        state.plotWaypoints(ctx);
      }
      animationFrameId = window.requestAnimationFrame(render);
    };
    render();
    return () => {
      window.cancelAnimationFrame(animationFrameId);
    };
  }, [canvasRef]);

  return (
    <canvas
      className="w-full h-full border"
      ref={canvasRef}
      width={CANVAS_SIZE}
      height={CANVAS_SIZE}
    ></canvas>
  );
}
