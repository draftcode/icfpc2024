import {
  WaypointVizState,
} from "@/components/spaceviz/state";
import { useCallback, useRef } from "react";

const CANVAS_SIZE = 4000;

export default function Visualizer({ state }: { state: WaypointVizState }) {
  const canvasCancelRef = useRef<(() => void) | null>(null);

  const initCanvas = useCallback(
    (canvas: HTMLCanvasElement | null) => {
      if (canvas) {
        const remove = state.addEventListeners(canvas);
        let animationFrameId: number = 0;
        const render = () => {
          const ctx = canvas.getContext("2d");
          if (ctx) {
            ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
            state.plotWaypoints(ctx);
            state.plotBasis(ctx);
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
    },
    [state],
  );

  return (
    <canvas
      className="w-full h-full border"
      ref={initCanvas}
      width={CANVAS_SIZE}
      height={CANVAS_SIZE}
    ></canvas>
  );
}
