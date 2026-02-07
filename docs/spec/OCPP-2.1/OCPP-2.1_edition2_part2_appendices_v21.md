OCPP 2.1
Part 2 - Appendices

v2.1, 2025-12-03

Table of Contents

Disclaimer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  1
Version History . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  2
1. Security Events . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
2. Standardized Units of Measure . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  4
3. Standardized Components . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  5
3.1. Controller Components . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  5
3.2. Physical Components . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  13
3.3. Summary List of Standardized Components . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  31
4. Standardized Variables . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  35
5. Reason Codes. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  45
6. Standardized additionalInfo types . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  48
6.1. Generic. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  48
6.2. Ad hoc payment . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  48
7. Standardized values for enumerations as string . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  50
7.1. IdTokenEnumStringType . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  50
7.2. ChargingLimitSourceEnumStringType. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  50
7.3. ConnectorEnumStringType . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  50
7.4. SigningMethodEnumStringType. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  51

v2.1, 2025-12-03

Disclaimer

Disclaimer

Copyright © 2010 – 2025 Open Charge Alliance. All rights reserved.

This document is made available under the *Creative Commons Attribution-NoDerivatives 4.0 International Public License*
(https://creativecommons.org/licenses/by-nd/4.0/legalcode).

OCPP 2.1 - © Open Charge Alliance 2025

1/52

Part 2 - Appendices

v2.1, 2025-12-03

Version History

Version History

NOTE

The appendix can be updated independently of the OCPP release. As a result the version numbering of the
Appendix is not the same as the OCPP release.

Appendix version Date

OCPP Version

Description

2.1

2.0

1.4

1.3

1.2

1.1

1.0

2025-12-03

OCPP 2.1 Edition 2 Appendix version for OCPP 2.1. Updated parts are marked with

"(Updated in v2.1)"

2025-01-23

OCPP 2.1 Edition 1 Appendix version for OCPP 2.1 Updated parts are marked with

"(Updated in v2.0)"

2024-05-06

OCPP 2.0.1 Edition
3

Appendix version for Edition 3 Updated parts are marked with
"(Updated in v1.4)"

2022-12-15

OCPP 2.0.1

Appendix version for Errata 2 (2022)
Updated parts are marked with "(Updated in v1.3)".

2021-10-01

OCPP 2.0.1

Appendix version for Errata 1 (2021)
Appendix 3: Updated components are marked with "(Updated in v1.2)".
Appendix 3: Added ConnectedEV component for info from ISO15118
and CHAdeMO.
Appendix 5: Added reason MissingDeviceModelInfo

2020-03-23

2018-04-11

OCPP 2.0.1

Update for OCPP 2.0.1

OCPP 2.0

First release of this Appendix for OCPP 2.0

OCPP 2.1 - © Open Charge Alliance 2025

2/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 1. Security Events

Chapter 1. Security Events

The table below provides a list of security events. Security events that are implemented SHALL be stored at the security log and
security events that are implemented and marked as critical SHALL also be pushed to the CSMS.
This is a non-exhaustive list of security events, when a security event matches the description of one of the Security Events in this
section, for interoperability reasons, the Security Event from this section SHALL be used, instead of adding a new (proprietary)
Security Event. Some security events like; InvalidCsmsCertificate, InvalidChargingStationCertificate, etc. are mandatory to be
implemented. Please refer to Part 2 - Specification for which security events are mandatory to be implemented.

(Updated in v2.0)

Security Event

FirmwareUpdated

FailedToAuthenticateAtCsms

CsmsFailedToAuthenticate

SettingSystemTime

StartupOfTheDevice

ResetOrReboot

Description

The Charging Station firmware is updated

The authentication credentials provided by the Charging Station were
rejected by the CSMS

The authentication credentials provided by the CSMS were rejected by
the Charging Station

The system time on the Charging Station was changed more than
ClockCtrlr.TimeAdjustmentReportingThreshold seconds

The Charging Station has booted

The Charging Station was rebooted or reset

SecurityLogWasCleared

The security log was cleared

ReconfigurationOfSecurityParameters

MemoryExhaustion

InvalidMessages

AttemptedReplayAttacks

Security parameters, such as keys or the security profile used, were
changed

The Flash or RAM memory of the Charging Station is getting full

The Charging Station has received messages that are not valid OCPP
messages, if signed messages, signage invalid/incorrect

The Charging Station has received a replayed message (other than the
CSMS trying to resend a message because it there was for example a
network problem)

TamperDetectionActivated

The physical tamper detection sensor was triggered

InvalidFirmwareSignature

The firmware signature is not valid

InvalidFirmwareSigningCertificate

The certificate used to verify the firmware signature is not valid

Critical

Yes

No

No

Yes

Yes

Yes

Yes

No

Yes

No

No

Yes

Yes

Yes

InvalidCsmsCertificate

The certificate that the CSMS uses was not valid or could not be verified Yes

InvalidChargingStationCertificate

The certificate sent to the Charging Station using the
CertificateSignedRequest message is not a valid certificate

DiscardedRenewedClientCertificate

InvalidTLSVersion

InvalidTLSCipherSuite

MaintenanceLoginAccepted

MaintenanceLoginFailed

The Charging Station discarded the renewed client certificate, because
it was unable to successfully establish a connection using it.

The TLS version used by the CSMS is lower than 1.2 and is not allowed
by the security specification

The CSMS did only allow connections using TLS cipher suites that are
not allowed by the security specification

Successful login to the local maintenance interface. It is recommended
to include information like the user identification and the origin of the
login attempt, which can be an ip-address or a touch screen for
example, to the techInfo field. For this the following format is strongly
recommended: '{\'user\': \'…\', \'origin\': \'…\'}'

Failed login attempt to the local maintenance interface. It is
recommended to include information like the user identification and the
origin of the login attempt, which can be an ip-address or a touch screen
for example, to the techInfo field. For this the following format is
strongly recommended: '{\'user\': \'…\', \'origin\': \'…\'}'

Yes

Yes

Yes

Yes

Yes

Yes

OCPP 2.1 - © Open Charge Alliance 2025

3/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 2. Standardized Units of Measure

Chapter 2. Standardized Units of Measure

The standardized values for Unit of Measure. Default value of "unit" is always "Wh".

Value

A

ASU

B

Description

Amperes (current)

Arbitrary Strength Unit (Signal Strength)

Bytes

Celsius

Degrees (temperature)

dB

dBm

Deg

Decibel (for example Signal Strength)

Power relative to 1mW (10log(P/1mW))

Degrees (angle/rotation)

Fahrenheit

Degrees (temperature)

Hz

mHz

K

lx

m

ms2

N

Ohm

kPa

Hertz (frequency)

milliHertz (frequency)

Degrees Kelvin (temperature)

Lux (Light Intensity)

Meter (length)

m/s2 (Acceleration)

Newtons (Force)

Ohm (Impedance)

kiloPascal (Pressure)

Percent

Percentage

RH

RPM

s

V

VA

kVA

VAh

kVAh

var

kvar

varh

kvarh

W

kW

Wh

kWh

Relative Humidity%

Revolutions per Minute

Seconds (Time)

Voltage (DC or r.m.s. AC)

Volt-Ampere (apparent power)

kiloVolt-Ampere (apparent power)

Volt-Ampere-hours (apparent energy)

kiloVolt-Ampere-hours (apparent energy)

vars (reactive power)

kilovars (reactive power)

var-hours (reactive energy)

kilovar-hours (reactive energy)

Watts (power)

kilowatts (power)

Watt-hours (energy). Default

kilowatt-hours (energy)

OCPP 2.1 - © Open Charge Alliance 2025

4/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 3. Standardized Components

Chapter 3. Standardized Components

This appendix provides a list of all standardized component names for OCPP 2.1 for controller components and for physical
components. A summary table listing just all components without variables is provided at the end of this appendix in Summary List
of Standardized Components.

3.1. Controller Components

This is the list of Standardized Controller Components for OCPP 2.1 and typical Variables that might be associated with them.

IMPORTANT

This list does not imply that these Components are required, nor does it imply that the listed Variables are
required for a Component or no other Variables are allowed to be associated with a Component.

3.1.1. AlignedDataCtrlr

Description

Logical Component responsible for configuration relating to the reporting of clock-aligned meter data.

Variables

Enabled

Available

Interval

Type

boolean

boolean

integer

Description

If this variable reports a value of true, Aligned Data is enabled.

If this variable reports a value of true, Aligned Data is supported.

Size (in seconds) of the clock-aligned data interval, intended to be transmitted in
the MeterValuesRequest message.

Measurands

MemberList

Clock-aligned measurand(s) to be included in MeterValuesRequest, every
AlignedDataInterval seconds.

SendDuringIdle

SignReadings

TxEndedInterval

boolean

boolean

integer

TxEndedMeasurands

MemberList

If set to true, the Charging Station SHALL NOT send clock aligned meter values
when a transaction is ongoing.

If set to true, the Charging Station SHALL include signed meter values in the
TransactionEventRequest to the CSMS.

Size (in seconds) of the clock-aligned data interval, intended to be transmitted in
the TransactionEventRequest (eventType = Ended) message.

Clock-aligned periodic measurand(s) to be included in the meterValues element
of TransactionEventRequest (eventType = Ended) for every
TxEndedAlignedDataInterval of the transaction.

3.1.2. AuthCtrlr (Updated in v1.2)

Description

Logical Component responsible for configuration relating to the use of authorization for Charging Station use.

Variables

Enabled

Type

boolean

Description

If set to false, then no authorization is done before starting a transaction or when
reading an idToken. If an idToken was provided, then it will be put in the idToken
field of the TransactionEventRequest. If no idToken was provided, then idToken in
TransactionEventRequest will be left empty and type is set to
NoAuthorization.

AdditionalInfoItemsPerMessag
e

integer

Maximum number of AdditionalInfo items that can be sent in one message.

AuthorizeRemoteStart

boolean

DisableRemoteAuthorization

boolean

LocalAuthorizeOffline

boolean

LocalPreAuthorize

boolean

Whether a remote request to start a transaction in the form of
RequestStartTransactionRequest message should be authorized beforehand like
a local action to start a transaction.

When set to true this instructs the Charging Station to not issue any
AuthorizationRequests, but only use Authorization Cache and Local Authorization
List to determine validity of idTokens.

Whether the Charging Station, when Offline, will start a transaction for locally-
authorized identifiers.

Whether the Charging Station, when online, will start a transaction for locally-
authorized identifiers without waiting for or requesting an AuthorizeResponse
from the CSMS.

OCPP 2.1 - © Open Charge Alliance 2025

5/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

Chapter 3. Standardized Components

MasterPassGroupId

string

IdTokens that have this id as groupId belong to the Master Pass Group.

OfflineTxForUnknownIdEnabled boolean

If this key exists, the Charging Station supports Unknown Offline Authorization.

3.1.3. AuthCacheCtrlr (Updated in v2.1)

Description

Logical Component responsible for configuration relating to the use of a local cache for authorization for Charging Station use.

Variables

Enabled

Available

LifeTime

Policy

Type

boolean

boolean

integer

Description

If this variable exists, the Charging Station supports an Authorization Cache.

If this variable reports a value of true, Authorization Cache is supported.

Indicates in seconds how long it takes until a token expires in the authorization
cache since it is last used.

OptionList

Cache Entry Replacement Policy: (LRU,LFU) LeastRecentlyUsed or
LeastFrequentlyUsed. Allowed values: LRU, LFU.

DisablePostAuthorize

boolean

Storage

integer

When set to true this variable disables the behavior to request authorization for
an idToken that is stored in the cache with a status other than Accepted, as
stated in C10.FR.03 and C12.FR.05.

Indicates the number of bytes currently used by the Authorization Cache.
MaxLimit indicates the maximum number of bytes that can be used by the
Authorization Cache.

Entries

integer

Amount of IdTokens currently in the Authorization Cache.

3.1.4. CHAdeMOCtrlr (Updated in  v1.2)

Description

A CHAdeMO Controller component communicates with an EV using the wired CANbus protocol to exchange information and
control charging using the CHAdeMO protocol

Variables

Enabled

Active

Complete

Tripped

Problem

SelftestActive(Set)

SelftestActive

Type

boolean

boolean

boolean

boolean

boolean

boolean

boolean

Description

CHAdeMO controller enabled

Connected

Protocol session ended normally

CHAdeMO protocol terminated abnormally

CHAdeMO controller fault

Start self-test by setting to true

Self-test running when reported as true

Specific CHAdeMO interface data from vehicle:

CHAdeMOProtocolNumber

integer

CHAdeMO protocol number (H'102.0)

VehicleStatus

DynamicControl

HighCurrentControl

HighVoltageControl

AutoManufacturerCode

boolean

boolean

boolean

boolean

integer

Vehicle status (H'102.5.3)

Vehicle is compatible with dynamic control (H'110.0.0)

Vehicle is compatible with high current control (H'110.0.1)

Vehicle is compatible with high voltage control (H'110.1.2)

Auto manufacturer code (H'700.0)
A single byte manufacturer code assigned by CHAdeMO association

3.1.5. ClockCtrlr

Description

Provides a means to configure management of time tracking by Charging Station.

Variables

DateTime

Type

Description

dateTime

Contains the current date and time.

OCPP 2.1 - © Open Charge Alliance 2025

6/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

NtpServerUri

string

NtpSource

string

Chapter 3. Standardized Components

This contains the address of the NTP server. Multiple NTP servers can be
configured as backups, etc. If the NTP client supports it, it can also connect to
multiple NTP servers simultaneous to get a more reliable time source. Variable
instance value is single digit NTP priority (1=highest).

When an NTP client is implemented, this variable can be used to configure the
client: Use the NTP server provided via DHCP, or use the manually configured
NTP server.

TimeOffset

string

Configured local time offset in the format: "+01:00", "-02:00" etc.

NextTimeOffsetTransitionDateT
ime

dateTime

Date time of the next time offset transition.

TimeSource

TimeZone

string

string

Via this variable, the Charging Station provides the CSMS with the option to
configure a clock source, if more than 1 are implemented.

Configured current local time zone in the format: "Europe/Oslo", "Asia/Singapore"
etc.

TimeAdjustmentReportingThres
hold

integer

If set, then time adjustments with an absolute value in seconds larger than this
need to be reported as a security event SettingSystemTime.

3.1.6. CustomizationCtrlr (New in v1.2)

Description

Logical Component responsible for configuration relating to custom vendor-specific implementations, using the DataTransfer
message and CustomData extensions.

Variables

Type

Description

CustomImplementationEnabled boolean

This standard configuration variable can be used to enable/disable custom
implementations that the Charging Station supports. The instance name of the
variable matches the vendorId of the customization in CustomData or
DataTransfer messages.

CustomTriggers

MemberList

This variable defines the names of custom triggers that Charging Station
supports in a customTrigger field of Triggermessagerequest.

3.1.7. DeviceDataCtrlr

Description

Logical Component responsible for configuration relating to the exchange and storage of Charging Station Device Model data.

Variables

BytesPerMessage

ItemsPerMessage

ValueSize

Type

integer

integer

integer

Description

Message Size (in bytes) - maxLimit used to report constraint on message size.
Which message is specified in the instance.

Maximum number of entries that can be sent in one message. Which entries in
which message is specified in the instance.

Can be used to limit the following fields: SetVariableData.attributeValue,
GetVariableResult.attributeValue, VariableAttribute.value,
VariableCharacteristics.valuesList and EventData.actualValue.

3.1.8. DisplayMessageCtrlr

Description

Logical Component responsible for configuration relating to the display of messages to Charging Station users.

Variables

Enabled

Available

DisplayMessages

Type

boolean

boolean

integer

Description

Whether Display Message is enabled.

Whether Display Message is supported.

Amount of different messages that are currently configured in this Charging
Station, via SetDisplayMessageRequest.

SupportedFormats

MemberList

List of message formats supported by this Charging Station. Possible values: See
MessageFormatEnumType.

OCPP 2.1 - © Open Charge Alliance 2025

7/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

Chapter 3. Standardized Components

SupportedPriorities

MemberList

SupportedStates

MemberList

Language

OptionList

List of the priorities supported by this Charging Station. Possible values: See
MessagePriorityEnumType.

List of the states during which to display a message supported by this Charging
Station. Possible values: See MessageStateEnumType.

The default language of this Charging Station (per RFC 5646 language code).
Supported languages are reported in the valuesList.

3.1.9. ISO15118Ctrlr (Updated in  v1.3)

Description

Communicates with an EV to exchange information and control charging using the ISO 15118 protocol.

Variables

Enabled

Active

Tripped

Complete

Problem

SeccId

SelftestActive(Set)

SelftestActive

ContractValidationOffline

CentralContractValidationAllow
ed

PnCEnabled

Type

boolean

boolean

boolean

boolean

boolean

string

boolean

boolean

boolean

boolean

boolean

V2GCertificateInstallationEnabl
ed

boolean

ContractCertificateInstallationE
nabled

boolean

RequestMeteringReceipt

boolean

OrganizationName

string

CountryName

string

Specific ISO15118 interface data from vehicle:

MaxScheduleEntries

integer

RequestedEnergyTransferMode OptionList

Description

ISO15118 controller enabled

Connected

ISO15118 communication session aborted

ISO15118 communication session ended

ISO15118 controller fault

The name of the SECC in the string format as required by ISO 15118.

Start self-test by setting to true

Self-test running when reported as true

Supports validation of a contract certificate when offline

Contract certificates can be validated by the CSMS

If this variable is true, then ISO 15118 plug and charge as described by use case
C07 - Authorization using Contract Certificates is enabled.
If this variable is false, then ISO 15118 plug and charge as described by use case
C07 - Authorization using Contract Certificates is disabled.

If this variable is true, then ISO 15118 V2G Charging Station certificate
installation as described by use case A02 - Update Charging Station Certificate
by request of CSMS and A03 - Update Charging Station Certificate initiated by the
Charging Station is enabled.
If this variable is false, then ISO 15118 V2G Charging Station certificate
installation as described by use case A02 - Update Charging Station Certificate
by request of CSMS and A03 - Update Charging Station Certificate initiated by the
Charging Station is disabled.

If this variable is true, then ISO 15118 contract certificate installation/update as
described by use case M01 - Certificate installation EV and M02 - Certificate
Update EV is enabled.
If this variable is false, then ISO 15118 contract certificate installation/update as
described by use case M01 - Certificate installation EV and M02 - Certificate
Update EV is disabled.

If this variable is true, then Charging Station shall request a metering receipt from
EV before sending a fiscal meter value to CSMS.

The organizationName of the CSO operating the charging station.
It is used as the organizationName (O) of the SECC leaf certificate.
Example: "John Doe Charging Services Ltd"
Note: This value will usually be identical to SecurityCtrlr.OrganizationName, but it
does not have to be.

The countryName of the SECC in the ISO 3166-1 format.
It is used as the countryName (C) of the SECC leaf certificate.
Example: "DE"

MaxEntriesSAScheduleType (15118-2)
or MaximumSupportingPoints (15118-20) Number of allowed schedule periods

RequestedEnergyTransferMode
"AC_single_phase_core", "AC_three_phase_core", "DC_core, "DC_extended",
"DC_combo_core", "DC_unique"

OCPP 2.1 - © Open Charge Alliance 2025

8/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 3. Standardized Components

3.1.10. LocalAuthListCtrlr (Updated in v1.2)

Description

Logical Component responsible for configuration relating to the use of Local Authorization Lists for Charging Station use.

Variables

Enabled

Entries

Available

ItemsPerMessage

BytesPerMessage

Storage

Type

boolean

integer

boolean

integer

integer

integer

DisablePostAuthorize

boolean

SupportsExpiryDateTime

boolean

Description

Whether Local Authorization List is enabled.

Amount of IdTokens currently in the Local Authorization List. The maxLimit of
this variable SHALL be provided to report the maximum number of IdTokens that
can be stored in the Local Authorization List.

Whether Local Authorization List is supported.

Maximum number of identifications that can be sent in a single
SendLocalListRequest.

Message Size (in bytes) - puts a constraint on SendLocalListRequest message
size.

Indicates the number of bytes currently used by the Local Authorization List.
MaxLimit indicates the maximum number of bytes that can be used by the Local
Authorization List.

When set to true this variable disables the behavior to request authorization for
an idToken that is stored in the local authorization list with a status other than
Accepted, as stated in C14.FR.03.

When set to true Charging Station will disregard idTokens for authorization as if
not present in the Local Authorization List when current date/time is past the
value of cacheExpiryDateTime.

3.1.11. MonitoringCtrlr (Updated in  v1.3)

Description

Logical Component responsible for configuration relating to the exchange of monitoring event data.

Variables

Enabled

Available

ItemsPerMessage

BytesPerMessage

MonitoringBase

MonitoringLevel

OfflineQueuingSeverity

Type

boolean

boolean

integer

integer

Description

Whether Monitoring is enabled.

Whether Monitoring is supported.

Maximum number of items.

Message Size (in bytes) - puts constraint on message size.

optionList

Currently used MonitoringBase. (readonly)

integer

integer

Currently use MonitoringLevel (readonly)

When set and the Charging Station is offline, the Charging Station shall queue
any notifyEventRequest messages triggered by a monitor with a severity number
equal to or lower than the severity configured here. Value ranging from 0
(Emergency) to 9 (Debug).

Shows the currently used MonitoringBase. Valid values according
MonitoringBaseEnumType: All, FactoryDefault, HardwiredOnly. (readonly)

Shows the currently used MonitoringLevel. Valid values are severity levels of
SetMonitoringLevelRequest: 0-9. (readonly)

ActiveMonitoringBase

OptionList

ActiveMonitoringLevel

integer

3.1.12. OCPPCommCtrlr (Updated in  v1.4)

Description

Logical Component responsible for configuration relating to information exchange between Charging Station and CSMS.

Variables

ActiveNetworkProfile

Type

integer

Description

Indicates the configuration profile the station uses at that moment to connect to
the network.

FileTransferProtocols

MemberList

List of supported file transfer protocols.

HeartbeatInterval

MessageAttempts

integer

integer

Interval in seconds of inactivity (no OCPP exchanges) with CSMS after which the
Charging Station should send HeartbeatRequest.

How often the Charging Station should try to submit a TransactionEventRequest
message when the CSMS fails to process it.

OCPP 2.1 - © Open Charge Alliance 2025

9/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

MessageAttemptInterval

integer

MessageTimeout

integer

MinimumStatusDuration

integer

NetworkConfigurationPriority

string

NetworkProfileConnectionAtte
mpts

OfflineThreshold

integer

integer

Chapter 3. Standardized Components

How long in seconds the Charging Station should wait before resubmitting a
TransactionEventRequest message that the CSMS failed to process.

Message timeout in seconds. The message timeout setting in a Charging Station
can be configured in the messageTimeout field in the NetworkConnectionProfile.

Minimum duration that a Charging Station or EVSE status is stable before
StatusNotificationRequest is sent to the CSMS.

A comma separated ordered list of the priority of the possible Network
Connection Profiles.

Specifies the number of connection attempts the Charging Station executes
before switching to a different profile.

When the offline period in seconds of a Charging Station exceeds the
OfflineThreshold it is recommended to send a StatusNotificationRequest for all
its Connectors when the Charging Station is back online.

PublicKeyWithSignedMeterValu
e

boolean

This Configuration Variable can be used to configure whether a public key needs
to be sent with a signed meter value.

QueueAllMessages

boolean

RetryBackOffRepeatTimes

integer

RetryBackOffRandomRange

integer

RetryBackOffWaitMinimum

integer

UnlockOnEVSideDisconnect

boolean

WebSocketPingInterval

FieldLength

integer

integer

When this variable is set to true, the Charging Station will queue all message until
they are delivered to the CSMS.

When the Charging Station is reconnecting, after a connection loss, it will use this
variable for the amount of times it will double the previous back-off time.

When the Charging Station is reconnecting, after a connection loss, it will use this
variable as the maximum value for the random part of the back-off time.

When the Charging Station is reconnecting, after a connection loss, it will use this
variable as the minimum back-off time, the first time it tries to reconnect.

When set to true, the Charging Station SHALL unlock the cable on the Charging
Station side when the cable is unplugged at the EV. For an EVSE with only fixed
cables, the mutability SHALL be ReadOnly and the actual value SHALL be false.
For a charging station with fixed cables and sockets, the variable is only
applicable to the sockets.

Number of seconds between pings.

This variable is used to report the length of <field> in <message> when it is larger
than the length that is defined in the standard OCPP message schema.

3.1.13. WebPaymentsCtrlr (New in v2.0)

Description

Logical Component to configure the creation of URLs for web payment, e.g. via dynamic QR codes.

Variable

URLTemplate

Type

string

URLParameters

MemberList

TOTPVersion

string

ChargingStationId

string

Description

URL template

List of supported URL parameters
valuesList: "maxtime", "maxenergy", "maxcost"

Version of TOTP algorithm.
valuesList: list of supported TOTP versions, e.g. "v1"

(Optional) Charging station Id to use in URL.
When absent will default to Charging Station identity, as defined in
SecurityCtrlr.Identity.

ValidityTime

SharedSecret

Length

integer

string

integer

Time in seconds to show QR, e.g. 30

<random text> set to a random value on first boot

Length of TOTP, e.g. 8

QRCodeQuality

OptionList

Low, Medium, Quartile, High

3.1.14. ReservationCtrlr

Description

Logical Component responsible for configuration relating to reservations.

Variables

Type

Description

OCPP 2.1 - © Open Charge Alliance 2025

10/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

Enabled

Available

NonEvseSpecific

boolean

boolean

boolean

3.1.15. SampledDataCtrlr

Description

Chapter 3. Standardized Components

Whether Reservation is enabled.

Whether Reservation is supported.

If this configuration variable is present and set to true: Charging Station supports
Reservation without specifying an EVSE.

Logical Component responsible for configuration relating to the reporting of sampled meter data.

Variables

Enabled

Available

SignReadings

Type

boolean

boolean

boolean

Description

If this variable reports a value of true, Sampled Data is enabled.

If this variable reports a value of true, Sampled Data is supported.

If set to true, the Charging Station includes signed meter values in the
MeterValuesRequest to the CSMS.

TxEndedMeasurands

MemberList

TxEndedInterval

integer

TxStartedMeasurands

MemberList

TxUpdatedMeasurands

MemberList

Sampled measurands to be included in the meterValues element of
TransactionEventRequest (eventType = Ended), every TxEndedSampleInterval
seconds from the start of the transaction.

Interval in seconds between sampling of metering (or other) data, intended to be
transmitted in the TransactionEventRequest (eventType = Ended) message.

Sampled measurand(s) to be taken at the start of any transaction to be included
in the meterValues field of the first TransactionEventRequest message send at
the start of a transaction (eventType = Started).

Sampled measurands to be included in the meterValues element of every
TransactionEventRequest (eventType = Updated), every
SampledDataTxUpdatedInterval seconds from the start of the transaction.

TxUpdatedInterval

integer

Interval in seconds between sampling of metering (or other) data, intended to be
transmitted via TransactionEventRequest (eventType = Updated) messages.

RegisterValuesWithoutPhases

boolean

If this variable reports a value of true, then meter values of measurand
Energy.Active.Import.Register will only report the total energy over all phases
without reporting the individual phase values.
If this variable is absent or false, then the value for each phase is reported,
possibly also with a total value (depending on the meter).

3.1.16. SecurityCtrlr (Updated in v2.0)

Description

Logical Component responsible for configuration relating to security of communications between Charging Station and CSMS.

Variables

BasicAuthPassword

Type

string

Identity

string

OrganizationName

string

CertSigningWaitMinimum

integer

CertSigningRepeatTimes

integer

Description

The basic authentication password that is used for HTTP Basic Authentication.
The string is a passwordString (see Part 2: 2.1.4) consisting of minimum 16 and
a maximum as defined by the maxLimit of BasicAuthPassword, which must be
at least 40 characters. The password SHALL be sent as a UTF-8 encoded string
(NOT encoded into octet string or base64). This configuration variable is write-
only, so that it cannot be accidentally stored in plaintext by the CSMS when it
reads out all configuration variables.
This configuration variable is required unless only "security profile 3 - TLS with
client side certificates" is implemented.

The Charging Station identity. The string is an identifierString string (see Part 2:
2.1.4), so it SHALL only contain characters that are allowed for identifierString.
Maximum length was chosen to ensure compatibility with EVSE ID from [EMI3]
"Part 2: business objects".

This configuration variable is used to set the organization name of the CSO or an
organization trusted by the CSO. It is used to set the O (organizationName) RDN
in the subject field of the client certificate.

Seconds to wait before generating another CSR in case CSMS does not return a
signed certificate.

Number of times to resend a SignCertificateRequest when CSMS does nor return
a signed certificate.

OCPP 2.1 - © Open Charge Alliance 2025

11/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

AllowSecurityProfileDowngrade boolean

3.1.17. SmartChargingCtrlr

Description

Chapter 3. Standardized Components

If this variable is implemented and set to true, then the Charging Station allows
downgrading the security profile from 3 to 2. For security reasons it is not
allowed to revert from profile 2 or profile 3 to security profile 1 using OCPP.

Logical Component responsible for configuration relating to smart charging.

Variables

Enabled

Available

Type

boolean

boolean

Description

Whether Smart Charging is enabled.

Whether Smart Charging is supported.

ACPhaseSwitchingSupported

boolean

ProfileStackLevel

integer

If defined and true, this EVSE supports the selection of which phase to use for 1
phase AC charging.

Maximum acceptable value for stackLevel in a ChargingProfile. Since the lowest
stackLevel is 0, this means that if SmartChargingCtrlr.ProfileStackLevel = 1, there
can be at most 2 valid charging profiles per Charging Profile Purpose per EVSE.

RateUnit

MemberList

A list of supported quantities for use in a ChargingSchedule. Allowed values: 'A'
and 'W'.

PeriodsPerSchedule

integer

Maximum number of periods that may be defined per ChargingSchedule.

Indicates whether a Charging Station should respond to external control signals
that influence charging.

Indicates if the Charging Station should include the externally set charging
limit/schedule in the message when it sends a NotifyChargingLimitRequest
message. This might increase the data usage significantly, especially when an
external system sends new profiles/limits with a short interval. Default is false
when omitted.

If defined and true, this Charging Station supports switching from 3 to 1 phase
during a transaction.

Amount of Charging profiles currently installed on the Charging Station. MaxLimit
used to limit number of Charging profiles installed at any time.

If at the Charging Station side a change in the limit in a ChargingProfile is lower
than this percentage, the Charging Station MAY skip sending a
NotifyChargingLimitRequest or a TransactionEventRequest message to the
CSMS. It is RECOMMENDED to set this key to a low value. See Smart Charging
signals to a Charging Station from multiple actors.

ExternalControlSignalsEnabled boolean

NotifyChargingLimitWithSched
ules

boolean

Phases3to1

Entries

boolean

integer

LimitChangeSignificance

integer

Updated in v2.1

3.1.18. TariffCostCtrlr

Description

Logical Component responsible for configuration relating to tariff and cost display.

Variables

Available

Available

Enabled

Enabled

Enabled

Interval

Interval

MaxElements

Instance

Type

Description

Tariff

Cost

Tariff

Cost

RunningCos
t

boolean

boolean

boolean

boolean

boolean

Tariff

integer

Cost

Tariff

integer

integer

Whether TariffType structure is supported.

Whether local cost calculation is supported.

Whether support for TariffType structure is enabled.

Whether local cost calculation is enabled.

Whether Charging Station will provide running cost updates.

Maximum interval in seconds to use when evaluating tariff
conditions.

Interval to use for running cost updates.

Maximum number of prices elements supported in each
TariffEnergy/Time/FixedType.

ConditionsSupported

Tariff

boolean

Whether Charging Station supports conditions in TariffType.

OCPP 2.1 - © Open Charge Alliance 2025

12/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

HandleFailedTariff

Tariff

OptionList

string

string

string

string

OfflineTariffFallbackMes
sage

<rfc5646
language>

TariffFallbackMessage

TotalCostFallbackMessa
ge

Currency

3.1.19. TxCtrlr

Description

Chapter 3. Standardized Components

Deauthorize, UseDefaultTariff or CentralCost when tariff cannot
be processed.

Message to be shown when Charging Station is offline.

Message (and/or tariff information) to be shown to an EV Driver
when there is no driver specific tariff information available.

Message to be shown to an EV Driver when the Charging Station
cannot retrieve the cost for a transaction at the end of the
transaction.

Currency used by this Charging Station in a ISO 4217 formatted
currency code.

Logical Component responsible for configuration relating to transaction characteristics and behaviour.

Variables

EVConnectionTimeOut

Type

integer

TxBeforeAcceptedEnabled

boolean

TxStartPoint

MemberList

TxStopPoint

MemberList

MaxEnergyOnInvalidId

integer

StopTxOnInvalidId

boolean

Description

Interval in seconds from between "starting" of a transaction until incipient
transaction is automatically canceled, due to failure of EV driver to (correctly)
insert the charging cable connector(s) into the appropriate socket(s). The
Charging Station SHALL go back to the original state, probably: 'Available'.
"Starting" might be the swiping of the RFID, pressing a start button, a
RequestStartTransactionRequest being received etc.

With this configuration variable the Charging Station can be configured to allow
charging before having received a BootNotificationResponse with
RegistrationStatus: Accepted. See: Transactions before being accepted by a
CSMS.

Defines when the Charging Station starts a new transaction: first
transactioneventRequest: eventType = Started.
When any event in the given list occurs, the Charging Station SHALL start a
transaction The Charging Station SHALL only send the Started event once for
every transaction.
It is advised to put all events that should be part of a transaction in the list, in
case the start event never occurs. Because the possible events don’t always have
to come in the same order it is possible to provide a list of events. Which ever
comes first will then cause a transaction to be started.
For example: EVConnected, Authorized would mean that a transaction is started
when an EV is detected (Cable is connected), or when an EV Driver swipes his
RFID card en the CSMS successfully authorizes the ID for charging.

Defines when the Charging Station ends a transaction: last
transactioneventRequest: eventType = Ended.
When any event in the given list is no longer valid, the Charging Station SHALL
end the transaction. The Charging Station SHALL only send the Ended event once
for every transaction.

Maximum amount of energy in Wh delivered when an identifier is deauthorized by
the CSMS after start of a transaction.

whether the Charging Station will stop an ongoing transaction when it receives a
non- Accepted authorization status in TransactionEventResponse for this
transaction.

StopTxOnEVSideDisconnect

boolean

When set to true, the Charging Station SHALL administratively stop the
transaction when the cable is unplugged from the EV.

3.2. Physical Components

This is a non-exhaustive list of Standardized Physical Components that SHALL be used when mapping a real Charging Station to
the Device Model (for monitoring purposes).
When the physical component that is to be mapped, matches the description of one of the Standardized Components in this
section, for interoperability reasons, the Standardized Component from this section SHALL be used, instead of adding a new
(proprietary) component.
The list of typically used variables that is given for each Component is also non-exhaustive and all variables are optional. See also
Part 1, paragraph 4.5. If a description of a variable is empty, please refer to the description in Standardized Variables.

OCPP 2.1 - © Open Charge Alliance 2025

13/52

Part 2 - Appendices

Chapter 3. Standardized Components

v2.1, 2025-12-03

3.2.1. AccessBarrier

Description

Allows physical access of vehicles to a charging site to be controlled.

Typically used variables

Description

Enabled

Active

Problem

Open

3.2.2. AcDcConverter

Description

Provides a variable DC current source to force energy directly into an EV battery stack, under tight control of the EV’s battery
management system.

Typically used variables

Description

Enabled

Problem

Tripped

Overload

DCVoltage

DCCurrent

Power

Temperature

FanSpeed

(not commanded Out of Service)

some problem/fault exists

A problem requiring intervention has occurred

Excessive current/power consumption

measured DC voltage

measured DC current

measured power

temperature of converter

Speed of cooling fan(s)

3.2.3. AcPhaseSelector

Description

Allows a specific AC phase to be selected (typically at EVSE tier) for single phase vehicle charging in order to lower overall (e.g.
site) phase imbalance.

Typically used variables

Description

Changing

Enabled

Active

Problem

PhaseRotation

3.2.4. Actuator

Description

A general purpose electro-mechanical output system, with optional completion tracking sensing. Each output should use a
Variable instance key indicating the nature of the output.

Typically used variables

Description

Enabled

Active

Problem

State

Non-Default

3.2.5. AirCoolingSystem

Description

Fans (or equivalent devices) used to provide cooling.

OCPP 2.1 - © Open Charge Alliance 2025

14/52

Part 2 - Appendices

Chapter 3. Standardized Components

v2.1, 2025-12-03

Description

Typically used variables

Description

Enabled

Active

Problem

FanSpeed

Cooling system enabled to run

Cooling

fault: e.g. fan stalled/slow

Speed of cooling fan(s)

3.2.6. AreaVentilation

Description

Fans (or equivalent devices) used to ensure that EVs that require ventilation during charging

Typically used variables

Description

Enabled

Active

Problem

FanSpeed

Area ventilation enabled

Ventilating

fault: e.g. fan stalled/slow

Speed of cooling fan(s)

3.2.7. BayOccupancySensor

Description

Sensor (optical, ground loop, ultrasonic, etc.) to detect whether the associated parking/charging bay is physically vacant, or is
occupied by a vehicle or other obstruction

Typically used variables

Description

Enabled

Active

Percent

Sensor is sensing for occupancy

Occupied

percentage obstruction (for analogue sensors).

3.2.8. BeaconLighting

Description

Beacon Lighting to help EV drivers to locate nearby charging places, and/or to determine charging availability state, usually by
color variation.

Typically used variables

Description

Enabled

Enabled(Set)=0

Active

Problem

Percent

Beacon Lighting operational

Disable beacon lighting

On

Beacon lighting fault

Lighting Level (% of maximum)

Percent(Set)=x%

Lighting Level (% of maximum)

Power

Color

Lighting Wattage

Displayed color/intensity

3.2.9. CableBreakawaySensor

Description

A sensor that detects when a charging cable (captive or removable) has been forcibly pulled from the Charging Station.

Typically used variables

Description

Enabled

Active

Tripped

Breakaway sensor operational

Tripped

Breakaway detected: manual check/fix required

OCPP 2.1 - © Open Charge Alliance 2025

15/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 3. Standardized Components

3.2.10. CaseAccessSensor

Description

Reports when an access door/panel is open

Typically used variables

Description

Enabled

Enabled(Set)=0

Active

Tripped

Problem

Access sensor is enabled to detect/report opening/closing of access door/panel

Disable reporting of access

Open

An access door/panel that needs manual reset action has been activated

A fault exists in the Sensor mechanism itself

3.2.11. ChargingStation

Description

The entire Charging Station as a logical entity

Typically used variables

Description

Enabled

Problem

Tripped

Overload

Available for use (not commanded Out of Service)

Some problem/fault exists

A problem requiring local/manual intervention has occurred.

Excessive current/power consumption

SupplyPhases

Number of AC supply phases connected

SupplyPhases(MaxLimit)

Number of AC supply phases supported

PhaseRotation

ACVoltage

AC wiring phase rotation

Measured incoming AC voltage [per phase]

ACVoltage(MaxLimit)

Designed maximum operating AC voltage

ACCurrent

Power

Power(MaxLimit)

VoltageImbalance

CurrentImbalance

VendorName

Model

ECVariant

SerialNumber

OperatingTimes

ChargeProtocol

AvailabilityState

Measured total AC current [per phase]

Measured/calculated total power being consumed, including standby/ancilliary loads

Designed total operating load power, including standby/ancilliary loads

voltage imbalance in three phase supply

current imbalance in three phase supply

Charging Station vendor name (as reported in BootNotification)

Charging Station model (as reported in BootNotification)

Engineering Change Variant

Charging Station serial number

recurring operating times

Charging Control Protocol applicable to the Charging Station

Indicates if the Charging Station is available or not (replaces the Charging Station Status
values reported by the StatusNotification)

AllowNewSessionsPendingFirmware
Update

Indicates whether new sessions can be started on EVSEs, while Charging Station is waiting
for all EVSEs to become Available in order to start a pending firmware update.

3.2.12. ChargingStatusIndicator

Description

The Charging Status Indicator, provides visible feedback to the user about the connection and charging status of an
EVSE/Connector. This is commonly in the form of multi-colored lighting.

Typically used variables

Active

Color

Description

Lighted

Displayed color

OCPP 2.1 - © Open Charge Alliance 2025

16/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 3. Standardized Components

3.2.13. ConnectedEV (Updated in v2.0)

Required

Component

Description

no

compone
ntName

ConnectedEV

evse

* (EVSE to which EV is connected)

ConnectedEV is a component that represents a connected vehicle for which data is received via
an ISO 15118 or CHAdeMO interface. The information that is received, is represented as variables
of ConnectedEV.

NOTE

In below tables the notation "<variable>(type)" refers to the attributeType <type> of the variable. For example,
"ACCurrent(MinSet)" refers to value of the attributeType MinSet of the variable ACCurrent. "DCCurrent(Target)
refers to the value of the attributeType Target of the variable DCCurrent.

ConnectedEV for ISO 15118

Variable

Available

Unit

boolean

ISO 15118-2 value

ISO 15118-20 value

When true this means an EV is connected.
When false, all other fields are meaningless and should be empty strings.

Protocol and static vehicle information

VehicleId

string

EVCCID (from SessionSetupReq)
Six bytes, represented as hexbinary encoded
string, e.g. "010203040A0B", containing the
EVCC MAC address.

EVCCID (from SesstionSetupReq)
Max 255 bytes with the first 3 bytes
containing the WMI as defined in ISO
3780:2009.

VehicleCertificate["Le
af"]

string

VehicleCertificate["Su
bCA1"]

string

n/a

n/a

VehicleCertificate["Su
bCA2"]

string

n/a

VehicleCertificate["Ro
ot"]

ProtocolAgreed

string

n/a

multi-instance
string

string

ProtocolSupportedBy
EV["1"]
to
ProtocolSupportedBy
EV["20"]

A string with the following comma-separated
items:
“<uri>,<major>,<minor>”.
This is the protocol uri and version
information that was agreed upon between
EV and EVSE in the
supportedAppProtocolReq handshake from
ISO 15118.
Example for ISO 15118-2:
"urn:iso:15118:2:2013:MsgDef,1,0"
(1,0 = version 1.0)
For DIN SPEC 70121:
"urn:din:70121:2012:MsgDef"

A string with the following comma-separated
items:
“<uri>,<major>,<minor>”.
This is information from the
SupportedAppProtocolReq message from
ISO 15118
This variable has 20 instances; one for each
priority number. Priority is a number from 1
to 20 as a string, i.e. "1", "2", etc.
Example:
- ConnectedEV.ProtocolSupportedByEV["1"] =
"urn:iso:15118:2:2013:MsgDef,2,0"
- ConnectedEV.ProtocolSupportedByEV["2"] =
"urn:iso:15118:2:2010:MsgDef,1,0"

The PEM encoded X.509 Leaf certificate of
the vehicle certificate chain

The PEM encoded X.509 intermediate
SubCA1 certificate when present in the
vehicle certificate chain

The PEM encoded X.509 intermediate
SubCA2 certificate, when present in the
vehicle certificate chain

Optional. The PEM encoded X.509 Root
certificate of the vehicle certificate chain

Same definition.
Example for ISO 15118-20:
"urn:iso:std:iso:15118:-20:DC,1,0"
(or AC, ACDP, WPT, etc.)

Same definition.
Example for ISO 15118-20:
"urn:iso:std:iso:15118:-20:DC,1,0"
(or AC, ACDP, WPT, etc.)

OCPP 2.1 - © Open Charge Alliance 2025

17/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 3. Standardized Components

Variable

Unit

ISO 15118-2 value

ISO 15118-20 value

Voltage and current values: (from ChargeParameterDiscoveryReq unless stated otherwise)

ACCurrent(MinSet)

ACCurrent(MaxSet)

ACVoltage(MaxSet)

DCCurrent(MinSet)

DCCurrent(MaxSet)

DCCurrent(Target)

DCVoltage(MinSet)

DCVoltage(MaxSet)

DCVoltage(Target)

A

A

V

A

A

A

V

V

V

Power, energy and time values:

Power(MaxSet)

DischargePower(Max
Set)

DischargePower

W

W

W

EVMinCurrent

EVMaxCurrent

EVMaxVoltage

-

EVMaximumCurrentLimit

-

-

-

EVMinimumChargeCurrent

EVMaximumChargeCurrent

EVTargetCurrent (CurrentDemandReq)

EVTargetCurrent (DC_ChargeLoopReq)

-

EVMaximumVoltageLimit

EVMinimumVoltage

EVMaximumVoltage

EVTargetVoltage (CurrentDemandReq)

EVTargetVoltage (DC_ChargeLoopReq)

EVMaximumPowerLimit (DC)

-

-

EnergyImport(MaxSet
)

Wh

EVEnergyCapacity (DC)

EnergyImport(MinSet) Wh

-

EnergyImport(Target) Wh

EVEnergyRequest (DC)
EAmount (AC)

BatteryCapacity

Wh

EVEnergyCapacity (DC)

DepartureTime

dateTime

DepartureTime
Provided as seconds since message receipt.
Converted to absolute time.

EVMaximumChargePower (AC/DC)
All phases combined.

EVMaximumDischargePower (AC/DC)
Rated maximum, all phases combined (Actual
discharging power is reported via EVSE)

EVMaximumDischargePower (AC/DC) (BPT
ChargeLoopReq)
Actual maximum, all phases combined

EVMaximumEnergyRequest
(ScheduleExchangeReq,
AC/DC_ChargeLoopReq)

EVMinimumEnergyRequest
(ScheduleExchangeReq,
AC/DC_ChargeLoopReq)

EVTargetEnergyRequest
(ScheduleExchangeReq,
AC/DC_ChargeLoopReq)

BatteryCapacity
(AC/DC_ChargeLoopReq.DisplayParameters)

DepartureTime (ScheduleExchangeReq,
AC/DC_ChargeLoopReq)
Provided as seconds since message receipt.
Converted to absolute time.

RemainingTimeBulk

RemainingTimeFull(M
axSet)

RemainingTimeFull

StateOfChargeBulk

StateOfCharge(MaxSe
t)

StateOfCharge

ChargingCompleteBul
k

s

s

s

%

%

%

RemainingTimeToBulkSoC
(CurrentDemandReq)

-

-

-

RemainingTimeToFullSoc
(CurrentDemandReq)

RemainingTimeToMaximumSOC
(AC/DC_ChargeLoopReq.DisplayParameters)

BulkSoC

FullSoC

DC_EVStatus.EVRESSSOC

-

MaximumSOC
(AC/DC_ChargeLoopReq.DisplayParameters)

PresentSOC
(AC/DC_ChargeLoopReq.DisplayParameters)

boolean

BulkChargingComplete

-

ChargingCompleteFull boolean

ChargingComplete

ChargingComplete
(AC/DC_ChargeLoopReq.DisplayParameters)

Error status values:

ChargingState
with a memberlist consisting of the following values:

* BatteryOvervoltage

* BatteryUndervoltage

-

-

* ChargingCurrentDeviation

FAILED_ChargingCurrentDifferential

* BatteryTemperature

FAILED_RESSTemperatureInhibit

<EVTerminationCode>

…

…

…

OCPP 2.1 - © Open Charge Alliance 2025

18/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 3. Standardized Components

Variable

Unit

ISO 15118-2 value

ISO 15118-20 value

* VoltageDeviation

FAILED_ChargingVoltageOutOfRange

* ChargingSystemError

FAILED_EVRESSMalfunction

* VehicleShiftPosition

FAILED_EVShiftPosition

* VehicleChargingEnabled

-

* ChargingSystemIncompatibility

FAILED_ChargingSystemIncompatibility

* ChargerConnectorLockFault

FAILED_ChargerConnectorLockFault

…

…

…

…

…

…

ConnectedEV for CHAdeMO

Variable

Available

Unit

CHAdeMO value

boolean

Is true when an EV is connected

Protocol and static vehicle information

VehicleId

ProtocolAgreed

string

string

Vehicle ID (H'710 + H'711 + H'712)
Three times 8 bytes, represented as hexbinary encoded string, e.g.
"010203040A0B0C0D111213141A1B1C1D212223242A2B2C2D". A
concatenation of H'710 + H'711 + H'712.

Lowest of Chademo protocol number from EV (H'102.0) and charger
(H'109.0)
Example for CHAdeMO 2.0.1: "CHAdeMO,3"
(3 = version reported by CHAdeMO for v2.0.1)

ProtocolSupportedByEV["1"]

string

Chademo protocol number (H'102.0) supported by EV
A string with the following comma-separated items:
“CHAdeMO,<version>”.
Example for CHAdeMO 2.0.1:
- ConnectedEV.ProtocolSupportedByEV = "CHAdeMO,3"

Voltage and current values:

ACCurrent(MinSet)

ACCurrent(MaxSet)

ACVoltage(MaxSet)

DCCurrent(MinSet)

DCCurrent(MaxSet)

DCCurrent(Target)

DCVoltage(MinSet)

DCVoltage(MaxSet)

DCVoltage(Target)

Power, energy and time values:

Power(MaxSet)

EnergyImport(MaxSet)

EnergyImport(Target)

DepartureTime

RemainingTimeBulk

RemainingTimeFull(MaxSet)

RemainingTimeFull

StateOfChargeBulk

StateOfCharge(MaxSet)

StateOfCharge

ChargingCompleteBulk

ChargingCompleteFull

Error status values:

A

A

V

A

A

A

V

V

V

W

Wh

Wh

dateTime

s

s

s

%

%

%

-

-

-

Minimum charge current (H'100.0)

-

Charging current request (H'102.3)
If HighCurrentControl is true, use the value from Charging current
request (extended) (H'110.1,2).

Minimum battery voltage (H'100.2,3)

Maximum battery voltage (H'100.4,5)

Target battery voltage (H'102.1,2)

-

Total capacity of traction battery * 100 (H'101.5,6)

-

-

-

Maximum charging time * 60 (H'101.2)

Estimated charging time * 60 (H'101.3)

-

Charged rate reference constant (H'100.6)

State of charge (H'102.6)

boolean

boolean

-

-

ChargingState
with a memberlist consisting of the following values:

OCPP 2.1 - © Open Charge Alliance 2025

19/52

Part 2 - Appendices

v2.1, 2025-12-03

Variable

* BatteryOvervoltage

* BatteryUndervoltage

* ChargingCurrentDeviation

* BatteryTemperature

* VoltageDeviation

* ChargingSystemError

* VehicleShiftPosition

* VehicleChargingEnabled

* ChargingSystemIncompatibility

* ChargerConnectorLockFault

ConnectedEV for WPT

Unit

CHAdeMO value

Chapter 3. Standardized Components

Battery overvoltage (H'102.4.0)

Battery undervoltage (H'102.4.1)

Battery current deviation (H'102.4.2)

High battery temperature (H'102.4.3)

Battery voltage deviation (H'102.4.4)

Charging system error (H'102.5.2)

Vehicle shift position (H'102.5.1)

Vehicle charging enabled (H'102.5.0)

-

-

The following parameters coming from the WPT_ChargeLoopReq messages can be represented in the ConnectedEV component.
This information might be useful for troubleshooting, but is not required for normal operation.

Variable

WPT_EVPCPowerRequest

WPT_EVPCPowerOutput

Unit

decimal

decimal

WPT_EVPCChargeDiagnostics

OptionList

ISO 15118-20 value

Power the EVPC would like to have as output in Watt.

Power measured at the output of the EVPC electronics in Watt.

Values are:
"EVPCNoIssue",
"EVPCTempOverheatDetected",
"EVPCPowerTransferAnomalyDetected",
"EVPCAnomalyDetected"

WPT_EVPCOperatingFrequency

decimal

EVPC measured MF-WPT operating frequency.

WPT_EVPCCoilCurrentRequest

decimal

EVPC wants the primary device to set a specific (preferred) coil current
value.

WPT_EVPCCoilCurrentInformation decimal

Secondary device coil current (AC).

WPT_EVPCCurrentOutputInformati
on

decimal

WPT_EVPCVoltageOutputInformati
on

decimal

DC current supplied to the EV.

DC bus or battery voltage.

ConnectedEV for ACDP

The following parameters from ACDP_VehiclePositioningReq, ACPD_ConnectReq, ACDP_DisconnectReq and
ACDP_SystemStatusReq can be represented in the ConnectedEV component. This information might be useful for troubleshooting,
but is not required for normal operation.

Variable

ACDP_EVMobilityStatus

ACDP_EVPositioningSupport

Unit

boolean

boolean

ACDP_EVElectricalChargingDevice
Status

OptionList

ISO 15118-20 value

Is true when an EV is immobilized

Is true when EV has positioning support

Values are:
"State_A" (disconnected),
"State_B", "State_C", "State_D" (connected)

ACDP_EVReadyToCharge

boolean

Element signalizes if the EV is READY or NOT READY to charge.

ACDP_EVImmobilizationRequest

boolean

ACDP_EVImmobilized

boolean

ACDP_WLANStrength

ACDP_EVCPStatus

decimal

OptionList

Represents the request of immobilization of the EV. This may be
related to the hand brake status.

The immobilization of the EV is a mandatory precondition to activate
the pantograph.

Element signalizes EV WiFi reception signal strength (-dBm)

Values are:
"State_A", "State_B", "State_C", "State_D", "State_E".
Refer to IEC 61851-23-1.

OCPP 2.1 - © Open Charge Alliance 2025

20/52

Part 2 - Appendices

v2.1, 2025-12-03

Variable

ACDP_EVSOC

Unit

decimal

ACDP_EVErrorCode

OptionList

Chapter 3. Standardized Components

ISO 15118-20 value

For operation usable SOC status in %. This value may differ from
physical SOC of the battery.
This parameter can be same as PresentSOC in DisplayParameter.

One of:
"OK_NoEVError", "FAILED", "FAILED_EmergencyEvent",
"FAILED_Breaker", "FAILED_RESSTemperatureInhibit", "FAILED_RESS",
"FAILED_ChargingCurrentDifferential",
"FAILED_ChargingVoltageOutOfRange", "FAILED_Reserved1",
"FAILED_Reserved2"

ACDP_EVTimeout

boolean

Indicates the occurrence of a timeout in the EVCC.

3.2.14. Connector

Description

A means to connect an EV to a Charging Station with either a socket, an attached cable & inline connector, or any wireless power
transfer device.

Typically used variables

Description

Enabled

Problem

Tripped

ConnectorType

Connector available for use (not commanded Out of Service)

problem/fault exists (e.g. over-temperature)

A problem requiring intervention has occurred.

A value of ConnectorStringEnumType (See Appendix 7). Specific type of connector, including
sub-variant information. Note: Distinct and orthogonal to Charging Protocol, Power Type,
Phases.

SupplyPhases

AC phases connected

SupplyPhases(MaxLimit)

AC phases Max

PhaseRotation

ChargeProtocol

AvailabilityState

AC wiring phase rotation

Charging Control Protocol applicable to the Connector

Indicates if the Connector is available or not (replaces the Status values reported by the
StatusNotification)

3.2.15. ConnectorHolsterRelease

Description

A mechanism present in a connector holster to prevent the connector from being removed inappropriately: typically unlocks
connector after authorization.

Typically used variables

Description

Enabled

Active

Problem

State

Unlocked for removal/return

3.2.16. ConnectorHolsterSensor

Description

A mechanism to report when a tethered cable connector has been removed from its normal stowage position. May be used for
detection of connectors left un-holstered, and possible penalty billing.

Typically used variables

Description

Enabled

Active

Problem

Un-Holstered

OCPP 2.1 - © Open Charge Alliance 2025

21/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 3. Standardized Components

3.2.17. ConnectorPlugRetentionLock

Description

Locking mechanism to retain an inserted plug, both to prevent on-load disconnection, and to prevent theft of charging cables

Typically used variables

Description

Enabled

Active

Problem

Tripped

Tries

Tries(SetLimit)

Tries(MaxLimit)

Retention mechanism enabled

Locked

Locking Failed

Stall protection fuse blown, etc.

(Re)tries taken on last attempt

Configured auto retry count

Maximum auto retry count

3.2.18. ConnectorProtectionRelease

Description

External protective mechanism (e.g. an external shutter or a connector holster lock mechanism) to prevent contact with
conductors that may become "live" under other failure modes

Typically used variables

Description

Enabled

Active

Problem

Tripped

Protection in effect (locked except when in use)

Unlocked

Lock/Unlock mechanism fault

protective mechanism triggered (fuse)

3.2.19. Controller

Description

An embedded logic controller

Typically used variables

Active

Problem

Interval[Heartbeat]

Manufacturer

Model

ECVariant

SerialNumber

VersionNumber

VersionDate

FirmwareVersion

MaxMsgElements

SelftestActive(Set)

SelftestActive

Description

Running

Controller fault

Heartbeat interval

Controller manufacturer name

Controller model number

Engineering Change variant

Controller hardware serial number

Hardware version number

Hardware version date

Firmware version number (as reported in BootNotification)

Array of implementation-defined limits to the number of elements of specific type that the
Charging Station can accept in one message.

Start self-test

Self-test running

3.2.20. ControlMetering

Description

Energy, Power, Electricity meter, used to measure energy, current, voltages etc.

Typically used variables

Description

Power

ACCurrent

Measured power

Measured AC current [per phase]

OCPP 2.1 - © Open Charge Alliance 2025

22/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

DCCurrent

DCVoltage

Measured DC current

Measured DC voltage

3.2.21. CPPWMController

Description

Chapter 3. Standardized Components

Control Pilot PWM Controller: provides and senses the IEC 61851-1 / SAE J1772 low voltage DC and PWM signalling between an
EVSE and EV over a control pilot line.

Typically used variables

Description

Enabled

Active

Problem

DCVoltage

State

Percentage

SelftestActive(Set)

SelftestActive

3.2.22. DataLink

Description

Connected

CP PWM controller fault

Control Pilot wire DC Voltage (0-12V)

IEC 61851-1 states ("A" to "E")

1kHz Duty Cycle

Start self-test

Self-test running

Provides a communications link from a Charging Station to a CSMS. It may use fixed infrastructure, mobile telephony data
services, WiFi, or other connectivity channels.

Typically used variables

Description

Enabled

Active

Fallback

Complete

Problem

IMSI

ICCID

Data link enabled

Connected

Using Backup SIM/Network Preference

Link connection terminated

Communications module or link connection fault

International Mobile Subscriber Identity number of mobile data SIM card

Integrated Circuit Card IDentifier of mobile data SIM card.

NetworkAddress

SignalStrength

Current network address

Data signal strength/quality

3.2.23. Display

Description

Provides information and feedback to the user.

Typically used variables

Description

Enabled

Problem

Color

Count[HeightInChars]

Count[WidthInChars]

DataText[Visible]

State

Display configured to show information

Display fault

Display color (monochrome/backlighting)

Display height (characters)

Display width (characters)

Current Display Contents

Alphanumeric code indicating current message purpose

3.2.24. ElectricalFeed

OCPP 2.1 - © Open Charge Alliance 2025

23/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

Chapter 3. Standardized Components

Represents an incoming electrical connection to a Charging Station, that may be a grid/distribution network connection, of a
connection to local power generation and/or storage. Each electrical feed can record the electrical and other characteristics of
that feed, including power rating, fusing, upstream metering, etc. When a Charging Station has more than one electrical feed, it
must represent which feed supplies each EVSE, and which feed supplies the house load of the Charging Station itself. Simple
Charging Stations with only a single electrical feed may omit all electrical feed information, in which case it is inferred that all
power is supplied from a single feed, and what would otherwise be ElectricalFeed data (Variables) may be reported as being
associated with the ChargingStation component.

Typically used variables

Description

Connected

Enabled

Active

Problem

PowerType

Power

Energy

DCVoltage

SupplyPhases

PhaseRotation

ACVoltage

3.2.25. ELVSupply

Description

Represents the low voltage power supply (typically 12V DC and often other ELV voltages) that provides operating power for
controllers, relays, and other electrical components.

Typically used variables

Description

EnergyImportRegister

Standby/house energy meter register reading

Power

instantaneous standby power consumption

Power(MaxLimit)

Design maximum standby power consumption

Fallback

Running on backup energy;

Fallback(MaxLimit): =1

has backup

StateOfCharge

Time

backup battery SOC

(estimated) operating time on backup energy

3.2.26. EmergencyStopSensor

Description

An "Emergency Stop" button that should be pressed by the user or other nearby persons if serious faulty behavior is observed (e.g.
smoke/flames from EV or Charging Station).

Typically used variables

Description

Enabled

Active

Tripped

Emergency Stop action armed

Pressed/Latched

Needs manual reset

3.2.27. EnvironmentalLighting

Description

Provides reporting/control of general illumination lighting in use at Charging Station.

Typically used variables

Description

Enabled

Enabled(Set)=0

Active

Problem

Environmental Lighting operational

Disable Environmental lighting

On

Environmental lighting fault

OCPP 2.1 - © Open Charge Alliance 2025

24/52

Part 2 - Appendices

Chapter 3. Standardized Components

v2.1, 2025-12-03

Description

Percent

Lighting Level (% of maximum)

Percent(Set)=x%

Lighting Level (% of maximum)

Power

Color

Lighting Wattage

Displayed color/intensity

3.2.28. EVRetentionLock

Description

A locking mechanism on the EV side as a safety measure to prevent it being disconnected while high currents are flowing.

Typically used variables

Description

Retention locking detection in effect

Locked to EV

Has unlocked

Lock Problem (e.g. failed to lock/unlock)

Enabled

Active

Complete

Problem

3.2.29. EVSE

Description

The entire chain of components responsible for transporting energy from the incoming supply to the electric vehicle (or vice versa)

Typically used variables

Description

Enabled

Problem

Tripped

Overload

SupplyPhases

PhaseRotation

AllowReset

ACVoltage

ACCurrent

DCVoltage

DCCurrent

Power

VoltageImbalance

CurrentImbalance

ChargeProtocol

ChargingTime

Ready for use (not commanded Out of Service)

some problem/fault exists

A problem requiring intervention has occurred

Excessive current/power consumption

AC phases connected

AC wiring phase rotation

When true: EVSE can be reset individually

Measured total AC voltage [per phase]

Measured total AC current [per phase]

Measured total DC voltage [per phase]

Measured total DC current [per phase]

Measured Power

voltage imbalance in three phase supply

current imbalance in three phase supply

Charging Control Protocol applicable to the EVSE

Total time duration that EV is taking energy from an EVSE. Short pauses in charging (e.g.
battery pre-, post-conditioning) are included

PostChargingTime

Total time since EV has taken energy from EVSE

Count[ChargingProfiles]

Charging Profiles present

Count[ChargingProfiles](MaxLimit) Maximum Charging Profiles supported

ISO15118EvseId

The name of the EVSE in the string format as required by ISO 15118 and IEC 63119-2.
Example: "DE*ICE*E*1234567890*1"

3.2.30. ExternalTemperatureSensor

Description

Reports ambient air temperature

Typically used variables

Description

Active

Problem

Temperature above MaxSet or MinSet

Temperature sensor fault

OCPP 2.1 - © Open Charge Alliance 2025

25/52

Part 2 - Appendices

Chapter 3. Standardized Components

v2.1, 2025-12-03

Description

Temperature

3.2.31. FiscalMetering

Description

Ambient temperature

Provides energy transfer readings that are the basis for billing.

Typically used variables

Description

Problem

EnergyImport

Metering Fault (e.g. read error)

Energy transferred to EV during session

EnergyImportRegister

Cumulative import reading

EnergyExport

EnergyExportRegister

Manufacturer[Meter]

Manufacturer[CT]

Model[Meter]

Model[CT]

ECVariant

SerialNumber[Meter]

SerialNumber[CT]

Certificate

Energy transferred from EV during session

Cumulative export reading

Meter manufacturer name

Current transformer manufacturer name

Meter model number

CT model number

Meter engineering change variant

Meter serial number

CT serial number(s)

OptionsSet [MeterValueAlignedData] Set of measurands to read and report at clock-aligned time intervals while charging.

OptionsSet
[TxnStoppedAlignedData]

Set of measurands to be read at clock-aligned time intervals while charging and reported in
TransactionStopped

3.2.32. FloodSensor

Description

A sensor reporting whether the Charging Station is experiencing water ingress/pooling.

Typically used variables

Description

Enabled

Active

Tripped

Height

Percent

Water presence/level sensing in effect

Flooding

Water level safety sensor tripped

Absolute water height above reference (ground) level.

Height as percentage between reference minimum (0%) and maximum allowable (100%).
Values below 0% and above 100% are possible.

3.2.33. GroundIsolationProtection

Description

An Isolation Tester as part of their own self-test mechanisms, to confirm the isolation of floating circuitry when no Evs are
connected

Typically used variables

Description

Enabled

Active

Complete

Problem

Impedance

Electrical isolation testing enabled

Leakage

Isolation test completed

Isolation fault

Isolation resistance/impedance

OCPP 2.1 - © Open Charge Alliance 2025

26/52

Part 2 - Appendices

Chapter 3. Standardized Components

v2.1, 2025-12-03

3.2.34. Heater

Description

Heater to ensure reliable operation in cold environments

Typically used variables

Description

Enabled

Active

Problem

Tripped

Power

Power(MaxLimit)

Power(MaxSet)

Temperature(MinSet)

Temperature(MaxSet)

Heater hardware operation enabled

Heating

Heater fault

Heater equipment permanent fault

Instantaneous heater power level

Maximum heater power

Configured heater power

Cut-in temperature

Cut-out temperature

3.2.35. HumiditySensor

Description

Reports relative air humidity

Typically used variables

Description

Enabled

Problem

Humidity

Humidity sensor fault

RH(%)

3.2.36. LightSensor

Description

Reports ambient light levels.

Typically used variables

Description

Enabled

Problem

Light

Lighting sensor fault

The ambient light level

3.2.37. LiquidCoolingSystem

Description

A liquid based cooling system, typically used to cool the connector cables of very high power Charging Stations.

Typically used variables

Description

Enabled

Active

Problem

Temperature

Cooling system enabled to run

Liquid circulating

3.2.38. LocalAvailabilitySensor

Description

Accepts local signal inputs controlling whether new Charging Sessions can start and/or whether ongoing sessions should
continue. Typically connected to a site/building power supply, to automatically report unavailability when closed.

Typically used variables

Description

Enabled

Active

Local Availability input sensing in operation

Out of Service

OCPP 2.1 - © Open Charge Alliance 2025

27/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

Problem

Local Availability sensing circuit error

Chapter 3. Standardized Components

3.2.39. LocalController

Description

The entire Local Controller as a logical entity

Common Variables

Description

Enabled

Problem

Identity

Tripped

Manufacturer

Model

ECVariant

SerialNumber

ChargingStation

Available for use (not commanded Out of Service)

Some problem/fault exists

Local Controller identity

A problem requiring local/manual intervention has occurred.

Local Controller manufacturer name

Local Controller manufacturer model

Engineering Change Variant

Local Controller serial number

List of Charging Stations Identities connected to this LocalController. (not to be
confused with the ChargingStation Component)

3.2.40. LocalEnergyStorage (updated in v1.3)

Description

Local energy storage device

Typically used variables

Description

EnergyCapacity

Identity

Maximum storage capacity

Local Energy Storage identity

3.2.41. OverCurrentProtection

Description

Protects equipment by disconnecting the electrical supply when the current drawn (on any phase) exceeds the rated value to a
substantial degree.

Typically used variables

Description

Active

Operated

ACCurrent

Tripped. Trip when over MaxSet/MaxLimit.

Breaker opened and auto-reclosed

Measured total AC current [per phase]

3.2.42. OverCurrentProtectionRecloser

Description

Recloser mechanism of an OverCurrentProtection to perform re-arm retries after a trip, or may be set for remotely controlled re-
arming on command.

Typically used variables

Description

Enabled

Active

Active(Set)

Complete

Problem

Mode

Tries

Auto reclosing enabled

Reclosing

Initiate manual reclose

Reclose cycle completed

Recloser Fault

Reclose Mode (None, Auto, Commanded)

(Re)tries taken on last attempt

Tries(SetLimit)

Configured auto retry count

OCPP 2.1 - © Open Charge Alliance 2025

28/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

Tries(MaxLimit)

3.2.43. PowerContactor

Description

Maximum auto retry count

Chapter 3. Standardized Components

Switches on and off the power to the EV after all authorization and safety requirements have been met. May have secondary
contacts to report closure state.

Typically used variables

Active

Tripped

Problem

Description

Closed

Mirror contact protection tripped

Close/Open failed

3.2.44. RCD

Description

A Residual Current Device (US: ground fault breaker) protects human life and/or downstream equipment by quickly detecting
abnormal current flows (usually indicative in earth faults) in the Charging Station, cable, or EV during charging.

Typically used variables

Description

Tripped

Operated

Breaker opened (manual reset required)

Breaker opened and auto-reclosed

3.2.45. RCDRecloser

Description

A motorized recloser mechanism of an RCD that may be configured to perform re-arm retries after a trip, or may be set for
remotely controlled re-arming on command.

Typically used variables

Description

Enabled

Active

Active(Set)

Complete

Problem

Tries

Tries(SetLimit)

Tries(MaxLimit)

Auto reclosing enabled

Reclosing in progress

Initiate manual reclose

Reclose cycle completed

Recloser Fault

(Re)tries taken on last attempt

Configured auto (re)try count

Maximum auto (re)try count

3.2.46. RealTimeClock

Description

Represents realtime clock hardware that can maintain accurate date & time information in a Charging Station, even in the case of
simultaneous CSMS uncontactability and power outages or resets.

Typically used variables

Description

Active

DCVoltage

Fallback

RTC running OK

Battery voltage

Battery failing

Fallback(MaxLimit)

RTC has backup-power. MaxLimit = 1

Problem

RTC fault

OCPP 2.1 - © Open Charge Alliance 2025

29/52

Part 2 - Appendices

Chapter 3. Standardized Components

v2.1, 2025-12-03

3.2.47. ShockSensor

Description

Measures impact forces/accelerations experienced, indicative of possible damage.

Typically used variables

Description

Enabled

Active

Force

Shock sensing enabled

Shock

detected force (vector)

3.2.48. SpacesCountSignage

Description

Electronic signage allowing a charging controller for a large charging facility to advertise counts of available spaces to passing
traffic.

Typically used variables

Description

Spaces count signage enabled

Not Blank

Enabled

Active

Count

3.2.49. Switch

Description

A general purpose electromechanical input device, with optional remote defaulting/resetting of values. Each input should use a
Variable instance key indicating the nature of the input.

Typically used variables

Description

Enabled

Active

State

Non-Default

3.2.50. TemperatureSensor

Description

Temperature sensor at a point inside the Charging Station; multiple sensing points for a single sensing controller. Multiple sensing
points for a single sensing controller may be reported using distinct Variable instance keys.

Typically used variables

Description

Active

Problem

Temperature

High temperature (over MaxSet)

Internal temperature sensor fault

Enclosure temperature

3.2.51. TiltSensor

Description

Measures Tilt angle from normal reference position (normally 90 degree vertical).

Typically used variables

Description

Enabled

Active

Angle

Tilt sensing enabled

Tilted

Measured tilt (vector) from vertical

3.2.52. TokenReader

OCPP 2.1 - © Open Charge Alliance 2025

30/52

Part 2 - Appendices

v2.1, 2025-12-03

Description

An authorization token reader (e.g. RFID)

Typically used variables

Description

Chapter 3. Standardized Components

Enabled

Enabled(Set)=0

Operated

Problem

Token

TokenType

Token reader enabled

Token reader disabled: allow charging without token authentication/authorization

token data read event

token reader fault

String read by TokenReader

Type of Token. Value is one of IdTokenEnumStringType (See Appendix 7).

3.2.53. UpstreamProtectionTrigger

Description

Circuitry designed to trigger the disconnection of power to the structure by an upstream protection device after a severe problem
has been detected

Typically used variables

Description

Upstream protection enabled

Force triggering of upstream protection

Upstream protection triggered

Upstream protection fault

Enabled

Active(Set)

Tripped

Problem

3.2.54. UIInput

Description

A logical input mechanism (e.g. set of buttons) that is part of a UI whose use may be communicated to the CSMS (in near real
time). May support momentary inputs ("Operated") or modal state ("Active"). Multiple input sources should use explicit Variable
instance keys (where the input function is key name).

Typically used variables

Description

Enabled

Operated

Active

UI input enabled

3.2.55. VehicleIdSensor

Description

Reports an identifier associated with a vehicle occupying a charging bay. The identifier may be a vehicle registration number via
ANPR hardware, a VIN, or other local identifier of the vehicle based on medium range/active RFID, or any other relevant technology
and result.

Typically used variables

Description

Enabled

Active

VehicleIdSensor enabled

Processing

3.3. Summary List of Standardized Components

Following is a list that sums up all above-mentioned standardized component names.

Component

Description

ACDERCtrlr

Responsible for configuration relating to DER capabilities that the EVSE of the Charging Station can emulate by
using ISO 15118-20 ChargeLoop messages to control the inverter in the EV. The component is located at the
EVSE level, since it represents the DER capabilities of the EVSE.

AlignedDataCtrlr Logical Component responsible for configuration relating to the reporting of clock-aligned meter data.

AuthCacheCtrlr

Logical Component responsible for configuration relating to the use of a local cache for authorization for
Charging Station use.

OCPP 2.1 - © Open Charge Alliance 2025

31/52

Part 2 - Appendices

v2.1, 2025-12-03

Component

Description

Chapter 3. Standardized Components

AuthCtrlr

Logical Component responsible for configuration relating to the use of authorization for Charging Station use.

BatterySwapCtrlr Responsible for configuration relating to Battery swapping.

CHAdeMOCtrlr

A CHAdeMO Controller component communicates with an EV using the wired CANbus protocol to exchange
information and control charging using the CHAdeMO protocol

ClockCtrlr

Provides a means to configure management of time tracking by Charging Station.

CustomizationCtr
lr

Responsible for configuration relating to custom vendor-specific implementations, like the DataTransfer
message and CustomData extensions or CustomTriggers.

DCDERCtrlr

Responsible for configuration relating to DER capabilities of the DC inverter of the EVSE in the Charging Station.
The component is located at the EVSE level, since it represents the DER capabilities, also referred to as
nameplate information, of the EVSE.

DeviceDataCtrlr

Logical Component responsible for configuration relating to the exchange and storage of Charging Station
Device Model data.

DisplayMessageC
trlr

Logical Component responsible for configuration relating to the display of messages to Charging Station users.

ISO15118Ctrlr

Communicates with an EV to exchange information and control charging using the ISO 15118 protocol.

LocalAuthListCtrl
r

Logical Component responsible for configuration relating to the use of Local Authorization Lists for Charging
Station use.

MonitoringCtrlr

Logical Component responsible for configuration relating to the exchange of monitoring event data.

PaymentCtrlr

Logical Component responsible for configuration relating to payment terminals.

OCPPCommCtrlr Logical Component responsible for configuration relating to information exchange between Charging Station

and CSMS.

ReservationCtrlr

Logical Component responsible for configuration relating to reservations.

SampledDataCtrlr Logical Component responsible for configuration relating to the reporting of sampled meter data.

SecurityCtrlr

Logical Component responsible for configuration relating to security of communications between Charging
Station and CSMS.

SmartChargingCt
rlr

Logical Component responsible for configuration relating to smart charging.

TariffCostCtrlr

Logical Component responsible for configuration relating to tariff and cost display.

TxCtrlr

Logical Component responsible for configuration relating to transaction characteristics and behaviour.

V2XChargingCtrlr Responsible for configuration relating to V2X charging/discharging. This component exists on the EVSE tier

hierarchy.

WebPaymentsCtr
lr

Responsible for configuration of a dynamic QR code for ad hoc payments.

AccessBarrier

Allows physical access of vehicles to a charging site to be controlled.

AcDcConverter

Provides a variable DC current source to force energy directly into an EV battery stack, under tight control of the
EV’s battery management system.

AcPhaseSelector Allows a specific AC phase to be selected (typically at EVSE tier) for single phase vehicle charging in order to

lower overall (e.g. site) phase imbalance.

Actuator

A general purpose electro-mechanical output system, with optional completion tracking sensing. Each output
should use a Variable instance key indicating the nature of the output.

AirCoolingSystem Fans (or equivalent devices) used to provide cooling.

AreaVentilation

Fans (or equivalent devices) used to ensure that EVs that require ventilation during charging

BatteryCartridge BatteryCartridge represents the battery cartridge that is currently inserted into the EVSE of a battery swap

station

BayOccupancySe
nsor

Sensor (optical, ground loop, ultrasonic, etc.) to detect whether the associated parking/charging bay is
physically vacant, or is occupied by a vehicle or other obstruction

BeaconLighting

Beacon Lighting to help EV drivers to locate nearby charging places, and/or to determine charging availability
state, usually by color variation.

CableBreakawayS
ensor

A sensor that detects when a charging cable (captive or removable) has been forcibly pulled from the Charging
Station.

CaseAccessSens
or

Reports when an access door/panel is open

ChargingStation The entire Charging Station as a logical entity

ChargingStatusIn
dicator

The Charging Status Indicator, provides visible feedback to the user about the connection and charging status
of an EVSE/Connector. This is commonly in the form of multi-colored lighting.

OCPP 2.1 - © Open Charge Alliance 2025

32/52

Part 2 - Appendices

v2.1, 2025-12-03

Component

Description

Chapter 3. Standardized Components

ConnectedEV

ConnectedEV is a component that represents a connected vehicle for which data is received via an ISO 15118
or CHAdeMO interface. The generic information that is received, is represented as variables of ConnectedEV.
Any protocol-specific information is represented in variables of the ISO15118Ctrlr or CHAdeMOCtrlr component.

Connector

A means to connect an EV to a Charging Station with either a socket, an attached cable & inline connector, or
any wireless power transfer device.

ConnectorHolster
Release

A mechanism present in a connector holster to prevent the connector from being removed inappropriately:
typically unlocks connector after authorization.

ConnectorHolster
Sensor

A mechanism to report when a tethered cable connector has been removed from its normal stowage position.
May be used for detection of connectors left un-holstered, and possible penalty billing.

ConnectorPlugRe
tentionLock

Locking mechanism to retain an inserted plug, both to prevent on-load disconnection, and to prevent theft of
charging cables

ConnectorProtect
ionRelease

External protective mechanism (e.g. an external shutter or a connector holster lock mechanism) to prevent
contact with conductors that may become 'live' under other failure modes

Controller

An embedded logic controller

ControlMetering Energy, Power, Electricity meter, used to measure energy, current, voltages etc.

CPPWMControlle
r

Control Pilot PWM Controller: provides and senses the IEC 61851-1 / SAE J1772 low voltage DC and PWM
signalling between an EVSE and EV over a control pilot line.

DataLink

Provides a communications link from a Charging Station to a CSMS. It may use fixed infrastructure, mobile
telephony data services, WiFi, or other connectivity channels.

Display

Provides information and feedback to the user.

DistributionPanel Defines the Distribution Panel, with it’s fuses and connections to both Charging Stations and other Distribution

Panel’s.

ElectricalFeed

Represents an incoming electrical connection to a Charging Station, that may be a grid/distribution network
connection, of a connection to local power generation and/or storage. Each electrical feed can record the
electrical and other characteristics of that feed, including power rating, fusing, upstream metering, etc. When a
Charging Station has more than one electrical feed, it must represent which feed supplies each EVSE, and which
feed supplies the house load of the Charging Station itself. Simple Charging Stations with only a single electrical
feed may omit all electrical feed information, in which case it is inferred that all power is supplied from a single
feed, and what would otherwise be ElectricalFeed data (Variables) may be reported as being associated with the
ChargingStation component.

ELVSupply

Represents the low voltage power supply (typically 12V DC and often other ELV voltages) that provides
operating power for controllers, relays, and other electrical components.

EmergencyStopS
ensor

An 'Emergency Stop' button that should be pressed by the user or other nearby persons if serious faulty
behavior is observed (e.g. smoke/flames from EV or Charging Station).

EnvironmentalLig
hting

Provides reporting/control of general illumination lighting in use at Charging Station.

EVRetentionLock A locking mechanism on the EV side as a safety measure to prevent it being disconnected while high currents

are flowing.

EVSE

The entire chain of components responsible for transporting energy from the incoming supply to the electric
vehicle (or vice versa)

ExternalTemperat
ureSensor

Reports ambient air temperature

FiscalMetering

Provides energy transfer readings that are the basis for billing.

FloodSensor

A sensor reporting whether the Charging Station is experiencing water ingress/pooling.

GroundIsolationP
rotection

An Isolation Tester as part of their own self-test mechanisms, to confirm the isolation of floating circuitry when
no Evs are connected

Heater

Heater to ensure reliable operation in cold environments

HumiditySensor Reports relative air humidity

LightSensor

Reports ambient light levels.

LiquidCoolingSys
tem

LocalAvailabilityS
ensor

A liquid based cooling system, typically used to cool the connector cables of very high power Charging Stations.

Accepts local signal inputs controlling whether new Charging Sessions can start and/or whether ongoing
sessions should continue. Typically connected to a site/building power supply, to automatically report
unavailability when closed.

LocalController

The entire Local Controller as a logical entity

LocalEnergyStora
ge

Energy storage

OCPP 2.1 - © Open Charge Alliance 2025

33/52

Part 2 - Appendices

v2.1, 2025-12-03

Component

Description

Chapter 3. Standardized Components

NetworkConfigur
ation

The instances of component NetworkConfiguration represent network connection configurations.

OverCurrentProte
ction

Protects equipment by disconnecting the electrical supply when the current drawn (on any phase) exceeds the
rated value to a substantial degree.

OverCurrentProte
ctionRecloser

Recloser mechanism of an OverCurrentProtection to perform re-arm retries after a trip, or may be set for
remotely controlled re-arming on command.

PowerContactor Switches on and off the power to the EV after all authorization and safety requirements have been met. May

have secondary contacts to report closure state.

RCD

A Residual Current Device (US: ground fault breaker) protects human life and/or downstream equipment by
quickly detecting abnormal current flows (usually indicative in earth faults) in the Charging Station, cable, or EV
during charging.

RCDRecloser

A motorized recloser mechanism of an RCD that may be configured to perform re-arm retries after a trip, or may
be set for remotely controlled re-arming on command.

RealTimeClock

Represents realtime clock hardware that can maintain accurate date & time information in a Charging Station,
even in the case of simultaneous CSMS uncontactability and power outages or resets.

ShockSensor

Measures impact forces/accelerations experienced, indicative of possible damage.

SpacesCountSign
age

Electronic signage allowing a charging controller for a large charging facility to advertise counts of available
spaces to passing traffic.

Switch

A general purpose electromechanical input device, with optional remote defaulting/resetting of values. Each
input should use a Variable instance key indicating the nature of the input.

TemperatureSens
or

Temperature sensor at a point inside the Charging Station, multiple sensing points for a single sensing
controller. Multiple sensing points for a single sensing controller may be reported using distinct Variable
instance keys.

TiltSensor

Measures Tilt angle from normal reference position (normally 90 degree vertical).

TokenReader

An authorization token reader (e.g. RFID)

UpstreamProtecti
onTrigger

Circuitry designed to trigger the disconnection of power to the structure by an upstream protection device after
a severe problem has been detected

UIInput

A logical input mechanism (e.g. set of buttons) that is part of a UI whose use may be communicated to the
CSMS (in near real time). May support momentary inputs ('Operated') or modal state ('Active'). Multiple input
sources should use explicit Variable instance keys (where the input function is key name).

VehicleIdSensor Reports an identifier associated with a vehicle occupying a charging bay. The identifier may be a vehicle

registration number via ANPR hardware, a VIN, or other local identifier of the vehicle based on medium
range/active RFID, or any other relevant technology and result.

OCPP 2.1 - © Open Charge Alliance 2025

34/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 4. Standardized Variables

Chapter 4. Standardized Variables

This is a non-exhaustive list of Standardized Variables that SHALL be used when the Charging Station and CSMS want to exchange
information about a Variable. See also Part 1, paragraph 4.5.

Variables that are specific to a Controller Component are not included in this list, but are part of section 3.1 Controller Components.

Name

ACCurrent

DataType Unit

Description

decimal

A

RMS AC Current (in amperes). For 3-phase circuits, each phase (and
optional neutral) is represented by a Variable instance equal to a value
of the PhaseEnumType (e.g. L1,N). Unkeyed values reported for a
Component declared to be multi-phase are assumed to be an average
of all per-phase readings and written values are common per-phase
settings. Example(s): ChargingStation: Total AC current consumption
(all EVSE’s, ancillaries), EVSE: Total current consumed by EVSE:
includes losses (AC→DC) and EVSE specific ancillaries (e.g. fans),
ElectricalFeed: Inflow AC current on feed

If defined and true, this EVSE supports the selection of which phase to
use for 1 phase AC charging.

RMS AC Voltage (in volts). For 3-phase circuits, each phase (and
optional neutral) is represented by a Variable instance equal to a value
of the PhaseEnumType (e.g. L1,N). Unkeyed values reported for a
Component declared to be multi-phase are assumed to be an average
ofall per-phase readings and written values are common per-phase
settings. Example(s): ElectricalFeed: Input Voltage

Component is in its non-resting / active state: e.g: On, Engaged,
Locked. Some Components may have secondary functions that have
corresponding Active Variables with an explicit Variable instance.,
Note: Monitoring of changes in the Active state of any Component can
be specified by setting Delta monitoring on the boolean value with a
delta values of 1. Setting/clearing an Active Variable activates/stops
the associated functionality, where remotely controllable. Only
components that are Available and Enabled can be in the Active state.

Shows the currently used MonitoringBase.

Shows the currently use MonitoringLevel.

Indicates the configuration profile the station uses to connect to the
network.

ACPhaseSwitchingSupported

boolean

ACVoltage

decimal

V

Active

boolean

ActiveMonitoringBase

ActiveMonitoringLevel

ActiveNetworkProfile

OptionList

integer

boolean

ActiveTransactionId

string

Active transaction on charging station or EVSE.

AdditionalInfoItemsPerMessage

integer

AdditionalRootCertificateCheck

boolean

AllowEnergyTransferResumption

boolean

AllowNewSessionsPendingFirmwa
reUpdate

boolean

AllowReset

boolean

AllowSecurityProfileDowngrade

boolean

Angle

decimal

Deg

Attempts

integer

Maximum number of additionalInfo items that can be sent in one
message.

When set to true, only one certificate (plus a temporarily fallback
certificate) of certificateType CSMSRootCertificate is allowed to be
installed at a time.

This variable defines whether energy transfer is allowed to be resumed
when the transaction is resumed after a reset or power outage.

Indicates whether new sessions can be started on EVSEs, while
Charging Station is waiting for all EVSEs to become Available in order
to start a pending firmware update.

Component can be reset. Can be used to announce that an EVSE can
be reset individually.

If this variable is implemented and set to true, then the Charging
Station allows downgrading the security profile from 3 to 2.

Angle(s) relative to normal/design idle position. Multiple Variable
instance values may be used to indicate angular position in multiple
axes (e.g. Left-Right, Forward-Back).

Number of attempts (INCLUDING the original attempt) in the last
successful or attempted, cycle of operation. Applies typically to self-
monitoring motorized electro-mechanical equipment, etc. {Null}:
Unknown, 0: Not Attempted/Not allowed, 1: Single attempt/No retries
[allowed], 2-N: [up to] N tries [allowed]

OCPP 2.1 - © Open Charge Alliance 2025

35/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 4. Standardized Variables

Name

DataType Unit

Description

AuthorizeRemoteStart

boolean

AvailabilityState

OptionList

Available

BasicAuthPassword

BytesPerMessage

boolean

string

integer

CentralContractValidationAllowed boolean

CertSigningRepeatTimes

integer

CertSigningWaitMinimum

integer

Certificate

CertificateEntries

CertificateStatusSource

ChargeProtocol

string

integer

string

string

ChargingCompleteBulk

ChargingCompleteFull

ChargingProfilePersistence

boolean

boolean

boolean

ChargingState

OptionList

Whether a remote request to start a transaction in the form of
RequestStartTransactionRequest message should be authorized
beforehand like a local action to start a transaction.

A value of ConnectorStatusEnumType (See part 2): replicates
ConnectorStatus values reported in StatusNotification messages.

The Component exists and is locally configured/wired for use, but
might not be (remotely) Enabled.

The basic authentication password is used for HTTP Basic
Authentication.

Message Size (in bytes) - puts constraint on GetReportRequest,
GetMonitoringReportRequest or GetVariableRequest message size.

If this variable exists and has the value true, then Charging Station can
provide a contract certificate that it cannot validate, to the CSMS for
validation as part of the AuthorizeRequest.

This variable can be used to configure the amount of times the
Charging Station SHALL double the previous back-off time, starting
with the number of seconds configured at CertSigningWaitMinimum,
every time the back-off time expires without having received the
CertificateSignedRequest containing the from the CSR generated
signed certificate.

This configuration variable defines how long the Charging Station has
to wait before generating another CSR, in the case the CSMS accepts
the SignCertificateRequest, but never returns the signed certificate.

Digital Certificate (in Base64 encoding)

Amount of Certificates currently installed on the Charging Station.

When present, this variable tells CSMS whether Charging Station uses
OCSP or CRL to check for revoked certificates.

The Charging Control Protocol applicable to a Connector. CHAdeMO:
CHAdeMO protocol, ISO15118: ISO15118 V2G protocol (wired or
wireless) as used with CCS, CPPWM: IEC61851-1 / SAE J1772
protocol (ELV DC & PWM signalling via Control Pilot wire),
Uncontrolled: No charging power management applies (e.g. Schuko
socket), Undetermined: Yet to be determined (e.g. before plugged in),
Unknown: Not determinable, NOTE: ChargeProtocol is distinct from
and orthogonal to connectorType.

Charging up to StateOfChargeBulk has completed.

Charging up to StateOfCharge.maxSet has completed.

If an instance of this variable is true, then charging profiles with the
chargingProfilePurpose mentioned in the variableInstance are
persistent, i.e. they are stored persistently and will still exist after a
reboot.

This variable reports the current transaction charging state for an
EVSE.

ChargingTime

Color

string

decimal

s

Time from earliest to latest substantive energy transfer

Standard 24 bit hexadecimal RGB values. Reg Green Blue color
intensity, expressed as standard 24 bit hexadecimal RGB values: 3 00-
FF (0-255), in order RRGGBB). E.g. 000000: Black, FF0000: Red,
00FF00: Green, 0000FF: Blue, FFFF00:Yellow, FFFFFF: White, 008000:
Medium intensity green.

Points to a communication parent component (data flow source), to
allow rendering the communication hierarchy in a UI.

Component’s operation cycle has completed. Used only in event
notifications, where it is always true.

If set to true the Charging Station supports tariffs with conditions.

This Configuration Variable can be used to limit the following fields:
SetVariableData.attributeValue and VariableCharacteristics.valuesList.

CommunicationParent

string

Complete

ConditionsSupported

ConfigurationValueSize

ConnectedTime

ConnectorType

boolean

boolean

integer

decimal

s

Time since logical connection established

OptionList

A value of ConnectorStringEnumType (See Appendix 7). Specific type
of connector, including sub-variant information. Note: Distinct and
orthogonal to Charging Protocol, Power Type, Phases.

OCPP 2.1 - © Open Charge Alliance 2025

36/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 4. Standardized Variables

Name

DataType Unit

Description

ContractCertificateInstallationEna
bled

boolean

ContractValidationOffline

boolean

If this variable is true, then ISO 15118 contract certificate
installation/update as described by use case M01 - Certificate
installation EV

If this variable is true, then Charging Station will try to validate a
contract certificate when it is offline.

Count

CountryName

Currency

integer

string

string

General purpose integer count variable for Component state reporting

The countryName of the SECC in the ISO 3166-1 format.

Currency in a ISO 4217 formatted currency code.

CurrentImbalance

decimal

Percent

Percentage current imbalance in an AC three phase supply.

CustomImplementationEnabled

boolean

CustomTriggers

DCCurrent

MemberLi
st

decimal

A

DCInputPhaseControl

boolean

This standard configuration variable is used to enable/disable the
custom implementation named in the variableInstance.

This variable defines the names of custom triggers that Charging
Station supports in a customTrigger field of TriggerMessageRequest.

DC Current (in amperes). May be an instantaneous measurement, or a
period average, depending on context/equipment.

When DCInputPhaseControl is true, then the values of numberPhases
and PhaseToUse in a ChargingSchedulePeriodType will select the input
phases from the grid to be used by the DC EVSE.

decimal

V

DC Voltage (volts). May be an instantaneous measurement, or a period
average, depending on context/equipment.

DCVoltage

DataText

DateTime

DepartureTime

string

dateTime

dateTime

DisablePostAuthorize

boolean

DisableRemoteAuthorization

boolean

DischargePower

decimal

DisplayMessages

ECVariant

integer

string

EVConnectionTimeOut

integer

s

ElectricalParent

Enabled

string

boolean

Energy

decimal Wh

Text associated with a Component, e.g. a Display.

Point in time value, in [RFC3339] datetime format. Time zone optional.

Time in [RFC3339] datetime format, when an EV intends to leave the
charging station.

When set to true this variable disables the behavior to request
authorization for an

When set to true this instructs the Charging Station to not issue any
AuthorizationRequests, but only use Authorization Cache and Local
Authorization List to determine validity of idTokens.

The variableCharacteristic maxLimit holds the maximum rated
discharge power that this EVSE can provide. The variableCharacteristic
maxSet holds the maximum configured discharge power that this
EVSE can provide. The Actual value of the instantaneous (real)
discharge power is recommended to be supported, but not required.
Discharge power is represented by a positive value.

Maximum number of different messages that can configured in this
Charging Station simultaneous, via SetDisplayMessageRequest.

Production series variants reflecting internal design changes or sub-
component substitutions not affecting external functionality.

Interval from between 'starting' of a transaction until incipient
transaction is automatically canceled, due to failure of EV driver to
(correctly) insert the charging cable connector(s) into the appropriate
socket(s).

Points to a electrical parent component (energy flow source), to allow
rendering the electrical hierarchy in a UI.

The Component is Enabled for operation. For Available components
that cannot be selectively (remotely) enabled / disabled, this value is
always true. Note: Available cannot be false of Enabled is true, so
during inventory reporting, Enabled=1 also logically states
Available=true

Energy quantity (in Wh) for reporting/configuring values related to
stored energy (i.e. not transferred energy).

EnergyCapacity

EnergyExport

decimal Wh

Energy capacity in Wh of an energy storage device.

decimal Wh

Total energy transferred: e.g. from EV during (ongoing or terminated)
charging session (in wH by default)

EnergyExportRegister

decimal Wh

Cumulative export kWh register value, such as from a (certified) fiscal
energy meter.

EnergyImport

decimal Wh

Total energy transferred.

OCPP 2.1 - © Open Charge Alliance 2025

37/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 4. Standardized Variables

Name

DataType Unit

Description

EnergyImportRegister

decimal Wh

Entries

integer

ExternalConfigChangeDate

DateTime

ExternalConstraintsProfileDisallow
ed

boolean

ExternalControlSignalsEnabled

boolean

Fallback

boolean

Cumulative export kWh register value, such as from a (certified) fiscal
energy meter.

General purpose variable for reporting/managing numbers of entries in
repetitive data structures. maxLimit characteristic reports maximum
possible entries.

Date/time when the configuration was changed externally, i.e. outside
of CSMS, for example by a local service action.

Indicates whether a Charging Station allows an external system to
submit a ChargingStationExternalConstraints charging
profile.

Indicates whether a Charging Station is able to respond to external
control signals that influence charging. If the variable is true, but CSMS
has set [configkey-external-constraints-profile-disallowed] = true, then
external control signals are only allowed during a charging profile with
a chargingProfilePeriod = ExternalLimits or ExternalSetpoint.

Component is operating in a fallback, or backup mode. In inventory
reports, a Value of 1 for the maxLimit characteristic indicates that the
component can enter a fallback state (i.e. a fallback mode is present).

FanSpeed

FieldLength

FileTransferProtocols

FirmwareVersion

Force

decimal

RPM

Fan Speed (in RPM). A value of 0 represents stopped/stalled. An
empty value indicates that fan speed cannot be read.

integer

MemberLi
st

string

decimal

N

This variable is used to report the length of <field> in <message> when
it is larger

List of supported file transfer protocols.

Version number of firmware.

Reports (impact) force/ acceleration values (estimates) in one or more
directions, in units of Newtons or “g”. Multiple force readings in
different (orthogonal) dimensions may be reported using Variable
instance values, such as Down, Right, Forward.

Formats

Frequency

MemberLi
st

List of message formats supported by this Charging Station. Possible
values: ASCII, HTML, URI, UTF-8.

decimal

Hz

Frequency of AC power, signal, or component operation.

FrequencySchedule

string

FuseRating

decimal

A

HandleFailedTariff

OptionList

HeartbeatInterval

integer

s

A JSON-formatted string with an array of { time, freq } pairs, in which
time is

Current rating of a fuse/breaker. Variable instances keyed by phase
identifier (L1/L2/L3/N).

This configuration determines how to act when a driver-specific tariff
is received, which cannot be processed.

Interval of inactivity (no OCPP exchanges) with CSMS after which the
Charging Station should send HeartbeatRequest.

Height

Humidity

Hysteresis

ICCID

IMSI

ISO15118EvseId

IdToken

Identity

Impedance

decimal

m

Height above(+)/below(-) reference level (ground level unless context
demands otherwise).

decimal

RH

The relative humidity in %.

decimal

Percent

string

string

string

string

string

decimal

Ohm

Specifies the width of a 'dead band' (as a percentage of the threshold)
around the central value of a threshold setting (e.g. MinSet, MaxSet,
monitor thresholds) to avoid repeated triggering when the measured
quantity lies close to the threshold and is subject to small variations.

ICCID (Integrated Circuit Card IDentifier) of mobile data SIM card.

IMSI (International Mobile Subscriber Identity) number of mobile data
SIM card

EVSE ID in string format as used in ISO 15118 and IEC 63119-2

The IdToken used to authorize a charging transaction.

The Charging Station identity.

Impedance: Primary value is real (resistive only) impedance. Where a
complex impedance is to be reported, the imaginary part (reactance)
must be represented with a separate Variable instance value of
'reactance'. Reactance values are expressed at the (nominal) relevant
operating frequency of the Component (e.g. 50/60Hz for mains
electricity feed).

OCPP 2.1 - © Open Charge Alliance 2025

38/52

Part 2 - Appendices

v2.1, 2025-12-03

Name

Interval

ItemsPerMessage

Label

Language

Length

LifeTime

Light

DataType Unit

Description

Chapter 4. Standardized Variables

s

Minimum Interval (in seconds) between (attempted) operations.

integer

integer

string

OptionList

decimal

integer

m

s

Maximum number of ComponentVariable entries that can be sent in
one GetReportRequest or GetMonitoringReportRequest message.

Label for a component. Specifies a non-unique label to be used in a
hierarchy UI rendering, in place of the unique component name and
instance

Default language code, per RFC 5646, of this Charging Station.

General Purpose linear distance measure.

Indicates how long it takes until a token expires in the authorization
cache since it is last used.

decimal

lx

(Ambient) light level. The value is in Lux.

LimitChangeSignificance

decimal

LocalAuthorizeOffline

boolean

LocalFrequencyUpdateThreshold

integer

mHz

LocalLoadBalancing

LocalPreAuthorize

LogicalParent

Manufacturer

MasterPassGroupId

decimal

boolean

string

string

string

MaxCertificateChainSize

integer

MaxElements

integer

MaxEnergyOnInvalidId

integer

Wh

MaxExternalConstraintsId

integer

MaxPeriodicEventStreams

integer

MaxPriceElements

integer

MaxSoc

Measurands

Message

integer

MemberLi
st

string

MessageAttemptInterval

integer

s

MessageAttempts

MessageTimeout

integer

integer

MinimumStatusDuration

integer

s

s

If at the Charging Station side a change in the limit in a ChargingProfile
is lower than this percentage, the Charging Station MAY skip sending a
NotifyChargingLimitRequest or a TransactionEventRequest message
to the CSMS.

Whether the Charging Station, when Offline, will start a transaction for
locally-authorized identifiers.

The amount of change in net frequency in mHz is needed to trigger a
recalculation of the setpoint.

Variable with instances to control local load-balancing.

Whether the Charging Station, when online, will start a transaction for
locally-authorized identifiers without waiting for or requesting an
AuthorizeResponse from the CSMS.

Points to a logical parent component, to allow rendering a
comprehensive overview of the Charging Station components in a UI.

Component Manufacturer name

IdTokens that have this id as groupId belong to the Master Pass
Group.

This configuration variable can be used to limit the size of the
'certificateChain' field from the CertificateSignedRequest PDU.

For TariffCostCtrlr: Specifies the maximum number of prices elements
that the Charging Station supports in each energy, chargingTime,
_idleTime and fixedFee of a TariffType.

Maximum amount of energy in Wh delivered when an identifier is
deauthorized by the CSMS after start of a transaction.

Defines the highest value that a charging profile id of a
ChargingStationExternalConstraints profile in the Charging
Station can have.

The maximum number of open periodic event streams that Charging
Station supports.

For ISO15118Ctrlr: The maximum number of priceRuleStacks and
priceLevelScheduleEntries that Charging Station is able to accept in a
ChargingScheduleType.

The maximum state of charge that a battery will be charged to.

Measurand(s) to be included in MeterValuesRequest or
TransactionEventRequest

Specific stored message for display.

How long the Charging Station should wait before resubmitting a
TransactionEventRequest message that the CSMS failed to process.

How often the Charging Station should try to submit a
TransactionEventRequest message when the CSMS fails to process it.

The purpose of the message timeout is to be able to consider a
request message as not sent and continue with other tasks when the
message did not arrive due to communication errors or software
failure.

Minimum duration that a Charging Station or EVSE status is stable
before StatusNotificationRequest is sent to the CSMS.

OCPP 2.1 - © Open Charge Alliance 2025

39/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 4. Standardized Variables

Name

Mode

Model

DataType Unit

Description

string

string

NetworkProfileConnectionAttempt
s

integer

Specifies the number of connection attempts the Charging Station
executes before switching to a different profile.

Operating mode string from among valid options (communicated by
OptionList, etc. during capability/configuration discovery).

Manufacturer’s Model code/number of Component, including suffixes
etc. to identify functional, regional or linguistic variation, but NOT
engineering change level internal variation not affecting external
behaviour, etc.

Current network address of a Component.

A comma separated ordered list of the priority of the possible Network
Connection Profiles. The list of possible available profile slots for the
network configuration profiles SHALL be reported, via the valuesList
characteristic of this Variable.

Date time of the next time offset transition. On this date time, the clock
displayed to the EV driver will be given the new offset as configured via
TimeOffsetNextTransition.

For ReservationCtrlr: If this configuration variable is present and set to
true: Charging Station supports reservation where EVSE id is not
specified.

For ISO15118Ctrlr: The SECC (EVSE) uses the NotificationMaxDelay
element in the EVSEStatus to indicate the time in seconds until it
expects the EVCC (EV) to react on the action request indicated in
EVSENotification.

Indicates if the Charging Station should include the externally set
charging limit/schedule in the message when it sends a
NotifyChargingLimitRequest message.

This contains the address of the NTP server.

Use the NTP server provided via DHCP, or use the manually configured
NTP server.

When set and the Charging Station is offline, the Charging Station shall
queue any NotifyEventRequest messages triggered by a monitor with a
severity number equal to or lower than the severity configured here.

Message (and/or tariff information) to be shown to an EV Driver when
Charging Station is offline.

When the offline period of a Charging Station exceeds the
OfflineThreshold it is recommended to send a
StatusNotificationRequest for all its Connectors when the Charging
Station is back online.

If this key exists and is true, the Charging Station supports Unknown
Offline Authorization.

The Component operated in an instantaneous, transient, or
immediately self-resetting pattern. Used only in event notifications,
where it is always true.

Recurring operating times in iCalendar RRULE format.

The organizationName of the CSO operating the charging station.

Component is in Overload state.

NetworkAddress

NetworkConfigurationPriority

string

SequenceL
ist

NextTimeOffsetTransitionDateTim
e

DateTime

NonEvseSpecific

boolean

NotificationMaxDelay

integer

s

NotifyChargingLimitWithSchedules boolean

NtpServerUri

NtpSource

string

OptionList

OfflineQueuingSeverity

integer

OfflineTariffFallbackMessage

string

OfflineThreshold

integer

s

OfflineTxForUnknownIdEnabled

boolean

boolean

string

string

boolean

Operated

OperatingTimes

OrganizationName

Overload

Percent

PeriodsPerSchedule

PhaseRotation

Phases3to1

PhysicalParent

PnCEnabled

Policy

decimal

Percent

Generic dimensionless value reporting/setting value.

integer

string

boolean

string

boolean

Maximum number of periods that may be defined per
ChargingSchedule.

This variable describes the phase rotation of a Component relative to
its parent Component, using a

If defined and true, this Charging Station supports switching from 3 to
1 phase during a transaction.

Points to a physical parent component (container), to allow rendering
an overview of the Charging Station component locations in a UI.

If this variable is true, then ISO 15118 plug and charge as described by
use case C07 - Authorization using Contract Certificates is enabled.

OptionList

Cache Entry Replacement Policy: least recently used, least frequently
used, first in first out, other custom mechanism.

OCPP 2.1 - © Open Charge Alliance 2025

40/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 4. Standardized Variables

Name

DataType Unit

Description

PostChargingTime

decimal

s

Elapsed time in seconds since last substantive energy transfer

Power

decimal W,kW

Present

Problem

ProfileStackLevel

Protecting

boolean

boolean

integer

boolean

ProtocolAgreed

string

ProtocolSupported

string

ProtocolSupportedByEV

string

PublicKey

string

PublicKeyWithSignedMeterValue

boolean

QueueAllMessages

boolean

Instantaneous (real) Power (measured/calculated, including power
factor for AC). Where a component (e.g. AC to DC Power Converter)
has multiple power measurements, the default (unkeyed) instance is
“input” power.

Component exists, but might not be locally configured/wired for use,
nor (remotely) Enabled.

Component itself has a 'Problem' condition that impacts in any
significant way on its normal operation. By definition, 'Problem' state
includes (logical OR) 'Fault' state. 'Problem' specifically INCLUDES
inability to operate that is propagated (up/down/sideways) from any
other associated/connected/containing/contained Component.

Maximum acceptable value for stackLevel in a ChargingProfile.

Applies to 'sensor' type Components that have an associated
protection capability, whereby they can disconnect power (e.g. using
the main PowerContactor) if the sensed quantity is outside
preset/configured limits. If Protecting is true, the Component is
actively preventing/interrupting charging.

For ConnectedEV: A string with the following comma-separated items:
“<uri>,<major>,<minor>”. This is the protocol uri and version
information that was agreed upon between EV and EVSE in the
supportedAppProtocolReq handshake from ISO 15118.

For ISO15118Ctrlr: A string with the following comma-separated items:
“<uri>,<major>,<minor>”. <uri> is in the format as used in the
SupportedAppProtocolReq message from ISO 15118-2 and ISO 15118-
20. This variable has at most 20 instances, one for each supported
protocol version.

For ConnectedEV: A string with the following comma-separated items:
“<uri>,<major>,<minor>”. This is information from the
SupportedAppProtocolReq message from ISO 15118. Each priority is
given its own variable instance. Priority is a number from 1 to 20 as a
string.

Configuration variable that can be used to retrieve the public key for a
meter connected to a specific EVSE.

This Configuration Variable can be used to configure whether a public
key needs to be sent with a signed meter value.

When this variable is set to true, the Charging Station will queue all
message until they are delivered to the CSMS.

RateUnit

string

A list of supported quantities (A, W) for use in a ChargingSchedule.

RegisterValuesWithoutPhases

boolean

RemainingTimeBulk

RemainingTimeFull

ReportingValueSize

integer

integer

integer

s

s

RequestMeteringReceipt

boolean

ResetRetries

integer

ResumptionTimeout

integer

s

If this variable reports a value of true, then meter values of measurand
Energy.Active.Import.Register will only report the total energy
over all phases without reporting the individual phase values.

Number of seconds remaining to charge to bulk state of charge, given
by StateOfChargeBulk.

Number of seconds remaining to charge to 100% state of charge.

This Configuration Variable can be used to limit the following fields:
GetVariableResult.attributeValue, VariableAttribute.value and
EventData.actualValue.

For ISO15118Ctrlr: If this variable is true, then Charging Station shall
request a metering receipt

Number of times to retry a reset of the Charging Station when a reset
was unsuccessful.

This variable defines the maximum number of seconds that a
transaction may be interrupted by a power outage and still be resumed
afterwards.

SampledMeasurands

MemberLi
st

The set of measurands to be sampled by the DataCollector
component.

SamplingInterval

decimal

s

The sampling interval in seconds.

SeccId

string

The name of the SECC in the string format as required by ISO 15118.

OCPP 2.1 - © Open Charge Alliance 2025

41/52

Part 2 - Appendices

DataType Unit

Description

Chapter 4. Standardized Variables

v2.1, 2025-12-03

Name

SecurityProfile

SendDuringIdle

SerialNumber

integer

boolean

string

ServiceRenegotiationSupport

boolean

SetpointPriority

OptionList

SignReadings

boolean

SignStartedReadings

boolean

SignUpdatedReadings

boolean

SignalStrength

decimal

dBm

This configuration variable is used to report the security profile used
by the Charging Station.

For AlignedDataCtrlr: If set to true, the Charging Station SHALL only
send clock aligned meter values when there is no transaction ongoing.

Serial number of Component.

For ISO15118Ctrlr: If set to 'True' the SECC (EVSE) is capable of
ServiceRenegotiation.

Defines which setpoint shall be used when a
ChargingStationExternalConstraints profile

If set to true, the Charging Station SHALL include signed meter values
in the TransactionEventRequest(Ended).

If set to true, the Charging Station SHALL include signed meter values
for context = Transaction.Begin in the metervalues field in the
TransactionEventRequest(Started or Updated).

If set to true, the Charging Station SHALL include signed meter values
in the metervalues field in the TransactionEventRequest(Updated).

(Radio/Wired/Optical) data signal strength, in ASU (typically 0-31 or 99
for unknown). Or dbmW (typically -140 to -50).

SlotStatus

OptionList

This variable represents the status of the door of the battery slot.

SoC

SoH

State

integer

Percent

SoC of the component BatteryCartridge which refers to the battery that
is inserted at the EVSE.

integer

Percent

SoH of the component BatteryCartridge which refers to the battery that
is inserted at the EVSE.

string

A state code or name identifier string, to allow the internal state of
components to be reported and/or controlled

StateOfCharge

decimal

Percent

StateOfChargeBulk

decimal

Percent

StopTxOnEVSideDisconnect

boolean

StopTxOnInvalidId

boolean

Storage

integer

B

SupplyPhases

integer

SupportedAdditionalPurposes

MemberLi
st

SupportedEnergyTransferModes MemberLi

SupportedFormats

SupportedIdTokenTypes

SupportedLimits

SupportedOperationModes

SupportedPriorities

st

MemberLi
st

MemberLi
st

MemberLi
st

MemberLi
st

MemberLi
st

Energy Storage Device (e.g. battery) state of charge, expressed as a
percentage of nominal design 0-100% operating range. The value of
StateOfCharge.maxSet represents the maximum state of charge for a
full battery and is usually at or near 100%.

Energy Storage Device (e.g. battery) state of charge up to which fast
charging is possible. Above this percentage charging speed will drop
significantly.

When set to true, the Charging Station SHALL deauthorize the
transaction when the cable is unplugged from the EV.

Whether the Charging Station will deauthorize an ongoing transaction
when it receives a non- Accepted authorization status in
TransactionEventResponse for this transaction.

In bytes. Amount of storage occupied. Storage(maxLimit) specifies
absolute limit Storage(MaxSet) restricts usage to specified Max, if
supported.

Number of alternating current phases connected/available. 1 or 3 for
AC, 0 means DC (no alternating phases). Null value indicates that the
number of phases (e.g. in use) is unknown.

This configuration variable lists the additional charging profile
purposes, that have been introduced in OCPP 2.1, that are supported
by the Charging Station.

Lists the energy transfer services that are supported by the Charging
Station.

For DisplayMessageCtrlr: List of message formats supported by this
Charging Station.

The subset of the list of supported IdTokenTypes as defined in
Appendix 7.

This variable defines which transaction limits in TransactionLimitType
are supported by the Charging Station.

Lists the V2X operation modes that are supported by the Charging
Station.

For DisplayMessageCtrlr: List of the priorities supported by this
Charging Station.

OCPP 2.1 - © Open Charge Alliance 2025

42/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 4. Standardized Variables

Name

DataType Unit

Description

SupportedProviders

string

SupportedStates

SupportsDynamicProfiles

MemberLi
st

boolean

SupportsEvseSleep

boolean

SupportsExpiryDateTime

boolean

SupportsLimitAtSoC

boolean

SupportsMaxOfflineDuration

boolean

SupportsRandomizedDelay

boolean

SupportsUseLocalTime

boolean

Suspending

Suspension

boolean

boolean

TargetSoc

integer

Percent

TariffFallbackMessage

string

A comma-separated list of all providers (eMSPs) that are supported on
this Charging Station. The providers are listed using country and
provider ID from the EMAID, as defined in ISO 15118-20.

For DisplayMessageCtrlr: List of the states during which to display a
message supported by this Charging Station.

When this variable has value True, then the Charging Station supports
charging profiles of type Dynamic.

When reported as true the Charging Station supports the evseSleep
flag in a ChargingSchedulePeriod, which requests the EVSE electronics
to go to sleep during operationMode = 'Idle'.

For LocalAuthListCtrlr: When set to true Charging Station will disregard
idTokens for authorization as if not present in the Local Authorization
List when current date/time is past the value of cacheExpiryDateTime.

When this variable has value True, then the Charging Station supports
the field limitAtSoC in ChargingSchedul, which will cap the limit or
setpoint in the ChargingSchedulePeriodType by the value of
limitAtSoC.limit.

When this variable has value True, then the Charging Station supports
the fields maxOfflineDuration and invalidAfterOfflineDuration in
ChargingProfile.

When this variable has value True, then the Charging Station supports
the field randomizedDelay in ChargingSchedule, which will delay the
start of each charging schedule period by a random number between 0
and randomizedDelay.

When this variable has value True, then the Charging Station supports
the field useLocalTime in ChargingSchedule.

If Suspending is true, the Component can is currently suspending
charging.

Applies to 'sensor' type Components that have a charging suspension
capability, typically for safety or equipment protection reasons. If
Suspension is true, the component can suspend charging when the
sensed quantity is outside preset/configured limits.

For BatterySwapCtrlr: The state of charge that a battery must have in
order to be eligible for swapping.

Message (and/or tariff information) to be shown to an EV Driver when
there is no driver specific tariff information available.

decimal

Celsius,
Fahrenheit

Temperature(s) of component (in Celsius, by default). Components
may have multiple indexed temperature sensors.

Temperature

Time

TimeAdjustmentReportingThresho
ld

dateTime

integer

s

TimeOffset

string

TimeSource

TimeZone

Timeout

Timeout

Token

TokenType

SequenceL
ist

string

decimal

integer

s

s

string

OptionList

TotalCostFallbackMessage

string

Point in time value, in ISO 8601 datetime format. Time zone optional.

When the clock time is adjusted forwards or backwards for more then
TimeAdjustmentReportingThreshold number of seconds, a
SecurityEventNotification( 'SettingSystemTime' ) is sent by the
charging station.

A Time Offset with respect to Coordinated Universal Time (aka UTC or
Greenwich Mean Time) in the form of an [RFC3339] time (zone) offset
suffix, including the mandatory “+” or “-“ prefix.

Via this variable, the Charging Station provides the CSMS with the
option to configure a clock source.

Configured current local time zone in the format: 'Europe/Oslo',
'Asia/Singapore' etc.

Generic timeout value for Component operation (in seconds).

For BatterySwapCtrlr: Timeout in seconds in which a set of batteries
must be inserted or removed after successful authorization.

String of bytes representing an ID token.

Type of Token. Value is one of IdTokenEnumStringType (See Appendix
7).

Message to be shown to an EV Driver when the Charging Station
cannot retrieve the cost for a transaction at the end of the transaction.

Tries

integer

Number of attempts done by a Component.

OCPP 2.1 - © Open Charge Alliance 2025

43/52

Part 2 - Appendices

v2.1, 2025-12-03

Name

Tripped

Chapter 4. Standardized Variables

DataType Unit

Description

boolean

Single-shot device requires explicit intervention to re-prime/activate to
normal.

TxBeforeAcceptedEnabled

boolean

TxEndedInterval

integer

s

TxEndedMeasurands

TxStartPoint

TxStartedMeasurands

TxStopPoint

MemberLi
st

MemberLi
st

MemberLi
st

MemberLi
st

With this configuration variable the Charging Station can be configured
to allow charging before having received a BootNotificationResponse
with status: Accepted.

Interval between sampling of metering (or other) data, intended to be
transmitted in the TransactionEventRequest(Ended) message.

Sampled measurands to be included in the meterValues element of
TransactionEventRequest(Ended).

Start points for a transaction.

Sampled measurands to be included in the meterValues element of
TransactionEventRequest(Started).

Stop points of a transaction.

TxUpdatedInterval

integer

s

Interval between sampling of metering (or other) data, intended to be
transmitted in the TransactionEventRequest(Updated) message.

TxUpdatedMeasurands

MemberLi
st

Sampled measurands to be included in the meterValues element of
TransactionEventRequest(Updated).

UnlockOnEVSideDisconnect

boolean

UpstreamInterval

integer

s

UpstreamMeasurands

MemberLi
st

V2GCertificateInstallationEnabled boolean

string

string

VehicleCertificate

VehicleId

VersionDate

VersionNumber

VoltageImbalance

WorkingMode

When set to true, the Charging Station SHALL unlock the cable on the
Charging Station side when the cable is unplugged at the EV.

Interval between sampling of metering (or other) data, intended to be
transmitted via TransactionEventRequest(Updated) messages for
location = Upstream only.

Sampled measurands to be included in the meterValues element of
every TransactionEventRequest(Updated) for location = Upstream
only.

If this variable is true, then ISO 15118 V2G Charging Station certificate
installation as described by use case A02 - Update Charging Station
Certificate by request of CSMS

For ConnectedEV: The PEM encoded X.509 leaf/intermediate/root
certificate when present in the vehicle certificate chain.

ID that EV provides to charging station. Encoded as a hexbinary string.
In ISO 15118 the EVCCID is 6 bytes (MAC address), in CHAdeMO the
vehicle id can be 24 bytes.

dateTime

string

[RFC3339]

Version number of hardware

decimal

Percent

Percentage voltage imbalance in three phase supply.

OptionList

This variable represents the current working mode of the battery in
BatteryCartridge component.

OCPP 2.1 - © Open Charge Alliance 2025

44/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 5. Reason Codes

Chapter 5. Reason Codes

The table below provides a list of standardized reason codes that can be used in the optional statusInfo element (of type
StatusInfoType) of a response.

For each reason code, some messages that might typically return them are shown. This is not an exhaustive list and only indicative.

StatusInfo is optional. Any implementation should be able to function properly without statusInfo, because every message has the
response code values that are essential to perform the function. The reasonCode and additionalInfo in StatusInfoType are meant to
provide more insight on what is happening and maybe allow for some automatic recovery.

The grouping of the reasonCodes in the table below is only to make it easier to look-up codes for certain situations. The grouping
does not affect the meaning of the reasonCode.

IMPORTANT

The existence of a reason code in this table does not imply a requirement to use it nor does it imply a
requirement to any of the mentioned messages.

(Updated in v2.0)

Group

Reason code

Description

Typically used for

Charging Profiles

DuplicateProfile

InvalidProfile

InvalidProfileId

InvalidSchedule

A charging profile with same stackLevel -
chargingProfilePurpose combination already
exists on the Charging Station and has an
overlapping validity period.

SetChargingProfile

Provided chargingProfile contains invalid
elements.

SetChargingProfile,
RequestStartTransaction

Provided chargingProfile has an id that is within
an invalid range.

SetChargingProfile,
RequestStartTransaction

Provided chargingSchedule contains invalid
elements.

SetChargingProfile,
RequestStartTransaction

InvalidStackLevel

Provided value for stackLevel is invalid.

SetChargingProfile

InvalidOperationMode

Provided operationMode is invalid for this
chargingProfilePurpose

NoFreqWattCurve

NoPhaseForDC

PhaseConflict

NoSignalWattCurve

RateLimitExceeded

UnsupportedKind

UnsupportedPurpose

UnsupportedRateUnit

SetChargingProfile

SetChargingProfile

A frequency-watt curve is missing in a charging
schedule period with operation mode =
LocalFrequency.

Phase selection for a DC EVSE is not supported

SetChargingProfile

Phase conflict between applicable charging
profiles

SetChargingProfile

A signal-watt curve is missing in a charging
schedule period when an AFRRSignalRequest is
received.

AFRRSignal

A charging profile of the same purpose is
submitted too frequently

The requested charging profile kind is not
supported

SetChargingProfile

SetChargingProfile

The requested charging profile purpose is not
supported

SetChargingProfile

A chargingRateUnit is provided that is not
supported.

SetChargingProfile

Charging Station

CSNotAccepted

FixedCable

BootNotification of Charging Station has not (yet)
been accepted by CSMS.

RequestStartTransaction,
RequestStopTransaction

The connector has its own fixed cable that cannot
be unlocked.

UnlockConnector

NoCable

No cable is connected at this time.

UnknownConnectorId

Connector Id is not known on EVSE

UnlockConnector

ChangeAvailability,
UnlockConnector

OCPP 2.1 - © Open Charge Alliance 2025

45/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 5. Reason Codes

Group

Reason code

Description

Typically used for

UnknownConnectorType

Connector type is not known on EVSE

ReserveNow

UnknownEvse

EVSE is not known on Charging Stations

ChangeAvailability,
ReserveNow,
RequestStartTransaction

Swap Station

Network
Configuration

Miscellaneous

BatterySoHLow

Battery State of Health is too low

BatterySwap

Battery State of Charge has unacceptable value

BatterySwap

BatterySoC

BatteryDamaged

BatteryUnknown

BatteryType

Battery is damaged

Battery has unknown serial number

Battery type not accepted

NoBatteryAvailable

No battery available for swapping

PriorityNetworkConf

InvalidConfSlot

InvalidNetworkConf

A network configuration variable of an
NetworkConfiguration instance present in
NetworkConfigurationPriority is not allowed to be
changed

A value for configurationSlot is used that is not
present in
NetworkConfigurationPriority.valuesList

Some values in NetworkConfiguration instance
are invalid

NoSecurityDowngrade

Security profile downgrade is not allowed

DuplicateRequestId

A requestId is provided, that has already been
used for this type of request.

InvalidMessageSeq

Message should not be sent at this moment in
current scenario.

BatterySwap

BatterySwap

BatterySwap

BatterySwap,
RequestBatterySwap

SetVariablesRequest of
NetworkConfiguration

SetNetworkProfileRequest

SetVariablesRequest of
NetworkConfigurationPriorit
y,SetNetworkProfileRequest

SetVariablesRequest of
SecurityProfile,
SetNetworkProfileRequest

UpdateFirmware,
PublishFirmware and
requests for reports.

(generic),
SetChargingProfile with
ISO15118

MissingDevModelInfo

NoError

NotFound

Information needed for operation is missing from
Device Model

(generic)

No error has occurred, but some extra information
is in additionalInfo .

(generic)

No object(s) found that match a provided ID or
criteria.

ClearVariableMonitoring,
CustomerInformation,
GetChargingProfiles,
GetDisplayMessages,
GetInstalledCertificateIds,
GetReport

Unspecified

No reason is specified, but some extra
information is in additionalInfo

UnsupportedRequest

This request is not supported.

(generic)

(generic)

Operations and
Permissions

Security

FwUpdateInProgress

Operation is not possible, because a firmware
update is in progress.

Reset

NotEnabled

ReadOnly

WriteOnly

Feature is not enabled.

ClearCache

Targeted variable is read-only and cannot be set. SetVariables

Targeted variable is write-only and cannot be
read.

GetVariables

InvalidCSR

Provided CSR is invalid

SignCertificate

OCPP 2.1 - © Open Charge Alliance 2025

46/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 5. Reason Codes

Group

Reason code

Description

InvalidCertificate

Provided certificate is invalid.

InvalidURL

Provided URL is invalid.

Typically used for

CertificateSigned,
InstallCertificate

UpdateFirmware,
PublishFirmware

RedirectNotAllowed

HTTP Redirection is not allowed

LogStatusNotification

System Errors

Transactions

InternalError

OutOfMemory

OutOfStorage

Operation cannot be completed due to an internal
error.

(generic)

Operation not possible, because system does not
have enough memory.

(generic)

Operation not possible, because system does not
have enough storage.

(generic)

InvalidIdToken

TxInProgress

Provided idToken is not valid.

A transaction is in progress.

TxNotFound

There is no such transaction.

TxStarted

A transaction had already started (e.g. due to
cable being plugged in).

Values and Ranges

RequestStartTransaction

ChangeAvailability, Reset,
RequestStartTransaction

RequestStopTransaction,
SetChargingProfile,
GetVehicleCertificate

RequestStartTransaction

InvalidValue

MissingParam

An invalid value has been provided.

A parameter that is required for the request is
missing.

(generic)

(generic)

TooLargeElement

Provided element is too large to handle.

TooManyElements

Too many elements have been provided.

CertificateSigned,
InstallCertificate

SetChargingProfile,
SetVariables, SendLocalList

UnsupportedParam

A parameter was provided that is not supported.

(generic)

ValueOutOfRange

Provided value is out of range.

SetVariables,
SetVariableMonitoring

ValuePositiveOnly

Provided value is not greater than zero.

ValueTooHigh

ValueTooLow

Provided value is too high.

Provided value is too low.

ValueZeroNotAllowed

Provided value cannot be zero.

(generic)

(generic)

(generic)

(generic)

OCPP 2.1 - © Open Charge Alliance 2025

47/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 6. Standardized additionalInfo types

Chapter 6. Standardized additionalInfo types

(Updated in v2.0)

Standardized and recommended names (types) for idToken.additionalInfo that can be used to provide additional information
related to an ID token. For interoperability reasons, these types must be used if the data to be transmitted matches the description.

additionalInfo consists of:

• idToken.additionalInfo.type: specifies the type of content

• idToken.additionalInfo.additionalIdToken: specifies the value

6.1. Generic

AdditionalInfo fields can be used to provide additional information about an RFID card.

additionalInfo.type

EVCCID

ISO14443SubType

Description

The EVCCID of EV is added as additionalInfo to an idToken for ISO 15118-20
sessions.

The subtype of an ISO 14443 RFID card. For example: 'VDE-AR-E-2532-100' (a
secure variant of ISO 14443).

6.2. Ad hoc payment

A number of AdditionalInfo fields are prescribed to be used for ad hoc payment (use case C18). The configuration variable
PaymentCtrlr.PaymentDetails determines which values the payment terminal provides that need to be included in
idToken.additionalInfo for an ad hoc authorization.

additionalInfo.type

PspRef

SessionRef

MerchantRef

PaymentBrand

ReadingMethod

Description

Payment Service Provider reference id for payment session. Only use when
PspRef is not in idToken.

Payment session reference id from terminal (not the same as pspRef)

Merchant (CSO) reference id for payment session

Brand of ad hoc payment card. See predefined list of values in table below.

Contactless / Contact / Magstripe

PaymentRecognition

Credit/debit card or digital wallet. See predefined list of values in table below.

CardBin

CardLast4Digits

CardExpiryDate

HashedCardNr

WalletUserId

Card first 6 digits

Card last 4 digits

The expiry date of the card. Format: YYYY/MM

The hashed card number.

User ID for a digital wallet, e.g. Alipay.

6.2.1. Predefined values for PaymentBrand

PaymentBrand

Description

AMEX

ApplePay

Bancontact

BankAxept

Carnet

CartesBancaires

Dankort

Diners

Discover

OCPP 2.1 - © Open Charge Alliance 2025

48/52

Part 2 - Appendices

Chapter 6. Standardized additionalInfo types

Description

v2.1, 2025-12-03

PaymentBrand

EftposAustralia

Elo

Girocard

GooglePay

Hipercard

Interac

JCB

Maestro

Mastercard

SamsungPay

UnionPay

VPay

Visa

6.2.2. Predefined values for PaymentRecognition

Description

Credit card

Debit card

PaymentRecognition

CC

Debit

Alipay

ApplePay

GooglePay

GrabPay

PayPal

SamsungPay

WeChatPay

OCPP 2.1 - © Open Charge Alliance 2025

49/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 7. Standardized values for enumerations as string

Chapter 7. Standardized values for enumerations as string

7.1. IdTokenEnumStringType

Standardized values for idToken.type.

Before OCPP 2.1 this used to be an enumeration. This has been changed to a predefined set of strings for more flexibility.

(Updated in v2.0)

Value

Central

DirectPayment

eMAID

EVCCID

ISO14443

ISO15693

KeyCode

Local

MacAddress

NoAuthorization

VIN

Description

A centrally, in the CSMS (or other server) generated id (for example used for a
remotely started transaction that is activated by SMS). No format defined, might
be a UUID.

IdToken from a payment terminal that authorized a payment card. Usually a
reference id from payment service provider.

Electro-mobility account id as defined in ISO 15118

EVCCID of EV. For ISO 15118-2 this is the MAC address. For ISO 15118-20 this is
an identifier up to 255 characters.

ISO 14443 UID of RFID card. It is represented as an array of 4 or 7 bytes in
hexadecimal representation.

ISO 15693 UID of RFID card. It is represented as an array of 8 bytes in
hexadecimal representation.

A private key-code to authorize a charging transaction. For example: Pin-code.

A locally generated id (e.g. internal id created by the Charging Station). Needs no
checking by CSMS. No format defined, might be a UUID

MacAddress of the EVCC (Electric Vehicle Communication Controller) that is
connected to the EVSE. Used when MAC address is used for authorization
(Autocharge).

Transaction is started and no authorization possible. Charging Station only has a
start button or mechanical key etc. IdToken field SHALL be left empty.

Vehicle Identification Number of EV.

7.2. ChargingLimitSourceEnumStringType

Standardized values for a chargingLimitSource field.

Before OCPP 2.1 this used to be an enumeration. This has been changed to a predefined set of strings for more flexibility.

(Updated in v2.0)

Value

EMS

Other

SO

CSO

Description

Indicates that an Energy Management System has sent a charging limit.

Indicates that an external source, not being an EMS or system operator, has sent
a charging limit.

Indicates that a System Operator (DSO or TSO) has sent a charging limit.

Indicates that the CSO has set this charging profile.

7.3. ConnectorEnumStringType

Standardized values for a connectorType field.

• Fixed cable connections have a name that starts with "c" for captive cabled.

• Socket connections have a name that starts with "s" for socket.

• Wireless connections have a name that starts with "w" for wireless.

• Swappable battery types have a name that starts with "b" for battery.

OCPP 2.1 - © Open Charge Alliance 2025

50/52

Part 2 - Appendices

v2.1, 2025-12-03

Chapter 7. Standardized values for enumerations as string

Before OCPP 2.1 this used to be an enumeration. This has been changed to a predefined set of strings for more flexibility.

(Updated in v2.0)

Value

bBatterySlot

cCCS1

cCCS2

cChaoJi

cG105

cGBT-DC

cLECCS

cMCS

cNACS

cNACS-CCS1

cCCS1-NACS

cTesla

cType1

cType2

cUltraChaoJi

s309-1P-16A

s309-1P-32A

s309-3P-16A

s309-3P-32A

sBS1361

sCEE-7-7

sType1

sType2

sType3

wInductive

wResonant

OppCharge

Description

Slot of a battery swap station to accept battery cartridges (type unspecified)

Combined Charging System 1 (captive cabled) a.k.a. Combo 1

Combined Charging System 2 (captive cabled) a.k.a. Combo 2

ChaoJi (captive cabled) a.k.a. CHAdeMO 3.0

JARI G105-1993 (captive cabled) a.k.a. CHAdeMO (captive cabled)

GB/T 20234.3 DC connector (captive cabled)

Light Equipment Combined Charging System IS17017 (captive cabled)

Megawatt Charging System (captive cabled)

North American Charging Standard J3400 (captive cabled)

Built-in NACS to CCS1 adapter (captive cabled)

Built-in CCS1 to NACS adapter (captive cabled)

Tesla Connector (captive cabled)

IEC62196-2 Type 1 connector (captive cabled) a.k.a. J1772

IEC62196-2 Type 2 connector (captive cabled) a.k.a. Mennekes connector

Ultra-ChaoJi for megawatt charging (captive cabled)

16A 1 phase IEC60309 socket

32A 1 phase IEC60309 socket

16A 3 phase IEC60309 socket

32A 3 phase IEC60309 socket

UK domestic socket a.k.a. 13Amp

CEE 7/7 16A socket. May represent 7/4 and 7/5 a.k.a Schuko

IEC62196-2 Type 1 socket a.k.a. J1772

IEC62196-2 Type 2 socket a.k.a. Mennekes connector

IEC62196-2 Type 3 socket a.k.a. Scame

Wireless inductively coupled connection (generic)

Wireless resonant coupled connection (generic)

Pantograph down connector

Other1PhMax16A

Other single phase (domestic) sockets not mentioned above, rated at no more
than 16A. CEE7/17, AS3112, NEMA 5-15, NEMA 5-20, JISC8303, TIS166, SI 32,
CPCS-CCC, SEV1011, etc.

Other1PhOver16A

Other single phase sockets not mentioned above (over 16A)

Other3Ph

Pan

Undetermined

Unknown

Other 3 phase sockets not mentioned above. NEMA14-30, NEMA14-50.

Pantograph up connector

Yet to be determined (e.g. before plugged in)

Unknown/not determinable

7.4. SigningMethodEnumStringType

Standardized values for the signingMethod in a SignedMeterValueType.
Columns Algorithm, Curve, Key Length and Hash Algorithm are for information only and not part of the standardized value.

(Updated in v2.0)

SigningMethod

ECDSA-secp192k1-SHA256

ECDSA-secp256k1-SHA256

ECDSA-secp192r1-SHA256

ECDSA-secp256r1-SHA256

Algorithm

Curve

Key Length

Hash Algorithm

ECDSA

ECDSA

ECDSA

ECDSA

secp192k1

secp256k1

secp192r1

secp256r1

192 bits

256 bits

192 bits

256 bits

SHA-256

SHA-256

SHA-256

SHA-256

OCPP 2.1 - © Open Charge Alliance 2025

51/52

Part 2 - Appendices

v2.1, 2025-12-03

SigningMethod

ECDSA-brainpool256r1-SHA256

ECDSA-secp384r1-SHA256

ECDSA-brainpool384r1-SHA256

Chapter 7. Standardized values for enumerations as string

Algorithm

Curve

Key Length

Hash Algorithm

ECDSA

ECDSA

ECDSA

brainpool256r1

256 bits

secp384r1

384 bits

brainpool384r1

384 bits

SHA-256

SHA-256

SHA-256

OCPP 2.1 - © Open Charge Alliance 2025

52/52

Part 2 - Appendices

