import { useTeamRank } from "@/components/api";
import clsx from "clsx";
import Link from "next/link";
import { useState } from "react";
import { useCookies } from "react-cookie";

export default function Sidebar({ current }: { current?: string }) {
  const [cookies, setCookie] = useCookies([
    "hideTop",
    "badgeType",
    "hideLambdaman",
    "hideSpaceship",
    "hide3d",
    "hideEfficiency",
  ]);
  const [hideTop, setHideTop] = useState(cookies.hideTop);
  const [hideLambdaman, setHideLambdamanRaw] = useState(cookies.hideLambdaman);
  const [hideSpaceship, setHideSpaceshipRaw] = useState(cookies.hideSpaceship);
  const [hide3d, setHide3dRaw] = useState(cookies.hide3d);
  const [hideEfficiency, setHideEfficiencyRaw] = useState(
    cookies.hideEfficiency,
  );
  const [badgeType, setBadgeTypeRaw] = useState(cookies.badgeType ?? "rank");
  const { data, error } = useTeamRank();

  const setHideLambdaman = (b: boolean) => {
    setHideLambdamanRaw(b);
    setCookie("hideLambdaman", JSON.stringify(b), { path: "/" });
  };
  const setHideSpaceship = (b: boolean) => {
    setHideSpaceshipRaw(b);
    setCookie("hideSpaceship", JSON.stringify(b), { path: "/" });
  };
  const setHide3d = (b: boolean) => {
    setHide3dRaw(b);
    setCookie("hide3d", JSON.stringify(b), { path: "/" });
  };
  const setHideEfficiency = (b: boolean) => {
    setHideEfficiencyRaw(b);
    setCookie("hideEfficiency", JSON.stringify(b), { path: "/" });
  };
  const setBadgeType = (badgeType: string) => {
    setBadgeTypeRaw(badgeType);
    setCookie("badgeType", badgeType, { path: "/" });
  };
  if (error) {
    throw error;
  }
  if (!data) {
    return null;
  }
  return (
    <div>
      <label className="label cursor-pointer justify-normal gap-x-1">
        <input
          type="checkbox"
          checked={hideTop}
          onChange={(e) => {
            setHideTop(e.target.checked);
            setCookie("hideTop", JSON.stringify(e.target.checked), {
              path: "/",
            });
          }}
          className="checkbox checkbox-xs"
        />
        <span className="label-text">1位のやつは隠す</span>
      </label>
      <hr />
      <form>
        <div className="form-control">
          <label className="label cursor-pointer justify-normal gap-x-1">
            <input
              type="radio"
              name="radio-10"
              className="radio radio-xs"
              checked={badgeType === "rank"}
              onChange={() => setBadgeType("rank")}
            />
            <span className="label-text">順位</span>
          </label>
        </div>
        <div className="form-control">
          <label className="label cursor-pointer justify-normal gap-x-1">
            <input
              type="radio"
              name="radio-10"
              className="radio radio-xs"
              checked={badgeType === "score"}
              onChange={() => setBadgeType("score")}
            />
            <span className="label-text">スコア</span>
          </label>
        </div>
        <div className="form-control">
          <label className="label cursor-pointer justify-normal gap-x-1">
            <input
              type="radio"
              name="radio-10"
              className="radio radio-xs"
              checked={badgeType === "diff"}
              onChange={() => setBadgeType("diff")}
            />
            <span className="label-text">差分</span>
          </label>
        </div>
        <div className="form-control">
          <label className="label cursor-pointer justify-normal gap-x-1">
            <input
              type="radio"
              name="radio-10"
              className="radio radio-xs"
              checked={badgeType === "bestScore"}
              onChange={() => setBadgeType("bestScore")}
            />
            <span className="label-text">トップスコア</span>
          </label>
        </div>
      </form>
      <hr />
      <ul className="menu menu-xs w-56 shrink-0">
        <li>
          <h2
            onClick={() => setHideLambdaman(!hideLambdaman)}
            className={clsx(
              "menu-dropdown-toggle",
              !hideLambdaman && "menu-dropdown-show",
            )}
          >
            Lambdaman <CategoryBadge rank={data.lambdaman.rank} />
          </h2>
          <ul
            className={clsx(
              "menu-dropdown",
              !hideLambdaman && "menu-dropdown-show",
            )}
          >
            <li>
              <Link
                className={clsx("/lambdaman" === current ? "active" : null)}
                href="/lambdaman"
              >
                エディタ
              </Link>
            </li>
            {data.lambdaman.problems.map(
              ({ id, rank, our_score, best_score }) => {
                return (
                  <li className={clsx(hideTop && rank === 1 && "hidden")}>
                    <Link
                      className={clsx(
                        `/lambdaman/${id}` === current ? "active" : null,
                      )}
                      href={`/lambdaman/${id}`}
                    >
                      Lambdaman {id}
                      <Badge
                        badge_type={badgeType}
                        rank={rank}
                        best_score={best_score}
                        our_score={our_score}
                      />
                    </Link>
                  </li>
                );
              },
            )}
          </ul>
        </li>

        <li>
          <h2
            onClick={() => setHideSpaceship(!hideSpaceship)}
            className={clsx(
              "menu-dropdown-toggle",
              !hideSpaceship && "menu-dropdown-show",
            )}
          >
            Spaceship
            <CategoryBadge rank={data.spaceship.rank} />
          </h2>
          <ul
            className={clsx(
              "menu-dropdown",
              !hideSpaceship && "menu-dropdown-show",
            )}
          >
            <li>
              <Link
                className={clsx(
                  "/spaceship/editor" === current ? "active" : null,
                )}
                href="/spaceship/editor"
              >
                エディタ
              </Link>
            </li>
            {data.spaceship.problems.map(
              ({ id, rank, our_score, best_score }) => {
                return (
                  <li className={clsx(hideTop && rank === 1 && "hidden")}>
                    <Link
                      className={clsx(
                        `/spaceship/${id}` === current ? "active" : null,
                      )}
                      href={`/spaceship/${id}`}
                    >
                      Spaceship {id}
                      <Badge
                        badge_type={badgeType}
                        rank={rank}
                        best_score={best_score}
                        our_score={our_score}
                      />
                    </Link>
                  </li>
                );
              },
            )}
          </ul>
        </li>

        <li>
          <h2
            onClick={() => setHide3d(!hide3d)}
            className={clsx(
              "menu-dropdown-toggle",
              !hide3d && "menu-dropdown-show",
            )}
          >
            3D
            <CategoryBadge rank={data.threed.rank} />
          </h2>
          <ul
            className={clsx("menu-dropdown", !hide3d && "menu-dropdown-show")}
          >
            {data.threed.problems.map(({ id, rank, our_score, best_score }) => {
              return (
                <li className={clsx(hideTop && rank === 1 && "hidden")}>
                  <Link
                    className={clsx(`/3d/${id}` === current ? "active" : null)}
                    href={`/3d/${id}`}
                  >
                    3D {id}
                    <Badge
                      badge_type={badgeType}
                      rank={rank}
                      best_score={best_score}
                      our_score={our_score}
                    />
                  </Link>
                </li>
              );
            })}
          </ul>
        </li>

        <li>
          <h2
            onClick={() => setHideEfficiency(!hideEfficiency)}
            className={clsx(
              "menu-dropdown-toggle",
              !hideEfficiency && "menu-dropdown-show",
            )}
          >
            Efficiency
            <CategoryBadge rank={data.efficiency.rank} />
          </h2>
          <ul
            className={clsx(
              "menu-dropdown",
              !hideEfficiency && "menu-dropdown-show",
            )}
          >
            {data.efficiency.problems.map(
              ({ id, rank, our_score, best_score }) => {
                return (
                  <li className={clsx(hideTop && rank === 1 && "hidden")}>
                    <Link
                      className={clsx(
                        `/efficiency/${id}` === current ? "active" : null,
                      )}
                      href={`/efficiency/${id}`}
                    >
                      Efficiency {id}
                      <Badge
                        badge_type="rank"
                        rank={rank}
                        best_score={best_score}
                        our_score={our_score}
                      />
                    </Link>
                  </li>
                );
              },
            )}
          </ul>
        </li>
      </ul>
    </div>
  );
}

