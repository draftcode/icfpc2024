import { CheckCircleIcon } from "@heroicons/react/24/solid";
import clsx from "clsx";
import Link from "next/link";

const EFFICIENCY_PROBLEMS = [
  { id: 1, solved: true },
  { id: 2, solved: true },
  { id: 3, solved: true },
  { id: 4, solved: true },
  { id: 5, solved: true },
  { id: 6, solved: true },
  { id: 7, solved: true },
  { id: 8, solved: true },
  { id: 9, solved: false },
  { id: 10, solved: false },
  { id: 11, solved: false },
  { id: 12, solved: false },
  { id: 13, solved: true },
];

export default function Sidebar({ current }: { current?: string }) {
  return (
    <ul className="menu menu-xs w-56 my-4 shrink-0">
      <li>
        <h2 className="menu-title">Efficiency</h2>
        <ul>
          {EFFICIENCY_PROBLEMS.map(({ id, solved }) => {
            const badge = solved ? (
              <CheckCircleIcon className="size-4 text-blue-500" />
            ) : null;
            return (
              <li>
                <Link
                  className={clsx(
                    `/efficiency/${id}` === current ? "active" : null,
                  )}
                  href={`/efficiency/${id}`}
                >
                  {badge}
                  Efficiency {id}
                </Link>
              </li>
            );
          })}
        </ul>
      </li>
    </ul>
  );
}
