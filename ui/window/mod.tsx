import CenterPanel from "./center-panel/mod";
import SidePanel from "./side-panel";

export default function Window() {
    return <div class="absolute size-full flex bg-white">
        <SidePanel />
        <CenterPanel />
    </div>;
}
