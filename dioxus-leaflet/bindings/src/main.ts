import * as L from 'leaflet';
import {LeafletMouseEvent} from 'leaflet';
import {ApiRecvChannel, EventChannel} from "./interop";
import {isApiRecv, isMapApiRecv, isMapSetViewRecv,} from "./requests";
import {isTileLayerApiRecv, processTileLayerApiRecv, TileLayerContainer} from "./api/tileLayer";

export async function mainTask(id: string, apiRecv: ApiRecvChannel, eventSink: EventChannel): Promise<void> {
    console.log(`[dioxus-leaflet] Starting main task for ${id}.`);

    let map = L.map(id);

    eventSink.send("Ready");

    await initMapWithView(map, apiRecv);

    map.on('click', (event: LeafletMouseEvent) => {
        console.log('[dioxus-leaflet]: click:', event.latlng, event.containerPoint);
        window.__dioxusLeaflet.__debugObjects.push(event);
    });

    await mainLoop(id, map, apiRecv, eventSink);

    drop(id, map);
}

async function mainLoop(id: string, map: L.Map, apiRecv: ApiRecvChannel, eventSink: EventChannel) {
    console.log(`[dioxus-leaflet] Entering main loop for ${id}.`);

    let tileLayers: TileLayerContainer = {};

    while (true) {
        let req = await apiRecv.recv();

        if (req == "Stop") {
            break;
        }

        if (isApiRecv(req)) {
            if (isTileLayerApiRecv(req)) {
                processTileLayerApiRecv(req, map, tileLayers)
            }
        }
    }
}

async function initMapWithView(map: L.Map, apiRecv: ApiRecvChannel) {
    while (true) {
        let req = await apiRecv.recv();
        if (isMapApiRecv(req)) {
            let mapRecv = req.Map;
            if (isMapSetViewRecv(mapRecv)) {
                map.setView([mapRecv.SetView.lat, mapRecv.SetView.lng], mapRecv.SetView.zoom);
                break;
            }
        }
    }
}

function drop(id: string, map: L.Map) {
    map.remove();
}
