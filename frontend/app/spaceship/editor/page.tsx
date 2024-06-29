"use client";

import {
  useCommunicationLog,
  useCommunicationSubmit,
  useProblem,
} from "@/components/api";
import {
  WaypointVizState,
  calculateSplitWaypoints,
} from "@/components/spaceviz/state";
import { parseReqPoints } from "@/components/spaceviz/state";
import Link from "next/link";
import { useCallback, useEffect, useRef, useState } from "react";

const CANVAS_SIZE = 4000;

export default function Page({
  searchParams: { base },
}: {
  searchParams: { base: string };
}) {
  if (!base || !/^\d+$/.test(base)) {
    return <Editor />;
  }
  const { data: baseData, error: baseError } = useCommunicationLog(
    parseInt(base),
  );
  if (baseError) {
    throw baseError;
  }
  if (!baseData) {
    return null;
  }
  const ss = baseData.decoded_request.split(" ");
  if (
    ss.length !== 3 ||
    ss[0] !== "solve" ||
    !/^spaceship\d+$/.test(ss[1]) ||
    !/^\d+$/.test(ss[2])
  ) {
    return <Editor />;
  }
  const initProblemID = parseInt(ss[1].replace("spaceship", ""));
  const initPath = ss[2];
  return <Editor initProblemID={initProblemID} initPath={initPath} />;
}

function Editor({
  initProblemID,
  initPath,
}: {
  initProblemID?: number;
  initPath?: string;
}) {
  const canvasCancelRef = useRef<(() => void) | null>(null);
  const vizStateRef = useRef<WaypointVizState>(new WaypointVizState());
  const [problem, setProblem] = useState(initProblemID ?? 1);
  const { data: problemData, error: problemError } = useProblem(
    "spaceship",
    problem,
  );
  const [path, setPath] = useState(initPath ?? "");
  const [debugStep, setDebugStep] = useState(0);

  useEffect(() => {
    const reqPoints = parseReqPoints(problemData?.content ?? "");
    vizStateRef.current.setCheckPointsAndInitViewport(reqPoints);
  }, [problemData?.content]);

  useEffect(() => {
    const [w1, w2] = calculateSplitWaypoints(
      path.slice(0, debugStep),
      path.slice(debugStep),
    );
    vizStateRef.current.setWaypoints(w1, w2);
  }, [path, debugStep]);
  const {
    trigger: triggerSubmit,
    isMutating,
    data: submitData,
    error: submitError,
  } = useCommunicationSubmit(`solve spaceship${problem} ${path}`);

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
  if (submitError) {
    throw submitError;
  }
  if (!problemData) {
    return null;
  }

  const insertOne = (s: string) => {
    setPath(path.slice(0, debugStep) + s + path.slice(debugStep));
    setDebugStep(debugStep + 1);
  };
  const deleteOne = () => {
    setPath(path.slice(0, Math.max(0, debugStep - 1)) + path.slice(debugStep));
    setDebugStep(Math.max(0, debugStep - 1));
  };

  return (
    <div className="space-y-2 p-4">
      <ProblemChooser problem={problem} setProblem={setProblem} />
      <input
        type="text"
        value={path}
        onChange={(e) => {
          setPath(e.target.value);
          setDebugStep(0);
        }}
        className="input input-bordered w-full"
      />
      <div className="flex gap-x-4">
        <div>
          <canvas
            className="w-full h-full border"
            ref={initCanvas}
            width={CANVAS_SIZE}
            height={CANVAS_SIZE}
          ></canvas>
        </div>

        <div className="w-[70em] space-y-4">
          <div>
            ステップ: {debugStep}/{path.length}
          </div>
          <div className="grid grid-cols-4">
            <button
              className="btn btn-xs"
              onClick={() => setDebugStep((s) => Math.max(0, s - 10))}
            >
              10戻る
            </button>
            <button
              className="btn btn-xs"
              onClick={() => setDebugStep((s) => Math.max(0, s - 1))}
            >
              1戻る
            </button>
            <button
              className="btn btn-xs"
              onClick={() => setDebugStep((s) => Math.min(path.length, s + 1))}
            >
              1進む
            </button>
            <button
              className="btn btn-xs"
              onClick={() => setDebugStep((s) => Math.min(path.length, s + 10))}
            >
              10進む
            </button>
          </div>
          <div>
            <div className="flex">
              <button
                className="btn btn-xs size-10 bg-gray-300"
                onClick={() => insertOne("7")}
              >
                7
              </button>
              <button
                className="btn btn-xs size-10 bg-gray-300"
                onClick={() => insertOne("8")}
              >
                8
              </button>
              <button
                className="btn btn-xs size-10 bg-gray-300"
                onClick={() => insertOne("9")}
              >
                9
              </button>
            </div>
            <div className="flex">
              <button
                className="btn btn-xs size-10 bg-gray-300"
                onClick={() => insertOne("4")}
              >
                4
              </button>
              <button
                className="btn btn-xs size-10 bg-gray-300"
                onClick={() => insertOne("5")}
              >
                5
              </button>
              <button
                className="btn btn-xs size-10 bg-gray-300"
                onClick={() => insertOne("6")}
              >
                6
              </button>
            </div>
            <div className="flex">
              <button
                className="btn btn-xs size-10 bg-gray-300"
                onClick={() => insertOne("1")}
              >
                1
              </button>
              <button
                className="btn btn-xs size-10 bg-gray-300"
                onClick={() => insertOne("2")}
              >
                2
              </button>
              <button
                className="btn btn-xs size-10 bg-gray-300"
                onClick={() => insertOne("3")}
              >
                3
              </button>
            </div>
          </div>

          <button
            className="btn btn-sm bg-gray-300"
            onClick={() => deleteOne()}
          >
            現在の一文字削除
          </button>

          <button
            className="btn btn-sm bg-primary text-white"
            disabled={isMutating}
            onClick={() => triggerSubmit()}
          >
            提出
          </button>

          {submitData && (
            <div>
              <Link
                href={`/spaceship/editor?base=${submitData.id}`}
                className="underline text-blue-500"
                target="_blank"
              >
                保存されたリクエストへのパーマリンク
              </Link>
              <div>{submitData.decoded_response}</div>
            </div>
          )}
        </div>
      </div>
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
