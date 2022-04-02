use super::VecI32;



#[derive(Debug)]
pub enum Event {
    Init,
    Exit,
    Show,
    Hide,
    KeyPress,
    KeyRelease,
    KeyRepeat,

    KeyPressExt,
    KeyReleaseExt,
    KeyRepeatExt,
    PointerUp { pos: VecI32 },
    PointerDown { pos: VecI32 },
    PointerMove { pos: VecI32 },

//comes from inkview only after calling AddScrolledArea function
    Scroll, //par1 is (irect *) -- scrolled area from wich scrolling was started, par2 is (deltaX (highest word) and deltaY(lowest word))
    PointerLong { pos: VecI32 },
    PointerHold { pos: VecI32 },
    PointerDrag { pos: VecI32 }, //like EVT_POINTERMOVE, but has non sensitive zone, which smooths finger touch bounce.
    PointerCancel { pos: VecI32 },
    PointerChanged { pos: VecI32 },

    Orientation,
    Focus,
    Unfocus,
    Activate,
    MtSync,
    TouchUp,
    TouchDown,
    TouchMove,
    Repaint,

    QnMove,

    QnReleaseEASE,

    QnBorder,

    Snapshot,
    Fsincoming,
    Fschanged,

    MpStatechanged,
    MpTrackchanged,

    Prevpage,
    Nextpage,
    Opendic,
    ControlPanelAboutToOpen,
    Update,

    PanelBluetoothA2dp,

    Tab,
    Panel,
    PanelIcon,
    PanelText,
    PanelProgress,
    PanelMplayer,
    PanelUsbdrive,
    PanelNetwork,
    PanelClock,
    PanelBluetooth,
    PanelTasklist,
    PanelObreeySync,
    PanelSetreadingmode,
    PanelSetreadingmodeInvert,
    PanelFrontLight,

    GlobalRequest,
/* 
enum globalaction_on_event_e {
	GLOBALACTION_ON_KEYPRESS = 0, //masked by mpc->gka0
	GLOBALACTION_ON_KEYHOLD, //masked by mpc->gka1
	GLOBALACTION_ON_DOUBLECLICK, //masked by mpc->gka2
};*/
    GlobalAction, //send to taskmanager par1 = key, par2 = enum globalaction_on_event_e
    Foreground,
    Background,
    SubTaskClose,
    ConfigChanged,
    SaveState,
    ObreeyConfigChanged,

    Sdin,
    Sdout,
    UsbStoreIn,
    UsbStoreOut,

    BtRxComplete,
    BtTxComplete,

    SynthEnded,
    DicClosedARD,
    ShowKeyboard,

    TextClear,
    ExtKb,
    Letter,

    Callback,

    ScanProgress,
    StopScan,
    StartScan,
    ScanStopped,
    PostponeTimedPowerOff,
    FrameActivated,
    FrameDeactivated,
    ReadProgressChanged,
    DumpBitmapsDebugInfo,

    NetConnected,
    NetDisconnected,
    NetFoundNewFw,
    SynthPosition,
    AsyncTaskFinished, // used for framework-2 async_code realization

    StopPlaying,
    AvrcpCommand,

    AudioChanged, //audio output routing was changed

    PackageJobChanged,
    Custom,
}

#[derive(Debug)]
pub enum EventError {
    UnknownEventType
}

