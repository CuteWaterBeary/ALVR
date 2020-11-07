#![allow(non_upper_case_globals, non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use jni::{
    objects::*,
    sys::jintArray,
    sys::{jfloatArray, jobjectArray, jstring},
    *,
};

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_LatencyCollector_DecoderInput(
    _: JNIEnv,
    _: JObject,
    frame_index: i64,
) {
    decoderInput(frame_index);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_LatencyCollector_DecoderOutput(
    _: JNIEnv,
    _: JObject,
    frame_index: i64,
) {
    decoderOutput(frame_index);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_initializeNative(
    env: JNIEnv,
    instance: JObject,
    activity: JObject,
    asset_manager: JObject,
    vr_thread: JObject,
    ar_mode: u8,
    initial_refresh_rate: i32,
) {
    initializeNative(
        env.get_native_interface() as _,
        *instance as _,
        *activity as _,
        *asset_manager as _,
        *vr_thread as _,
        ar_mode,
        initial_refresh_rate,
    )
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_destroyNative(
    env: JNIEnv,
    _: JObject,
) {
    destroyNative(env.get_native_interface() as _)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_getLoadingTextureNative(
    _: JNIEnv,
    _: JObject,
) -> i32 {
    getLoadingTextureNative()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_getSurfaceTextureIDNative(
    _: JNIEnv,
    _: JObject,
) -> i32 {
    getSurfaceTextureIDNative()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_getWebViewSurfaceTextureNative(
    _: JNIEnv,
    _: JObject,
) -> i32 {
    getWebViewSurfaceTextureNative()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_renderNative(
    _: JNIEnv,
    _: JObject,
    rendered_frame_index: i64,
) {
    renderNative(rendered_frame_index)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_renderLoadingNative(
    _: JNIEnv,
    _: JObject,
) {
    renderLoadingNative()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_sendTrackingInfoNative(
    env: JNIEnv,
    _: JObject,
    udp_receiver_thread: JObject,
) {
    sendTrackingInfoNative(env.get_native_interface() as _, *udp_receiver_thread as _)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_sendGuardianInfoNative(
    env: JNIEnv,
    _: JObject,
    udp_receiver_thread: JObject,
) {
    sendGuardianInfoNative(env.get_native_interface() as _, *udp_receiver_thread as _)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_sendMicDataNative(
    env: JNIEnv,
    _: JObject,
    udp_receiver_thread: JObject,
) {
    sendMicDataNative(env.get_native_interface() as _, *udp_receiver_thread as _)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_onChangeSettingsNative(
    _: JNIEnv,
    _: JObject,
    suspend: i32,
) {
    onChangeSettingsNative(suspend)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_onSurfaceCreatedNative(
    _: JNIEnv,
    _: JObject,
    surface: JObject,
) {
    onSurfaceCreatedNative(*surface as _)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_onSurfaceDestroyedNative(
    _: JNIEnv,
    _: JObject,
) {
    onSurfaceDestroyedNative()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_onSurfaceChangedNative(
    _: JNIEnv,
    _: JObject,
    surface: JObject,
) {
    onSurfaceChangedNative(*surface as _)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_onResumeNative(
    _: JNIEnv,
    _: JObject,
) {
    onResumeNative()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_onPauseNative(
    _: JNIEnv,
    _: JObject,
) {
    onPauseNative()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_isVrModeNative(
    _: JNIEnv,
    _: JObject,
) -> u8 {
    isVrModeNative()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_getDeviceDescriptorNative(
    env: JNIEnv,
    _: JObject,
    device_descriptor: JObject,
) {
    getDeviceDescriptorNative(env.get_native_interface() as _, *device_descriptor as _)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_setFrameGeometryNative(
    _: JNIEnv,
    _: JObject,
    width: i32,
    height: i32,
) {
    setFrameGeometryNative(width, height)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_setRefreshRateNative(
    _: JNIEnv,
    _: JObject,
    refresh_rate: i32,
) {
    setRefreshRateNative(refresh_rate)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_setStreamMicNative(
    _: JNIEnv,
    _: JObject,
    stream_mic: u8,
) {
    setStreamMicNative(stream_mic)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_setFFRParamsNative(
    _: JNIEnv,
    _: JObject,
    foveation_mode: i32,
    foveation_strength: f32,
    foveation_shape: f32,
    foveation_vertical_offset: f32,
) {
    setFFRParamsNative(
        foveation_mode,
        foveation_strength,
        foveation_shape,
        foveation_vertical_offset,
    )
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_onHapticsFeedbackNative(
    _: JNIEnv,
    _: JObject,
    start_time: i64,
    amplitude: f32,
    duration: f32,
    frequency: f32,
    hand: u8,
) {
    onHapticsFeedbackNative(start_time, amplitude, duration, frequency, hand)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_onGuardianSyncAckNative(
    _: JNIEnv,
    _: JObject,
    timestamp: i64,
) {
    onGuardianSyncAckNative(timestamp)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_OvrContext_onGuardianSegmentAckNative(
    _: JNIEnv,
    _: JObject,
    timestamp: i64,
    segment_index: i32,
) {
    onGuardianSegmentAckNative(timestamp, segment_index)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_ServerConnection_initializeSocket(
    env: JNIEnv,
    instance: JObject,
    hello_port: i32,
    port: i32,
    device_name: JString,
    broadcast_addr_list: jobjectArray,
    refresh_rates: jintArray,
    render_width: i32,
    render_height: i32,
    fov: jfloatArray,
    device_type: i32,
    device_sub_type: i32,
    device_capability_flags: i32,
    controller_capability_flags: i32,
    ipd: f32,
) {
    initializeSocket(
        env.get_native_interface() as _,
        *instance as _,
        hello_port,
        port,
        **device_name as _,
        broadcast_addr_list as _,
        refresh_rates as _,
        render_width,
        render_height,
        fov as _,
        device_type,
        device_sub_type,
        device_capability_flags,
        controller_capability_flags,
        ipd,
    )
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_ServerConnection_closeSocket(
    _: JNIEnv,
    _: JObject,
) {
    closeSocket()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_ServerConnection_runLoop(
    env: JNIEnv,
    instance: JObject,
    server_address: JString,
    server_port: i32,
) {
    runLoop(
        env.get_native_interface() as _,
        *instance as _,
        **server_address as _,
        server_port,
    )
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_ServerConnection_interruptNative(
    _: JNIEnv,
    _: JObject,
) {
    interruptNative()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_ServerConnection_isConnectedNative(
    _: JNIEnv,
    _: JObject,
) -> u8 {
    isConnectedNative()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_ServerConnection_getServerAddress(
    env: JNIEnv,
    _: JObject,
) -> jstring {
    getServerAddress(env.get_native_interface() as _, ) as _
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_ServerConnection_getServerPort(
    _: JNIEnv,
    _: JObject,
) -> i32 {
    getServerPort()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_ServerConnection_sendNative(
    _: JNIEnv,
    _: JObject,
    native_buffer: i64,
    buffer_length: i32,
) {
    sendNative(native_buffer, buffer_length)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_ServerConnection_setSinkPreparedNative(
    _: JNIEnv,
    _: JObject,
    prepared: u8,
) {
    setSinkPreparedNative(prepared)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_polygraphene_alvr_Utils_setFrameLogEnabled(
    _: JNIEnv,
    _: JObject,
    debug_flags: i64,
) {
    setFrameLogEnabled(debug_flags)
}