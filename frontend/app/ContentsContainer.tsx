"use client";

import { DateTime } from "luxon";
import Link from "next/link";
import { useEffect, useState } from "react";

const DEADLINE = DateTime.fromISO("2024-07-01T12:00:00Z");

export default function ContentsContainer({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div>
      <TopBar />
      {children}
    </div>
  );
}

function TopBar() {
  return (
    <div className="navbar justify-between pb-4">
      <Link
        className="btn btn-ghost normal-case text-xl hover:text-black hover:underline focus:text-black"
        href="/"
      >
        Spica [kari]
      </Link>
      <div className="items-center">
        <TimeLeft />
        <OfficialLink />
      </div>
    </div>
  );
}

function TimeLeft() {
  const [hour, setHour] = useState(0);
  const [min, setMin] = useState(0);

  useEffect(() => {
    const refreshClock = () => {
      const diff = DEADLINE.diffNow(["minute", "hours"]);
      setHour(diff.hours);
      setMin(Math.floor(diff.minutes));
    };
    refreshClock();
    const timerId = setInterval(refreshClock, 1000);
    return () => clearInterval(timerId);
  }, [hour, min]);

  return (
    <div>
      {hour > 0 || min > 0 ? (
        <div className="font-mono">
          <p>〆切 {DEADLINE.toFormat("ccc HH:mm")}</p>
          <div className="text-2xl">
            あと {hour}時間 {min}分
          </div>
        </div>
      ) : null}
    </div>
  );
}

function OfficialLink() {
  return (
    <ul className="menu menu-horizontal px-1">
      <li>
        <Link
          className="hover:text-white hover:underline focus:text-white"
          href="https://icfpcontest2024.github.io/scoreboard.html"
          target="_blank"
        >
          公式
        </Link>
      </li>
    </ul>
  );
}
