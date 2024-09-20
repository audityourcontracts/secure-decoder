// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SignatureDecoder.sol";
import "./GnosisSafeMath.sol";

contract SafeDecoder is SignatureDecoder {
    using GnosisSafeMath for uint256;
    event AddressEmitted(address indexed emittedAddress, string message);
    address[] public addresses;

    function returnSigners(
        bytes32 dataHash,
        bytes memory data,
        bytes memory signatures,
        uint256 requiredSignatures
    ) public returns (address[] memory){
        // Check that the provided signature data is not too short
        //require(signatures.length >= requiredSignatures.mul(65), "GS020");
        // There cannot be an owner with address 0.
        address lastOwner = address(0);
        address currentOwner;
        uint8 v;
        bytes32 r;
        bytes32 s;
        uint256 i;
        for (i = 0; i < requiredSignatures; i++) {
            (v, r, s) = signatureSplit(signatures, i);
            if (v == 0) {
                // If v is 0 then it is a contract signature
                // When handling contract signatures the address of the contract is encoded into r
                currentOwner = address(uint160(uint256(r)));

                // Check that signature data pointer (s) is not pointing inside the static part of the signatures bytes
                // This check is not completely accurate, since it is possible that more signatures than the threshold are send.
                // Here we only check that the pointer is not pointing inside the part that is being processed
                //require(uint256(s) >= requiredSignatures.mul(65), "GS021");

                // Check that signature data pointer (s) is in bounds (points to the length of data -> 32 bytes)
                //require(uint256(s).add(32) <= signatures.length, "GS022");

                // Check if the contract signature is in bounds: start of data is s + 32 and end is start + signature length
                uint256 contractSignatureLen;
                // solhint-disable-next-line no-inline-assembly
                assembly {
                    contractSignatureLen := mload(add(add(signatures, s), 0x20))
                }
                //require(uint256(s).add(32).add(contractSignatureLen) <= signatures.length, "GS023");

                // Check signature
                bytes memory contractSignature;
                // solhint-disable-next-line no-inline-assembly
                assembly {
                    // The signature data for contract signatures is appended to the concatenated signatures and the offset is stored in s
                    contractSignature := add(add(signatures, s), 0x20)
                }
                if (!isAddressInArray(currentOwner)) {
                    addresses.push(currentOwner);
                }
                emit AddressEmitted(currentOwner, "v is 0");
                //require(ISignatureValidator(currentOwner).isValidSignature(data, contractSignature) == EIP1271_MAGIC_VALUE, "GS024");
            } else if (v == 1) {
                // If v is 1 then it is an approved hash
                // When handling approved hashes the address of the approver is encoded into r
                currentOwner = address(uint160(uint256(r)));
                // Hashes are automatically approved by the sender of the message or when they have been pre-approved via a separate transaction
                //(msg.sender == currentOwner || approvedHashes[currentOwner][dataHash] != 0, "GS025");
                if (!isAddressInArray(currentOwner)) {
                    addresses.push(currentOwner);
                }
                emit AddressEmitted(currentOwner, "v is 1");
            } else if (v > 30) {
                // If v > 30 then default va (27,28) has been adjusted for eth_sign flow
                // To support eth_sign and similar we adjust v and hash the messageHash with the Ethereum message prefix before applying ecrecover
                currentOwner = ecrecover(keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", dataHash)), v - 4, r, s);
                if (!isAddressInArray(currentOwner)) {
                    addresses.push(currentOwner);
                }
                emit AddressEmitted(currentOwner, "v is > 30");
            } else {
                // Default is the ecrecover flow with the provided data hash
                // Use ecrecover with the messageHash for EOA signatures
                currentOwner = ecrecover(dataHash, v, r, s);
                if (!isAddressInArray(currentOwner)) {
                    addresses.push(currentOwner);
                }
                emit AddressEmitted(currentOwner, "v is something else");
            }
            //require(currentOwner > lastOwner && owners[currentOwner] != address(0) && currentOwner != SENTINEL_OWNERS, "GS026");
            lastOwner = currentOwner;
            if (!isAddressInArray(currentOwner)) {
                addresses.push(currentOwner);
            }
        }
        return getAddresses();
    }

    function isAddressInArray(address _addr) internal view returns (bool) {
        for (uint256 i = 0; i < addresses.length; i++) {
            if (addresses[i] == _addr) {
                return true; // Address is already in the array
            }
        }
        return false; // Address not found in the array
    }

    function addAddress(address _addr) public {
        if (!isAddressInArray(_addr)) {
            addresses.push(_addr); // Only push if the address is not already in the array
        }
    }

    function getAddresses() public view returns (address[] memory) {
        return addresses;
    }

}
