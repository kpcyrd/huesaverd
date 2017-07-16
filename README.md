# huesaverd

Turns your screen on and off based on hue light status. This requires homeassistant. Even though this was written for hue lights, you can query any enitity as long as it uses `"state": "on"`.

```
RUST_LOG=huesaverd=info cargo run "$DISPLAY" http://192.168.1.2:8123 light.over_the_rainbow
```

## License

GPLv3
