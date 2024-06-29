import { CommunicationLog } from "@/components/api";
import { DateTime } from "luxon";
import { useEffect, useState } from "react";

export default function CommunicationContainer({
  log,
  children,
}: {
  log: CommunicationLog;
  children: React.ReactNode;
}) {
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
      <div className="pl-4 space-y-2">{children}</div>
    </div>
  );
}
