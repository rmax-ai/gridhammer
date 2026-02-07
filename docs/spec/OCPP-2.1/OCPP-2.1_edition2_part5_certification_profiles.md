OCPP 2.1
Part 5 - Certification Profiles

Edition 2, 2025-12-03

Table of Contents

Disclaimer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  1
Version History . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  2
1. Introduction & Reading Guide . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
2. Certification profiles . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  4
3. Features. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  8
3.1. Optional feature list for charging station . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  9
3.2. Optional feature list for CSMS . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  13
4. List of test cases . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15
4.1. Introduction. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15
4.2. Test Cases Core . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15
4.3. Test Cases Advanced Security . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  50
4.4. Test Cases Smart Charging . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  51
4.5. Test Cases ISO 15118 Support. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  56
4.6. Test Cases Bidirectional Power Transfer (2.1). . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  60
4.7. Test Cases DER Control (2.1) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  61
5. OCPP 2.1 Mandatory Controller components per profile . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  62
6. Appendix A: additional questions for the Protocol Implementation Conformance Statement . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  63
6.1. Questions for Charging Stations . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  63
6.2. Questions for CSMSs. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  63
7. Appendix B: Hardware feature set . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  64
8. Appendix C: Features vs. OCPP use cases . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  65

Edition 2, 2025-12-03

Disclaimer

Copyright © 2010 – 2025 Open Charge Alliance. All rights reserved.

