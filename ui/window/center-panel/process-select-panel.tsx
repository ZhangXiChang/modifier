import { invoke } from "@tauri-apps/api/tauri";
import { createSignal, getOwner, runWithOwner } from "solid-js";
import { setCenterControlPanel } from "./mod";

export default function ProcessSelectPanel() {
    const owner = getOwner();
    const [processNames, setProcessNames] = createSignal(<></>);
    (async () => {
        try {
            const all_process_info = await invoke("get_all_process_info") as {
                process_id: number,
                exe_file_name: string,
            }[];
            runWithOwner(owner, () => setProcessNames(all_process_info.map((info) =>
                <div
                    onclick={(e) => {
                        for (const child of e.currentTarget.parentElement?.children!) {
                            child.className = "";
                        }
                        e.currentTarget.className = "bg-blue-3";
                    }}
                    ondblclick={async () => {
                        try {
                            await invoke("test", { processId: info.process_id });
                            runWithOwner(owner, () => setCenterControlPanel(<div>{info.process_id}</div>));
                        } catch (err) {
                            console.error(err);
                        }
                    }}
                >{info.exe_file_name}</div>,
            )));
        } catch (err) {
            console.error(err);
        }
    })();
    return <div class="flex flex-1 flex-col overflow-x-hidden overflow-y-auto">
        {processNames()}
    </div>;
}
