import {
    EventChannelsContainer,
    initWithMainTask,
    registerEventsBus,
    StopSignalContainer,
    WebDioxusChannel
} from "./interop";
import {mainTask} from "./main";
import {version as leafletVersion} from "leaflet";

type DioxusLeafletApi = {
    init: (channel: WebDioxusChannel, id: string) => Promise<void>;
    registerEventsChannel: (channel: WebDioxusChannel, id: string) => Promise<void>;
    version: string,
    leafletVersion: string,
    __pendingEventChannels: EventChannelsContainer;
    __stopSignals: StopSignalContainer;
    __debugObjects: any[],
}

declare global {
    interface Window {
        __dioxusLeaflet: DioxusLeafletApi,
    }
}

if (window !== undefined) {
    window.__dioxusLeaflet = {
        init: initDioxusLeaflet,
        registerEventsChannel: registerEventsBus,
        version: "0.1.0",
        leafletVersion: leafletVersion,
        __pendingEventChannels: {},
        __stopSignals: {},
        __debugObjects: [],
    }
}

async function initDioxusLeaflet(mainBus: WebDioxusChannel, id: string) {
    await initWithMainTask(mainBus, id, mainTask);
}
