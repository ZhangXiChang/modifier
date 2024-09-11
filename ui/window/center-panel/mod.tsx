import { createSignal, getOwner, runWithOwner } from "solid-js";
import ProcessSelectPanel from "./process-select-panel";
import TitlePanel from "./title-panel";

export const [centerControlPanel, setCenterControlPanel] = createSignal(<></>);

export default function CenterPanel() {
    const owner = getOwner();
    return <div class="flex flex-1 flex-col">
        <TitlePanel />
        <div data-tauri-drag-region class="h-32px flex items-center">
            <div data-tauri-drag-region class="w-32px flex justify-center">
                <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={() => runWithOwner(owner, () => setCenterControlPanel(<ProcessSelectPanel />))}>
                    <div class="i-line-md:document-add h-24px w-24px"></div>
                </div>
            </div>
        </div>
        {centerControlPanel()}
    </div>;
}
