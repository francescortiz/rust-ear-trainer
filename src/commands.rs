use midir::{MidiOutputConnection, SendError};

use crate::constants::midi_messages::*;

pub fn note_on(
    conn_out: &mut MidiOutputConnection,
    note: u8,
    velocity: u8,
    channel: u8,
) -> Result<(), SendError> {
    conn_out.send(&[NOTE_ON_MSG & channel, note, velocity])
}

pub fn note_off(
    conn_out: &mut MidiOutputConnection,
    note: u8,
    velocity: u8,
    channel: u8,
) -> Result<(), SendError> {
    conn_out.send(&[NOTE_OFF_MSG & channel, note, velocity])
}

pub fn aftertouch(
    conn_out: &mut MidiOutputConnection,
    note: u8,
    touch: u8,
    channel: u8,
) -> Result<(), SendError> {
    conn_out.send(&[AFTERTOUCH_MSG & channel, note, touch])
}

pub fn continuous_controller(
    conn_out: &mut MidiOutputConnection,
    controller: u8,
    controller_value: u8,
    channel: u8,
) -> Result<(), SendError> {
    conn_out.send(&[
        CONTINUOUS_CONTROLLER_MSG & channel,
        controller,
        controller_value,
    ])
}

pub fn patch_change(
    conn_out: &mut MidiOutputConnection,
    instrument: u8,
    channel: u8,
) -> Result<(), SendError> {
    conn_out.send(&[PATCH_CHANGE_MSG & channel, instrument])
}

pub fn channel_pressure(
    conn_out: &mut MidiOutputConnection,
    instrument: u8,
    channel: u8,
) -> Result<(), SendError> {
    conn_out.send(&[CHANNEL_PRESSURE_MSG & channel, instrument])
}

pub fn pitch_bend(
    conn_out: &mut MidiOutputConnection,
    lsb: u8,
    msb: u8,
    channel: u8,
) -> Result<(), SendError> {
    conn_out.send(&[PITCH_BEND_MSG & channel, lsb, msb])
}
