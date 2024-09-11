import { open } from "@tauri-apps/api/shell";

export default function SidePanel() {
    return <div class="w-64px flex flex-col bg-gray-1">
        <div data-tauri-drag-region class="h-64px flex items-center justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={() => open("https://github.com/ZhangXiChang/node-network")}>
                <div class="i-line-md:github-loop h-48px w-48px" />
            </div>
        </div>
    </div>;
}
