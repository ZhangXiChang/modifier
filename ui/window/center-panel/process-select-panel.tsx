import { invoke } from "@tauri-apps/api/tauri";
import { createSignal, For, getOwner, JSX, runWithOwner, Setter } from "solid-js";

export default function ProcessSelectPanel(props: {
    set_center_control_panel: Setter<JSX.Element>
}) {
    const owner = getOwner();
    const [processNames, setProcessNames] = createSignal<{
        pid: number,
        exe_file_name: string,
    }[]>([]);
    (async () => {
        try {
            const system_process_info_list = await invoke("get_system_process_info_list") as {
                pid: number,
                exe_file_name: string,
            }[];
            runWithOwner(owner, () => setProcessNames(system_process_info_list));
        } catch (err) {
            console.error(err);
        }
    })();
    return <div class="flex flex-1 flex-col overflow-x-hidden overflow-y-auto">
        <For each={processNames()}>
            {(process_info) => <div
                onclick={(e) => {
                    for (const child of e.currentTarget.parentElement?.children!) {
                        child.className = "";
                    }
                    e.currentTarget.className = "bg-blue-3";
                }}
                ondblclick={async () => {
                    try {
                        await invoke("open_process", { pid: process_info.pid });
                        //await invoke("test");
                        runWithOwner(owner, () => props.set_center_control_panel(<></>));
                    } catch (err) {
                        console.error(err);
                    }
                }}>
                {process_info.exe_file_name}
            </div>}
        </For>
    </div>;
}
