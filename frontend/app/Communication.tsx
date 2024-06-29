import { CommunicationLog } from "@/components/api";
import Markdown from "react-markdown";
import CommunicationContainer from "./CommunicationContainer";

export default function Communication({ log }: { log: CommunicationLog }) {
  return (
    <CommunicationContainer log={log}>
      <div className="font-mono bg-base-200 border p-2">
        <div className="font-mono">
          <textarea className="w-full" rows={1} disabled>
            {log.decoded_request}
          </textarea>
        </div>
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
    </CommunicationContainer>
  );
}
