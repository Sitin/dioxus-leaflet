import {ApiRecv, CtrlRecv, MapApiRecv, MapView} from "./core.gen";

export type Request = ApiRecv | CtrlRecv;

export function isApiRecv(req: Request): req is ApiRecv {
    return ["Init", "Stop"].indexOf(req as CtrlRecv) < 0;
}

export function isMapApiRecv(req: Request): req is { Map: MapApiRecv } {
    return (req as { Map: MapApiRecv }).Map !== undefined;
}

export function isMapSetViewRecv(req: MapApiRecv): req is { SetView: MapView } {
    return (req as { SetView: MapView }).SetView !== undefined;
}
