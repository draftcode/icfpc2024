"use client";

import { CellState, LambdamanMap } from "@/components/lambdaman_map";
import { useState } from "react";
import useSWR from "swr";

// 256 "R"s
const P = `((lambda (v0) (v0 (v0 (v0 (v0 (v0 (v0 (v0 (v0 (v0 (v0 "R"))))))))))) (lambda (v0) (string-append v0 v0)))`;

const fetcher = (url: string): Promise<any> =>
  fetch(url).then((res) => res.json());

export default function Page() {
  const initialId = window.location.search.match(/id=(\d+)/)?.[1];
  const [id, setId] = useState(initialId ? Number(initialId) : 1);
  const [prog, setProg] = useState(P);

  const { data: data1 } = useSWR(`/api/lambdaman?id=${id}`, fetcher);
  const lambdamanMapData: string = data1?.data ?? "";

  const { data: data2 } = useSWR(`/api/eval_scheme?program=${prog}`, fetcher);

  const ok = data2?.result && typeof data2.result === "string";
  const output: string = ok ? data2?.result : data2?.error || data2?.result;
  const outType = ok ? typeof output : "error";

  return (
    <div className="container mx-auto">
      <div>
        <label>Problem ID</label>
        <input
          className="input input-primary"
          type="number"
          min={1}
          value={id}
          onChange={(e) => setId(Number(e.target.value))}
        ></input>
      </div>
      <div>
        <label>
          Program (evaluated by{" "}
          <a className="link" href="https://www.biwascheme.org/">
            BiwaScheme
          </a>
          )
        </label>
        <textarea
          className="textarea textarea-primary w-full"
          value={prog}
          onChange={(e) => setProg(e.target.value)}
        ></textarea>
        Output ({outType}):
        <textarea
          className="textarea textarea-bordered w-full"
          value={output}
        ></textarea>
      </div>

      <Board data={lambdamanMapData} dirs={ok ? output : ""} />
    </div>
  );
}

function Board({ data, dirs }: { data: string; dirs: string }) {
  const orig = new LambdamanMap(data);
  const lm = orig.clone();
  lm.walk(dirs);

  return (
    <div>
      <div className="m-4">
        <label className="label-text">Map after move:</label>
        <Map lm={lm} />
      </div>
      <div className="m-4">
        <label className="label-text">Original map:</label>
        <Map lm={orig} />
      </div>
    </div>
  );
}

function Map({ lm }: { lm: LambdamanMap }) {
  const pills = lm.remaining;
  return (
    <div>
      <text className="text-sm">{pills} pills</text>
      <div>
        {lm.chars.map((row) => (
          <div className="flex">
            {row.map((c) => (
              <Cell c={c} />
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}

function Cell({ c }: { c: CellState }) {
  const className =
    "text-center h-1 w-1 " +
    {
      [CellState.Wall]: "bg-amber-800",
      [CellState.Pill]: "bg-orange-200",
      [CellState.Lambda]: "bg-blue-400",
      [CellState.Done]: "bg-slate-100",
    }[c];

  return <span className={className}></span>;
}
