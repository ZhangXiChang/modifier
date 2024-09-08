import MenuPanel from "./menu-panel";

export default function CenterPanel() {
    return <div class="flex flex-1 flex-col">
        <MenuPanel />
        <div class="flex-1"></div>
    </div>;
}
