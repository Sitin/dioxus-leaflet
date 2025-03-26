import {TileLayer, TileLayerApiRecv} from "../core.gen";
import {Request} from "../requests";
import * as L from "leaflet";

export type TileLayerApiCall = { TileLayer: { id: string; recv: TileLayerApiRecv } };

export function isTileLayerApiRecv(req: Request): req is TileLayerApiCall {
    return (req as TileLayerApiCall).TileLayer !== undefined;
}

export function isTileLayerAddRecv(req: TileLayerApiRecv): req is { Add: TileLayer } {
    return (req as { Add: TileLayer }).Add !== undefined;
}

export function isTileLayerRemoveRecv(req: TileLayerApiRecv): req is "Remove" {
    return req === "Remove";
}

export function isTileLayerSetOpacityRecv(req: TileLayerApiRecv): req is { SetOpacity: number } {
    return (req as { SetOpacity: number }).SetOpacity !== undefined;
}

export type TileLayerContainer = {
    [id: string]: L.TileLayer,
}

export function processTileLayerApiRecv(req: TileLayerApiCall, map: L.Map, layers: TileLayerContainer) {
    if (isTileLayerAddRecv(req.TileLayer.recv)) {
        let id = req.TileLayer.id;
        let tiles = req.TileLayer.recv.Add.tiles;
        let options = req.TileLayer.recv.Add.options;

        let layer = L.tileLayer(tiles, options);
        map.addLayer(layer);
        layers[id] = layer;

        console.log(`[dioxus-leaflet] Tile layer added: ${id}.`);
    } else if (isTileLayerRemoveRecv(req.TileLayer.recv)) {
        let id = req.TileLayer.id;
        if (layers[id] !== undefined) {
            layers[id].removeFrom(map);
            delete layers[id];
        }
        delete layers[id];
        console.log(`[dioxus-leaflet] Tile layer removed: ${id}.`);
    } else if (isTileLayerSetOpacityRecv(req.TileLayer.recv)) {
        let id = req.TileLayer.id;
        let opacity = req.TileLayer.recv.SetOpacity;

        if (layers[id] !== undefined) {
            layers[id].setOpacity(opacity);
            console.log(`[dioxus-leaflet] Tile layer ${id} opacity: ${opacity}.`);
        }
    }
}
