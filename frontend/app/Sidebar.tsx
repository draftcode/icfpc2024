import { CheckCircleIcon } from "@heroicons/react/24/solid";
import clsx from "clsx";
import Link from "next/link";

const LAMBDAMAN_PROBLEMS = [
  { id: 1 },
  { id: 2 },
  { id: 3 },
  { id: 4 },
  { id: 5 },
  { id: 6 },
  { id: 7 },
  { id: 8 },
  { id: 9 },
  { id: 10 },
  { id: 11 },
  { id: 12 },
  { id: 13 },
  { id: 14 },
  { id: 15 },
  { id: 16 },
  { id: 17 },
  { id: 18 },
  { id: 19 },
  { id: 20 },
  { id: 21 },
];

const SPACESHIP_PROBLEMS = [
  { id: 1 },
  { id: 2 },
  { id: 3 },
  { id: 4 },
  { id: 5 },
  { id: 6 },
  { id: 7 },
  { id: 8 },
  { id: 9 },
  { id: 10 },
  { id: 11 },
  { id: 12 },
  { id: 13 },
  { id: 14 },
  { id: 15 },
  { id: 16 },
  { id: 17 },
  { id: 18 },
  { id: 19 },
  { id: 20 },
  { id: 21 },
  { id: 22 },
  { id: 23 },
  { id: 24 },
  { id: 25 },
];

const THREED_PROBLEMS = [
  { id: 1 },
  { id: 2 },
  { id: 3 },
  { id: 4 },
  { id: 5 },
  { id: 6 },
  { id: 7 },
  { id: 8 },
  { id: 9 },
  { id: 10 },
  { id: 11 },
  { id: 12 },
];

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
        <h2 className="menu-title">Lambdaman</h2>
        <ul>
          {LAMBDAMAN_PROBLEMS.map(({ id }) => {
            return (
              <li>
                <Link
                  className={clsx(
                    `/lambdaman/${id}` === current ? "active" : null,
                  )}
                  href={`/lambdaman/${id}`}
                >
                  Lambdaman {id}
                </Link>
              </li>
            );
          })}
        </ul>
      </li>

      <li>
        <h2 className="menu-title">Spaceship</h2>
        <ul>
          {SPACESHIP_PROBLEMS.map(({ id }) => {
            return (
              <li>
                <Link
                  className={clsx(
                    `/spaceship/${id}` === current ? "active" : null,
                  )}
                  href={`/spaceship/${id}`}
                >
                  Spaceship {id}
                </Link>
              </li>
            );
          })}
        </ul>
      </li>

      <li>
        <h2 className="menu-title">3D</h2>
        <ul>
          {THREED_PROBLEMS.map(({ id }) => {
            return (
              <li>
                <Link
                  className={clsx(
                    `/3d/${id}` === current ? "active" : null,
                  )}
                  href={`/3d/${id}`}
                >
                  3D {id}
                </Link>
              </li>
            );
          })}
        </ul>
      </li>

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
