"use client";

import { useProblem } from "@/components/api";
import {
  WaypointVizState,
  calculateWaypoints,
} from "@/components/spaceviz/state";
import { parseReqPoints } from "@/components/spaceviz/state";
import { useCallback, useEffect, useRef, useState } from "react";

const CANVAS_SIZE = 4000;

export default function Page() {
  const canvasCancelRef = useRef<(() => void) | null>(null);
  const vizStateRef = useRef<WaypointVizState>(new WaypointVizState());
  const [problem, setProblem] = useState(1);
  const { data: problemData, error: problemError } = useProblem(
    "spaceship",
    problem,
  );
  const [path, setPath] = useState("");

  useEffect(() => {
    const reqPoints = parseReqPoints(problemData?.content ?? "");
    vizStateRef.current.setCheckPointsAndInitViewport(reqPoints);
  }, [problemData?.content]);

  useEffect(() => {
    const waypoints = calculateWaypoints(path);
    vizStateRef.current.setWaypoints(waypoints);
  }, [path]);

  const initCanvas = useCallback((canvas: HTMLCanvasElement | null) => {
    if (canvas) {
      const remove = vizStateRef.current.addEventListeners(canvas);
      let animationFrameId: number = 0;
      const render = () => {
        const ctx = canvas.getContext("2d");
        if (ctx) {
          ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
          vizStateRef.current.plotWaypoints(ctx);
          vizStateRef.current.plotBasis(ctx);
        }
        animationFrameId = window.requestAnimationFrame(render);
      };
      render();
      canvasCancelRef.current = () => {
        remove();
        window.cancelAnimationFrame(animationFrameId);
      };
    } else {
      if (canvasCancelRef.current) {
        canvasCancelRef.current();
      }
    }
  }, []);

  if (problemError) {
    throw problemError;
  }
  if (!problemData) {
    return null;
  }

  return (
    <div className="space-y-2 p-4">
      <ProblemChooser problem={problem} setProblem={setProblem} />
      <input
        type="text"
        value={path}
        onChange={(e) => setPath(e.target.value)}
        className="input input-bordered w-full"
      />
      <canvas
        className="w-full h-full border"
        ref={initCanvas}
        width={CANVAS_SIZE}
        height={CANVAS_SIZE}
      ></canvas>
    </div>
  );
}

function ProblemChooser({
  problem,
  setProblem,
}: {
  problem: number;
  setProblem: (problem: number) => void;
}) {
  const arr = [];
  for (let i = 1; i <= 25; i++) {
    arr.push(
      <button
        key={i}
        onClick={() => setProblem(i)}
        className={`btn btn-sm ${problem === i ? "btn-active" : ""}`}
      >
        {i}
      </button>,
    );
  }
  return <div className="join">{arr}</div>;
}
