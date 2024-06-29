import {
  WaypointVizState,
  calculateWaypoints,
} from "@/components/spaceviz/state";
import { useEffect, useMemo, useRef } from "react";

const CANVAS_SIZE = 4000;

export default function Visualizer({
  path,
  reqPoints,
}: {
  path: string;
  reqPoints: [number, number][];
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const waypoints = useMemo(() => calculateWaypoints(path), [path]);

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
