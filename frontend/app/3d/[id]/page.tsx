"use client";

import Communication from "@/app/Communication";
import Sidebar from "@/app/Sidebar";
import { useCommunicationsWithRequestPrefix } from "@/components/api";
import Link from "next/link";

export default function Home({
  params: { id: idStr },
  searchParams,
}: {
  params: { id: string };
  searchParams: { page: string };
}) {
  const page = parseInt(searchParams.page ?? "1") - 1;
  const { data, error } = useCommunicationsWithRequestPrefix(
    `solve 3d${idStr}\n`,
    page * 10,
    10,
  );
  if (!data) {
    return null;
  }
  if (error) {
    throw error;
  }
  return (
    <div className="flex gap-x-4">
      <Sidebar current={`/3d/${idStr}`} />
      <div className="grow">
        <div className="space-y-4">
          {data.map((log) => {
            return <Communication key={log.id} log={log} />;
          })}
        </div>
        <div className="flex items-center justify-center p-4 gap-x-4">
          <Link
            className="btn btn-sm"
            href={`/3d/${idStr}?page=${Math.max(0, page - 1) + 1}`}
          >
            Prev Page
          </Link>
          <div>
            {page * 10 + 1} から {page * 10 + Math.max(10, data.length)}
          </div>
          <Link
            className="btn btn-sm"
            href={`/3d/${idStr}?page=${page + 2}`}
          >
            Next Page
          </Link>
        </div>
      </div>
    </div>
  );
}
