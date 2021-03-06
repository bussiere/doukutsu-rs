use crate::sound::pixtone::{Channel, Envelope, PixToneParameters, Waveform};

pub static PIXTONE_TABLE: [PixToneParameters; 51] = [
    PixToneParameters::empty(), // 0
    PixToneParameters {
        channels: [
            Channel {
                enabled: true,
                length: 3000,
                carrier: Waveform {
                    waveform_type: 0,
                    pitch: 99.0,
                    level: 32,
                    offset: 0,
                },
                frequency: Waveform {
                    waveform_type: 2,
                    pitch: 1.0,
                    level: 55,
                    offset: 197,
                },
                amplitude: Waveform {
                    waveform_type: 5,
                    pitch: 0.0,
                    level: 0,
                    offset: 0,
                },
                envelope: Envelope {
                    initial: 63,
                    time_a: 0,
                    value_a: 63,
                    time_b: 164,
                    value_b: 28,
                    time_c: 255,
                    value_c: 0,
                },
            }, Channel::disabled(), Channel::disabled(), Channel::disabled()
        ]
    },
    PixToneParameters {
        channels: [
            Channel {
                enabled: true,
                length: 4000,
                carrier: Waveform {
                    waveform_type: 1,
                    pitch: 54.0,
                    level: 32,
                    offset: 0,
                },
                frequency: Waveform {
                    waveform_type: 5,
                    pitch: 0.1,
                    level: 33,
                    offset: 0,
                },
                amplitude: Waveform {
                    waveform_type: 0,
                    pitch: 0.0,
                    level: 32,
                    offset: 0,
                },
                envelope: Envelope {
                    initial: 53,
                    time_a: 57,
                    value_a: 44,
                    time_b: 128,
                    value_b: 24,
                    time_c: 255,
                    value_c: 0,
                },
            }, Channel::disabled(), Channel::disabled(), Channel::disabled()
        ]
    },
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(), // 10
    PixToneParameters::empty(),
    PixToneParameters {
        channels: [Channel {
            enabled: true,
            length: 10000,
            carrier: Waveform {
                waveform_type: 5,
                pitch: 7.3,
                level: 32,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 5,
                pitch: 0.2,
                level: 29,
                offset: 0,
            },
            amplitude: Waveform {
                waveform_type: 0,
                pitch: 0.0,
                level: 32,
                offset: 0,
            },
            envelope: Envelope {
                initial: 63,
                time_a: 91,
                value_a: 63,
                time_b: 149,
                value_b: 25,
                time_c: 255,
                value_c: 0,
            },
        }, Channel {
            enabled: true,
            length: 1000,
            carrier: Waveform {
                waveform_type: 0,
                pitch: 6.0,
                level: 32,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 3,
                pitch: 1.0,
                level: 32,
                offset: 0,
            },
            amplitude: Waveform {
                waveform_type: 0,
                pitch: 0.0,
                level: 32,
                offset: 0,
            },
            envelope: Envelope {
                initial: 63,
                time_a: 64,
                value_a: 63,
                time_b: 128,
                value_b: 63,
                time_c: 255,
                value_c: 63,
            },
        }, Channel::disabled(), Channel::disabled()]
    },
    PixToneParameters::empty(),
    PixToneParameters {
        channels: [Channel {
            enabled: true,
            length: 10000,
            carrier: Waveform {
                waveform_type: 1,
                pitch: 246.0,
                level: 23,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 4,
                pitch: 0.6,
                level: 22,
                offset: 239,
            },
            amplitude: Waveform {
                waveform_type: 4,
                pitch: 6.0,
                level: 63,
                offset: 0,
            },
            envelope: Envelope {
                initial: 0,
                time_a: 11,
                value_a: 63,
                time_b: 13,
                value_b: 63,
                time_c: 255,
                value_c: 0,
            },
        }, Channel::disabled(), Channel::disabled(), Channel::disabled()]
    },
    PixToneParameters {
        channels: [Channel {
            enabled: true,
            length: 1000,
            carrier: Waveform {
                waveform_type: 5,
                pitch: 1.0,
                level: 32,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 3,
                pitch: 1.0,
                level: 63,
                offset: 0,
            },
            amplitude: Waveform {
                waveform_type: 0,
                pitch: 0.0,
                level: 63,
                offset: 0,
            },
            envelope: Envelope {
                initial: 0,
                time_a: 28,
                value_a: 63,
                time_b: 53,
                value_b: 31,
                time_c: 210,
                value_c: 31,
            },
        }, Channel::disabled(), Channel::disabled(), Channel::disabled()]
    },
    PixToneParameters {
        channels: [Channel {
            enabled: true,
            length: 5000,
            carrier: Waveform {
                waveform_type: 2,
                pitch: 50.0,
                level: 39,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 3,
                pitch: 0.5,
                level: 40,
                offset: 217,
            },
            amplitude: Waveform {
                waveform_type: 1,
                pitch: 0.0,
                level: 32,
                offset: 0,
            },
            envelope: Envelope {
                initial: 63,
                time_a: 64,
                value_a: 63,
                time_b: 128,
                value_b: 34,
                time_c: 198,
                value_c: 32,
            },
        }, Channel {
            enabled: true,
            length: 5000,
            carrier: Waveform {
                waveform_type: 5,
                pitch: 10.0,
                level: 39,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 3,
                pitch: 0.5,
                level: 24,
                offset: 217,
            },
            amplitude: Waveform {
                waveform_type: 1,
                pitch: 4.0,
                level: 32,
                offset: 0,
            },
            envelope: Envelope {
                initial: 0,
                time_a: 4,
                value_a: 63,
                time_b: 128,
                value_b: 34,
                time_c: 198,
                value_c: 32,
            },
        }, Channel::disabled(), Channel::disabled()]
    },
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(), // 20
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters {
        channels: [Channel {
            enabled: true,
            length: 3000,
            carrier: Waveform {
                waveform_type: 1,
                pitch: 17.0,
                level: 34,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 3,
                pitch: 2.0,
                level: 40,
                offset: 0,
            },
            amplitude: Waveform {
                waveform_type: 4,
                pitch: 1.0,
                level: 31,
                offset: 0,
            },
            envelope: Envelope {
                initial: 63,
                time_a: 64,
                value_a: 63,
                time_b: 225,
                value_b: 63,
                time_c: 255,
                value_c: 0,
            },
        }, Channel::disabled(), Channel::disabled(), Channel::disabled()]
    },
    PixToneParameters {
        channels: [Channel {
            enabled: true,
            length: 1000,
            carrier: Waveform {
                waveform_type: 1,
                pitch: 5.0,
                level: 32,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 3,
                pitch: 1.0,
                level: 63,
                offset: 0,
            },
            amplitude: Waveform {
                waveform_type: 0,
                pitch: 0.0,
                level: 0,
                offset: 0,
            },
            envelope: Envelope {
                initial: 63,
                time_a: 64,
                value_a: 63,
                time_b: 128,
                value_b: 31,
                time_c: 255,
                value_c: 0,
            },
        }, Channel::disabled(), Channel::disabled(), Channel::disabled()]
    },
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters {
        channels: [Channel {
            enabled: true,
            length: 3000,
            carrier: Waveform {
                waveform_type: 0,
                pitch: 13.0,
                level: 24,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 3,
                pitch: 2.0,
                level: 40,
                offset: 0,
            },
            amplitude: Waveform {
                waveform_type: 4,
                pitch: 1.0,
                level: 31,
                offset: 0,
            },
            envelope: Envelope {
                initial: 63,
                time_a: 64,
                value_a: 63,
                time_b: 225,
                value_b: 63,
                time_c: 255,
                value_c: 0,
            },
        }, Channel {
            enabled: true,
            length: 3000,
            carrier: Waveform {
                waveform_type: 5,
                pitch: 6.0,
                level: 32,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 5,
                pitch: 1.0,
                level: 32,
                offset: 0,
            },
            amplitude: Waveform {
                waveform_type: 3,
                pitch: 6.0,
                level: 0,
                offset: 0,
            },
            envelope: Envelope {
                initial: 0,
                time_a: 0,
                value_a: 63,
                time_b: 45,
                value_b: 23,
                time_c: 255,
                value_c: 0,
            },
        }, Channel::disabled(), Channel::disabled()]
    },
    PixToneParameters::empty(),
    PixToneParameters { // 30
        channels: [Channel {
            enabled: true,
            length: 10000,
            carrier: Waveform {
                waveform_type: 2,
                pitch: 168.0,
                level: 32,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 0,
                pitch: 0.5,
                level: 29,
                offset: 173,
            },
            amplitude: Waveform {
                waveform_type: 0,
                pitch: 0.0,
                level: 32,
                offset: 0,
            },
            envelope: Envelope {
                initial: 0,
                time_a: 13,
                value_a: 63,
                time_b: 68,
                value_b: 35,
                time_c: 255,
                value_c: 0,
            },
        }, Channel::disabled(), Channel::disabled(), Channel::disabled()]
    },
    PixToneParameters::empty(),
    PixToneParameters {
        channels: [Channel {
            enabled: true,
            length: 5000,
            carrier: Waveform {
                waveform_type: 5,
                pitch: 10.0,
                level: 32,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 4,
                pitch: 4.0,
                level: 32,
                offset: 0,
            },
            amplitude: Waveform {
                waveform_type: 0,
                pitch: 0.0,
                level: 63,
                offset: 0,
            },
            envelope: Envelope {
                initial: 63,
                time_a: 6,
                value_a: 63,
                time_b: 45,
                value_b: 8,
                time_c: 119,
                value_c: 46,
            },
        }, Channel {
            enabled: true,
            length: 1000,
            carrier: Waveform {
                waveform_type: 0,
                pitch: 4.0,
                level: 32,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 3,
                pitch: 1.0,
                level: 63,
                offset: 0,
            },
            amplitude: Waveform {
                waveform_type: 0,
                pitch: 0.0,
                level: 63,
                offset: 0,
            },
            envelope: Envelope {
                initial: 63,
                time_a: 64,
                value_a: 63,
                time_b: 128,
                value_b: 63,
                time_c: 255,
                value_c: 63,
            },
        }, Channel::disabled(), Channel::disabled()]
    },
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(), // 40
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters {
        channels: [Channel {
            enabled: true,
            length: 5000,
            carrier: Waveform {
                waveform_type: 0,
                pitch: 880.0,
                level: 19,
                offset: 0,
            },
            frequency: Waveform {
                waveform_type: 0,
                pitch: 0.0,
                level: 32,
                offset: 0,
            },
            amplitude: Waveform {
                waveform_type: 0,
                pitch: 8.0,
                level: 32,
                offset: 0,
            },
            envelope: Envelope {
                initial: 0,
                time_a: 11,
                value_a: 63,
                time_b: 34,
                value_b: 25,
                time_c: 255,
                value_c: 0,
            },
        }, Channel::disabled(), Channel::disabled(), Channel::disabled()]
    },
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(),
    PixToneParameters::empty(), // 50
];
