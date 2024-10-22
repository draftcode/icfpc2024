"use client";

import { useCommunications } from "@/components/api";
import Link from "next/link";
import Sidebar from "./Sidebar";
import Communication from "./Communication";

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
    <div className="flex gap-x-4">
      <Sidebar current="/" />
      <div className="grow">
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
          <Link className="btn btn-sm" href={`/?page=${page + 2}`}>
            Next Page
          </Link>
        </div>
      </div>
    </div>
  );
}