This document is made available under the *Creative Commons Attribution-NoDerivatives 4.0 International Public License*
(https://creativecommons.org/licenses/by-nd/4.0/legalcode).

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

1/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Version History

Version

2.1 Edition 2

Date

2025-12-03

Description

OCPP 2.1 Edition 2. This is the first published version of
OCPP 2.1 Part 5.

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

2/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Chapter 1. Introduction & Reading Guide

This document describes the certification profiles for OCPP 2.1. These profiles are sets of use cases that can be certified via the
Open Charge Alliance. This document contains the details on what is part of the OCPP 2.1 Certification. This document contains:

• The certification profiles and an overview of the functionality per profile.

• The list of optional features. This list contains specific functionality that is not mandatory for certification, but which can

optionally be certified.

• The list of test cases for each of the certification profiles.

• The overview of the controller components that must be implemented per profile for certification testing.

For clarity: in the context of the OCPP Certification Program, the term test case refers to a sequence of messages for testing a use
case from OCPP. The term feature refers to a functionality, that can be tested with one or more test cases (see Features for a more
detailed explanation). Instead of making specific test cases mandatory or optional, the certification program for OCPP 2.1 works
with features that are optional. Depending on whether the System Under Test (SUT) has implemented a feature, the test case(s)

that belong to this feature, must be successfully passed or not.

Figure 1. Link between different OCPP Documents in OCPP Certification Program

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

3/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Chapter 2. Certification profiles

The OCPP protocol has been designed to support a wide variety of charging stations ranging from simple AC home chargers to
advanced DC hyperchargers and megawatt chargers. It will be obvious that these charging stations and associated CSMSs will
have very different capabilities. As a result it does not make sense to require every vendor to certify for the full OCPP functionality,
when only subset is needed for the specific application.

The OCPP certification is built around certification profiles that describe a set of supported functions. A full OCPP certification
comprises all certification profiles, but it is possible to get certified for a subset, since not all OCPP functionality may be needed for
some vendors.

The OCPP "Core" profile must always be present. It contains the basic OCPP functionality. On top of that other profiles can be
added to the certification. These profiles are independent of each other, the only exception being the "ISO 15118 support" profile,
which requires a number of "Advanced security" and "Smart charging" test cases to be implemented.

The following table lists the certification profiles and an overview of the functionality per profile:

Table 1. Certification profiles

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

4/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Certification Profile

Description

Core

Basic Authentication

TLS - server-side certificate

Update Charging Station Password for HTTP Basic Authentication

Security Event Notification

Booting a Charging Station

Configuring a Charging Station

Resetting a Charging Station / EVSE

Authorization incl. GroupId

Stop Transaction with a Master Pass

Local start transaction - Cable plugin first & Authorization first

Start / Stop transaction options

Disconnect cable on EV-side

Check Transaction status

Remote start / stop transaction

Remote unlock Connector

Remote Trigger

Change Availability - Charging Station / EVSE / Connector

Clock-aligned Meter & Sampled Meter Values

Install CA certificates

Retrieve & delete certificates from Charging Station

AdditionalRootCertificateCheck

Retrieve Log Information

Get / Clear Customer Information

Secure Firmware Update

Store / Clear Authorization Data in Authorization Cache

Authorization through authorization cache

Local Authorization List Management

Authorization through local authorization list

Send / Get Local Authorization List

Advanced Device Management

Get /Monitoring report

Set Monitoring Base

Set Variable Monitoring

Set Monitoring Level

Get Custom Report

Clear / Remove Monitoring

Event Notification

Advanced User Interface

Set / Get Display Message

Clear a Display Message

Show EV Driver Running / Final Total Cost During / After Charging

Show EV Driver-specific Tariff Information

Update Tariff Information During Transaction

Configure Fallback Tariff Information & Total Cost Message

Reservation

Reserve a specific EVSE

Reserve an unspecified EVSE

Reserve a connector with a specific type

Reservations using GroupIdToken
Cancel reservation of an EVSE

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

5/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Certification Profile

Description

Payment (2.1)

Set Default Tariff

Local Cost Calculation

Prepaid card

Integrated Payment Terminal

Standalone Payment Terminal

Settlement
QR codes

Advanced Security

TLS - Client-side certificate

Update Charging Station Certificate
Upgrade Charging Station Security Profile

Smart Charging (2.0.1 / 2.1)

Set charging profile

Remote start transaction with charging profile

Get Composite Schedule

Get Charging Profile

Clear Charging Profile

Priority Charging (2.1)

Dynamic Charging Profiles (2.1)

EMS Control

Dynamic charging profiles by external system

External V2X control (Bidirectional Power Transfer)

External V2X control (with ISO 15118-20)

ISO 15118 support (2.0.1 / 2.1)

This certification profile covers both ISO 15118-2 (2.0.1 and 2.1) as well as ISO 15118-

20 (2.1).

ISO 15118 Certificate Management:

Update Charging Station Certificate

(Contract) Certificate Installation / Update EV

Get Certificate Status

Install V2G / MO / OEMRoot CA certificates

Retrieve V2G / MO certificates from Charging Station

Delete a certificate from a Charging Station

ISO 15118 EIM / PnC Authorization:

Authorization using External Identification Means

Authorization using Contract Certificates

ISO 15118 Smart Charging:

Set charging profile

Remote start transaction with charging profile

Get Composite Schedule

Get Charging Profile

Clear Charging Profile

Renegotiating a Charging Schedule
ISO 15118 signed meter values

Central & Local V2X control

Frequency support

V2X Authorisation - ISO15118-20

Idle, minimizing energy consumption

Bidirectional Power Transfer (2.1)

Prerequisite for this certification profile is supporting the Certification profile Smart
Charging.

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

6/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Certification Profile

DER control (2.1)

Description

Starting a V2X session with DER control

Configure DER control settings at CS

Charging station reporting a DER event

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

7/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Chapter 3. Features

The concept of certification profiles is not enough to cover the variety in OCPP implementations. The OCPP specification contains
many optional features, often in the form of optional message fields or configuration variables, that a vendor can use to support
advanced functions. Whereas the certification profiles determine which OCPP functionality is implemented, the features describe
how much of a certain functionality in a profile has been implemented.

A Test System uses the features to determine which test cases have to be executed for a charging station or CSMS. For example,
the set of TxStartPoints that a charging station supports, has a big impact on the execution of certain test cases. The behavior of a
charging station that starts a transaction based on a successful authorization is different from a charging station that starts a
transaction as soon as a cable is connected. Similarly, a CSMS that only controls DC fast chargers will not need functionality to
unlock a cable at the charging station. For such a CSMS the vendor may decide to not implement the feature Support for unlocking
connector.

In most cases a feature corresponds the existence of a configuration variable or its value.

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

8/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

3.1. Optional feature list for charging station

The following table lists the optional features. These are features that are not mandatory to implement within a certification profile.
Where applicable the associated configuration variable is mentioned in parenthesis.

Table 2. Optional features for charging stations

Id

Core

C-01

C-02

C-03

C-04

C-06

C-06.1

C-06.2

C-07

C-08

C-09

C-09.1

C-09.2

C-09.3

C-09.4

C-09.5

C-09.6

C-10

C-10.1

C-10.2

C-10.3

Feature

Charging Station

Support for offline authorization of transactions

Support for allowing Offline Authorization for Unknown Ids
(OfflineTxForUnknownIdEnabled)

Support for maximizing energy for invalid ids (MaxEnergyOnInvalidId)

Support to limit StatusNotifications (MinimumStatusDuration)

Authorization status after cable disconnected on EV side
(StopTxOnEVSideDisconnect)

Conditional.
Supporting this feature depends
on whether at least one of the
following feature combinations

is supported;
- Certification Profile: Local
Authorization List Management
AND at least one of the
following local authorization

options; C-30 or C-31 or C-32
- C-02: Support for allowing
offline authorization for
unknown ids AND at least one
of the following local
authorization options; C-30 or

C-31 or C-32 or C-33 or C-34
- C-49: Authorization Cache
AND at least one of the
following local authorization

options; C-30 or C-31 or C-32
- C-35: Local Authorization -
NoAuthorization - start
(Because there is no
authorization, no local
authorization mechanism is
needed.)

Optional

Optional

Optional

(At least one of the suboptions
below is required)

Support for maintaining authorization when cable disconnected on EV side

Optional

Support for not maintaining authorization when cable disconnected on EV side

Optional

Support for using a Master Pass for charging stations with UI
(MasterPassGroupId)

Support for using a Master Pass for charging stations without UI
(MasterPassGroupId)

Supported Transaction Start points (TxStartPoint)

Optional

Optional

(At least one of the suboptions
below is required)

Start transaction options - EVConnected

Start transaction options - Authorized

Start transaction options - DataSigned

Start transaction options - PowerPathClosed

Start transaction options - EnergyTransfer

Start transaction options - ParkingBayOccupancy

Supported Transaction Stop points (TxStopPoint)

Stop transaction options - EVConnected

Stop transaction options - Authorized

Stop transaction options - PowerPathClosed

Optional

Optional

Optional

Optional

Optional

Optional

(At least one of the suboptions
below is required)

Optional

Optional

Optional

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

9/68

Part 5 - Certification Profiles

C-23

C-25

C-26

C-28

C-29

C-29.1

C-29.2

C-29.3

C-29.4

C-29.5

C-29.6

Edition 2, 2025-12-03

Id

C-10.4

C-10.5

C-12

C-12.1

C-12.2

C-13

C-14

C-20

C-21

Feature

Stop transaction options - EnergyTransfer

Stop transaction options - ParkingBayOccupancy

Charging Station

Optional

Optional

Unlocking of connector when cable disconnected on EV side
(UnlockOnEVSideDisconnect)

(At least one of the suboptions
below is required)

Support for unlocking connector when cable disconnected on EV side

Support for not unlocking when cable disconnected on EV side

Support for Reset per EVSE (AllowReset)

Support for retrieving / deleting CustomerInformation - CustomerIdentifier

Allowing New Sessions Pending a FirmwareUpdate
(AllowNewSessionsPendingFirmwareUpdate )

Support for queuing all or only Transaction related messages until they are
delivered to the CSMS (QueueAllMessages)

Optional

Optional

Optional

Optional

Optional

Optional

Time related settings

Supported time sources (TimeSource)

Support for setting a TimeOffset (TimeOffset)

Support for setting the TimeZone (TimeZone)

Toggle sending clock aligned meter values when a transaction is ongoing / Idle
(AlignedDataSendDuringIdle)

TriggerMessage

{ list } at least Heartbeat

Optional

Optional

Optional

(Select all supported
suboptions)

Trigger message - MeterValues

Trigger message - TransactionEvent

Trigger message - LogStatusNotification

Trigger message - FirmwareStatusNotification

Trigger message - StatusNotification

Trigger message - BootNotification

C-29.7 (2.1)

Trigger message - CustomTrigger

Authorization options for local start

C-30

C-31

C-32

C-33

C-34

C-35

Authorization - using RFID ISO14443

Authorization - using RFID ISO15693

Authorization - using KeyCode

Authorization - using locally generated id

Authorization - MacAddress

Authorization - NoAuthorization

Authorization options for remote start (mandatory to support at least one)

C-36

C-37

C-38

C-39

C-40

C-40.1

C-40.2

C-40.3

C-40.4

C-40.5

Authorization - using RFID ISO14443

Authorization - using RFID ISO15693

Authorization - using centrally, in the CSMS (or other server) generated id

Authorization - NoAuthorization

Supported MeterValue Measurands

SampledTxStartedMeasurands

SampledTxUpdatedMeasurands

SampledTxEndedMeasurands

AlignedDataMeasurands

AlignedDataTxEndedMeasurands

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

(Please fill in the following
fields)

{ list of supported } at least one

{ list of supported } at least one

{ list of supported } at least one

{ list of supported } at least one

{ list of supported } at least one

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

10/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Id

C-41

Feature

Supported Cipher Suites

C-42

C-43

C-47

C-48

C-48.1

C-48.2

C-58

C-49

C-59

C-51

C-52

C-53

C-54

C-56

C-57

C-60

C-61

Signed Metervalues (SampledDataSignReadings)

Install Firmware with ongoing transaction(s)

Support for falling back to default OCPP reconnection mechanism when
NetworkConnection profile connection has failed

Authorization of remote start (AuthorizeRemoteStart)

Option for authorization in case of a remote start

Option for no authorization in case of a remote start

Option for disabling remote authorization (DisableRemoteAuthorization)

Authorization Cache (AuthCacheEnabled)

Option for disabling remote authorization for cached invalid idTokens
(AuthCacheDisablePostAuthorize)

Configurable TxStartPoint

Configurable TxStopPoint

Support for lifetime cached token (AuthCacheLifeTime)

Support for providing the SummaryInventory

Support for cancelling ongoing log file upload

Support for cancelling ongoing firmware update

Security Profile 1 - Unsecured Transport with Basic Authentication

C-62 (2.1)

Support for resuming transactions (ImmediateAndResume)

C-63 (2.1)

Support for transaction limits

C-64 (2.1)

Support for resuming transaction after interruption

C-65 (2.1)

Support for DataCollectorLog

Authorization options for local stop

C-70

C-71

C-72

C-75

Reservation

Authorization - using RFID ISO14443

Authorization - using RFID ISO15693

Authorization - using KeyCode

Authorization - NoAuthorization

R-0

R-1

R-2

R-3

Support for Reservation

Support for reservations of connectorType

Support for reservation of unspecified EVSE (ReservationNonEvseSpecific)

Support for disabling Reservations (ReservationEnabled)

Advanced Device Management

DM-0

Support for Advanced Device Management

Charging Station

{ list of cipher suites } → at
least one of
TLS_ECDHE_ECDSA_WITH_AES

_128_GCM_SHA256
TLS_ECDHE_ECDSA_WITH_AES

_256_GCM_SHA384

OR
TLS_RSA_WITH_AES_128_GCM

_SHA256
TLS_RSA_WITH_AES_256_GCM
_SHA384

Optional

Optional

Optional

(At least one of the suboptions
below is required)

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Conditional.
Supporting this feature depends
on whether at least one
connectorType is supported
that is part of the
ConnectorEnumType list from
part 2 specification.

Optional

Optional

Optional

Supported policies for replacing cached entries (AuthCachePolicy)

{ list of supported } at least one

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

11/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Id

DM-3

Feature

Queue notifyEventRequest messages for specific severities
(OfflineMonitoringEventQueuingSeverity)

Local Authorization List Management

LA-0

LA-1

LA-3

Support for Local Authorization List Management

Authorization list support (LocalAuthListEnabled)

Option for disabling remote authorization for invalid idTokens stored at the Local
Authorization List (LocalAuthListDisablePostAuthorize)

Advanced User Interface

UI-0

UI-1

UI-1.1

UI-1.2

UI-1.3

UI-2

UI-2.1

UI-2.2

UI-2.3

UI-2.4

Support for Advanced User Interface

Supported message priorities (DisplayMessageSupportedPriorities)

AlwaysFront

InFront

NormalCycle

Supported message formats (DisplayMessageSupportedFormats)

ASCII

HTML

URI

UTF8

UI-3 (2.1)

Multi-language support

Payment (2.1)

P-0

P-1

P-2

P-2.1

P-2.2

P-2.3

P-2.4

Support for Payment

Support for Tariff conditions

Supported Payment options

Payment by prepaid card

Integrated payment terminal

Stand alone payment terminal

QR code payment

Advanced Security

Additional root certificate check mechanism implemented
(AdditionalRootCertificateCheck)

Update Charging Station Certificate - CertificateSignedRequest Timeout
(CertSigningWaitMinimum,CertSigningRepeatTimes)

Security downgrades from profile 3 to 2 (AllowSecurityProfileDowngrade)

Charging Station

Optional

Optional

Optional

Optional

Optional

(At least one of the suboptions
below is required, if UI-0 is
supported)

Optional

Optional

Optional

(At least one of the suboptions
below is required, if UI-0 is
supported)

Optional

Optional

Optional

Optional

Optional, based on AQ-20

Optional

Optional

(At least one of the suboptions
below is required, if P-0 is
supported)

Optional

Optional

Optional

Optional

Optional

Optional

(At least one of the suboptions
below is required)

Support for disallowing security downgrades from profile 3 to 2

Support for allowing security downgrades from profile 3 to 2

Optional

Optional

Smart Charging (2.0.1 / 2.1)

SC-2

Supported charging rate units (ChargingScheduleChargingRateUnit)

SC-2.1

SC-2.2

A

W

SC-3 (2.1)

Support for limiting based on SoC (limitAtSoC)

SC-5 (2.1)

Support for using local time (useLocalTime)

SC-5.1

SC-5.2

TimeOffset

TimeZone

(Select all supported
suboptions)

Optional

Optional

Optional

(At least one of the suboptions
below is required)

Optional

Optional

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

12/68

Part 5 - Certification Profiles

AS-2

AS-3

AS-4

AS-4.1

AS-4.2

Edition 2, 2025-12-03

Id

Feature

SC-6 (2.1)

Support for using priority charging (PriorityCharging)

SC-7 (2.1)

Support for using randomized delays (randomizedDelay)

SC-8 (2.1)

Support for dynamic charging profiles

SC-9 (2.1)

Support for operationMode Idle

SC-9.1 (2.1)

Support for operationMode Idle with EvseSleep

EMS Control (2.1)

SC-10 (2.1)

Support for EMS Control

ISO 15118 support (2.0.1 / 2.1)

ISO-1 (2.1)

Supported ISO 15118 version

ISO-1.1

ISO 15118-2

ISO-1.2

ISO 15118-20

ISO-4

ISO-5

Support for retrieving / deleting CustomerInformation - CustomerCertificate

Charging Station can provide a contract certificate that it cannot validate to the
CSMS (CentralContractValidationAllowed)

ISO-6 (2.1)

Support for ServiceRenegotiation

Bidirectional Power Transfer (2.1)

BPT-1

BPT-2

Frequency support

Support for local loadbalancing (LocalLoadBalancing)

DER control (2.1)

No optional features for this profile

3.2. Optional feature list for CSMS

Charging Station

Optional

Optional

Optional

Optional

Optional

Optional

(At least one of the suboptions
below is required)

Optional, required if ISO-1.2 is
supported

Optional

Optional

Optional

Optional

Optional

Optional

The features of a CSMS are not determined by configuration variables. Features in the list below are allowed not to be supported by
a CSMS.

Table 3. Optional features for CSMS

Id

Core

C-11

C-13

C-14

C-15

C-16

C-17

C-29

C-29.1

C-29.2

C-29.3

C-29.4

C-29.5

Feature

Support for unlocking connector for charging station with detachable cable
(UnlockConnector message).

Support for Reset per EVSE

Support for retrieving / deleting CustomerInformation - CustomerIdentifier

Support for scheduled firmware updates

Support for checking the TransactionStatus

Support for retrieving the ConfigurationInventory

TriggerMessage

Trigger message - MeterValues

Trigger message - TransactionEvent

Trigger message - LogStatusNotification

Trigger message - FirmwareStatusNotification

Trigger message - StatusNotification

C-29.7 (2.1)

Trigger message - CustomTrigger

Authorization options for local start

C-30

C-31

C-32

Authorization - using RFID ISO14443

Authorization - using RFID ISO15693

Authorization - using KeyCode

CSMS

Optional

Optional

Optional

Optional

Optional

Optional

(Select all supported
suboptions)

Optional

Optional

Optional

Optional

Optional

Optional

Required

Required

Optional

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

13/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Id

C-33

C-34

C-35

Feature

Authorization - using locally generated id

Authorization - MacAddress

Authorization - NoAuthorization

Authorization options for remote start (mandatory to support at least one)

C-36

C-37

C-38

C-39

C-44

C-45

C-46

C-50

C-50.1

C-50.2

C-61

Authorization - using RFID ISO14443

Authorization - using RFID ISO15693

Authorization - using centrally, in the CSMS (or other server) generated id

Authorization - NoAuthorization

Support for sending a BootNotification Pending before Accepting

Support for Multiple elements GetVariablesRequest

Support for Multiple elements SetVariablesRequest

GetBaseReport - FullInventory

GetBaseReport - FullInventory - During onboarding

GetBaseReport - FullInventory - Manual trigger

Security Profile 1 - Unsecured Transport with Basic Authentication

C-65 (2.1)

Support for DataCollectorLog

C-76 (2.1)

Support for Battery Swapping Stations

Reservation

R-0

R-1

R-2

Support for Reservation

Support for reservations of connectorType

Support for reservations of unspecified EVSE

Local Authorization List Management

LA-0

LA-2

Support for Local Authorization List Management

Support for GetLocalListVersion

Advanced Device Management

DM-0

Support for Advanced Device Management

Advanced User Interface

UI-0

Support for Advanced User Interface

Payment (2.1)

P-0

Support for Payment

Advanced Security

No optional features for this profile

Smart Charging (2.0.1 / 2.1)

SC-3 (2.1)

Support for limiting based on SoC (limitAtSoC)

SC-4

Support for TxDefaultProfile on EVSEID #0

SC-6 (2.1)

Support for using priority charging (PriorityCharging)

SC-7 (2.1)

Support for using randomized delays (randomizedDelay)

SC-10 (2.1)

Support for EMS Control

ISO 15118 support (2.0.1 / 2.1)

CSMS

Optional

Optional

Required

Required

Required

Optional

Optional

Optional

Optional

Optional

(At least one of the suboptions
below is required)

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

Optional

ISO-4

Support for retrieving / deleting CustomerInformation - CustomerCertificate

Optional

Bidirectional Power Transfer (2.1)

BPT-1

Frequency support

DER control (2.1)

No optional features for this profile

Optional

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

14/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Chapter 4. List of test cases

4.1. Introduction

This table lists the test cases that are part of the OCPP Certification program. For each of the test cases, the columns "Conf. Test for Charging Station" and "Conf. Test for CSMS" indicate

whether the test case is mandatory or not within a Certification Profile. The abbreviations have the following meaning:

• M = Mandatory . This means that IF you implement the certification profile this test case belongs to, you MUST successfully pass this test case.

• C = Conditional. This means that IF you meet a condition, you MUST pass this test case. Most conditions refer to the optional features that are listed in the Features.

4.2. Test Cases Core

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

Basic Authentication

TC_A_01

Valid username/password combination

M

TC_A_02

Username does not equal ChargingStationId

TC_A_03

Invalid password

Update Charging Station Password for HTTP
Basic Authentication

TC_A_09

Accepted

TC_A_10

Rejected

TLS - server-side certificate

TC_A_04

Valid certificate

TC_A_05

Invalid certificate

TC_A_06

TLS version too low

Upgrade Charging Station Security Profile

TC_A_19

Accepted

TC_A_20

No valid CSMSRootCertificate installed

M

M

M

M

M

C

C

M

M

M

M

M

M

M

C

If the last CSMSRootCertificate can be
removed and Security Profile 1 is supported.

C-61 OR Advanced
Security

AQ-1 and C-61

Can the last CSMSRootCertificate

be removed?
Security Profile 1 - Unsecured
Transport with Basic
Authentication

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

15/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_A_22

Downgrade security profile - Rejected

Cold Boot Charging Station

TC_B_01

Accepted

TC_B_02

Pending

TC_B_03

Rejected

TC_B_30

Pending/Rejected - SecurityError

TC_B_31

Pending/Rejected - TriggerMessage

Status change during offline period

TC_B_51

> Offline Threshold

TC_B_52

< Offline Threshold

Get Variables

TC_B_06

single value

TC_B_07 multiple values

TC_B_32

Unknown component

TC_B_33

Unknown variable

TC_B_34

Not supported attribute type

Set Variables

TC_B_09

single value

TC_B_10 multiple values

TC_B_35

Unknown component

TC_B_36

Unknown variable

TC_B_37

Not supported attribute type

TC_B_11

invalidly formatted values

TC_B_39

Read-only

Get Base Report

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

CSMS: If Pending mechanism is implemented

C-44

BootNotification Pending

For CSMS: if CSMS can be configured to first
respond to a BootNotificationRequest with
status Pending or Rejected

C-44 or NOT AQ-16

For CSMS: if CSMS can be configured to first
respond to a BootNotificationRequest with
status Pending or Rejected

C-44 or NOT AQ-16

BootNotification Pending or
Does the CSMS reject unknown
Charging Stations during
websocket connection setup?

BootNotification Pending or
Does the CSMS reject unknown
Charging Stations during
websocket connection setup?

If the CSMS supports multiple elements in a
GetVariablesRequest

C-45

multiple values elements
GetVariablesRequest

If the CSMS supports multiple elements in a
SetVariablesRequest

C-46

multiple values elements
SetVariablesRequest

M

C

C

C

M

C

M

C

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

16/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_B_12

ConfigurationInventory

TC_B_13

FullInventory

TC_B_14

SummaryInventory

TC_B_15

Not Supported base report

TC_B_53

Test mandatory DM variables via FullInventory

Reset Charging Station

Without ongoing transaction - OnIdle

With Ongoing Transaction - OnIdle

TC_B_20
(2.1)

TC_B_21
(2.1)

TC_B_22 With Ongoing Transaction - Immediate

TC_B_23

Unavailable persists reset

TC_B_41
(2.1)

With multiple ongoing transactions - OnIdle

Reset EVSE

TC_B_25 Without ongoing transaction

TC_B_26 With Ongoing Transaction - OnIdle

TC_B_27 With Ongoing Transaction - Immediate

TC_B_28

Not Supported

TC_B_29 With ongoing transaction - Not Supported

Reset ImmediateAndResume

TC_B_101
(2.1)

With Ongoing Transaction -
TxResumptionTimeout 0 or <omitted> -
Rejected

TC_B_102
(2.1)

With ongoing transaction - Energy Transfer
Suspended

TC_B_103
(2.1)

With Ongoing Transaction - Resuming Energy
Transfer

M

M

C

C

M

C

C

M

M

C

C

C

C

C

C

M

C

C

C

C

M

M

M

C

C

C

M

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C-17 OR DM-0

ConfigurationInventory

C-50.2

GetBaseReport - FullInventory -
Manual trigger

For CS: If reportBase SummaryInventory is not
supported. This is the case when Certification
Profile Advanced Device Management is not
supported.

C-56 OR DM-0

Not C-56

NOT HFS-13

NOT HFS-13

For CS: if no. of EVSEs > 1

HFS-8 > 1 AND NOT
HFS-13

For CS: Charging Station does not support
resetting an individual EVSE

For CS: Charging Station does not support
resetting an individual EVSE

C-13

C-13

C-13

Reset per EVSE

Reset per EVSE

Reset per EVSE

NOT C-13

Reset per EVSE

NOT C-13

Reset per EVSE

C-62

C-62

Support for resuming transactions

Support for resuming transactions

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

17/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C-62

Support for resuming transactions

M

M

M

M

M

TC_B_104
(2.1)

Without ongoing transaction

Set new NetworkConnectionProfile

TC_B_42

Accepted

TC_B_43

Rejected

TC_B_44

Failed

Set new NetworkConnectionProfile (2.1)

TC_B_100
(2.1)

Identity and password

TC_B_105
(2.1)

Add new NetworkConfiguration using
SetVariables

TC_B_107
(2.1)

Add and remove slot from
NetworkConfigurationPriority

TC_B_108
(2.1)

Prevent overwriting configured Network Profile
slot

TC_B_109
(2.1)

TC_B_110
(2.1)

TC_B_111
(2.1)

When changing
SecurityCtrlr.Identity/BasicAuthPassword the
NetworkProfiles.Identity/BasicAuthPassword
must be cleared

No security downgrade to profile #1

No security downgrade to profile #1 - DM

Migrate to new ConnectionProfile

TC_B_45

Success - Same CSMS Root

TC_B_46

Fallback mechanism - Same CSMS Root

TC_B_47

Fallback after
NetworkProfileConnectionAttempts per
NetworkConfigurationPriority failed - New
CSMS Root - New CSMS

C

M

M

M

M

M

M

M

M

M

M

C

For CS: at least two configuration slots for
networkConnectionProfiles must be supported

For CS: at least two configuration slots for
networkConnectionProfiles must be supported

For CS: at least two configuration slots for
networkConnectionProfiles must be supported

AS-2 and C-47

Additional Root Certificate check
mechanism implemented &
Reconnect after
NetworkProfileConnectionAttempt
s

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

18/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

TC_B_49

Fallback after
NetworkProfileConnectionAttempts per
NetworkConfigurationPriority failed - Same
CSMS Root

TC_B_50

Success - New CSMS Root - New CSMS

Network Reconnection

TC_B_57

After connection loss

TC_B_58 WebSocket Subprotocol negotiation

Local start transaction

TC_C_02

Authorization Invalid/Unknown

C

M

M

C

M

M

TC_C_06

Authorization Blocked

C

M

TC_C_07

Authorization Expired

C

M

For CS: at least two configuration slots for
networkConnectionProfiles must be supported

C-47

Reconnect after
NetworkProfileConnectionAttempt
s

For CS: at least two configuration slots for
networkConnectionProfiles must be supported

(C-30 or C-31 or C-32)
and NOT AQ-2

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /

KeyCode and
Does the Charging Station have a
cable lock, which prevents the EV
driver to connect the EV and EVSE
before authorization?

NOT AQ-2 and (C-30 or
C-31 or C-32)

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode

NOT AQ-2 and (C-30 or
C-31 or C-32)

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode

Charging Station:
- The Charging Station supports at least one of
the following local start authorization options

C-30, C-31, C-32
- The Charging Station does NOT have a cable
lock that prevents the EV driver to connect the
EV and EVSE before authorization.

For CS:
- The Charging Station supports at least one of
the following local start authorization options:

C-30, C-31, C-32
- The Charging Station does NOT have a cable
lock, which prevents the EV driver to connect

the EV and EVSE
before authorization.

For CS:
- The Charging Station supports at least one of
the following local start authorization options :

C-30, C-31, C-32
- The Charging Station does NOT have a cable
lock, which prevents the EV driver to connect

the EV and EVSE
before authorization.

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

19/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

TC_E_38
(2.1)

EV not ready

TC_C_56

Authorization Unknown

TC_C_05

Authorization invalid - Cable lock

C

C

C

TC_C_100
(2.1)

Authorization first - Cable plugin timeout

C

M

For CS: not supporting start transaction
options EnergyTransfer

Charging Station:
- The Charging Station supports at least one of
the following local start authorization options:
C-30, C-31, C-32

For CS:
- The Charging Station has a cable lock, which
prevents the EV driver to connect the EV and

EVSE before authorization.
- The Charging Station supports at least one of
the following local start authorization options

C-30, C-31, C-32
- The Charging Station does NOT have the
following configuration: TxStartPoint ReadOnly
AND value Authorized is NOT set.

For CS:
- The Charging Station supports at least one of
the following local start authorization options

C-30, C-31, C-32
- The Charging Station does NOT have the
following configuration: TxStartPoint ReadOnly
AND value Authorized is NOT set.

NOT C-09.5 and NOT
Product Subtype
"Mode 1/2-only
Charging Station" AND
NOT HFS-13

(C-30 or C-31 or C-32)
and NOT AQ-2

Start transaction options -
EnergyTransfer

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode

(C-30 or C-31 or C-32)
and AQ-2

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode

(C-30 or C-31 or C-32) Local Authorization - using RFID

ISO14443 / RFID ISO15693 /
KeyCode

Local Stop Transaction

TC_C_04

Different idToken

TC_E_06

Accepted

Authorization by GroupId

C

C

Charging Station:
- The Charging Station supports at least one of
the following local start authorization options:
C-70, C-71, C-72

The Charging Station supports E07 Transaction
locally stopped by IdToken with at least one of
the following local start authorization options:
C-70, C-71, C-72, C-75

C-70 or C-71 or C-72 Local Authorization - using RFID

ISO14443 / RFID ISO15693 /
KeyCode

C-70 or C-71 or C-72 or
C-75

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode / NoAuthorization

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

20/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_C_39

Success

TC_C_45 Master pass - Not able to start transaction +

groupId

TC_C_42

Not stopped by GroupId

Offline Authorization

TC_C_26

Unknown Id

Stop Transaction with a Master Pass

TC_C_47 With UI - All transactions

TC_C_48 With UI - With UI - Specific transactions

TC_C_49 Without UI

Store Authorization Data in the Authorization
Cache

TC_C_32

Persistent over reboot

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

C

C

M

For CS: the Charging Station supports at least
one of the following local start authorization
options: C-30, C-31, C-32

(C-30 and C-70) or (C-
31 and C-71) or (C-32
and C-72)

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode

For CS: the Charging Station supports at least
one of the following local start authorization
options: C-30, C-31, C-32 and Master Pass

((C-30 and C-70) or (C-
31 and C-71) or (C-32
and C-72)) AND (C-07
OR C-08)

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode and Master Pass

For CS: the Charging Station supports at least
one of the following local start authorization
options: C-30, C-31, C-32

(C-30 and C-70) or (C-
31 and C-71) or (C-32
and C-72)

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode

If the feature Unknown Offline Authorization is
supported AND the Charging Station supports
at least one of the following local start
authorization options: C-30, C-31, C-32, C-33, C-
34

C-02 and (C-30 or C-31
or C-32 or C-33 or C-
34)

Unknown Offline Authorization

M

M

M

CS: If the feature Master Pass with UI is
supported AND the Charging Station supports
at least one of the following local start
authorization options: C-70, C-71, C-72

CS: If the feature Master Pass with UI is
supported AND the Charging Station supports
at least one of the following local start
authorization options: C-70, C-71, C-72

CS: If the feature Master Pass with UI is
supported AND the Charging Station supports
at least one of the following local start
authorization options: C-70, C-71, C-72

C-07 and (C-70 or C-71
or C-72)

Master Pass - With UI

C-07 and (C-70 or C-71
or C-72)

Master Pass - With UI

C-08 and (C-70 or C-71
or C-72)

Master Pass - Without UI

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

Authorization Cache

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

21/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_C_33

Update on AuthorizeResponse

TC_C_34

Update on TransactionResponse

TC_C_36

AuthCacheCtrlr.LocalPreAuthorize = false

TC_C_46

AuthCacheLifeTime

Clear Authorization Data in Authorization
Cache

TC_C_37

Accepted

TC_C_38

Rejected

Authorization by GroupId

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

If the Charging Station has an authorization
cache AND supports to set a lifetime for its
entries AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

C-49 and C-53 and (((C-
30 and C-70) or (C-31
and C-71) or (C-32 and
C-72)) or (C-36 or C-
37))

Authorization Cache

Authorization Cache

Authorization Cache

Authorization Cache &
AuthCacheLifeTime

M

M

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

Authorization Cache

Authorization Cache

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

22/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_C_41

Success with Authorization Cache

TC_C_44

Invalid status with Authorization Cache

Authorization through authorization cache

TC_C_08

Accepted

TC_C_09

Invalid & Not Accepted

TC_C_12

Invalid & Accepted

TC_C_10

Blocked

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

For CS:
- The Charging Station supports at least one of
the following local start authorization options:

C-49 and ((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72))

Authorization Cache

C-30, C-31, C-32
- If the Charging Station has an authorization
cache.

For CS:
- The Charging Station supports at least one of
the following local start authorization options:

C-49 and ((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72))

Authorization Cache

M

C-30, C-31, C-32
- If the Charging Station has an authorization
cache.

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

Authorization Cache

Authorization Cache

Authorization Cache

Authorization Cache

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

23/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_C_11

Expired

TC_C_13

Accepted but cable not connected yet.

TC_C_15

StopTxOnInvalidId = false,
MaxEnergyOnInvalidId > 0

TC_C_16

StopTxOnInvalidId = true

TC_C_17

StopTxOnInvalidId = false

TC_C_18

StopTxOnInvalidId = true,
MaxEnergyOnInvalidId > 0

TC_C_20

Invalid

TC_C_57

AuthCacheDisablePostAuthorize

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

C

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32 OR
supports at least one of the following remote
start authorization options: C-36, C-37

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start

authorization options: C-30, C-31, C-32
If MaxEnergyOnInvalidId is implemented.

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32

If the Charging Station has an authorization
cache AND the Charging Station supports at
least one of the following local start

authorization options: C-30, C-31, C-32
If MaxEnergyOnInvalidId is implemented.

M

If the Charging Station supports the option for
disabling remote authorization for cached
invalid idTokens AND has an authorization
cache AND the Charging Station supports at
least one of the following local start
authorization options: C-30, C-31, C-32, C-34 OR
supports at least one of the following remote
start authorization options: C-36, C-37

Authorization Cache

Authorization Cache

Authorization Cache &
MaxEnergyOnInvalidId

Authorization Cache

Authorization Cache

Authorization Cache &
MaxEnergyOnInvalidId

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

C-49 and (((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72)) or
(C-36 or C-37))

C-49 and C-03 and ((C-
30 and C-70) or (C-31
and C-71) or (C-32 and
C-72))

C-49 and ((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72))

C-49 and ((C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72))

C-49 and C-03 and ((C-
30 and C-70) or (C-31
and C-71) or (C-32 and
C-72))

C-59 and C-49 and (((C-
30 and C-70) or (C-31
and C-71) or (C-32 and
C-72)) or (C-36 or C-
37))

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

24/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Local start transaction - Cable plugin first

TC_E_03

Success

Local start transaction - Authorization first

TC_E_04

Success

TC_E_05
(2.1)

Cable plugin timeout

TC_E_52

DisableRemoteAuthorization

Start transaction options

TC_E_09

EVConnected

TC_E_10

Authorized - Local

TC_E_13

Authorized - Remote

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

M

M

M

M

C

C

C

C

C

C

C

Applicable if one or more of the local start
authorization options is implemented.

NOT AQ-2 and (C-30 or
C-31 or C-32 or C-33 or
C-34 or C-35 or ISO
15118 support)

Authorization options for local start

Applicable if one or more of the local start
authorization options is implemented.

C-30 or C-31 or C-32 or
C-33 or C-35

Authorization options for local start

Applicable if one or more of the local start
authorization options is implemented.

(C-30 or C-31 or C-32
or C-33 or C-35) AND
NOT HFS-13

Authorization options for local start

If the Charging Station supports the option for

disabling remote authorization and
The Charging Station supports at least one of
the following local start authorization options

C-58 and (C-30 or C-31
or C-32) and (C-49 or
Local Authorization
List Management)

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode & Authorization Cache &
Local Authorization List.

C-30, C-31, C-32 and
Either Authorization Cache or Local
Authorization List is supported.

TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value EVConnected
is a supported value. And it should be possible
to not set ParkingBayOccupancy.

(C-09.1 and (C-51 or
NOT C-09.6)) and NOT
AQ-2

TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value Authorized is

C-09.2 and (C-30 or C-
31 or C-32 or C-33 or C-
34 or C-35 or ISO
15118 support)

Supported Transaction Start Points
& Authorization options for local
start & Authorization - eMAID

a supported value.
- If one or more of the local start authorization
options is implemented.

TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value Authorized is
a supported value.

C-09.2

Supported Transaction Start points

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

25/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

TC_E_11

DataSigned

C

M

TC_E_01

PowerPathClosed

C

M

TC_E_02

EnergyTransfer

C

M

TC_E_12

ParkingBayOccupied

C

M

Stop transaction options

TC_E_14
(2.1)

EVDisconnected - Charging Station side

C

M

CS: TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value DataSigned is
a supported value. And it should be possible to
not set ParkingBayOccupancy and

EVConnected and Authorized.
CSMS: Must at least be able to receive a signed
MeterValue. It does not need to be able to read
it.

TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
PowerPathClosed is a supported value. And it
should be possible to not set
ParkingBayOccupancy and EVConnected and
Authorized and DataSigned.

TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
EnergyTransfer is a supported value. And it
should be possible to not set
ParkingBayOccupancy and EVConnected and
Authorized and DataSigned and
PowerPathClosed

TxStartPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
ParkingBayOccupied is a supported value.

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value EVConnected
is a supported value. And it should be possible
to not set EnergyTransfer and

PowerPathClosed and Authorized.
Charging Station does NOT have a fixed cable.

C-09.3 and (C-51 or
NOT (C-09.1 or C-09.2
or C-09.6))

Supported Transaction Start points

C-09.4 and (C-51 or
NOT (C-09.1 or C-09.2
or C-09.3 or C-09.6))

Supported Transaction Start points

Supported Transaction Start points

C-09.5 and (C-51 or
NOT (C-09.1 or C-09.2
or C-09.3 or C-09.4 or
C-09.6))

C-09.6

Supported Transaction Start points

Supported Transaction Stop points

HFS-1 and C-10.1 and
(C-52 or NOT (C-10.2
or C-10.3 or C-10.4))
AND NOT HFS-13

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

26/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

TC_E_20

EVDisconnected - EV side (able to charge IEC
61851-1 EV)

C

M

TC_E_54
(2.1)

EVDisconnected - EV side (not able to charge
IEC 61851-1 EV)

C

TC_E_15

StopAuthorized - Local

C

M

TC_E_21

StopAuthorized - Remote

C

M

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value EVConnected
is a supported value. And it should be possible
to not set EnergyTransfer and

PowerPathClosed AND
The Charging Station does NOT have following
configuration combination;
StopTxOnEVSideDisconnect mutability
ReadOnly with value true AND TxStopPoint
mutability is ReadOnly and contains Authorized

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value EVConnected
is a supported value. And it should be possible
to not set EnergyTransfer and

PowerPathClosed AND
The Charging Station does NOT have following
configuration combination;
StopTxOnEVSideDisconnect mutability
ReadOnly with value true AND TxStopPoint
mutability is ReadOnly and contains Authorized

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value Authorized is

a supported value.
The Charging Station supports E07 Transaction
locally stopped by IdToken with at least one of
the following local stop authorization options:
C-70, C-71, C-72, C75

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value Authorized is
a supported value.

(C-10.1 AND (NOT
(NOT C-52 AND (C-10.3
or C-10.4))) AND NOT
(NOT C-06.1 AND NOT
C-52 AND C-10.2))
AND (AQ-9 OR Product
Subtype "Mode 1/2-
only Charging Station")
AND NOT Product
Subtype "Wireless
Charging Station"

C-10.1 AND (NOT (NOT
C-52 AND (C-10.2 or C-
10.3 or C-10.4))) AND
(HFS-4 OR ISO 15118
support OR Product
Subtype "Wireless
Charging Station") AND
NOT Product Subtype
"Mode 1/2-only
Charging Station" AND
NOT HFS-13

C-10.2 and (C-70 or C-
71 or C-72 or C-75)

Supported Transaction Stop points

Supported Transaction Stop points

Supported Transaction Stop Points
& Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode / NoAuthorization

C-10.2

Supported Transaction Stop points

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

27/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

TC_E_16

Deauthorized - Invalid idToken

C

M

TC_E_17

Deauthorized - EV side disconnect

C

M

TC_E_39
(2.1)

Deauthorized - timeout

TC_E_07

PowerPathClosed - Local stop

TC_E_35

PowerPathClosed - Remote stop

M

M

C

C

C

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value Authorized or

PowerPathClosed is a supported value.
Charging Station: If one or more of the local

start authorization options is implemented.
AND either a cache, local authorization list or
UnknownIdtag (C15) is implemented.

This testcase is applicable if the value
Authorized is a supported value for

TxStopPoint AND
EVConnected, PowerPathClosed and
EnergyTransfer must not be set as TxStopPoint

AND
StopTxOnEVSideDisconnect true must be a
supported value.

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
PowerPathClosed is a supported value. And it

should be possible to not set Authorized.
The Charging Station supports E07 Transaction
locally stopped by IdToken with at least one of
the following local stop authorization options:
C-70, C-71, C-72, C75

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
PowerPathClosed is a supported value. And it
should be possible to not set Authorized.

(C-10.2 or C-10.3) and
C-01 and NOT C-35

Supported Transaction Stop Points
& Local Authorization options for
local start

Supported Transaction Stop points

C-10.2 and C-06.2 and
AQ-9 and NOT (NOT C-
52 AND (C-10.1 OR C-
10.3 OR C-10.4))

NOT HFS-13

C-10.3 and (C-52 or
NOT C-10.2) and (C-70
or C-71 or C-72 or C-
75)

Supported Transaction Stop Points
& Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode / NoAuthorization

C-10.3 and (C-52 or
NOT C-10.2)

Supported Transaction Stop points

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

28/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

TC_E_37

PowerPathClosed - EV side disconnect

C

TC_E_08

EnergyTransfer stopped - StopAuthorized

C

M

TC_E_22

EnergyTransfer stopped - SuspendedEV

TC_E_19

ParkingBayUnoccupied

Disconnect cable on EV-side

TC_E_24

Deauthorize transaction -
UnlockOnEVSideDisconnect is true

TC_E_25

Deauthorize transaction -
UnlockOnEVSideDisconnect is false

M

M

C

C

C

C

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
PowerPathClosed is a supported value. And it
should be possible to not set EnergyTransfer
and EVConnected.

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
EnergyTransfer is a supported value. And it
should be possible to not set PowerPathClosed
and Authorized.

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
EnergyTransfer is a supported value.

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
ParkingBayOccupancy is a supported value.
And it should be possible to not set
EnergyTransfer and Authorized and
PowerPathClosed and EVConnected.

C-10.3 and (C-52 or
NOT (C-10.1 or C-
10.4)) AND (AQ-9 OR
Product Subtype
"Mode 1/2-only
Charging Station" OR
Product Subtype
"Wireless Charging
Station")

C-10.4 and (C-52 or
NOT (C-10.2 or C-
10.3))

Supported Transaction Stop points

Supported Transaction Stop points

C-10.4

Supported Transaction Stop points

C-10.5 and (C-52 or
NOT (C-10.1 or C-10.2
or C-10.3 or C-10.4))

Supported Transaction Stop points

The Charging Station does NOT have a

permanently attached cable.

UnlockOnEVSideDisconnect can be set to true
StopTxOnEVSideDisconnect can be set to true

HFS-1 and C-06.2 and
C-12.1 and AQ-18 and
NOT (HFS-4 or (ISO
15118 support and
NOT AQ-9 ))

Support for not maintaining
authorization when cable
disconnected on EV side & Support
for unlocking connector when
cable disconnected on EV side

UnlockOnEVSideDisconnect can be set to false
StopTxOnEVSideDisconnect can be set to true

C-06.2 and C-12.2 and
AQ-18 and NOT (HFS-4
or (ISO 15118 support
and NOT AQ-9 ))

Support for not maintaining
authorization when cable
disconnected on EV side & Support
for not unlocking connector when
cable disconnected on EV side

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

29/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

TC_E_26

Suspend transaction

C

M

TC_E_27

Suspend transaction - Fixed cable connection
timeout

C

Transactions with fixed cost, energy or time

TC_E_100
(2.1)

CSMS specifies energy limit

TC_E_101
(2.1)

CSMS calculates costs (through
CostUpdatedRequest) and CS specifies limit

TC_E_102
(2.1)

TC_E_103
(2.1)

CSMS and CS both specify limits

CS calculates costs and CSMS specifies limit

TC_E_104
(2.1)

CSMS calculates costs (through
TransactionEventResponse) and specifies limit

TC_E_105
(2.1)

TC_E_106
(2.1)

CSMS specifies time limit

CS specifies energy limit

C

C

C

C

C

C

C

C

M

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
ParkingBayOccupancy or Authorized is a
supported value. And it should be possible to
not set EnergyTransfer and PowerPathClosed

and EVConnected.

UnlockOnEVSideDisconnect can be set to false
StopTxOnEVSideDisconnect can be set to false

TxStopPoint can either be ReadOnly with a
subset of the values or have a valueList of
supported values, that contains a subset. This
testcase is applicable if the value
ParkingBayOccupancy or Authorized is a
supported value. And it should be possible to
not set EnergyTransfer and PowerPathClosed

and EVConnected.
The Charging Station has a permanently

attached cable at the Charging Station side.

UnlockOnEVSideDisconnect can be set to false
StopTxOnEVSideDisconnect can be set to false

(C-10.2 or C-10.5) and
(C-52 or NOT (C-10.1
or C-10.3 or C-10.4))
and C-06.1 and C-12.2
and NOT (HFS-4 or
(ISO 15118 support
and NOT AQ-9 )) and
NOT Product Subtype
"Mode 1/2-only
Charging Station"

(C-10.2 or C-10.5) and
(C-52 or NOT (C-10.1
or C-10.3 or C-10.4))
and C-06.1 and C-12.2
and HFS-2 and NOT
(HFS-4 or (ISO 15118
support and NOT AQ-9
)) and NOT Product
Subtype "Mode 1/2-
only Charging Station"

C-63

C-63

Support for transaction limits

Support for transaction limits

CS: C-63
CSMS: P-0

CS: Support for transaction limits
CSMS: Support for Payment

C-63

C-63

C-63

C-63

Support for transaction limits

Support for transaction limits

Support for transaction limits

Support for transaction limits

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

30/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_E_107
(2.1)

TC_E_108
(2.1)

TC_E_109
(2.1)

TC_E_110
(2.1)

TC_E_111
(2.1)

CS specifies time limit

CS calculates costs and specifies limit

CSMS calculates costs and specifies cost limit

CSMS specifies energy limit

CSMS specifies time limit

Resuming transaction after interruption

TC_E_112
(2.1)

TxResumptionTimeout not expired -
TxAllowEnergyTransferResumption is false

TC_E_113
(2.1)

TxResumptionTimeout not expired -
TxAllowEnergyTransferResumption is true

TC_E_114
(2.1)

TC_E_115
(2.1)

TC_E_116
(2.1)

Powerloss - TxResumptionTimeout absent

Powerloss - TxResumptionTimeout = 0

Powerloss - TxResumptionTimeout expired

Retry sending transaction message when
failed

TC_E_41 Max retry count reached

TC_E_42

Success before reaching the max retry count

TC_E_50 Max retry count reached - CallError

TC_E_51

Success before reaching the max retry count -
CallError

Offline Behaviour

TC_E_40

Connection loss during transaction

TC_E_43

Transaction during offline period

TC_E_44

Stop transaction during offline period

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

M

M

C

C

C

M

C

C

C

C

C

C

C

M

M

M

M

M

C

C

C-63

C-63

P-0

P-0

P-0

C-64

C-64

Support for transaction limits

Support for transaction limits

Support for Payment

Support for Payment

Support for Payment

Support for resuming transaction
after interruption

Support for resuming transaction
after interruption

NOT C-62 AND NOT C-
64

Support for resuming transaction
after interruption

C-64

C-64

Support for resuming transaction
after interruption

Support for resuming transaction
after interruption

Charging Station: If offline authorization is
supported.

Charging Station: If one or more of the local
stop authorization options is implemented.

C-01

Offline transaction support

C-70 or C-71 or C-72 or
C-75 or ISO 15118
support

Local Authorization options for
local start & Authorization - eMAID

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

31/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_E_45

Stop transaction during offline period - Same
GroupId

Check Transaction status

TC_E_28

TransactionId unknown

TC_E_29

TC_E_30

TC_E_31

Transaction with id ongoing - with message in
queue

Transaction with id ongoing - without message
in queue

Transaction with id ended - with message in
queue

TC_E_32

Transaction with id ended - without message in
queue

TC_E_33 Without transactionId - with message in queue

TC_E_34 Without transactionId - without message in

queue

Reset Sequence Number

TC_E_53

CSMS accepting seqNo = 0 at start of
transaction

Remote start transaction

TC_F_01

Cable plugin first

TC_F_02

Remote start first - AuthorizeRemoteStart is
true

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

M

M

M

C

M

M

M

C

C

C

C

C

C

C

M

M

M

For CS: the Charging Station supports at least
one of the following local stop authorization
options: C-30, C-31, C-32 and Local
Authorization List or Authorization Cache

(C-70 or C-71 or C-72)
AND (Local
Authorization List
Management or C-49)

Local Authorization - using RFID
ISO14443 / RFID ISO15693 /
KeyCode and Local Authorization
List or Authorization Cache

Charging Station:
The following combination of conditions are

NOT true:
- No local authorization methods are supported

AND
- TxStopPoint mutability is ReadOnly and only

contains Authorized AND
- TxCtrlr.StopTxOnEVSideDisconnect mutability
is false and value is false

C-16

C-16

Check TransactionStatus

Check TransactionStatus

CSMS: C-16

CS:

CSMS:
Check TransactionStatus

NOT (
NOT ((C-30 and C-70)
or (C-31 and C-71) or
(C-32 and C-72) or (C-

35 and C-75)) AND
(NOT C-10.1 AND NOT
C-10.3 AND NOT C-
10.4 AND NOT C-10.5)

AND

NOT C-06.2
)

C-16

C-16

Check TransactionStatus

Check TransactionStatus

If the Charging Station does not have a cable
lock.

NOT AQ-2 and (C-36 or
C-37 or C-38 or C-39)

Authorization options for remote
start

If AuthorizeRemoteStart can be set to true

C-48.1 and (C-36 or C-
37 or C-38 or C-39)

Authorization options for remote
start

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

32/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_F_03

TC_F_04
(2.1)

Remote start first - AuthorizeRemoteStart is
false

Remote start first - Cable plugin timeout

Remote stop transaction

TC_F_08

Success

TC_F_09

Rejected

Remote unlock Connector

TC_F_05 With ongoing transaction

TC_F_06 Without ongoing transaction - Accepted

TC_F_07 Without ongoing transaction - No cable

connected

TC_F_10 Without ongoing transaction -
UnknownConnector

Trigger message

TC_F_11 MeterValues - Specific EVSE

TC_F_12 MeterValues - All EVSE

TC_F_13

TransactionEvent - Specific EVSE

TC_F_14

TransactionEvent - All EVSE

TC_F_15

LogStatusNotification - Idle

TC_F_16

LogStatusNotification - Uploading

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

M

M

C

C

C

C

C

C

C

C

C

C

M

M

If AuthorizeRemoteStart can be set to false

C-48.2 and (C-36 or C-
37 or C-38 or C-39)

Authorization options for remote
start

NOT HFS-13

If the Charging Station has a detachable cable,
with a mechanized locking mechanism.

HFS-1 and AQ-18

C

If the Charging Station has a detachable cable,

with a mechanized locking mechanism.
If the CSMS support the Unlocking connector
for Charging Station with detachable cable
(UnlockConnector) feature.

CSMS: C-11
CS: HFS-1 and AQ-18

If the Charging Station has a detachable cable,
with a mechanized locking mechanism.

HFS-1 and AQ-18

If the Charging Station has a detachable cable,
with a mechanized locking mechanism.

HFS-1 and AQ-18

C

C

C

C

C

If the SUT supports TriggerMessage for
requestedMessage MeterValues for a specific
EVSE.

If the SUT supports TriggerMessage for
requestedMessage MeterValues for a all EVSE.

If the SUT supports TriggerMessage for
requestedMessage TransactionEvent for a
specific EVSE.

If the SUT supports TriggerMessage for
requestedMessage TransactionEvent for a all
EVSE.

If the SUT supports TriggerMessage for
requestedMessage LogStatusNotification.

If the Charging Station supports
TriggerMessage for requestedMessage
LogStatusNotification.

C-29.1

TriggerMessage

C-29.1

TriggerMessage

C-29.2

TriggerMessage

C-29.2

TriggerMessage

C-29.3

TriggerMessage

C-29.3

TriggerMessage

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

33/68

Part 5 - Certification Profiles

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_F_17

FirmwareStatusNotification - Specific EVSE not
relevant

TC_F_18

FirmwareStatusNotification - Idle

TC_F_19

FirmwareStatusNotification - Downloading

TC_F_20

Heartbeat

TC_F_23

StatusNotification - Specific EVSE - Available

TC_F_24

StatusNotification - Specific EVSE - Occupied

TC_F_26

BootNotification - Rejected

TC_F_27

NotImplemented

TC_F_100 CustomTrigger

Connector status Notification

TC_G_01

Available to Occupied

TC_G_02

Occupied to Available

TC_G_20

Lock Failure

Change Availability EVSE

C

C

C

M

C

C

C

C

C

M

M

C

M

C

C

M

C

M

If the Charging Station supports
TriggerMessage for requestedMessage
FirmwareStatusNotification.

If the SUT supports TriggerMessage for
requestedMessage
FirmwareStatusNotification.

If the Charging Station supports
TriggerMessage for requestedMessage
FirmwareStatusNotification.

If the SUT supports TriggerMessage for
requestedMessage StatusNotification for a
specific EVSE.

If the SUT supports TriggerMessage for
requestedMessage StatusNotification for a
specific EVSE.

If the Charging Station supports
TriggerMessage for requestedMessage
BootNotification.

Feature no.

Feature

C-29.4

TriggerMessage

C-29.4

TriggerMessage

C-29.4

TriggerMessage

C-29.5

TriggerMessage

C-29.5

TriggerMessage

C-29.6

TriggerMessage

For CS: can only be done when
SignCombinedCertificate is notimplemented

NOT AQ-11

C-29.7

Trigger message

Charging Station: This can either be
implemented with the StatusNotification or

NotifyEvent message.
CSMS: Both StatusNotification and NotifyEvent
must be supported.

Charging Station: This can either be
implemented with the StatusNotification or

NotifyEvent message.
CSMS: Both StatusNotification and NotifyEvent
must be supported.

TC_G_03

Operative to inoperative

M

M

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

34/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_G_09

Operative to operative

TC_G_04

Inoperative to operative

TC_G_10

Inoperative to inoperative

TC_G_11 With ongoing transaction

TC_G_18

state persists across reboot

Change Availability Charging Station

TC_G_05

Operative to inoperative

TC_G_12

Operative to operative

TC_G_06

Inoperative to operative

TC_G_13

Inoperative to inoperative

TC_G_21

state persists across reboot

TC_G_14 With ongoing transaction

Change Availability Connector

TC_G_07

Operative to inoperative

TC_G_15

Operative to operative

TC_G_08

Inoperative to operative

TC_G_16

Inoperative to inoperative

TC_G_17 With ongoing transaction

TC_G_19

state persists across reboot

Clock-aligned Meter Values

TC_J_01

No transaction ongoing

TC_J_02

Transaction ongoing

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

Charging Station: This can either be
implemented with the StatusNotification or

NotifyEvent message.
CSMS: Both StatusNotification and NotifyEvent
must be supported.

Charging Station: This can either be
implemented with the StatusNotification or

NotifyEvent message.
CSMS: Both StatusNotification and NotifyEvent
must be supported.

Charging Station can choose which

measurands are supported (At least one).
This can either be implemented with the
MeterValues or NotifyEvent message.

C-40

C-40

Supported MeterValue Measurands

Supported MeterValue Measurands

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

35/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_J_03

EventType Ended

TC_J_04

Signed

TC_J_06

No Meter Values during transaction

Sampled Meter Values

TC_J_07

EventType Started - EVSE known

TC_J_08

Context Transaction.Begin - EVSE not known

TC_J_09

EventType Updated

TC_J_10

EventType Ended

TC_J_11

Signed

Remote start transaction with charging profile

TC_K_38

Ignore chargingProfile

Secure Firmware Update

TC_L_01

Installation successful

TC_L_02

InstallScheduled

TC_L_03

DownloadScheduled

TC_L_04

RevokedCertificate

TC_L_05

InvalidCertificate

TC_L_06

InvalidSignature

TC_L_07

DownloadFailed

TC_L_08

InstallVerificationFailed or InstallationFailed

TC_L_09

InstallationFailed

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

Charging Station: If signed MeterValues is

implemented
CSMS: Must at least be able to receive a signed
MeterValue. It does not need to be able to read
it.

C-40

C-42

Supported MeterValue Measurands

Supported MeterValue Measurands
& Signed Metervalues

If AlignedDataSendDuringIdle is supported.

C-28

AlignedDataSendDuringIdle

Charging Station can choose which
measurands are supported (At least one).

C-40

Supported MeterValue Measurands

C-40 and NOT AQ-8
AND (C-09.2 OR C-
09.6)

Supported MeterValue Measurands
& possibility to enforce EVSE being
known.

C-40

C-40

C-42

Supported MeterValue Measurands

Supported MeterValue Measurands

Supported MeterValue Measurands
& Signed Metervalues

Charging Station: If signed MeterValues is

implemented
CSMS: Must at least be able to receive a signed
MeterValue. It does not need to be able to read
it.

The Charging Station does NOT support Smart
Charging.

NOT Smart Charging

C-15

C-15

Scheduled firmware updates

Scheduled firmware updates

M

C

C

M

C

M

M

C

C

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

C

C

M

M

M

M

M

M

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

36/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_L_10

AcceptedCanceled

TC_L_11

Unable to cancel

TC_L_18 Missing firmware signing certificate and

signature

TC_L_12

Unable to download/install firmware with
ongoing transaction -
AllowNewSessionsPendingFirmwareUpdate is
true

TC_L_13

Unable to download/install firmware with
ongoing transaction -
AllowNewSessionsPendingFirmwareUpdate is
false

TC_L_14

TC_L_15

Unable to install and activate firmware with
ongoing transaction -
AllowNewSessionsPendingFirmwareUpdate is
true

Unable to install firmware with ongoing
transaction -
AllowNewSessionsPendingFirmwareUpdate is
false

TC_L_16

Able to update firmware with ongoing
transaction

Retrieve certificates from Charging Station

TC_M_12 CSMSRootCertificate

TC_M_13 ManufacturerRootCertificate

TC_M_17 CSMSRootCertificate &

ManufacturerRootCertificate

TC_M_18 All certificateTypes

TC_M_19 No matching certificate found

Delete a certificate from a Charging Station

TC_M_20

Success

TC_M_21

Failed

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

M

C

C

C

C

C

M

M

M

M

M

M

M

M

The Charging Station supports cancellling an
ongoing firmware update

The Charging Station does NOT supports
cancellling an ongoing firmware update

C-60

NOT C-60

AllowNewSessionsPendingFirmwareUpdate is

implemented.
The Charging Station is unable to download
AND install firmware while there is an ongoing
transaction.

M

AllowNewSessionsPendingFirmwareUpdate is

implemented.
The Charging Station is unable to download
AND install firmware while there is an ongoing
transaction.

AllowNewSessionsPendingFirmwareUpdate is

implemented.
The Charging Station is unable to install
firmware while there is an ongoing transaction

C-20 and NOT C-43
and NOT AQ-7 and
HFS-8 > 1

AllowNewSessionsPendingFirmwa
reUpdate

NOT C-43 and NOT AQ-
7

C-20 and NOT C-43
and AQ-7 and HFS-8 >
1

AllowNewSessionsPendingFirmwa
reUpdate

AllowNewSessionsPendingFirmwareUpdate is

NOT C-43 and AQ-7

implemented.
The Charging Station is unable to install
firmware while there is an ongoing transaction

If the Charging Station supports Install
Firmware with ongoing transaction(s)

C-43

Install Firmware with ongoing
transaction(s)

M

M

M

M

M

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

37/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_M_22 No matching certificate found

Install CA certificate

TC_M_01 CSMSRootCertificate

TC_M_02 ManufacturerRootCertificate

TC_M_05

Failed

TC_M_07 Rejected - Certificate invalid

TC_M_09 AdditionalRootCertificateCheck - Rejected

TC_M_30 AdditionalRootCertificateCheck - Reconnect
using new CSMS Root - Success

TC_M_31 AdditionalRootCertificateCheck - Reconnect
using new CSMS Root - Fallback mechanism

Retrieve Log Information

TC_N_25

Diagnostics Log - Success

TC_N_34

Rejected

TC_N_26

Diagnostics Log - Upload failed

TC_N_35

Security Log - Success

TC_N_36

Second Request

TC_N_100
(2.1)

DataCollectorLog - Success

TC_N_101
(2.1)

Validations

TC_N_102
(2.1)

TC_N_103
(2.1)

Authentication - HTTP

Authentication - HTTPS

Get Customer Information

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

M

M

M

M

C

C

C

M

M

M

C

C

C

C

C

M

M

M

M

M

M

M

C

M

M

If the Charging Station supports
AdditionalRootCertificateCheck with value true

If the Charging Station supports
AdditionalRootCertificateCheck with value true

If the Charging Station supports
AdditionalRootCertificateCheck with value true

AS-2

AS-2

AS-2

Additional Root Certificate check
mechanism implemented

Additional Root Certificate check
mechanism implemented

Additional Root Certificate check
mechanism implemented

Support for DataCollectorLog

If the Charging Station is able to cancel an
ongoing log file upload.

C-57

Charging station supports HTTP or HTTPS file
transfer protocol for logging

Charging station supports HTTP file transfer
protocol for logging

Charging station supports HTTPS file transfer
protocol for logging

CS: C-65
CSMS: C-65 OR
Bidirectional Power
Transfer OR DER
Control

Supported file transfer
protocols contains
HTTP or HTTPS

Supported file transfer
protocols contains
HTTP

Supported file transfer
protocols contains
HTTPS

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

38/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_N_27

Accepted + data

TC_N_28

Accepted + no data

TC_N_29 Not Accepted

Clear Customer Information

TC_N_30

Clear and report + data

TC_N_31

Clear and report + no data

TC_N_32

Clear and no report

TC_N_62

Clear and report - customerIdentifier

Data Transfer to the Charging Station

TC_P_01

Rejected / Unknown VendorId / Unknown
MessageId

Data Transfer to the CSMS

TC_P_02

Rejected / Unknown VendorId / Unknown
MessageId

CustomData

TC_P_03

Receive custom data

Battery Swap (2.1)

TC_S_102 Remote Start - not enough batteries

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

M

C

M

M

C

For CS: The Charging Station needs to support
Local Authorization and either the Local

Authorization List or Authorization Cache.
IdToken is used as customer information.

For CS: The Charging Station needs to support
Local Authorization and either the Local

Authorization List or Authorization Cache.
IdToken is used as customer information.

((C-30 and C-70) or (C-
31 and C-71)) and
(Local Authorization
List Management or C-
49)

((C-30 and C-70) or (C-
31 and C-71)) and
(Local Authorization
List Management or C-
49)

M

M

M

M

For CS: The Charging Station needs to support

Local Authorization or the Authorization Cache.
IdToken is used as customer information.

M

For CS: The Charging Station needs to support

Local Authorization or the Authorization Cache.
IdToken is used as customer information.

((C-30 and C-70) or (C-
31 and C-71)) and
(Local Authorization
List Management or C-
49)

((C-30 and C-70) or (C-
31 and C-71)) and
(Local Authorization
List Management or C-
49)

M

C

Support for retrieving / deleting
CustomerInformation - CustomerIdentifier

C-14

Charging Station must be able to Reject the
message.

M

CSMS must be able to Reject the message.

M

C

CS: HFS-13
CSMS: C-76

CSMS: Support for Battery
Swapping Stations

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

39/68

Part 5 - Certification Profiles

Feature no.

Feature

Support for Battery Swapping
Stations

C-76

HFS-13

HFS-13

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

TC_S_103 Remote Start - enough batteries available

C

TC_S_104 Charging - Variables validation

TC_S_105 Charging - Battery Swap Charging

Offline authorization through local
authorization list

TC_C_21

Accepted

TC_C_22

Invalid

TC_C_23

Blocked

TC_C_24

Expired

TC_C_25

Local Authorization List > Authorization Cache

Online authorization through local
authorization list

TC_C_27

Accepted

TC_C_28

Invalid & Not Accepted

TC_C_31

Invalid & Accepted

TC_C_29

Blocked

TC_C_30

Expired

TC_C_58

LocalAuthListDisablePostAuthorize

Authorization by GroupId

C

C

C

C

C

C

C

C

C

C

C

C

C

Local Authorization List Management

The Charging Station must support at least one
of the following local start authorization
options: C-30, C-31, C-32

LA-0 and C-30 and C-
70) or (C-31 and C-71)
or (C-32 and C-72

The Charging Station must support at least one
of the following local start authorization
options: C-30, C-31, C-32

LA-0 and (C-30 or C-31
or C-32)

The Charging Station must support at least one
of the following local start authorization
options: C-30, C-31, C-32

LA-0 and (C-30 or C-31
or C-32)

The Charging Station must support at least one
of the following local start authorization
options: C-30, C-31, C-32

LA-0 and (C-30 or C-31
or C-32)

The Charging Station must support at least one
of the following local start authorization
options: C-30, C-31, C-32

LA-0 and C-49 and (C-
30 or C-31 or (C-32 and
C-72))

LA-0

LA-0

LA-0

LA-0

LA-0

The Charging Station supports the option for
disabling remote authorization for invalid
idTokens stored at the Local Authorization List.

LA-0 and LA-3

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

40/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_C_40

Success with Local Authorization List

TC_C_43

Invalid status with Local Authorization List

Send Local Authorization List

TC_D_01

Full

TC_D_02

Differential Update

TC_D_03

Differential Remove

TC_D_04

Full with empy list

TC_D_05

Differential with empty list

TC_D_06

VersionMismatch

TC_D_07

Persistent over reboot

Get Local List Version

TC_D_08

Success

TC_D_09 No list available

TC_D_10

Function disabled

Clear Customer Information

TC_N_46

Update Local Authorization List

Reset Charging Station

TC_B_24

Reserved persists reset

Reserve a specific EVSE

TC_H_01

Accepted - Valid idToken

TC_H_02

Accepted - Different idToken

TC_H_03 Occupied - EVSE Reserved

TC_H_04 Occupied - EVSE Occupied

TC_H_05

Faulted

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

For CS:
- The Charging Station supports at least one of
the following local start authorization options:

C-30, C-31, C-32

For CS:
- The Charging Station supports at least one of
the following local start authorization options:

C-30, C-31, C-32

CS: LA-0 and C-30 and
C-70) or (C-31 and C-

71) or (C-32 and C-72
CSMS: LA-0

CS: LA-0 and C-30 and
C-70) or (C-31 and C-

71) or (C-32 and C-72
CSMS: LA-0

LA-0

LA-0

LA-0

LA-0

LA-0

LA-0

LA-0

CS: LA-0
CSMS: LA-0 and LA-2

GetLocalListVersion

LA-0

LA-0

LA-0

R-0

R-0

R-0

R-0

R-0

Reservation

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

41/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_H_06

Unavailable

TC_H_07

Reservation Ended / not used

TC_H_22

Configured to Reject

TC_H_23

Replace reservation

TC_H_19

Use a reserved EVSE with GroupId

Reserve an unspecified EVSE

TC_H_08

Accepted

TC_H_09 Occupied - EVSE Reserved

TC_H_10 Occupied - EVSE Occupied

TC_H_12

Unavailable

TC_H_13

Rejected

TC_H_14

Amount of EVSEs available equals the amount
of reservations

TC_H_24

GroupIdToken

Reserve a connector with a specific type

TC_H_15

Success

TC_H_16

Amount of available connectors of a type
equals the amount of reservations

Cancel reservation of an EVSE

TC_H_17

Success

TC_H_18

Rejected

TC_H_20

Charging Station cancels reservation when
Faulted

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

For CS: The configuration variable
ReservationCtrlr.ReservationEnabled is
implemented

For CS: Depending on configuration variable
ReservationNonSpecificEVSE

Depending on configuration variable
ReservationNonSpecificEVSE

Depending on configuration variable
ReservationNonSpecificEVSE

Depending on configuration variable
ReservationNonSpecificEVSE

Depending on the Charging Station not
supporting the configuration variable
ReservationNonSpecificEVSE

R-0

R-0

CS: R-0 and R-3
CSMS: R-0

R-0

R-0

R-0 and R-2

R-0 and R-2

R-0 and R-2

R-0 and R-2

R-0 and NOT R-2

C

For CS: Depending on configuration variable

R-0 and R-2

ReservationNonSpecificEVSE
For CSMS: this wil be tested with > 1 EVSE

Support reservations of
unspecified EVSE

Support reservations of
unspecified EVSE

Support reservations of
unspecified EVSE

Support reservations of
unspecified EVSE

Support reservations of
unspecified EVSE

Support reservations of
unspecified EVSE

For CS: Depending on configuration variable
ReservationNonSpecificEVSE

R-0 and R-2

Support reservations of
unspecified EVSE

C

C

C

R-0 and R-1

R-0 and R-1

Support reservations of
connectorType

Support reservations of
connectorType

R-0

R-0

R-0

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

42/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_H_21

Charging Station cancels reservation when
Unavailable

Set Default Tariff (2.1)

TC_I_100

validFrom

TC_I_101

startTimeOfDay, endTimeOfDay

TC_I_102

TariffMaxElements

TC_I_104

transaction with default tariff

TC_I_105

TariffConditionsSupported is false

TC_I_106

validations

Receive Driver Tariff (2.1)

TC_I_107 CS cannot process tariff -

UseDefault/CentralCost

TC_I_108 CS cannot process tariff - Deauthorize

TC_I_109 Goodflow

Clear Tariffs (2.1)

TC_I_110 DefaultTariff

TC_I_111

Tariff in use

Local Cost Calculation (2.1)

TC_I_113 Change transaction tariff - TariffMaxElements

TC_I_114 Change transaction tariff -

TariffConditionsSupported is false

TC_I_115 Change transaction tariff -

TariffConditionsSupported is true

TC_I_116 Change transaction tariff - goodflow

TC_I_117 Change transaction tariff - validations

TC_I_118 Cost Details of Transaction - no tariff

conditions

TC_I_119 Cost Details of Transaction - with tariff

conditions

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

Payment

R-0

P-0

P-0

P-0

P-0

CS: P-0 and NOT P-1
CSMS: P-0

No Support for Tariff conditions

P-0

P-0

P-0

P-0

P-0

P-0

P-0

P-0

CS: P-0 and P-1
CSMS: P-0

Support for Tariff conditions

P-0

P-0

P-0

CS: P-0 and P-1
CSMS: P-0

Support for Tariff conditions

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

43/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_I_120 Cost Details of Transaction - reservation

TC_I_121 Cost Details of Transaction - minCost/maxCost

TC_I_122 Cost Details of Transaction

Authorization with Prepaid card (2.1)

TC_C_103 Success

TC_C_104 No credit

Integrated Payment Terminal (2.1)

TC_C_105 CSMS rejects authorization

TC_C_106 Only Payment Terminal authorises

TC_C_107 Payment Terminal and CSMS authorises

TC_C_108 VAT number validation

TC_C_109 Cancelation prior to transaction - Only Payment

Terminal authorised - EVConnectionTimeout

TC_C_110 Cancelation prior to transaction - Payment

Terminal and CSMS authorised - stopped by EV
driver

TC_C_111 Cancelation prior to transaction - Only Payment

Terminal authorised - stopped by EV driver

TC_C_112 Cancelation prior to transaction - Payment
Terminal and CSMS authorised -
EVConnectionTimeout

TC_C_113 Cancelation after start of transaction - stopped

by EV driver

Settlement at end of transaction (2.1)

TC_C_114 Settled by CS, receipt by CS

TC_C_115 Settled by CSMS

TC_C_116 Settled by CS, receipt by CSMS

TC_C_117 Settled by CSMS, receipt by CSMS

TC_C_118 Settled by CS, receipt by CSMS

Settlement (2.1)

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Certification Profile: Reservation supported

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

Feature no.

Feature

P-0

P-0

P-0

CS: P-0 and P-2.1
CSMS: P-0

CS: P-0 and P-2.1
CSMS: P-0

Payment by prepaid card

Payment by prepaid card

P-0 and P-2.2

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

CS: P-0 and P-2.2
CSMS: P-0

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

CS: P-0 and P-2.2
CSMS: P-0

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

P-0

P-0

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

44/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_C_119 Is rejected or fails - Failed

TC_C_120 Is rejected or fails - Rejected

Incremental authorization (2.1)

TC_C_121 increasing enabled

TC_C_122 increasing disabled

Ad hoc payment via stand-alone payment
terminal (2.1)

TC_C_123 Local cost calculation

TC_C_124 Central cost calculation

TC_C_125 Central cost calculation

TC_C_126 Local cost calculation

Ad hoc payment via static or dynamic QR code
(2.1)

TC_C_127 No URL parameters

TC_C_128 URL parameter maxTime

TC_C_129 URL parameter maxCost

TC_C_130 URL parameter maxEnergy

TC_C_131 Success

TC_C_132 Invalid URL parameters

TC_C_133 Invalid totp

Get Custom Report

TC_B_16 with component criteria

TC_B_17 with component/variable

TC_B_18 with componentCriteria and

component/variables

TC_B_54 with component/variable, but no instance

TC_B_55 with component/variable/instance

TC_B_56 with component/variable, but no evseId

Get Monitoring Report

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

Advanced Device Management

Feature no.

Feature

CS: P-0 and P-2.2
CSMS: P-0

CS: P-0 and P-2.2
CSMS: P-0

Integrated payment terminal

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

P-0 and P-2.2

Integrated payment terminal

P-0 and P-2.3

Stand alone payment terminal

P-0 and P-2.3

Stand alone payment terminal

P-0

P-0

P-0 and P-2.4

QR code payment

P-0 and P-2.4

QR code payment

P-0 and P-2.4

QR code payment

P-0 and P-2.4

QR code payment

P-0

P-0

P-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

45/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_N_01 with monitoringCriteria

TC_N_02 with component/variable

TC_N_03 with component criteria and

component/variable

TC_N_47

Report all

TC_N_60 with component criteria and list of
components/variables

TC_N_104
(2.1)

TargetDeltaMonitoring

Set Monitoring Base

TC_N_05

success

TC_N_06

test removal custom monitors

Set Variable Monitoring

TC_N_08 One SetMonitoringData element

TC_N_09 Multiple elements on different component and

variable

TC_N_10 Multiple monitors on the same component and

variable

TC_N_11

Unknown component

TC_N_12

Value out of range - Delta monitor

TC_N_13

Value out of range - Threshold monitor

TC_N_15

Duplicate Variable type/severity combination

TC_N_24

Periodic event

TC_N_37

Unknown Variable

TC_N_39

Component/Variable combination does NOT
correspond

TC_N_40

Replace Variable Monitor

TC_N_41

Return to FactoryDefault

TC_N_43

First SetMonitoringData and third
SetMonitoringData are valid, but the second
contains an out of range value

TC_N_51 Modifying a VariableMonitor and trigger

TC_N_52

Removing a VariableMonitor

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

46/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_N_105
(2.1)

Set Frequent Periodic Variable Monitoring -
Periodic

TC_N_106
(2.1)

Set Frequent Periodic Variable Monitoring -
CSMS rejects stream

TC_N_107
(2.1)

TC_N_108
(2.1)

TC_N_109
(2.1)

Get Periodic Event Streams - Goodflow

Close Periodic Event Streams

Adjust Periodic Event Streams

Set Monitoring Level

TC_N_16

Success

TC_N_17 Out of range

Clear Monitoring

TC_N_18

Success

TC_N_19 Not found

TC_N_44

Rejected

Alert Event

TC_N_20

Threshold value exceeded

TC_N_21 HardWiredMonitor

TC_N_45

Delta value exceeded

TC_N_48

Variable monitoring on write only

TC_N_49

LowerThreshold/UpperThreshold cleared after
reboot

TC_N_50

Periodic Triggered

TC_N_53

Persistant over reboot

TC_N_56

Delta value NOT numeric exceeded

Offline Notification

TC_N_22 OfflineMonitoringEventQueuingSeverity set

equal or lower

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

DM-0

CS: If the Charging Station has at least one
hardWired monitor.

CS: DM-0 and AQ-5
CSMS: DM-0

CS: If the Charging Station has at least one
hardWired monitor.

CS: if the CS supports Delta monitoring on the
SecurityCtrlr.BasicAuthPassword

DM-0

CS: DM-0 and AQ-5
CSMS: DM-0

DM-0

DM-0 and AQ-10

DM-0

DM-0

DM-0

DM-0

DM-0 and DM-3

OfflineMonitoringEventQueuingSev
erity

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

47/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_N_23 OfflineMonitoringEventQueuingSeverity set

higher

Show costs to EV Driver

TC_I_01

Show EV Driver running total cost during
charging

TC_I_02

Show EV Driver Final Total Cost After Charging

TC_I_07

Show EV Driver running total cost during
charging - transactionEventResponse

Set Display Message

TC_O_01

Success

TC_O_26

Rejected

TC_O_13

Display message at StartTime

TC_O_14

Remove message after EndTime

TC_O_17 NotSupportedPriority

TC_O_18 NotSupportedState

TC_O_19 NotSupportedMessageFormat

TC_O_20

Persistent over reboot

TC_O_22 Multiple In front priority

TC_O_24

Second Alwaysfront priority

TC_O_36

State Charging

TC_O_37
(2.1)

State Idle

TC_O_38

State Unavailable

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

Advanced User Interface

DM-0 and DM-3

OfflineMonitoringEventQueuingSev
erity

UI-0

UI-0

UI-0

UI-0

UI-0

UI-0

UI-0

UI-1 and UI-2: Supported
MessagePriorities & Supported
MessageFormats

CS: In case it does not support 1 of the
MessagePriorities

CS: UI-0 and NOT (UI-
1.1 and UI-1.2 and UI-

1.3)
CSMS: UI-0

UI-0

CS: In case it does not support 1 of the
MessageFormats

CS: UI-0 and NOT (UI-
2.1 and UI-2.2 and UI-

2.3 and UI-2.4)
CSMS: UI-0

UI-0

If the Charging Station supports InFront priority

UI-0 and UI-1.2

If the Charging Station supports AlwaysFront
priority

UI-0 and UI-1.1

Supported MessagePriorities
InFront

Supported MessagePriorities
AlwaysFront

UI-0

UI-0 AND NOT HFS-13

UI-0

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

48/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_O_12

Replace DisplayMessage

TC_O_100
(2.1)

TC_O_101
(2.1)

unsupported language

Language preference of the EV Driver

Set Display Message - Specific transaction

TC_O_06

Success

TC_O_10

UnknownTransaction

TC_O_27

Display message at StartTime

TC_O_28

Remove message after EndTime

TC_O_30 Multiple In front priority

TC_O_32

Second Alwaysfront priority

Get all Display Messages

TC_O_02

Success

TC_O_03 No DisplayMessages configured

Get a Specific Display Message

TC_O_07

Id

TC_O_08

Priority

TC_O_09

State

TC_O_11

Unknown parameters

TC_O_33 No DisplayMessages configured

TC_O_34

Known Id, but not matching State

TC_O_35

Known Id, but not matching Priority

Clear Display Message

TC_O_04

Success

TC_O_05

Unknown Key

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature no.

Feature

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

UI-0

CS: UI-0 and AQ-19
CSMS: UI-0

CS: UI-0 and UI-3
CSMS: UI-0

UI-0

UI-0

UI-0

UI-0

CS: If value "InFront" supported

UI-0 and UI-1.2

CS: If value "AlwaysFront" supported

UI-0 and UI-1.1

Supported MessagePriorities
InFront

Supported MessagePriorities
AlwaysFront

Only if multiple messagePriorities supported

UI-0

UI-0

UI-0

UI-0

UI-0

UI-0

UI-0

UI-0

UI-0 and ((UI-1.1 and
UI-1.2) or (UI-1.2 and
UI-1.3) or (UI-1.3 and
UI-1.1))

UI-0

UI-0

UI-1: Supported MessagePriorities

Supported MessagePriorities

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

49/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

4.3. Test Cases Advanced Security

TC Id

OCPP Compliance Testing Tool scenario

TLS - Client-side certificate

TC_A_07 valid certificate

TC_A_08 Invalid certificate

Update Charging Station Certificate by request of
CSMS

TC_A_11 Success - Charging Station Certificate

TC_A_14 Invalid certificate

TC_A_15 SignCertificateRequest Rejected

TC_A_23 CertificateSignedRequest Timeout

Upgrade Charging Station Security Profile

TC_A_21 No valid ChargingStationCertificate installed

Delete a certificate from a Charging Station

TC_M_23 Unable to delete the Charging Station Certificate

Set new NetworkConnectionProfile (2.1)

AllowSecurityDowngrade is false

AllowSecurityDowngrade = false - DM

TC_B_112
(2.1)

TC_B_113
(2.1)

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature
no.

Feature

M

M

M

M

M

M

M

M

C

C

M

C

C

If the Charging Station supports
CertificateSignedRequest Timeout

If the last ChargingStationCertificate can be removed
(Via other means than OCPP).

AS-3

AQ-3

AS-4.1

AS-4.1

Support for disallowing security
downgrades from profile 3 to 2

Support for disallowing security
downgrades from profile 3 to 2

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

50/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

4.4. Test Cases Smart Charging

TC Id

OCPP Compliance Testing Tool scenario

Set Charging Profile

TC_K_01 TxDefaultProfile - Specific EVSE

TC_K_10 TxDefaultProfile - All EVSE

TC_K_60 TxProfile with ongoing transaction on the specified

EVSE

TC_K_02 TxProfile without ongoing transaction on the specified

EVSE

TC_K_11 Unable to set TxProfile on all EVSE at once

TC_K_03 ChargingStationMaxProfile

TC_K_19 ChargingProfileKind is Recurring

TC_K_12 ChargerRateUnit Rejected

TC_K_13 Persistent over reboot

TC_K_14 Unexisting EVSEid

TC_K_28 TxDefaultProfile with transaction ongoing

TC_K_15 Not Supported

TC_K_16 Unknown transactionId

TC_K_21 ValidFrom

TC_K_22 ValidTo

TC_K_23 StartSchedule

TC_K_70 Multiple Profiles

TC_K_100
(2.1)

maxOfflineDuration

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature
no.

Feature

M

M

M

M

M

M

M

C

M

M

M

M

M

M

M

M

M

C

M

M

M

M

M

M

Remark: for CSMS TxDefaultProfile is mandatory,
specific values ValidFrom/ValidTo, Duration and limit
are tested.

Remark for CSMS: combination TxProfile and Relative is
mandatory

SC-4

Support for TxDefaultProfile on
EVSEID #0

Remark: for CSMS ChargingStationMaxProfile is
mandatory, specific values ValidFrom/ValidTo, Duration
and limit are tested.

Remark for CSMS: TxDefaultProfile or
ChargingStationMaxProfile. Configurable
recurrencyKind: weekly and Daily are mandatory

If only 1 of the 2 ChargingRateUnits is supported by the
Charging Station

NOT (SC-
2.1 and
SC-2.2)

Remark for CSMS: at least two different stacklevels,
amount of ChargingProfiles: at least 2 different id’s

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

51/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_K_102
(2.1)

TC_K_103
(2.1)

TC_K_136
(2.1)

TC_K_104
(2.1)

TC_K_105
(2.1)

TC_K_106
(2.1)

TC_K_107
(2.1)

TC_K_108
(2.1)

TC_K_129
(2.1)

TC_K_130
(2.1)

TC_K_131
(2.1)

TC_K_132
(2.1)

TC_K_133
(2.1)

TC_K_134
(2.1)

TC_E_117
(2.1)

limitAtSoc

Local time - TimeOffset

Local time - TimeZone

PriorityCharging

ChargingStationMaxProfile persistent over reboot

randomizedDelay

randomizedDelay - validations

randomizedDelay - random for each tx

PriorityCharging persistent over reboot

PriorityCharging unsupported

LocalGeneration unsupported

useLocalTime unsupported

RandomizedDelay unsupported

LimitAtSoC unsupported

Resuming transaction after interruption -
TxResumptionTimeout not expired -
TxAllowEnergyTransferResumption is true

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

C

C

C

C

M

C

C

C

C

C

C

C

C

C

C

C

M

M

Feature
no.

Feature

SC-3

Support for limiting based on
SoC

SC-5.1

SC-5.2

Support for using local time
(TimeOffset)

Support for using local time
(TimeZone)

SC-6

Support for using priority
charging

SC-7

SC-7

SC-7

SC-6

Support for using randomized
delays

Support for using randomized
delays

Support for using randomized
delays

Support for using priority
charging

NOT SC-6

Not
supportin
g
Bidirectio
nal Power
Transfer

NOT SC-5

NOT SC-7

NOT SC-3

Replace charging profile

TC_K_04 With chargingProfileId

M

M

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

52/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Remote start transaction with charging profile

TC_K_37 Success

Get Composite Schedule

TC_K_39 No ChargingProfile installed on Charging Station

TC_K_40 Stacking ChargingProfiles

TC_K_41 Combining chargingProfilePurposes

TC_K_42

chargingRateUnit not supported

TC_K_43 Specific EVSE

TC_K_44 Charging Station

TC_K_47 Unknown EVSEId

TC_K_112
(2.1)

randomizedDelay

Get Charging Profile

TC_K_29 EvseId 0

TC_K_30 EvseId > 0

TC_K_31 No EvseId

TC_K_32

chargingProfileId

TC_K_33 EvseId > 0 + stackLevel

TC_K_34 EvseId > 0 + chargingLimitSource

TC_K_35 EvseId > 0 + chargingProfilePurpose

TC_K_36 EvseId > 0 + chargingProfilePurpose + stackLevel

Clear Charging Profile

TC_K_05 With chargingProfileId

TC_K_06 With stackLevel/purpose combination for one profile

TC_K_07 With unknown stackLevel/purpose combination

TC_K_08 Without previous charging profile

TC_K_09 Clearing a TxDefaultProfile - With ongoing transaction

TC_K_24 With stackLevel/purpose combination for multiple

profiles

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature
no.

Feature

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

C

M

C

M

M

M

M

M

M

M

M

M

M

M

M

M

C

Remark for Charging Station: At least max stacklevel 1

SC-2

ChargingRateUnit

Remark for Charging Station: amount of
ChargingProfiles at least 3

If one of the ChargingRateUnits is not supported.

NOT (SC-
2.1 and
SC-2.2)

SC-7

Support for using randomized
delays

Applicable if the Charging Station has more than one
EVSE.

HFS-8 > 1

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

53/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Priority charging

TC_K_118 Requesting priority charging remotely

TC_K_119 Requesting priority charging locally

Dynamic charging profiles from CSMS

TC_K_121
(2.1)

TC_K_122
(2.1)

TC_K_123
(2.1)

Pull

Push

validations

Idle operationMode

TC_Q_125
(2.1)

Idle, minimizing energy consumption - Idle with
EvseSleep

TC_Q_126
(2.1)

Idle, minimizing energy consumption - Idle with
EvseSleep unsupported

TC_Q_127
(2.1)

Idle, minimizing energy consumption - Charging profile
validations

TC_K_135
(2.1)

Set Charging Profile - EvseSleep unsupported

EMS Control

TC_K_48 Set / Update External Charging Limit (not on a

transaction)

TC_K_50 Reset / release external charging limit - Without ongoing

transaction

TC_K_51 Reset / release external charging limit - With ongoing

transaction

TC_K_52 Set External Charging Limit (not on a transaction) -

ChargingStationExternalConstraints in report

TC_K_120 Smart Charging with EMS and LocalGeneration

TC_K_109 Set Charging Profile - MaxExternalConstraintsId

TC_K_110 Set Charging Profile - MaxExternalConstraintsId -

validations

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature
no.

Feature

C

C

M

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

C

SC-6

SC-6

SC-8

SC-8

SC-8

Support for using priority
charging

Support for using priority
charging

Support for dynamic charging
profiles

Support for dynamic charging
profiles

Support for dynamic charging
profiles

SC-9.1

Support for Idle operationMode
with EvseSleep

Support for Idle operationMode

SC-9 AND
NOT SC-
9.1

SC-9

Support for Idle operationMode

NOT SC-
9.1

SC-10

Support for EMS control

SC-10

Support for EMS control

SC-10

Support for EMS control

SC-10

Support for EMS control

SC-10

Support for EMS control

SC-10

Support for EMS control

SC-10

Support for EMS control

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

54/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Dynamic charging profiles by external system

TC_K_124 No Dynamic charging profile configured

TC_K_125 Dynamic charging profile configured

External V2X control

TC_Q_114 With a charging profile from an External System -

Dynamic external limits control

TC_Q_115 With a charging profile from an External System -

Dynamic setpoint control

TC_Q_116 With a charging profile from an External System -

Scheduled external limits control

TC_Q_111 With a charging profile from CSMS - setpoint

TC_Q_112 With a charging profile from CSMS - limit

TC_Q_113 With a charging profile from CSMS - Duration expired

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

C

C

C

C

C

C

C

C

Bidirectional Power Transfer is supported

Bidirectional Power Transfer is supported

Bidirectional Power Transfer is supported

M

Bidirectional Power Transfer is supported

M

Bidirectional Power Transfer is supported

Bidirectional Power Transfer is supported

Feature
no.

Feature

SC-10

Support for EMS control

SC-10

Support for EMS control

EMS Control

EMS Control

EMS Control

EMS Control

EMS Control

EMS Control

SC-10
AND
Bidirectio
nal Power
Transfer

SC-10
AND
Bidirectio
nal Power
Transfer

SC-10
AND
Bidirectio
nal Power
Transfer

SC-10
AND
Bidirectio
nal Power
Transfer

SC-10
AND
Bidirectio
nal Power
Transfer

SC-10
AND
Bidirectio
nal Power
Transfer

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

55/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

4.5. Test Cases ISO 15118 Support

TC Id

OCPP Compliance Testing Tool scenario

Update Charging Station Certificate by request of CSMS

TC_A_12

Success - V2G Certificate

Authorization using Contract Certificates 15118

TC_C_50

Online - Local contract certificate validation - Accepted

TC_C_51

Online - Local contract certificate validation - Rejected

TC_C_52

Online - Central contract certificate validation - Accepted

TC_C_53

Online - Central contract validation fails

TC_C_54

Offline - ContractValidationOffline is true

TC_C_55

Offline - ContractValidationOffline is false

End of charging process 15118

TC_E_46

End of charging process 15118

Set Charging Profile

TC_K_01

TxDefaultProfile - Specific EVSE

TC_K_10

TxDefaultProfile - All EVSE

TC_K_60

TxProfile with ongoing transaction on the specified EVSE

TC_K_02

TxProfile without ongoing transaction on the specified
EVSE

TC_K_11

Unable to set TxProfile on all EVSE at once

TC_K_03

ChargingStationMaxProfile

TC_K_19

ChargingProfileKind is Recurring

TC_K_12

ChargerRateUnit Rejected

TC_K_13

Persistent over reboot

TC_K_14

Unexisting EVSEid

TC_K_28

TxDefaultProfile with transaction ongoing

TC_K_16

Unknown transactionId

TC_K_21

ValidFrom

Conf. test
for CSMS

Condition / remark

Feature
no.

Feature

Conf. Test
for
Charging
Station

M

M

M

M

M

M

M

M

M

M

M

M

C

C

M

M

M

M

M

M

M

M

M

M

C

M

M

M

M

M

Applicable if the Charging Station supports
central contract validation.

Applicable if the Charging Station supports
central contract validation.

ISO-5

ISO-5

If only 1 of the 2 ChargingRateUnits is
supported by the Charging Station

NOT (SC-
2.1 and
SC-2.2)

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

56/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_K_22

ValidTo

TC_K_23

StartSchedule

Replace charging profile

TC_K_04 With chargingProfileId

Get Composite Schedule

TC_K_39

No ChargingProfile installed on Charging Station

TC_K_40

Stacking ChargingProfiles

TC_K_41

Combining chargingProfilePurposes

TC_K_42

chargingRateUnit not supported

TC_K_43

Specific EVSE

TC_K_44

Charging Station

TC_K_47

Unknown EVSEId

Get Charging Profile

TC_K_29

EvseId 0

TC_K_30

EvseId > 0

TC_K_31

No EvseId

TC_K_32

chargingProfileId

TC_K_33

EvseId > 0 + stackLevel

TC_K_34

EvseId > 0 + chargingLimitSource

TC_K_35

EvseId > 0 + chargingProfilePurpose

TC_K_36

EvseId > 0 + chargingProfilePurpose + stackLevel

Clear Charging Profile

TC_K_05 With chargingProfileId

TC_K_06 With stackLevel/purpose combination for one profile

TC_K_24 With stackLevel/purpose combination for multiple profiles

TC_K_07 With unknown stackLevel/purpose combination

TC_K_08 Without previous charging profile

TC_K_09

Clearing a TxDefaultProfile - With ongoing transaction

Conf. test
for CSMS

Condition / remark

Feature
no.

Feature

Conf. Test
for
Charging
Station

M

M

M

M

M

M

C

M

M

M

M

M

M

M

M

M

M

M

C

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

M

If one of the ChargingRateUnits is not
supported.

NOT (SC-
2.1 and
SC-2.2)

Applicable if the Charging Station has more
than one EVSE.

HFS-8 > 1

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

57/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Charging with load leveling based on High Level
Communication

TC_K_53

Success

TC_K_54

No SASchedule (rejected)

TC_K_55

EV charging profile exceeds limits

TC_K_56

Offline

Renegotiating a Charging Schedule ISO 15118-2

TC_K_57

Initiated by EV

TC_K_58

Initiated by CSMS

TC_K_59

Initiated by CSMS - Send NotifyEVChargingNeeds

Renegotiating a Charging Schedule ISO 15118-20

TC_K_113
(2.1)

TC_K_114
(2.1)

TC_K_116
(2.1)

TC_K_115
(2.1)

TC_K_117
(2.1)

TC_K_126
(2.1)

Initiated by CSMS

Initiated by EV

Adjusting charging schedule when energy needs change

ISO 15118-20 Dynamic Control Mode

Success

Adjusting charging schedule

Sets no charging profile

Certificate Installation EV

TC_M_26

Success

TC_M_100
(2.1)

ISO 15118-20 - Success

TC_M_27

Failed

Certificate Update EV

TC_M_28

Success

TC_M_29

Failed

Retrieve certificates from Charging Station

Conf. test
for CSMS

Condition / remark

Feature
no.

Feature

Conf. Test
for
Charging
Station

M

M

M

C

C

C

C

C

C

C

C

M

C

C

M

M

M

M

M

M

M

M

M

M

M

M

M

ISO-1.1

ISO 15118-2

ISO-1.1

ISO 15118-2

ISO-1.2

ISO 15118-20

ISO-1.2

ISO 15118-20

ISO-1.2

ISO 15118-20

ISO-1.2

ISO 15118-20

ISO-1.1

ISO 15118-2

ISO-1.2

ISO 15118-20

ISO-1.1

ISO 15118-2

ISO-1.1

ISO 15118-2

ISO-1.1

ISO 15118-2

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

58/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

TC_M_14 V2GRootCertificate

TC_M_15 V2GCertificateChain

TC_M_16 MORootCertificate

Install CA certificate

TC_M_03 V2GRootCertificate

TC_M_04 MORootCertificate

TC_M_101
(2.1)

OEMRootCertificate

Get Charging Station Certificate status

TC_M_24

Success

TC_M_25 Rejected

Clear Customer Information

TC_N_63

Clear and report - customerCertificate

Conf. Test
for
Charging
Station

M

M

M

M

M

C

M

M

C

Conf. test
for CSMS

Condition / remark

Feature
no.

Feature

M

M

M

M

M

M

M

C

ISO-1.2

ISO 15118-20

If Support for retrieving / deleting
CustomerInformation - CustomerCertificate is
supported.

ISO-4

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

59/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

4.6. Test Cases Bidirectional Power Transfer (2.1)

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature
no.

Feature

TC_Q_109 Central V2X control with dynamic CSMS setpoint - push

TC_Q_110 Central V2X control with dynamic CSMS setpoint - pull

TC_Q_122 Local V2X control for load balancing - threshold

validations

TC_Q_123 Local V2X control for load balancing - not supported

TC_Q_124 Local V2X control for load balancing - good flow

TC_Q_128 Going offline during V2X operation -

invalidAfterOfflineDuration = true

TC_K_101 Set Charging Profile - Change operation mode

TC_Q_100 V2X Authorisation - V2X Tx Measurands defined

TC_B_116 Reset ImmediateAndResume - With Ongoing

Transaction and SmartCharging - resuming
energytransfer

V2X Authorisation - ISO15118-20

TC_Q_101 Processing charging needs

TC_Q_102 Charging only (V2X control) before starting V2X -

Allowed Energy Transfer modes omitted

TC_Q_103 Charging needs rejected

TC_Q_130 has ISO15118ServiceRenegotiationSupport - Charging

needs rejected

TC_Q_104 Scheduled Control

TC_Q_107 Charging only (V2X control) before starting V2X

TC_Q_108 Central V2X control with charging schedule

Frequency support

TC_Q_117 Central V2X control - push

TC_Q_118 Central V2X control - Duration expired

TC_Q_119 Local V2X control - Charging profile validations

TC_Q_120 Local V2X control - AFRR support

M

M

C

C

M

M

M

C

C

C

C

C

C

C

C

C

C

M

M

M

M

M

C

M

C

C

C

C

BPT-2

Support for local loadbalancing

NOT BPT-
2

CSMS: ISO 15118 is supported

For CSMS: ISO 15118 is supported

For CSMS: ISO 15118 is supported

ISO 15118 is supported

ISO-1.2 ISO 15118-20

CS: ISO-
1.2

CS: ISO-
1.2 AND
NOT ISO-6

ISO-1.2
AND ISO-6

CS: ISO 15118-20

CS: ISO 15118-20

ISO 15118-20

ISO-1.2 ISO 15118-20

CS: ISO-
1.2

CS: ISO 15118-20

BPT-1

Frequency support

BPT-1

Frequency support

BPT-1

Frequency support

BPT-1

Frequency support

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

60/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature
no.

Feature

TC_Q_121 Local V2X control

C

BPT-1

Frequency support

4.7. Test Cases DER Control (2.1)

TC Id

OCPP Compliance Testing Tool scenario

Conf. test
for CSMS

Condition / remark

Conf. Test
for
Charging
Station

Feature
no.

Feature

TC_R_100 Starting a V2X session with DER control in EVSE -

Persistent DERControls

TC_R_101 Starting a V2X session with DER control in EVSE -

Device model

TC_R_102 Configure DER control settings at CS - clearing

controlTypes

TC_R_103 Configure DER control settings at CS - validations

TC_R_104 Configure DER control settings at CS - superseding

future DER control

TC_R_105 Configure DER control settings at CS - superseding

active DER control

TC_R_106 Configure DER control settings at CS - Active DER
control supersedes new DER control

TC_R_107 Configure DER control settings at CS

TC_R_108 Charging station reporting a DER event

M

M

M

M

M

M

M

C

M

M

ISO-1.2 AND HFS-4

ISO 15118-20

ISO-1.2
AND HFS-
4

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

61/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Chapter 5. OCPP 2.1 Mandatory Controller components per
profile

Controller components contain variables that describe the supported features of a Charging Station and influence its behavior. In
OCPP 2.1 we have configuration variables that are required or optional, but these are contained by controller components.
Functionalities cannot be tested without the accompanying controller component, so for certification the following controller
components are mandatory:

Certification Profile / Feature Id

Description

Core

Core: DM-0

Core: LA-0

Core: UI-0

Core: R-0

Core: P-0 (2.1)

Advanced Security

Smart Charging

ISO 15118 Support

OCPPCommCtrlr

TxCtrlr

DeviceDataCtrlr

ClockCtrlr

SecurityCtrlr

SampledDataCtrlr

AlignedDataCtrlr

AuthCtrlr
BatterySwapCtrlr (for product subtype Battery Swapping Charging Station only)

MonitoringCtrlr

LocalAuthListCtrlr

TariffCostCtrlr
DisplayMessageCtrlr

ReservationCtrlr

TariffCostCtrlr
WebPaymentsCtrlr

SecurityCtrlr (already part of Core)

SmartChargingCtrlr

ISO15118Ctrlr
SmartChargingCtrlr

Bidirectional Power Transfer (2.1)

V2XChargingCtrlr

DER Control (2.1)

DCDERCtrlr (depending on AC vs DC Charging Station)
ACDERCtrlr (depending on AC vs DC Charging Station)

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

62/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Chapter 6. Appendix A: additional questions for the Protocol
Implementation Conformance Statement

To perform the certification testing, the test lab need some additional information (for the test selection). This concerns the
following questions:

6.1. Questions for Charging Stations

Id

AQ-1

AQ-2

AQ-3

AQ-4

AQ-5

AQ-6

AQ-7

AQ-8

AQ-9

AQ-10

AQ-11

AQ-18

AQ-19

AQ-20

Additional questions for lab testing

Can the last CSMSRootCertificate be removed?

Does the Charging Station have a cable lock, which prevents the EV driver to connect the EV and EVSE
before authorization?

Can the last ChargingStationCertificate be removed (via other means than OCPP)?

Is there at least one unsupported NumberOfPhases?

Does the Charging Station have at least one hardWired monitor? If yes, which hardWired monitor should
be used for the certification test

Does the Charging Station have a pre-configured monitor? If yes, which pre-configured monitor should
be used for the certification test

Is your Charging Station able to download firmware while there is an ongoing transaction?

Does your Charging Station enforce a selection of EVSE (by design) prior to authorization?

Does your Charging Station support charging an EV using IEC 61851-1 (Mode 3)?

Does your Charging Station support setting a Delta monitor on the WriteOnly component.variable
SecurityCtrlr.BasicAuthPassword?

Does your Charging Station support a combined charging station Certificate (for both OCPP and ISO
15118)

Does your Charging Station have at least one connector with an (automatic) mechanized locking
mechanism on Charging Station side? (this is always true for connectorTypes; sType2 and sType3)

Does your Charging Station have at least 1 unsupported language code?

Does your Charging Station support more than 1 language?

6.2. Questions for CSMSs

Id

AQ-12

AQ-13

AQ-13.1

AQ-13.2

AQ-14

AQ-14.1

AQ-14.2

AQ-16

AQ-17

Additional questions for lab testing

Is a FullInventory requested during onboarding / booting test cases?

Does your CSMS support Absolute values for the following Charging Profiles:

TxDefaultProfile

ChargingStationMaxProfile

Does your CSMS support Recurring values for the following Charging Profiles:

TxDefaultProfile

ChargingStationMaxProfile

Does the CSMS reject unknown Charging Stations during websocket connection setup?

Can your CSMS be configured to first respond to a BootNotificationRequest with status Pending or
Rejected?

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

63/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Chapter 7. Appendix B: Hardware feature set

The table below gives an overview of the hardware feature set Ids that are used for determining whether test cases are needed /
applicable for certification.

Table 4. Hardware features

Id

HFS-1

HFS-2

HFS-3

HFS-4

HFS-5

HFS-6

HFS-7

HFS-8

HFS-9

HFS-10

HFS-11

HFS-12

HFS-13

Hardware Feature

Charging Station has a detachable cable

Charging Station has a fixed cable

Charging Station has AC support

Charging Station has DC support

Charging Station has 1 phase support

Charging Station has 2 phase support

Charging Station has 3 phase support

No. EVSEs of Charging Station

Communication technology

RFID readers

DC power level (kW)

Number of displays

Charging Station has Battery Swapping support

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

64/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Chapter 8. Appendix C: Features vs. OCPP use cases

The table below gives an overview of the use cases / configuration variables that the features are applicable for / referring to in the
OCPP 2.1 Specificiation Part 2.

Table 5. Optional features vs. related use cases

Id

Core

C-01

C-02

C-03

C-04

C-06

Feature

Support for offline authorization of transactions

Support for allowing Offline Authorization for Unknown Ids
(OfflineTxForUnknownIdEnabled)

Related use cases

C15, C10, C11, C12

C15

Support for maximizing energy for invalid ids (MaxEnergyOnInvalidId)

C15, E05

Support to limit StatusNotifications (MinimumStatusDuration)

Configuration Variable for G01

Authorization status after cable disconnected on EV side
(StopTxOnEVSideDisconnect)

C-06.1

Support for maintaining authorization when cable disconnected on EV side

C-06.2

Support for not maintaining authorization when cable disconnected on EV side

Support for using a Master Pass for charging stations with UI (MasterPassGroupId)

Support for using a Master Pass for charging stations without UI (MasterPassGroupId) C16

E10

E09

C16

C-07

C-08

C-09

C-13

C-14

C-20

C-21

Supported Transaction Start points (TxStartPoint)

C-09.1

Start transaction options - EVConnected

C-09.2

Start transaction options - Authorized

C-09.3

Start transaction options - DataSigned

C-09.4

Start transaction options - PowerPathClosed

C-09.5

Start transaction options - EnergyTransfer

C-09.6

Start transaction options - ParkingBayOccupancy

C-10

Supported Transaction Stop points (TxStopPoint)

C-10.1

Stop transaction options - EVConnected

C-10.2

Stop transaction options - Authorized

C-10.3

Stop transaction options - PowerPathClosed

C-10.4

Stop transaction options - EnergyTransfer

C-10.5

Stop transaction options - ParkingBayOccupancy

C-12

Unlocking of connector when cable disconnected on EV side
(UnlockOnEVSideDisconnect)

C-12.1

Support for unlocking connector when cable disconnected on EV side

C-12.2

Support for not unlocking when cable disconnected on EV side

Support for Reset per EVSE (AllowReset)

Support for retrieving / deleting CustomerInformation - CustomerIdentifier

Allowing New Sessions Pending a FirmwareUpdate
(AllowNewSessionsPendingFirmwareUpdate)

E01

E01-S2

E01-S3

E01-S4

E01-S5

E01-S6

E01-S1

E06

E06-S2

E06-S3

E06-S5

E06-S6

E06-S1

E09, E10

E09, E10

E09, E10

B11, B12

N09, N10

Configuration Variable for L01

Support for queuing all or only Transaction related messages until they are delivered to
the CSMS (QueueAllMessages)

Optional

Time related settings

C-23

C-25

Supported time sources (TimeSource)

Support for setting a TimeOffset (TimeOffset)

C-26

Support for setting the TimeZone (TimeZone)

C-28

Toggle sending clock aligned meter values when a transaction is ongoing / Idle
(AlignedDataSendDuringIdle)

C-29

TriggerMessage

C-29.1

Trigger message - MeterValues

C-29.2

Trigger message - TransactionEvent

Configuration Variable (B05,
B06)

Configuration Variable (B05,
B06)

Configuration Variable for J01

F06

F06

F06

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

65/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Id

Feature

C-29.3

Trigger message - LogStatusNotification

C-29.4

Trigger message - FirmwareStatusNotification

C-29.5

Trigger message - StatusNotification

C-29.6

Trigger message - BootNotification

C-29.7
(2.1)

Trigger message - CustomTrigger

Authorization options for local start

C-30

C-31

C-32

C-33

C-34

C-35

Authorization - using RFID ISO14443

Authorization - using RFID ISO15693

Authorization - using KeyCode

Authorization - using locally generated id

Authorization - MacAddress

Authorization - NoAuthorization

Authorization options for remote start (mandatory to support at least one)

C-36

C-37

C-38

C-39

C-40

Authorization - using RFID ISO14443

Authorization - using RFID ISO15693

Authorization - using centrally, in the CSMS (or other server) generated id

Authorization - NoAuthorization

Supported MeterValue Measurands

(SampledDataTx{Started,Updated,Ended}Measurands,
AlignedDataMeasurands)

C-41

Supported Cipher Suites

Related use cases

F06

F06

F06

F06

F06

C01

C01

C04

C06

C06

C02

C01

C01

C05

C02

J01, J02

See requirement A00.FR.318,
A00.FR.319, A00.FR.421,
A00.FR.422

Signed Metervalues (SampledDataSignReadings)

J01, J02

C-42

C-43

C-47

Install and activate Firmware with ongoing transaction(s)
(AllowNewSessionsPendingFirmwareUpdate)

Support for falling back to default OCPP reconnection mechanism when
NetworkConnection profile connection has failed

C-48

Authorization of remote start (AuthorizeRemoteStart)

C-48.1

Option for authorization in case of a remote start

C-48.2

Option for no authorization in case of a remote start

C-58

Option for disabling remote authorization (DisableRemoteAuthorization)

C-49

C-59

C-51

C-52

C-53

C-54

C-56

C-57

C-60

C-61

Authorization Cache (AuthCacheEnabled)

Option for disabling remote authorization for cached invalid idTokens
(AuthCacheDisablePostAuthorize)

Configurable TxStartPoint

Configurable TxStopPoint

Support for lifetime cached token (AuthCacheLifeTime)

Supported policies for replacing cached entries (AuthCachePolicy)

Support for providing the SummaryInventory

Support for cancelling ongoing log file upload

Support for cancelling ongoing firmware update

Security Profile 1 - Unsecured Transport with Basic Authentication

C-62 (2.1) Support for resuming transactions (ImmediateAndResume)

C-63 (2.1) Support for transaction limits

C-64 (2.1) Support for resuming transaction after interruption

C-65 (2.1) Support for DataCollectorLog

Authorization options for local stop

Configuration Variable for L01

B10 (FR.07)

F01, F02

F01, F02

F01, F02

Configuration Variable (B05,
B06)

C10, C11, C12

Configuration Variable for C10,
C12

Configuration Variable for E01

Configuration Variable for E06

Configuration Variable for C10

Configuration Variable for C10,
C11, C12

B07

N01 (AcceptedCanceled)

L01, L02 (AcceptedCanceled)

A01

B13

E16

E17

N01

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

66/68

Part 5 - Certification Profiles

Related use cases

C01

C01

C04

C02

S01-S04

H01-H04

H01.S3

H01.S1

Edition 2, 2025-12-03

Id

C-70

C-71

C-72

C-75

Feature

Authorization - using RFID ISO14443

Authorization - using RFID ISO15693

Authorization - using KeyCode

Authorization - NoAuthorization

C-76 (2.1) Support for Battery Swapping Stations

Reservation

Support for Reservation

Support for reservations of connectorType

Support for reservations of unspecified EVSE (ReservationNonEvseSpecific)

R-0

R-1

R-2

R-3

Support for disabling Reservations (ReservationEnabled)

Configuration Variable for H01

Advanced Device Management

DM-0

DM-3

Support for Advanced Device Management

Queue notifyEventRequest messages for specific severities
(OfflineMonitoringEventQueuingSeverity)

Local Authorization List Management

LA-0

LA-1

LA-2

LA-3

Support for Local Authorization List Management

Authorization list support (LocalAuthListEnabled)

Support for GetLocalListVersion

Option for disabling remote authorization for invalid idTokens stored at the Local
Authorization List (LocalAuthListDisablePostAuthorize)

Advanced User Interface

UI-0

UI-1

UI-2

Support for Advanced User Interface

Supported message priorities (DisplayMessageSupportedPriorities)

Supported message formats (DisplayMessageSupportedFormats)

Payment (2.1)

P-0

P-1

P-2

P-2.1

P-2.2

P-2.3

P-2.4

Support for Payment

Support for Tariff conditions

Supported Payment options

Payment by prepaid card

Integrated payment terminal

Stand alone payment terminal

QR code payment

Advanced Security

Optional

Configuration Variable for N07

D01-D02

Configuration Variable for C13

D02

Configuration Variable for C14

N02-N15

O01

O01

C17-C-25

I07

C17

C18

C24

C25

AS-2

AS-3

Additional root certificate check mechanism implemented
(AdditionalRootCertificateCheck)

Configuration Variable for M05

Update Charging Station Certificate - CertificateSignedRequest Timeout
(CertSigningWaitMinimum,CertSigningRepeatTimes)

Configuration Variable for A02,
A03

Smart Charging

SC-2

Supported charging rate units (ChargingScheduleChargingRateUnit)

SC-3 (2.1) Support for limiting based on SoC (limitAtSoC)

SC-4

Support for TxDefaultProfile on EVSEID #0

SC-5 (2.1) Support for using local time (useLocalTime)

SC-5.1

TimeOffset

SC-5.2

TimeZone

SC-6 (2.1) Support for using priority charging (PriorityCharging)

SC-7 (2.1) Support for using randomized delays (randomizedDelay)

SC-8 (2.1) Support for dynamic charging profiles

SC-9 (2.1) Support for operationMode Idle

SC-9.1
(2.1)

Support for operationMode Idle with EvseSleep

K01

K01

K01

K01

K01

K01

K01

K28-K29

Q10

Q10

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

67/68

Part 5 - Certification Profiles

Edition 2, 2025-12-03

Id

Feature

EMS Control (2.1)

SC-10
(2.1)

Support for EMS Control

ISO 15118 support

ISO-1
(2.1)

Supported ISO 15118 version

ISO-1.1

ISO 15118-2

ISO-1.2

ISO 15118-20

Related use cases

K23-K27

C01-C08, M01-M06

C01-C08, M01-M07, Q01

ISO-4

ISO-5

ISO-6
(2.1)

Support for retrieving / deleting CustomerInformation - CustomerCertificate

N09/N10

Charging Station can provide a contract certificate that it cannot validate to the CSMS Configuration Variable for C07

Support for ServiceRenegotiation

Bidirectional Power Transfer (2.1)

BPT-1

BPT-2

Frequency support

Support for local loadbalancing (LocalLoadBalancing)

Q01-Q02

Q07-Q08

Q09

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

68/68

Part 5 - Certification Profiles

