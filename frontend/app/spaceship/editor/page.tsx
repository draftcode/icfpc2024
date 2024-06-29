"use client";

import {
  calculateWaypoints,
  WaypointVizState,
} from "@/components/spaceviz/state";
import { useEffect, useMemo, useRef, useState } from "react";

export default function Page() {
  const [path, setPath] = useState("236659");
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
      </form>
      <Visualizer key={vizPath} path={vizPath} />
    </div>
  );
}

const CANVAS_SIZE = 4000;

function Visualizer({ path }: { path: string }) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const waypoints = useMemo(() => calculateWaypoints(path), [path]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) {
      return;
    }

    const state = new WaypointVizState(waypoints);

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
