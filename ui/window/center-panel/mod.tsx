import TitlePanel from "./title-panel";

export default function CenterPanel() {
    const selectProcess = () => {
    };
    return <div class="flex flex-1 flex-col">
        <TitlePanel />
        <div data-tauri-drag-region class="h-32px flex items-center">
            <div data-tauri-drag-region class="w-32px flex justify-center">
                <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={selectProcess}>
                    <div class="i-line-md:document-add h-24px w-24px"></div>
                </div>
            </div>
        </div>
        <div class="flex-1"></div>
    </div>;
}
