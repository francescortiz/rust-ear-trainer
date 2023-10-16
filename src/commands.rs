use midir::{MidiOutputConnection, SendError};

use crate::constants::midi_messages::*;

pub fn note_on(
    mut conn_out: &mut MidiOutputConnection,
    note: u8,
    velocity: u8,
    channel: u8,
) -> Result<(), SendError> {
    return conn_out.send(&[NOTE_ON_MSG & channel, note, velocity]);
}

pub fn note_off(
    mut conn_out: &mut MidiOutputConnection,
    note: u8,
    velocity: u8,
    channel: u8,
) -> Result<(), SendError> {
    return conn_out.send(&[NOTE_OFF_MSG & channel, note, velocity]);
}

pub fn aftertouch(
    mut conn_out: &mut MidiOutputConnection,
    note: u8,
    touch: u8,
    channel: u8,
) -> Result<(), SendError> {
    return conn_out.send(&[AFTERTOUCH_MSG & channel, note, touch]);
}

pub fn continuous_controller(
    mut conn_out: &mut MidiOutputConnection,
    controller: u8,
    controller_value: u8,
    channel: u8,
) -> Result<(), SendError> {
    return conn_out.send(&[
        CONTINUOUS_CONTROLLER_MSG & channel,
        controller,
        controller_value,
    ]);
}

pub fn patch_change(
    mut conn_out: &mut MidiOutputConnection,
    instrument: u8,
    channel: u8,
) -> Result<(), SendError> {
    return conn_out.send(&[PATCH_CHANGE_MSG & channel, instrument]);
}

pub fn channel_pressure(
    mut conn_out: &mut MidiOutputConnection,
    instrument: u8,
    channel: u8,
) -> Result<(), SendError> {
    return conn_out.send(&[CHANNEL_PRESSURE_MSG & channel, instrument]);
}

pub fn pitch_bend(
    mut conn_out: &mut MidiOutputConnection,
    lsb: u8,
    msb: u8,
    channel: u8,
) -> Result<(), SendError> {
    return conn_out.send(&[PITCH_BEND_MSG & channel, lsb, msb]);
}
