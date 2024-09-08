import { invoke } from "@tauri-apps/api/tauri";
import { createSignal, getOwner, runWithOwner } from "solid-js";

export default function ProcessSelectPanel() {
    const owner = getOwner();
    const [processNames, setProcessNames] = createSignal(<></>);
    (async () => {
        try {
            const all_process_info = await invoke("get_all_process_info") as {
                process_id: number,
                exe_file_name: string,
            }[];
            runWithOwner(owner, () => setProcessNames(all_process_info.map((info) => <tr><td>{info.exe_file_name}</td></tr>)));
        } catch (err) {
            console.error(err);
        }
    })();
    return <div class="h-400px overflow-x-hidden overflow-y-auto">
        <table>
            <thead>
                <tr><th scope="col">进程名称</th></tr>
            </thead>
            <tbody>
                {processNames()}
            </tbody>
        </table>
    </div>;
}
