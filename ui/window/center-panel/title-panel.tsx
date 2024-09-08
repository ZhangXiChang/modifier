import { TauriEvent } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { createSignal } from "solid-js";

export default function TitlePanel() {
    const [windowToggleMaximizeIcon, setwindowToggleMaximizeIcon] = createSignal("i-mdi:window-maximize w-16px h-16px");
    appWindow.listen(TauriEvent.WINDOW_RESIZED, async () =>
        setwindowToggleMaximizeIcon(await appWindow.isMaximized() ?
            "i-mdi:window-restore w-16px h-16px" :
            "i-mdi:window-maximize w-16px h-16px",
        ),
    );
    return <div data-tauri-drag-region class="h-32px flex items-center">
        <label class="h-24px w-80px flex justify-center font-bold" style={{ "text-shadow": "0px 0px 10px gray" }}>修改器</label>
        <div class="flex-1" />
        <div data-tauri-drag-region class="w-32px flex justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={appWindow.minimize}>
                <div class="i-mdi:window-minimize h-16px w-16px" />
            </div>
        </div>
        <div data-tauri-drag-region class="w-32px flex justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={appWindow.toggleMaximize}>
                <div class={windowToggleMaximizeIcon()} />
            </div>
        </div>
        <div data-tauri-drag-region class="w-32px flex justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={appWindow.close}>
                <div class="i-mdi:window-close h-16px w-16px" />
            </div>
        </div>
    </div>;
}
