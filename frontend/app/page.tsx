"use client";

import { CommunicationLog, useCommunications } from "@/components/api";
import { DateTime } from "luxon";
import Link from "next/link";
import { useEffect, useState } from "react";
import Markdown from "react-markdown";

export default function Home({
  searchParams,
}: {
  searchParams: { page: string };
}) {
  const page = parseInt(searchParams.page ?? "1") - 1;
  const { data, error } = useCommunications(page * 10, 10);
  if (!data) {
    return null;
  }
  if (error) {
    throw error;
  }
  return (
    <main>
      <div className="space-y-4">
        {data.map((log) => {
          return <Communication key={log.id} log={log} />;
        })}
      </div>
      <div className="flex items-center justify-center p-4 gap-x-4">
        <Link
          className="btn btn-sm"
          href={`/?page=${Math.max(0, page - 1) + 1}`}
        >
          Prev Page
        </Link>
        <div>
          {page * 10 + 1} から {page * 10 + Math.max(10, data.length)}
        </div>
        <Link
          className="btn btn-sm"
          href={`/?page=${page + 2}`}
        >
          Next Page
        </Link>
      </div>
    </main>
  );
}

function Communication({ log }: { log: CommunicationLog }) {
  const created = DateTime.fromISO(log.created, { zone: "UTC" });
  const [relative, setRelative] = useState<string | null>(null);
  useEffect(() => {
    const refreshClock = () => {
      setRelative(created.toRelative({ locale: "ja" }));
    };
    refreshClock();
    const timerId = setInterval(refreshClock, 1000);
    return () => clearInterval(timerId);
  }, [setRelative]);
  return (
    <div className="p-4 border-2 border-dotted">
      <div className="flex justify-between">
        <h2 className="font-bold pb-2">Communication {log.id}</h2>
        <div>
          {relative} ({created.setZone("JST").toFormat("L/dd HH:mm:ss")})
        </div>
      </div>
      <div className="pl-4 space-y-2">
        <div className="font-mono bg-base-200 border p-2">
          <pre>
            <code>{log.decoded_request}</code>
          </pre>
        </div>
        <div>
          <form>
            <div role="tablist" className="tabs tabs-lifted tabs-xs">
              <input
                type="radio"
                name="response"
                role="tab"
                className="tab"
                aria-label="Markdown"
                defaultChecked
              />
              <div
                role="tabpanel"
                className="tab-content bg-base-200 border-base-300 p-2"
              >
                <div className="prose font-mono">
                  <Markdown>{log.decoded_response}</Markdown>
                </div>
              </div>

              <input
                type="radio"
                name="response"
                role="tab"
                className="tab"
                aria-label="Decoded"
              />
              <div
                role="tabpanel"
                className="tab-content bg-base-200 border-base-300 p-2"
              >
                <div className="font-mono">
                  <pre>
                    <code>{log.decoded_response}</code>
                  </pre>
                </div>
              </div>

              <input
                type="radio"
                name="response"
                role="tab"
                className="tab"
                aria-label="Raw"
              />
              <div
                role="tabpanel"
                className="tab-content bg-base-200 border-base-300 p-2"
              >
                <div className="font-mono">
                  <textarea className="w-full" rows={5} disabled>
                    {log.response}
                  </textarea>
                </div>
              </div>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
