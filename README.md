# Dioxus Leaflet

[Leaflet](https://leafletjs.com/) bindings for [Dioxus](https://dioxuslabs.com).

For now, this project is in POC state. It seems that it's possible to provide a meaningful functionality
for most of the users without covering the entire Leaflet API.

I'll would be happy to get any help from those who interested in a quick map solution for Dioxus.

## Development

The library is located in the [dioxus-leaflet](dioxus-leaflet) folder.

### Example App

Run the following command in the root of your project to start [example](./example) app for default platform:

```bash
dx serve -p example
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve -p example --platform desktop
```
