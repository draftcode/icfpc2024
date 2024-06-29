"use client";

import CommunicationContainer from "@/app/CommunicationContainer";
import Sidebar from "@/app/Sidebar";
import {
  CommunicationLog,
  useCommunicationsWithRequestPrefix,
  useProblem,
} from "@/components/api";
import {
  WaypointVizState,
  calculateWaypoints,
  parseReqPoints,
} from "@/components/spaceviz/state";
import Link from "next/link";
import { useEffect, useMemo, useRef } from "react";
import Markdown from "react-markdown";

export default function Home({
  params: { id: idStr },
  searchParams,
}: {
  params: { id: string };
  searchParams: { page: string };
}) {
  const page = parseInt(searchParams.page ?? "1") - 1;
  const { data: problemData, error: problemError } = useProblem(
    "spaceship",
    parseInt(idStr),
  );
  const { data, error } = useCommunicationsWithRequestPrefix(
    `solve spaceship${idStr} `,
    page * 10,
    10,
  );
  const reqPoints = useMemo(
    () => parseReqPoints(problemData?.content ?? ""),
    [problemData?.content],
  );
  if (!data || !problemData) {
    return null;
  }
  if (error) {
    throw error;
  }
  if (problemError) {
    throw problemError;
  }
  return (
    <div className="flex gap-x-4">
      <Sidebar current={`/spaceship/${idStr}`} />
      <div className="grow">
        <div className="space-y-4">
          {data.map((log) => {
            return (
              <SubmittedSpaceshipProblem
                key={log.id}
                log={log}
                reqPoints={reqPoints}
              />
            );
          })}
        </div>

        <div className="flex items-center justify-center p-4 gap-x-4">
          <Link
            className="btn btn-sm"
            href={`/spaceship/${idStr}?page=${Math.max(0, page - 1) + 1}`}
          >
            Prev Page
          </Link>
          <div>
            {page * 10 + 1} から {page * 10 + Math.max(10, data.length)}
          </div>
          <Link
            className="btn btn-sm"
            href={`/spaceship/${idStr}?page=${page + 2}`}
          >
            Next Page
          </Link>
        </div>
      </div>
    </div>
  );
}

function SubmittedSpaceshipProblem({
  log,
  reqPoints,
}: {
  log: CommunicationLog;
  reqPoints: [number, number][];
}) {
  const solution = (log.decoded_request || "").split(" ").pop();
  if (!solution?.match(/^\d+$/)) {
    return (
      <CommunicationContainer log={log}>
        <div className="font-mono bg-base-200 border p-2">
          <div className="font-mono">
            <textarea className="w-full" rows={1} disabled>
              {log.decoded_request}
            </textarea>
          </div>
        </div>
        <div className="bg-base-200 border-base-300 p-2">
          <div className="prose font-mono">
            <Markdown>{log.decoded_response}</Markdown>
          </div>
        </div>
      </CommunicationContainer>
    );
  }

  return (
    <CommunicationContainer log={log}>
      <div className="font-mono bg-base-200 border p-2">
        <div className="font-mono">
          <textarea className="w-full" rows={1} disabled>
            {log.decoded_request}
          </textarea>
        </div>
      </div>
      <Visualizer path={solution} reqPoints={reqPoints} />
      <div className="bg-base-200 border-base-300 p-2">
        <div className="prose font-mono">
          <Markdown>{log.decoded_response}</Markdown>
        </div>
      </div>
    </CommunicationContainer>
  );
}

const CANVAS_SIZE = 4000;

function Visualizer({
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
