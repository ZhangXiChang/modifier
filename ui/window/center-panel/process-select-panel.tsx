import { invoke } from "@tauri-apps/api/tauri";
import { createSignal, getOwner, runWithOwner } from "solid-js";
import { setCenterControlPanel } from "./mod";

export default function ProcessSelectPanel() {
    const owner = getOwner();
    const [processNames, setProcessNames] = createSignal(<></>);
    (async () => {
        try {
            const system_process_info_list = await invoke("get_system_process_info_list") as {
                pid: number,
                exe_file_name: string,
            }[];
            runWithOwner(owner, () => setProcessNames(system_process_info_list.map((process_info) =>
                <div
                    onclick={(e) => {
                        for (const child of e.currentTarget.parentElement?.children!) {
                            child.className = "";
                        }
                        e.currentTarget.className = "bg-blue-3";
                    }}
                    ondblclick={async () => {
                        try {
                            await invoke("open_process", { pid: process_info.pid });
                            runWithOwner(owner, () => setCenterControlPanel(<></>));
                        } catch (err) {
                            console.error(err);
                        }
                    }}
                >{process_info.exe_file_name}</div>,
            )));
        } catch (err) {
            console.error(err);
        }
    })();
    return <div class="flex flex-1 flex-col overflow-x-hidden overflow-y-auto">
        {processNames()}
    </div>;
}
