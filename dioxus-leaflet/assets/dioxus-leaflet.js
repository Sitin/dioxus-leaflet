if (!window._dioxusLeaflet) {
    window._dioxusLeaflet = (function () {
        let __container = {};

        function mapIds() {
            return Object.keys(__container);
        }

        function addMap(mapId, map) {
            removeMap(mapId);
            __container[mapId] = {
                map: map,
                tileLayers: {},
            };
        }

        function removeMap(mapId) {
            if (__container[mapId]) {
                let map = __container[mapId].map;
                if (map.remove) {
                    map.remove();
                }
                delete __container[mapId];
            }
        }

        function getTileLayer(mapId, layerId) {
            if (__container[mapId]) {
                if (__container[mapId].tileLayers[layerId]) {
                    return __container[mapId].tileLayers[layerId];
                }
            }
        }

        function addTileLayer(mapId, layerId, layer) {
            if (__container[mapId]) {
                removeTileLayer(mapId, layerId);

                let map = __container[mapId].map;
                let layers = __container[mapId].tileLayers;

                layer.addTo(map);
                layers[layerId] = layer;
            }
        }

        function removeTileLayer(mapId, layerId) {
            if (__container[mapId]) {
                let map = __container[mapId].map;
                let layers = __container[mapId].tileLayers;

                if (layers[layerId]) {
                    let layer = layers[layerId];
                    layer.removeFrom(map);
                    delete layers[layerId];
                }
            }
        }

        function _container() {
            return __container;
        }

        return {
            version: '0.1.0',
            mapIds: mapIds,
            addMap: addMap,
            removeMap: removeMap,
            getTileLayer: getTileLayer,
            addTileLayer: addTileLayer,
            removeTileLayer: removeTileLayer,
            _container: _container,
        };
    })();
}
