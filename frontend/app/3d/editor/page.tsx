"use client";

import {
  CellValue,
  parseStateString,
  serializeState,
  serializeToTSV,
} from "@/components/threededit/state";
import { useState } from "react";

export default function Page() {
  const [input, setInput] = useState("");
  const [state, setState] = useState(parseStateString(input));
  const size = getSize(state);
  const [minX, setMinX] = useState(size.minX);
  const [minY, setMinY] = useState(size.minY);
  const [maxX, setMaxX] = useState(size.maxX);
  const [maxY, setMaxY] = useState(size.maxY);
  const [copied, setCopied] = useState(false);

  const setCell = (cell: CellValue) => {
    const newState = new Map(state);
    if (cell.value === "") {
      newState.delete(`${cell.coord[0]},${cell.coord[1]}`);
    } else {
      newState.set(`${cell.coord[0]},${cell.coord[1]}`, cell);
    }
    setState(newState);
    setInput(serializeState(newState));
  };

  return (
    <div className="grid grid-cols-2 gap-x-2">
      <textarea
        className="textarea textarea-bordered font-mono"
        placeholder="ここに盤面だけはります"
        value={input}
        onChange={(e) => {
          const state = parseStateString(e.target.value);
          const size = getSize(state);
          setMinX(Math.min(minX, size.minX));
          setMinY(Math.min(minY, size.minY));
          setMaxX(Math.max(maxX, size.maxX));
          setMaxY(Math.max(maxY, size.maxY));
          setState(state);
          setInput(serializeState(state));
        }}
      ></textarea>
      <div className="space-y-2">
        <div className="space-x-2">
          <button
            className="btn btn-sm"
            onClick={() => {
              setMinX(minX - 1);
            }}
          >
            {"<"}
          </button>
          <button
            className="btn btn-sm"
            onClick={() => {
              setMinY(minY - 1);
            }}
          >
            {"^"}
          </button>
          <button
            className="btn btn-sm"
            onClick={() => {
              setMaxX(maxX + 1);
            }}
          >
            {">"}
          </button>
          <button
            className="btn btn-sm"
            onClick={() => {
              setMaxY(maxY + 1);
            }}
          >
            {"v"}
          </button>
          <button
            className="btn btn-sm"
            onClick={() => {
              navigator.clipboard.writeText(serializeToTSV(state));
              setCopied(true);
              setTimeout(() => {
                setCopied(false);
              }, 2000);
            }}
          >
            {copied ? "コピーしました" : "エクセル用にコピー"}
          </button>
        </div>
        <EditableState
          state={state}
          minX={minX}
          minY={minY}
          maxX={maxX}
          maxY={maxY}
          setCell={setCell}
        />
      </div>
    </div>
  );
}

function getSize(state: Map<string, CellValue>) {
  let minX = -1;
  let minY = -1;
  let maxX = 1;
  let maxY = 1;
  for (const cell of Array.from(state.values())) {
    minX = Math.min(minX, cell.coord[0]);
    minY = Math.min(minY, cell.coord[1]);
    maxX = Math.max(maxX, cell.coord[0]);
    maxY = Math.max(maxY, cell.coord[1]);
  }
  return { minX, minY, maxX, maxY };
}

function PlainState({ state }: { state: Map<string, CellValue> }) {
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (const cell of Array.from(state.values())) {
    minX = Math.min(minX, cell.coord[0]);
    minY = Math.min(minY, cell.coord[1]);
    maxX = Math.max(maxX, cell.coord[0]);
    maxY = Math.max(maxY, cell.coord[1]);
  }
  const rows = [];
  for (let y = minY; y <= maxY; y++) {
    const row = [];
    for (let x = minX; x <= maxX; x++) {
      row.push(<PlainCell key={`${x},${y}`} cell={state.get(`${x},${y}`)} />);
    }
    rows.push(
      <div key={`row-${y}`} className="flex">
        {row}
      </div>,
    );
  }
  return <div className="font-mono">{rows}</div>;
}

function PlainCell({ cell }: { cell: CellValue | undefined }) {
  if (cell) {
    return <div className="size-8 border text-center">{cell.value}</div>;
  } else {
    return <div className="size-8 border"></div>;
  }
}

function EditableState({
  state,
  minX,
  minY,
  maxX,
  maxY,
  setCell,
}: {
  state: Map<string, CellValue>;
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
  setCell: (cell: CellValue) => void;
}) {
  for (const cell of Array.from(state.values())) {
    minX = Math.min(minX, cell.coord[0]);
    minY = Math.min(minY, cell.coord[1]);
    maxX = Math.max(maxX, cell.coord[0]);
    maxY = Math.max(maxY, cell.coord[1]);
  }
  const rows = [];
  for (let y = minY; y <= maxY; y++) {
    const row = [];
    for (let x = minX; x <= maxX; x++) {
      row.push(
        <EditableCell
          key={`${x},${y}`}
          cell={state.get(`${x},${y}`)}
          x={x}
          y={y}
          setCell={setCell}
        />,
      );
    }
    rows.push(
      <div key={`row-${y}`} className="flex">
        {row}
      </div>,
    );
  }
  return <div className="font-mono">{rows}</div>;
}

function EditableCell({
  cell,
  x,
  y,
  setCell,
}: {
  cell: CellValue | undefined;
  x: number;
  y: number;
  setCell: (cell: CellValue) => void;
}) {
  return (
    <input
      className="size-8 border text-center"
      value={cell?.value ?? ""}
      onChange={(e) => {
        setCell({ coord: [x, y], value: e.target.value });
      }}
    />
  );
}
