OCPP 2.1
Part 0 - Introduction

Edition 2, 2025-12-03

Table of Contents

Disclaimer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  1
Version History . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  2
1. Introduction. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
1.1. OCPP version 2.1 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
1.2. Terms and abbreviations . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  3
1.3. References . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  5
2. New functionality in OCPP 2.1 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  6
3. OCPP 2.1 Documentation Structure. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  8
3.1. Overview of Specification Parts . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  8
3.2. Functional Blocks . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  9
3.3. All Functional Blocks and use cases . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  10
4. Basic implementation of OCPP 2.1 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  15

Edition 2, 2025-12-03

Disclaimer

Copyright © 2010 – 2025 Open Charge Alliance. All rights reserved.

This document is made available under the *Creative Commons Attribution-NoDerivatives 4.0 International Public License*
(https://creativecommons.org/licenses/by-nd/4.0/legalcode).

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

1/15

Part 0 - Introduction

Edition 2, 2025-12-03

Version History

Version

Date

Description

2.1 Edition 2

2025-12-03

OCPP 2.1 Edition 2. All errata from OCPP 2.1 Part 0 until and including Errata 2025-
11 have been merged into this version of the specification.

2.1 Edition 1

2025-01-23

OCPP 2.1 Edition 1

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

2/15

Part 0 - Introduction

Edition 2, 2025-12-03

Chapter 1. Introduction

Electric Vehicles (EVs) are becoming the new standard for mobility all over the world. This development is only possible with a
good coverage of Charging Stations. To advance the roll out of charging infrastructure, open communication standards play a key
role: to enable switching from charging network without necessarily replacing all the Charging Stations, to encourage innovation
and cost effectiveness and to allow many and diverse players participate in this new industry.

Additionally, the EV charging infrastructure is part of the Smart Grid, a larger and still evolving ecosystem of actors, devices and
protocols. In this Smart Grid ecosystem, open communication standards are key enablers for bidirectional power flows, real time
information exchange, demand control and eMobility services.

The Open Charge Point Protocol (OCPP) is the industry-supported de facto standard for communication between a Charging
Station and a Charging Station Management System (CSMS) and is designed to accommodate any type of charging technique.
OCPP is an open standard with no cost or licensing barriers for adoption.

1.1. OCPP version 2.1

This specification defines version 2.1 of OCPP.

Version 2.1 is an extension of OCPP 2.0.1. OCPP 2.1 has its own JSON schemas, but the schemas are OCPP 2.0.1 schemas that
have been extended with optional fields that are used by OCPP 2.1 functionality. With the minor exceptions mentioned below, all
application logic developed for OCPP 2.0.1 will continue to work in OCPP 2.1 without any changes. The new features of OCPP 2.1,
of course, require new application logic.

Use case A02 & A03

The application logic in a CSMS for OCPP 2.0.1 for use cases A02 & A03 requires a small change in order to work in OCPP

2.1.
The SignCertificateRequest message has been extended with a requestId field, such that the resulting
CertificateSignedRequest message can be accurately mapped to the request that initiated it. Use of requestId is optional for

Charging Station, but when present, CSMS will have to use it in the subsequent CertificateSignedRequest message.
Note, that the updated application logic remains valid to use in OCPP 2.0.1.

Use case N02

The application logic in a Charging Station for OCPP 2.0.1 for use case N02 requires a small change in order to work for

OCPP 2.1.
The message NotifyMonitoringReportRequest has been extended with a required field in VariableMonitoringType:
eventNotificationType. Charging Station has to provide this field. It provides essential information to CSMS about the type of
monitor (HardWiredMonitor, PreconfiguredMonitor, CustomMonitor) that was missing in OCPP 2.0.1. Existing OCPP 2.0.1
logic in a CSMS that is not aware of this new field, will continue to work.

1.2. Terms and abbreviations

This section contains the terminology and abbreviations that are used throughout this document.

1.2.1. Terms

Term

Meaning

Charging Station

The Charging Station is the physical system where an EV can be charged. A Charging Station has one or
more EVSEs.

Charging Station
Management
System (CSMS)

Electric Vehicle
Supply Equipment
(EVSE)

Charging Station Management System: manages Charging Stations and has the information for authorizing
Users for using its Charging Stations.

EVSE is considered as an independently operated and managed part of the Charging Station that can deliver
energy to one EV at a time.

Energy Management
System (EMS)

In this document this is defined as a device that manages the local loads (consumption and production)
based on local and/or contractual constraints and/or contractual incentives. It has additional inputs, such as
sensors and controls from e.g. PV, battery storage.

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

3/15

Part 0 - Introduction

Edition 2, 2025-12-03

1.2.2. Abbreviations

Term

CSO

CSMS

EMS

EV

EVSE

RFID

Meaning

Charging Station Operator

Charging Station Management System

Energy Management System.

Electric Vehicle

Electric Vehicle Supply Equipment

Radio-Frequency Identification

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

4/15

Part 0 - Introduction

Edition 2, 2025-12-03

1.3. References

Table 1. References

Reference

[IEC61851-1]

Description

IEC 61851-1 2017: EV conductive charging system - Part 1: General requirements. https://webstore.iec.ch/
publication/33644

[IEC62559-2:2015]

Definition of the templates for use cases, actor list and requirements list. https://webstore.iec.ch/
publication/22349

[ISO15118-1]

ISO 15118-1 specifies terms and definitions, general requirements and use cases as the basis for the other
parts of ISO 15118. It provides a general overview and a common understanding of aspects influencing the
charge process, payment and load leveling. https://webstore.iec.ch/publication/9272

[OCPP1.5]

[OCPP1.6]

http://www.openchargealliance.org/downloads/

http://www.openchargealliance.org/downloads/

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

5/15

Part 0 - Introduction

Edition 2, 2025-12-03

Chapter 2. New functionality in OCPP 2.1

OCPP 2.1 introduces new functionality compared to OCPP 2.0.1.
The application logic for OCPP 2.0.1 remains valid, but will have to be extended to support the new features of OCPP 2.1.

Most important new features of OCPP 2.1 include support for ISO 15118-20 and extensive support for bidirectional power transfer
(V2X), and control of Charging Stations and EVs as Distributed Energy Resources (DER). New use cases have been added that
describe ad hoc payment, and Charging Stations can now do local cost calculation based on tariff information from CSMS.

Below is a list of sections of Part 2 of the specification that have new or updated functionality.

A Security

A02/A03 A requestId has been added to SignCertificateRequest. Added support for ISO 15118-20 certificates.
A05 Downgrading from security profile 3 to 2 is no longer prohibited.

B Provisioning

B09 SetNetworkProfileRequest has been extended with basicAuthPassword and identity.
B13 New use case to support resuming transaction after a reset.

C Authorization

Length of IdToken has been extended to 255 characters.

IdToken type is now a predefined list instead of enumeration to allow for easier extension.

C07/C08 ISO 15118 authorization use cases updated with ISO 15118-20 flows.

C10 Explicit requirement added about expiration in authorization cache.

C17 New use case for authorization with prepaid card.

C18-C23 New use cases for ad hoc payment with integrated payment terminal.

C24 New use case for ad hoc payment via stand-alone payment terminal.
C25 New use case for ad hoc payment via dynamic QR code.

E Transactions

E16 New use case for transactions with cost, energy, time, SoC limit.

E17 New use case for resuming a transaction after forced reboot.

F Remote Control

F06 Added CustomTrigger to TriggerMessageRequest.

F07 Net use case for remote start of transaction with limits.

G Availability

Availability notification using NotifyEventRequest for component Connector is now the preferred method, instead of
StatusNotification.

I Tariff and Cost

Introducing local cost calculation

I07-I11 New use cases to set default/user tariffs on charging station.

I12 New use case to report calculated cost during and at end of transaction.

J Metervalues

New metervalue location: Upstream.
New measurands for bidirectional charging.

K Smart Charging

New charging profile purposes PriorityCharging and LocalGeneration.

Added operationMode to ChargingSchedulePeriodType to facilitate bidirectional charging scenarios.

K01 Added dynamic charging profiles for frequent and unscheduled updates of limits.

K23-K27 New use cases for topologies with energy management systems.

K18-K20 New uses cases to support ISO 15118-20.

K21-K22 New uses cases for priority charging to allow user to overrule charging profile.

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

6/15

Part 0 - Introduction

Edition 2, 2025-12-03

M Certificate Management

M01 Updated use case for ISO 15118-20.

N Diagnostics

N01 Added support for data collector log on charging station.

N02 Added monitoring types TargetDelta and TargetDeltaRelative.

N07 Added severity to NotifyEventRequest.
N11-14 New use cases for optimized frequent periodic variable monitoring via an event stream. This utilizes the new

unconfirmed message type: SEND.

N15 Use case to set a frequent periodic monitoring via event stream.

O Display Message

O01 Added multi-language support.

Q Bidirectional Power Transfer

New section that describes control of bidirectional charging via charging profiles.

Q01-Q04 V2X control with centrally controlled charging profiles.

Q05-Q06 V2X control with externally controlled charging profiles.

Q07-Q08 Central can local frequency control.

Q09 Local load-balancing with V2X.

Q10-Q12 Idle state, offline and resuming after offline.

R DER Control

New section that describes grid control when EV and charging station are considered to be a Distributed Energy Resource

(DER).

U01 DER control in EVSE.

U02 DER control in EV.

U03 Hybrid DER control in both EVSE and EV.

U04 Configure DER controls in charging station.
U05 Charging station reporting a DER event.

S Battery Swapping

New section that describes how to control a battery swap station.

S01 Battery Swap Local Autorization

S02 Battery Swap Remote Start

S03 Battery Swap In/Out
S04 Battery Swap Charging

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

7/15

Part 0 - Introduction

Edition 2, 2025-12-03

Chapter 3. OCPP 2.1 Documentation Structure

3.1. Overview of Specification Parts

For readability and implementation purposes, OCPP 2.1 is divided in seven parts.

Table 2. Parts

Part 0

Part 1

Part 2

Part 3

Part 4

Part 5

Part 6

Introduction (this document)

Architecture & Topology

Specification:

Use Cases and Requirements, Messages, Data Types and Referenced Components and Variables

Appendices:
Security Events, Standardized Units of Measure, Components and Variables

Schemas

Implementation Guide JSON

Certification Profiles

Test Cases

The OCPP 2.1 specification is written using a structure, based on [IEC62559-2:2015]: "Use case methodology - Part 2: Definition of
the template for use cases, actor list and requirements list".

Part 2, the specification, is divided into 'Functional Blocks'. These Functional Blocks contain use cases and requirements.
Messages, Data Types and Referenced Components and Variables are described at the end of the document. The Appendices can

be found in the separate document: Part 2 - Appendices.

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

8/15

Part 0 - Introduction

Edition 2, 2025-12-03

3.2. Functional Blocks

OCPP 2.1 consists of the following Functional Blocks.

Table 3. Functional Blocks

Clause Functional Block Title

Description

A.

B.

Security

Provisioning

C.

Authorization

D.

E.

Local Authorization List Management

Transactions

F.

Remote Control

G.

H.

Availability

Reservation

I.

Tariff and Cost

J.

K.

Metering

Smart Charging

L.

Firmware Management

This Functional Block describes a security specification for the OCPP
protocol.

This Functional Block describes all the functionalities that help a CSO
provision their Charging Stations, allowing them to be registered and
accepted on their network and retrieving basic configuration information
from these Charging Stations.

This Functional Block describes all the authorization related functionality:
AuthorizeRequest message handling/behavior and Authorization Cache
functionality.

This Functional Block describes functionality for managing the Local
Authorization List.

This Functional Block describes the basic OCPP Transaction related
functionality for transactions that are started/stopped on the Charging
Station.

This Functional Block describes three types of use cases for remote control
management from the CSMS: Remote Transaction Control, Unlocking a
Connector and Remote Trigger.

This functional Block describes the functionality of sending status
notification messages.

This Functional Block describes the reservation functionality of a Charging
Station.

This Functional Block provides tariff and cost information to an EV Driver,
when a Charging Station is capable of showing this on a display. Before a
driver starts charging tariff information needs to be given, detailed prices
for all the components that make up the tariff plan applicable to this driver
at this Charging Station. During charging the EV Driver needs to be shown
the running total cost, updated at a regular, fitting interval. When the EV
Driver stops charging the total cost of this transaction needs to be shown.

This Functional Block describes the functionality for sending meter values,
on a periodic sampling and/or clock-aligned timing basis.

This Functional Block describes all the functionality that enables the CSO
(or indirectly a third party) to influence the charging current/power of a
charging session, or set limits to the amount of power/current a Charging
Station can offer to an EV.

This Functional Block describes the functionality that enables a CSO to
update the firmware of a Charging Station.

Certificate Management

This Functional Block provides the installation and update of certificates.

M.

N.

Diagnostics

O.

Display Message

P.

Data Transfer

Q

R

Bidirectional Power Transfer

DER Control

S

Battery Swapping

This Functional Block describes the functionality that enables a CSO to
request and track the upload of a diagnostics file from a Charging Station,
and to manage the monitoring of Charging Station data.

With the DisplayMessage feature OCPP enables a CSO to display a
message on a Charging Station, that is not part of the firmware of the
Charging Station. The CSO gets control over these messages: the CSO can
set, retrieve (get), replace and clear messages.

This Functional Block describes the functionality that enables a party to add
custom commands to OCPP, enabling custom extension to OCPP.

This Functional block extends Smart Charging with bidirectional power
transfer (V2X).

This Functional Block describes how charging stations and EVs can be
controlled as Distributed Energy Resources. It provides functions to
configure grid code parameters on a charging station via CSMS. It is
designed to support DER settings from IEC 61850 and IEEE 2030.5 on the
grid side, and ISO 15118-20 Amendment 1 on the EV side.

This Functional block describes how to deal with battery swap stations in
OCPP and adds the BatterySwap message.

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

9/15

Part 0 - Introduction

Edition 2, 2025-12-03

3.3. All Functional Blocks and use cases

The following table shows the full list of use cases supported by OCPP 2.1 and which use cases were already supported by OCPP

1.6 [OCPP1.6] and OCPP 2.1 .

Clause

Functional Block

UC ID

Use case name

OCPP 1.6

New in
OCPP
2.0.1

New in
OCPP 2.1

A

Security

B

Provisioning

C

Authorization

A01

A02

A03

A04

A05

B01

B02

B03

B04

B05

B06

B07

B08

B09

B10

B11

B12

B13

C01

C02

C03

C04

C05

C06

C07

C08

C09

C10

C11

C12

C13

C14

C15

C16

C17

C18

Update Charging Station Password for HTTP
Basic Authentication

Update Charging Station Certificate by
request of CSMS

Update Charging Station Certificate initiated
by the Charging Station

Security Event Notification

Upgrade Charging Station Security Profile

Cold Boot Charging Station

Cold Boot Charging Station - Pending

Cold Boot Charging Station - Rejected

Offline Behavior Idle Charging Station

Set Variables

Get Variables

Get Base Report

Get Custom Report

Setting a new NetworkConnectionProfile

Migrate to new CSMS

Reset - Without Ongoing Transaction

Reset - With Ongoing Transaction

Reset - With Ongoing Transaction - Resuming
Transaction

EV Driver Authorization using RFID

Authorization using a start button

Authorization using credit/debit card

Authorization using PIN-code

Authorization for CSMS initiated transactions

Authorization using local id type

Authorization using Contract Certificates

Authorization at EVSE using ISO 15118
External Identification Means (EIM)

Authorization by GroupId

Store Authorization Data in the Authorization
Cache

Clear Authorization Data in Authorization
Cache

Start Transaction - Cached Id

Offline Authorization through Local
Authorization List

Online Authorization through Local
Authorization List

Offline Authorization of unknown Id

Stop Transaction with a Master Pass

Authorization with prepaid card

Authorization using locally connected
payment terminal

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

10/15

Part 0 - Introduction

Edition 2, 2025-12-03

Clause

Functional Block

UC ID

Use case name

OCPP 1.6

New in
OCPP
2.0.1

New in
OCPP 2.1

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

C19

C20

C21

C22

C23

C24

C25

D01

D02

E01

E02

E03

E04

E05

E06

E07

E08

E09

E10

E11

E12

E13

E14

E15

E16

E17

E18

F01

F02

F03

F04

F05

F06

F07

G01

G02

G03

G04

G05

H01

H02

Cancelation prior to transaction

Cancelation after start of transaction, before
costs have been incurred.

Settlement at end of transaction

Settlement is rejected or fails

Increasing authorization amount

Ad hoc payment via stand-alone payment
terminal

Ad hoc payment via static or dynamic QR
code

Send Local Authorization List

Get Local List Version

Start Transaction Options

Start Transaction - Cable Plugin First

Start Transaction - IdToken First

Transaction started while Charging Station is
offline

Start Transaction - Id not Accepted

Stop Transaction Options

Transaction locally stopped by IdToken

Transaction stopped while Charging Station
is offline

When cable disconnected on EV-side: Stop
Transaction

When cable disconnected on EV-side:
Suspend Transaction

Connection Loss During Transaction

Inform CSMS of an Offline Occurred
Transaction

Transaction related message not accepted
by CSMS

Check transaction status

End of charging process

Transactions with fixed cost, energy, SoC or
time

Resuming transaction after forced

Battery Swapping

Remote Start Transaction - Cable Plugin First

Remote Start Transaction - Remote Start First

Remote Stop Transaction

Remote Stop ISO 15118 charging from CSMS

Remotely Unlock Connector

Trigger Message

Remote start with fixed cost, energy or time

Status Notification

Heartbeat

Change Availability EVSE

Change Availability Charging Station

Lock Failure

Reservation

Cancel Reservation

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

D

E

LocalAuthorizationList

Transactions

F

RemoteControl

G

Availability

H

Reservation

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

11/15

Part 0 - Introduction

Edition 2, 2025-12-03

Clause

Functional Block

UC ID

Use case name

OCPP 1.6

New in
OCPP
2.0.1

New in
OCPP 2.1

I

Tariff and Costs

J

Metering

K

SmartCharging

o

o

o

o

o

o

o

o

o

o

o

o

o

H03

H04

I01

I02

I03

I04

I05

I06

I07

I08

I09

I10

I11

I12

J01

J02

J03

K01

K02

K03

K04

K05

K06

K07

K08

K09

K10

K11

K12

K13

K14

K15

K16

K17

K18

K19

K20

Use a reserved EVSE

Reservation Ended, not used

Show EV Driver-specific tariff information

Show EV Driver running total cost during
charging

Show EV Driver final total cost after charging

Show fallback tariff information

Show fallback total cost message

Update Tariff Information During Transaction

Local Cost Calculation - Set Default Tariff

Local Cost Calculation - Receive User Tariff

Local Cost Calculation - Get Tariffs

Local Cost Calculation - Clear Tariffs

Local Cost Calculation - Change transaction
tariff

Local Cost Calculation - Cost Details of
Transaction

Sending Meter Values not related to a
transaction

Sending transaction related Meter Values

Charging Loop with metering information
exchange

SetChargingProfile

Central Smart Charging

Local Smart Charging

Internal Load Balancing

Remote Start Transaction with Charging
Profile

Offline Behavior Smart Charging During
Transaction

Offline Behavior Smart Charging at Start of
Transaction

Get Composite Schedule

Get Charging Profiles

Clear Charging Profile

Set / Update External Charging Limit With
Ongoing Transaction

Set / Update External Charging Limit Without
Ongoing Transaction

Reset / release external charging limit

External Charging Limit with Local Controller

Charging with load leveling based on High
Level Communication

Optimized charging with scheduling to the
CSMS

Renegotiating a Charging Schedule

ISO 15118-20 Scheduled Control Mode

ISO 15118-20 Dynamic Control Mode

Adjusting charging schedule when energy
needs change

K21

Requesting priority charging remotely

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

12/15

Part 0 - Introduction

Edition 2, 2025-12-03

Clause

Functional Block

UC ID

Use case name

OCPP 1.6

New in
OCPP
2.0.1

New in
OCPP 2.1

K22

K23

K24

K25

K26

K27

K28

L01

L02

L03

L04

M01

M02

M03

M04

M05

M06

N01

N02

N03

N04

N05

N06

N07

N08

N09

N10

N11

N12

N13

N14

N15

O01

Requesting priority charging locally

Smart Charging with EMS connected to
Charging Stations

Smart Charging with EMS connected to Local
Controller

Smart Charging with EMS acting as a Local
Controller

Smart Charging with Hybrid Local & Cloud
EMS

Smart Charging with EMS and
LocalGeneration

Dynamic charging profiles

Secure Firmware Update

Non-Secure Firmware Update

Publish Firmware file on Local Controller

Unpublish Firmware file on Local Controller

Certificate Installation EV

Certificate Update EV

Retrieve list of available certificates from a
Charging Station

Delete a specific certificate from a Charging
Station

Install CA certificate in a Charging Station

Get Charging Station Certificate status

Retrieve Log Information

Get Monitoring report

Set Monitoring Base

Set Variable Monitoring

Set Monitoring Level

Clear / Remove Monitoring

Alert Event

Periodic Event

Get Customer Information

Clear Customer Information

Set Frequent Periodic Variable Monitoring

Get Periodic Event Streams

Close Periodic Event Streams

Adjust Periodic Event Streams

Periodic Event Streams

Set DisplayMessage

OO2

Set DisplayMessage for Transaction

O03

O04

O05

O06

P01

P02

Q01

Get All DisplayMessages

Get Specific DisplayMessages

Clear a DisplayMessage

Replace DisplayMessage

Data Transfer to the Charging Station

Data Transfer to the CSMS

V2X Authorization

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

L

Firmware Management

M

Certificate Management

N

Diagnostics

O

Display Message

P

Q

DataTransfer

Bidirectional Power
Transfer

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

13/15

Part 0 - Introduction

Edition 2, 2025-12-03

Clause

Functional Block

UC ID

Use case name

OCPP 1.6

New in
OCPP
2.0.1

New in
OCPP 2.1

Q02

Q03

Q04

Q05

Q06

Q07

Q08

Q09

Q10

Q11

Q12

R01

R02

R03

R04

R05

S01

S02

S03

S04

Charging only (V2X control) before starting
V2X

Central V2X control with charging schedule

Central V2X control with dynamic CSMS
setpoint

External V2X setpoint control with a charging
profile from CSMS

External V2X control with a charging profile
from an External System

Central V2X control for frequency support

Local V2X control for frequency support

Local V2X control for load balancing

Idle, minimizing energy consumption

Going offline during V2X operation

Resuming a V2X operation after an offline
period

Starting a V2X session with DER control in
EVSE

Starting a V2X session with DER control in EV

Starting a V2X session with hybrid DER
control in both EV and EVSE

Configure DER control settings at Charging
Station

Charging station reporting a DER event

Battery swap local authorization

Battery swap remote start

Battery swap in/out

Battery swap charging

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

o

R

DER Control

S

Battery Swapping

OCPP is used in many different regions and for many different charging solutions. Not all functionalities offered
by OCPP will be applicable to all implementations. Implementers can decide what specific functionalities apply to

NOTE

their charging solution.
For interoperability purposes, the Open Charge Alliance introduces Certification Profiles in Part 5 of the
specification.

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

14/15

Part 0 - Introduction

Edition 2, 2025-12-03

Chapter 4. Basic implementation of OCPP 2.1

This section is informative.

The OCPP protocol describes a large number of use cases and messages, which are not all needed to implement a basic Charging
Station or CSMS. The table below lists messages that are typically implemented to deliver basic functionality for an OCPP managed
Charging Station. The purpose of this list is to guide developers that are new to OCPP.

The basic implementation set for OCPP 2.1 is the same as for OCPP 2.0.1.

NOTE

this table does not define what needs to be done to become OCPP 2.1 "certified". The functionality that is to be
implemented to become OCPP 2.1 certified is described in Part 5 of the specification, "Certification Profiles".

Table 4. OCPP 2.1 Basic Implementation

Functionality

Use cases

Booting a Charging Station

B01-B04

Configuring a Charging Station B05-B07

Messages

BootNotification

SetVariables, GetVariables and GetReportBase (respond
correctly to requests with reportBase = ConfigurationInventory,
FullInventory and SummaryInventory).

Resetting a Charging Station

B11-B12

Reset

Authorization options

One of C01, C02 and C04

Authorize

Transaction mechanism

E01 (one of S1-S6), E02-E03,
E05, E06 (one of S1-S6), E07-
E08, One of E09-E10, E11-E13

TransactionEvent

Availability

Monitoring Events

G01, G03-G04

G05, N07

Sending transaction related
Meter values

J02

DataTransfer

P01-P02

Only ChangeAvailability and StatusNotification.

A basic implementation of the NotifyEvent message to be used
to report operational state changes and problem/error
conditions of the Charging Station, e.g. for Lock Failure. Also
used for reporting built-in monitoring events.

TransactionEvent

Any OCPP implementations should at least be able to reject any
request for DataTransfer if no (special) functionality is
implemented.

NOTE

Please also refer to the section on Minimum Device Model in part 1.

OCPP 2.1 Edition 2 - © Open Charge Alliance 2025

15/15

Part 0 - Introduction

