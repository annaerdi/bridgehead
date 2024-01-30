![DALL·E 2024-01-29 20 52 59 - Create a minimalistic banner with a bridge design, using a sky blue color palette  The style should be simple and non-fractal, emphasizing cleanliness](https://github.com/annaerdi/ctrl-hack-zk/assets/26500470/9186743b-4b2b-49c1-85c5-a415a1e16805)

# Bridgehead

## Team Name
MisTTaken

## Team Members
- Anna E.: Technical Lead
- Fridolin D.: Product Owner

## Setup

The project is based on ink! smart contracts. For setting up the environment, building the contract and deployong it on a locally running substrate node, please follow the [official ink! documentation](https://use.ink/getting-started/setup/).

Moreover, the smart contract is deployed on the Aleph Zero Testnet under the following address:
```
5FyL9d5XUE1Qjtx63bbdDdaqsE8osPSuKT9KHcjSaLo4pedv
```

## Project Overview

### Description: 
We introduce a secure, blockchain-based certification system for establishing communication bridges between users in close physical proximity. Our prototype uses a dynamic QR code exchange, like a digital handshake, to initiate smart contracts that validate user identity and permit communication.
Our vision for the telco industry is security by architecture. Due to our pessimistic defense approach blockchain and smart contracts are essential.

### Problem Statement
In the current telecommunication environment, the use of outdated public phone numbers and an optimistic security stance has led to increased vulnerabilities, particularly to spoofing and phishing attacks. Our solution aims to rectify these problems by introducing a new security paradigm. By verifying the proximity and authenticity of users, leaving systems vulnerable to remote attacks. We propose a solution that ensures user presence and consent through blockchain-certified exchanges.

![image](https://github.com/annaerdi/ctrl-hack-zk/assets/26500470/feaec1b8-0f7f-47e5-bb08-1549290cd021)

This is a real screenshot from my phone. Translated it reads: "Potential scam". This indicates an optimistic security approach which is the industry norm.
The phone number is well known since several years, it is a well known phishing number.

## Solution Approach

### Concept
Our approach leverages the blockchain to certify the identity verification process, enhancing security by requiring users to be in close physical proximity to "bridge" their contacts or DIDs. The QR code serves as a user-friendly mechanism to initiate smart contracts and is designed to change frequently to prevent misuse and enhance security.

### Flow chart
![flow chart](https://github.com/annaerdi/ctrl-hack-zk/assets/26500470/4f2ce105-f639-4e7a-86fd-6a363afa3d4c)

## Simplified Communication Bridging Flow

1. **Initiation by User A**: User A presents a QR code to initiate a secure communication link.
2. **Action by User B**: User B scans the QR code, signaling their intent to connect.
3. **System Processing**: The system extracts and validates User A's phone number from the QR code.
4. **Validation Outcome**:
   - If valid, the system adds User A's phone number to User B's contacts, enabling them to interact.
   - If invalid, the system notifies User B, and the process is terminated.

### Proposed Additions

- **Multi-Factor Authentication**: Add a step for additional security verification after phone number validation.
- **User Consent**: Introduce explicit consent confirmation from both users before adding the phone number to contacts.
- **Post-Connection Verification**: Implement periodic checks to ensure the ongoing validity of the phone number.
- **Auditing**: Log all steps of the process, particularly validation and consent, on the blockchain for transparency.
- **Alternative Methods**: Provide an alternative connection method such as manual entry or NFC in case of QR code scan issues.


### Sequence diagramm
![architecture2](https://github.com/annaerdi/ctrl-hack-zk/assets/26500470/362590da-3f3e-4708-bcbc-40cffa22dc0f)

## Simplified Sequence for Communication Bridging

1. **QR Code Display**: User A displays a QR code, which contains their phone number encrypted for security.
2. **QR Code Scan**: User B scans this QR code using their device.
3. **Number Extraction**: The system extracts the phone number from the scanned QR code data.
4. **Number Validation**: The system validates User A's phone number using blockchain verification.
5. **Interaction Decision**:
   - **Valid Number**: The system adds User A's phone number to User B's contacts and enables interaction.
   - **Invalid Number**: The system notifies User B of the invalid number and ends the process without establishing a connection.

### Additional Details for Sequence Diagram

- **Validation Process**: The blockchain verification step confirms the authenticity of User A's phone number through smart contract validation.
- **Contact Addition**: Upon successful validation, a smart contract triggers the addition of User A's contact to User B's device.
- **Feedback to Users**: Both users receive system feedback—User A is informed that their number has been shared, and User B is notified that they can now communicate with User A.
- **Error Handling**: In case of an invalid number, an error message is displayed to User B, and the contact is not added.


### Key Features
1. **Blockchain for Certification:** Utilizing blockchain to record and certify the two users digital communication bridging. (main idea)
2. **Dynamic QR Code Exchange:** Providing a secure, frequently changing QR code that requires physical presence, preventing remote attacks. (planned for next iterations)
3. **Smart Contract Initiation:** The QR code acts as a trigger for the smart contract, ensuring a secure and automated certification process. (partial implementation as a MVP)
4. **Proximity Verification:** Leveraging technology similar to Bluetooth to confirm the physical proximity of the parties involved in the bridging process. (planned for next iterations)

## Technologies Used

### Blockchain Technology
- **Aleph Zero:** We're utilizing Aleph Zero's fast-paced network, renowned for its scalability and speed, which is critical for handling a high volume of transactions without performance degradation.
- **Ink! Smart Contracts:** Our prototype uses Ink! smart contracts for their adaptability and security, allowing for an automated and trustless certification process between parties.

### Additional Technologies
- **Mobile Phones:** Serving as the primary interface for users to interact with the blockchain via QR codes and potentially, in the future, proximity detection.
- **Classical Telco Infrastructure:** The existing telecommunication infrastructure will be used for actual voice and data transmission once the secure bridge has been established through our blockchain system.


## Design Thinking Process:

1. **Week 1 - Problem Space**: We thoroughly analyzed the main problems telecom companies face and how blockchain could add value. We decided the most important area to focus on was improving the core security of the business.

2. **Week 2 - Solution Space**: It was difficult to choose which part of the prototype to develop first. There are many necessary features to take a significant step forward in the telecom industry. In the end, we chose to create a simple version of the smart contract for bridging two Digital IDs, which demonstrates our use of blockchain technology effectively.


## Impact Assessment

Our blockchain certification prototype is designed to:
- Ensure physical proximity during user bridging for enhanced security.
- Reduce the incidence of remote spoofing and phishing attacks.
- Provide a decentralized and transparent certification process for secure communications.


## Limitations of the Prototype

- **User Behavior Change:** The new system will require users to adopt new behaviors, such as scanning QR codes for each communication bridge.
- **Dynamic QR Code Management:** Managing the frequent updates of QR codes can introduce back-end complexity.
- **Proximity Detection Reliability:** Future implementations must ensure reliable proximity detection across varied environments.
- **Blockchain Scalability:** The Aleph Zero network is chosen specifically for its scalability to handle the expected high frequency of bridging requests.
- **Integration with Current Systems:** Our prototype must be compatible with existing telecom infrastructure and user devices.
- **User Privacy Concerns:** It is imperative to balance proximity verification with the preservation of user privacy.

## Future Work and Development

- **Improving QR Code User Interface:** We plan to create a more seamless experience for users initiating the smart contract through QR codes.
- **Enhancing Proximity Detection:** We will work on increasing the accuracy and reliability of proximity detection technologies.
- **Exploring Scalability Solutions:** Continuing research into blockchain scalability solutions to accommodate mass adoption.
- **Developing Privacy-Preserving Techniques:** We are committed to developing techniques that will verify proximity while safeguarding user privacy.


## Conclusion

Project Bridgehead is at the forefront of a new era in telecommunications security. By leveraging a pessimistic security approach with the latest blockchain technology, we aim to significantly reduce vulnerabilities and set a new benchmark for secure communication in the telecom industry.

## DALLE-3 generated art

### Our logos

![bridgehead-logo](https://github.com/annaerdi/ctrl-hack-zk/assets/26500470/b5b7d242-9fdc-4481-b008-41fa80081e05)

Prompt: "Create a futuristic and sleek logo for 'Project Bridgehead'. The logo should incorporate a digital handshake, blockchain elements, and a stylized bridge. The design must be modern, tech-oriented, and convey security, innovation, 
and connectivity. The color scheme should include bold and professional shades of blue, black, and silver, along with lila, representing secure and verified communication bridges between users in close proximity. 
The bridge element should be seamlessly integrated, symbolizing the bridging of communication. The logo should be suited for a blockchain-based communication project."

Prompt for the banner: "Create a minimalistic banner with a bridge design, using a sky blue color palette. The style should be simple and non-fractal, emphasizing cleanliness and clarity in the composition."
