// https://sourceforge.net/p/wsjt/wsjtx/ci/master/tree/Network/NetworkMessage.hpp#l150
// * Message Types
// * -------------
// *
// * Message       Direction Value                  Type
// * ------------- --------- ---------------------- -----------
// * Heartbeat     Out/In    0                      quint32
// *                         Id (unique key)        utf8
// *                         Maximum schema number  quint32
// *                         version                utf8
// *                         revision               utf8
// *
// *    The heartbeat  message shall be  sent on a periodic  basis every
// *    NetworkMessage::pulse   seconds   (see    below),   the   WSJT-X
// *    application  does  that  using the  MessageClient  class.   This
// *    message is intended to be used by servers to detect the presence
// *    of a  client and also  the unexpected disappearance of  a client
// *    and  by clients  to learn  the schema  negotiated by  the server
// *    after it receives  the initial heartbeat message  from a client.
// *    The message_aggregator reference server does just that using the
// *    MessageServer class. Upon  initial startup a client  must send a
// *    heartbeat message as soon as  is practical, this message is used
// *    to negotiate the maximum schema  number common to the client and
// *    server. Note  that the  server may  not be  able to  support the
// *    client's  requested maximum  schema  number, in  which case  the
// *    first  message received  from the  server will  specify a  lower
// *    schema number (never a higher one  as that is not allowed). If a
// *    server replies  with a lower  schema number then no  higher than
// *    that number shall be used for all further outgoing messages from
// *    either clients or the server itself.
// *
// *    Note: the  "Maximum schema number"  field was introduced  at the
// *    same time as schema 3, therefore servers and clients must assume
// *    schema 2 is the highest schema number supported if the Heartbeat
// *    message does not contain the "Maximum schema number" field.


// * Status        Out       1                      quint32
// *                         Id (unique key)        utf8
// *                         Dial Frequency (Hz)    quint64
// *                         Mode                   utf8
// *                         DX call                utf8
// *                         Report                 utf8
// *                         Tx Mode                utf8
// *                         Tx Enabled             bool
// *                         Transmitting           bool
// *                         Decoding               bool
// *                         Rx DF                  quint32
// *                         Tx DF                  quint32
// *                         DE call                utf8
// *                         DE grid                utf8
// *                         DX grid                utf8
// *                         Tx Watchdog            bool
// *                         Sub-mode               utf8
// *                         Fast mode              bool
// *                         Special Operation Mode quint8
// *                         Frequency Tolerance    quint32
// *                         T/R Period             quint32
// *                         Configuration Name     utf8
// *                         Tx Message             utf8
// *
// *    WSJT-X  sends this  status message  when various  internal state
// *    changes to allow the server to  track the relevant state of each
// *    client without the need for  polling commands. The current state
// *    changes that generate status messages are:
// *
// *      Application start up,
// *      "Enable Tx" button status changes,
// *      dial frequency changes,
// *      changes to the "DX Call" field,
// *      operating mode, sub-mode or fast mode changes,
// *      transmit mode changed (in dual JT9+JT65 mode),
// *      changes to the "Rpt" spinner,
// *      after an old decodes replay sequence (see Replay below),
// *      when switching between Tx and Rx mode,
// *      at the start and end of decoding,
// *      when the Rx DF changes,
// *      when the Tx DF changes,
// *      when settings are exited,
// *      when the DX call or grid changes,
// *      when the Tx watchdog is set or reset,
// *      when the frequency tolerance is changed,
// *      when the T/R period is changed,
// *      when the configuration name changes,
// *      when the message being transmitted changes.
// *
// *    The Special operation mode is  an enumeration that indicates the
// *    setting  selected  in  the  WSJT-X  "Settings->Advanced->Special
// *    operating activity" panel. The values are as follows:
// *
// *       0 -> NONE
// *       1 -> NA VHF
// *       2 -> EU VHF
// *       3 -> FIELD DAY
// *       4 -> RTTY RU
// *       5 -> WW DIGI
// *       6 -> FOX
// *       7 -> HOUND
// *
// *    The Frequency Tolerance  and T/R period fields may  have a value
// *    of  the maximum  quint32 value  which implies  the field  is not
// *    applicable.
// *

struct Heartbeat {

}

enum Type {
    Heartbeat,
    Status,
    Decode,
    Clear,
    Reply,
    QSOLogged,
    Close,
    Replay,
    HaltTx,
    FreeText,
    WSPRDecode,
    Location,
    LoggedADIF,
    HighlightCallsign,
    SwitchConfiguration,
    Configure,
};

struct NetworkMessageHeader {
    magic_number: u32 = 0xadbccbda,
    schema_number: u32 = 3,
}

struct  NetworkMessage {
    message_type: Type,
    header: NetworkMessageHeader,
}

impl NetworkMessage {
    pub fn encode(&self) {
        // method body would be defined here
    }

    pub fn decode(&self) {
        // method body would be defined here
    }
}