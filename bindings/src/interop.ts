import {CtrlRecv, CtrlResp} from "./core.gen";
import {delay} from "./utils";
import {Request} from "./requests";
import {Response} from "./responses";

export type WebDioxusChannel = {
    send: (res: CtrlResp) => void,
    recv: () => Promise<CtrlRecv>,
}

export type ApiRecvChannel = {
    send: (res: CtrlResp) => void,
    recv: () => Promise<Request>,
}

export type EventChannel = {
    send: (event: Response) => void,
}

export type EventChannelsContainer = {
    [id: string]: () => Promise<WebDioxusChannel>;
}

export type StopSignalContainer = {
    [id: string]: boolean;
}

export async function initWithMainTask(
    mainBus: WebDioxusChannel,
    id: string,
    task: (id: string, req: ApiRecvChannel, events: EventChannel) => Promise<void>
) {
    window.__dioxusLeaflet.__stopSignals[id] = false;

    mainBus.send("Created");

    let eventsBus = await awaitEventsBus(id, mainBus);

    await confirmChannelsInit(mainBus, eventsBus);

    await task(id, mainBus as ApiRecvChannel, eventsBus);

    window.__dioxusLeaflet.__stopSignals[id] = true;
    console.warn(`[dioxus-leaflet] Stopped main channel for '${id}'.`);
}

export async function registerEventsBus(eventsBus: WebDioxusChannel, id: string) {
    window.__dioxusLeaflet.__pendingEventChannels[id] = async () => {
        return eventsBus;
    };

    while (window.__dioxusLeaflet.__stopSignals[id] === false) {
        await delay(100);
    }

    delete window.__dioxusLeaflet.__stopSignals[id];
    console.warn(`[dioxus-leaflet] Stopped events channel for '${id}'.`);
}

export async function awaitEventsBus(id: string, mainBus: WebDioxusChannel): Promise<WebDioxusChannel> {
    while (true) {
        let request = await mainBus.recv();

        if (request == "Stop") {
            console.warn("[dioxus-leaflet] Stopped prematurely awaiting events channel.");
            window.__dioxusLeaflet.__stopSignals[id] = true;
            return;
        } else if (request == "Init") {
            break;
        } else {
            console.warn(`[dioxus-leaflet] Unexpected request during init '${request}'.`);
        }
    }

    let eventsChannel = await window.__dioxusLeaflet.__pendingEventChannels[id]();
    delete window.__dioxusLeaflet.__pendingEventChannels[id];

    return eventsChannel;
}

export async function confirmChannelsInit(requestChannel: WebDioxusChannel, eventsChannel: WebDioxusChannel) {
    /// !!! Do not change this order !!!
    eventsChannel.send("Initialized");
    requestChannel.send("Initialized");
}
