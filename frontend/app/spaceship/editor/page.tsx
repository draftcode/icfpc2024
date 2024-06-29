"use client";

import { parseReqPoints } from "@/components/spaceviz/state";
import { useMemo, useState } from "react";
import Visualizer from "../Visualizer";

export default function Page() {
  const [path, setPath] = useState("236659");
  const [reqPointsStr, setReqPointStr] = useState("");
  const [vizPath, setVizPath] = useState("236659");

  const reqPoints = useMemo(() => parseReqPoints(reqPointsStr), [reqPointsStr]);

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
        reqPoints={reqPoints}
      />
    </div>
  );
}