function Badge({
  badge_type,
  rank,
  best_score,
  our_score,
}: {
  badge_type: string;
  rank: number | null;
  best_score: number | null;
  our_score: number | null;
}) {
  if (badge_type === "rank") {
    if (rank === 1) {
      return <div className="badge badge-success">{rank} 位</div>;
    } else if (rank === null) {
      return <div className="badge badge-warning">No Rank</div>;
    }
    return <div className="badge badge-primary">{rank} 位</div>;
  }
  if (badge_type === "score") {
    if (our_score === best_score) {
      return <div className="badge badge-success">{our_score}</div>;
    } else if (our_score === null) {
      return <div className="badge badge-warning">No Score</div>;
    }
    return <div className="badge badge-primary">{our_score}</div>;
  }
  if (badge_type === "diff") {
    if (our_score === null || best_score === null) {
      return <div className="badge badge-warning">No Score</div>;
    }

    if (our_score === best_score) {
      return <div className="badge badge-success">0</div>;
    }
    return (
      <div className="badge badge-primary">
        {our_score - best_score > 0 ? "+" : ""}
        {our_score - best_score}
      </div>
    );
  }
  if (badge_type === "bestScore") {
    if (best_score === null) {
      return <div className="badge badge-warning">No Score</div>;
    }
    return <div className="badge badge-primary">{best_score}</div>;
  }
  return null;
}

function CategoryBadge({ rank }: { rank: number | null }) {
  if (rank === 1) {
    return <div className="badge badge-success ml-1">{rank} 位</div>;
  } else if (rank === null) {
    return <div className="badge badge-warning ml-1">No Rank</div>;
  }
  return <div className="badge badge-primary ml-1">{rank} 位</div>;
}