impl Event {
    pub(super) fn from_c_iv_event(evt: i32, arg0: i32, arg1: i32) -> Result<Self, EventError> {
        match evt {
            21 => Ok(Event::Init),
            22 => Ok(Event::Exit),
            23 => Ok(Event::Show),
            24 => Ok(Event::Hide),
            25 => Ok(Event::KeyPress),
            26 => Ok(Event::KeyRelease),
            28 => Ok(Event::KeyRepeat),
            40 => Ok(Event::KeyPressExt),
            41 => Ok(Event::KeyReleaseExt),
            42 => Ok(Event::KeyRepeatExt),
            29 => Ok(Event::PointerUp { pos: VecI32 { x: arg0, y: arg1 } }),
            30 => Ok(Event::PointerDown { pos: VecI32 { x: arg0, y: arg1 } }),
            31 => Ok(Event::PointerMove { pos: VecI32 { x: arg0, y: arg1 } }),
            33 => Ok(Event::Scroll), //par1 is (irect *) -- scrolled area from wich scrolling was started, par2 is (deltaX (highest word) and deltaY(lowest word))
            34 => Ok(Event::PointerLong { pos: VecI32 { x: arg0, y: arg1 } }),
            35 => Ok(Event::PointerHold { pos: VecI32 { x: arg0, y: arg1 } }),
            44 => Ok(Event::PointerDrag { pos: VecI32 { x: arg0, y: arg1 } }), //like EVT_POINTERMOVE, but has non sensitive zone, which smooths finger touch bounce.
            45 => Ok(Event::PointerCancel { pos: VecI32 { x: arg0, y: arg1 } }),
            46 => Ok(Event::PointerChanged { pos: VecI32 { x: arg0, y: arg1 } }),
            32 => Ok(Event::Orientation             ),
            36 => Ok(Event::Focus                   ),
            37 => Ok(Event::Unfocus                 ),
            38 => Ok(Event::Activate                ),
            39 => Ok(Event::MtSync                  ),
            47 => Ok(Event::TouchUp                 ),
            48 => Ok(Event::TouchDown               ),
            49 => Ok(Event::TouchMove               ),
            43 => Ok(Event::Repaint                 ),
            51 => Ok(Event::QnMove                  ),
            52 => Ok(Event::QnReleaseEASE           ),
            53 => Ok(Event::QnBorder                ),
            71 => Ok(Event::Snapshot                ),
            72 => Ok(Event::Fsincoming              ),
            73 => Ok(Event::Fschanged               ),
            81 => Ok(Event::MpStatechanged          ),
            82 => Ok(Event::MpTrackchanged          ),
            91 => Ok(Event::Prevpage                ),
            92 => Ok(Event::Nextpage                ),
            93 => Ok(Event::Opendic                 ),
            94 => Ok(Event::ControlPanelAboutToOpen ),
            95 => Ok(Event::Update                  ),
            118 => Ok(Event::PanelBluetoothA2dp        ),
            119 => Ok(Event::Tab                       ),
            120 => Ok(Event::Panel                     ),
            121 => Ok(Event::PanelIcon                 ),
            122 => Ok(Event::PanelText                 ),
            123 => Ok(Event::PanelProgress             ),
            124 => Ok(Event::PanelMplayer              ),
            125 => Ok(Event::PanelUsbdrive             ),
            126 => Ok(Event::PanelNetwork              ),
            127 => Ok(Event::PanelClock                ),
            128 => Ok(Event::PanelBluetooth            ),
            129 => Ok(Event::PanelTasklist             ),
            130 => Ok(Event::PanelObreeySync           ),
            131 => Ok(Event::PanelSetreadingmode       ),
            132 => Ok(Event::PanelSetreadingmodeInvert ),
            133 => Ok(Event::PanelFrontLight           ),
            149 => Ok(Event::GlobalRequest             ),
            150 => Ok(Event::GlobalAction          ), 
            151 => Ok(Event::Foreground            ),
            152 => Ok(Event::Background            ),
            153 => Ok(Event::SubTaskClose          ),
            154 => Ok(Event::ConfigChanged         ),
            155 => Ok(Event::SaveState             ),
            156 => Ok(Event::ObreeyConfigChanged   ),
            161 => Ok(Event::Sdin                  ),
            162 => Ok(Event::Sdout                 ),
            163 => Ok(Event::UsbStoreIn	        ),
            164 => Ok(Event::UsbStoreOut           ),
            171 => Ok(Event::BtRxComplete          ),
            172 => Ok(Event::BtTxComplete          ),
            200 => Ok(Event::SynthEnded            ),
            202 => Ok(Event::DicClosedARD          ),
            201 => Ok(Event::ShowKeyboard          ),
            209 => Ok(Event::TextClear             ),
            210 => Ok(Event::ExtKb                 ),
            211 => Ok(Event::Letter                ),
            212 => Ok(Event::Callback              ),
            213 => Ok(Event::ScanProgress          ),
            214 => Ok(Event::StopScan              ),
            215 => Ok(Event::StartScan             ),
            216 => Ok(Event::ScanStopped           ),
            217 => Ok(Event::PostponeTimedPowerOff ),
            218 => Ok(Event::FrameActivated        ),
            219 => Ok(Event::FrameDeactivated      ),
            220 => Ok(Event::ReadProgressChanged   ),
            221 => Ok(Event::DumpBitmapsDebugInfo  ),
            256 => Ok(Event::NetConnected     ),
            257 => Ok(Event::NetDisconnected  ),
            260 => Ok(Event::NetFoundNewFw    ),
            261 => Ok(Event::SynthPosition    ),
            262 => Ok(Event::AsyncTaskFinished),
            263 => Ok(Event::StopPlaying      ),
            264 => Ok(Event::AvrcpCommand     ),
            265 => Ok(Event::AudioChanged     ),
            266 => Ok(Event::PackageJobChanged),
            267 => Ok(Event::Custom           ),
            _ => Err(EventError::UnknownEventType)
        }
    }
}
