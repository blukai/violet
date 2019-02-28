command!(ping(_ctx, msg) {
    let _ = msg.channel_id.say("¡pong");
});

command!(pong(_ctx, msg) {
    let _ = msg.channel_id.say("¡ping");
});

command!(clear(_ctx, msg) {
    let _ = msg.channel_id.say("not implemented");
});
