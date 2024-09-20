// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {SafeDecoder} from "../src/SafeDecoder.sol";

contract SafeDecoderTest is Test {

    bytes32 dataHash;
    bytes data;
    bytes signatures;
    uint256 requiredSignatures;
    SafeDecoder decoder;

    function setUp() public {
        decoder = new SafeDecoder();
    }

    function test_RecoverSigners() public {
        dataHash = 0x186e111e15eb0d59a2feeb500f40413d6da00694ae92164de062af300157974c;
        data = hex"1901a630c4b736c54422250992ab01205a95e6824e0c34a2ab8d4406521b3d6da09a87ab52f44bb665650a1cb6e20d7659402e5acf82faaf2743051d0c496d625d65";
        signatures = hex"c13222e5cf3cf75e23221fbeeb569dd17be8bc84c39b010fd4cd2c88650e72853dd8ab927066423907f1996beab46f0c94850e193ce93ac25715a50ddafc67c01f970410639917b94312bea94a1ccd2a715d37de159bd3a2b877da2a4af96caa090b538a91aa46ed99d4a92c253fb3991c985f1fc5f8d060d6f6ec3e214409813320000000000000000000000000d967113224c354600b3151e27aaba53e3034f37200000000000000000000000000000000000000000000000000000000000000000169d415a0ed61f3da4b38f2dc83dc756510f4394c8415829e0b2ea49c5586cda86114e005f99a6648240abdbab8aad5af029633704e28c5ddcce0b7be2afe2c1620";
        requiredSignatures = 4;

        address[] memory addresses = decoder.returnSigners(dataHash, data, signatures, requiredSignatures);
        for (uint256 i = 0; i < addresses.length; i++) {
            address signer = addresses[i];
            console.log("The signer is ", signer);
        }
    }

    function test_RecoverSingleSigner1() public {
        dataHash = 0x4e82121a3bc2fb62c0b06ab5fff5ca965ceab4f51cc949c6e50d85ed63e6aa70;
        data = hex"1901a630c4b736c54422250992ab01205a95e6824e0c34a2ab8d4406521b3d6da09a7cd839f0a7ba33ab90b769cb93dc0aca579aad246b996db5cf34a4f31b7024e0";
        signatures = hex"8e64a89386af2f223b8433a1df65db8f0ff60544b2f02c56ec02b640d6fb15a11f7dffda5b99b0292b5e02d0cc44508d7d8f994515358e9da3016d8098b2258820";
        requiredSignatures = 1;

        address[] memory addresses = decoder.returnSigners(dataHash, data, signatures, requiredSignatures);
        for (uint256 i = 0; i < addresses.length; i++) {
            address signer = addresses[i];
            console.log("The signer is ", signer);
        }
        // Returns 0xD83b89E261D02B0f2f9E384B44907f8d380E9AF0
    }

    function test_RecoverSingleSigner2() public {
        dataHash = 0x4e82121a3bc2fb62c0b06ab5fff5ca965ceab4f51cc949c6e50d85ed63e6aa70;
        data = hex"1901a630c4b736c54422250992ab01205a95e6824e0c34a2ab8d4406521b3d6da09a7cd839f0a7ba33ab90b769cb93dc0aca579aad246b996db5cf34a4f31b7024e0";
        signatures = hex"3da7c6bd7c130430cf662de3d9af067c4a0629f849e29003c40ad3979cd670720c3ceb3a61e38fb7166353e3262eb6554c3f6571807cf22f9bdc4a83a56441661f";
        requiredSignatures = 1;

        address[] memory addresses = decoder.returnSigners(dataHash, data, signatures, requiredSignatures);
        for (uint256 i = 0; i < addresses.length; i++) {
            address signer = addresses[i];
            console.log("The signer is ", signer);
        }
        // Returns 0x9AF78003CecC2383d9D576A49c0C6b17fc34Ae34
    }

    function test_RecoverSingleSigner3() public {
        dataHash = 0x4e82121a3bc2fb62c0b06ab5fff5ca965ceab4f51cc949c6e50d85ed63e6aa70;
        data = hex"1901a630c4b736c54422250992ab01205a95e6824e0c34a2ab8d4406521b3d6da09a7cd839f0a7ba33ab90b769cb93dc0aca579aad246b996db5cf34a4f31b7024e0";
        signatures = hex"40082eba0f71627a3451d47132b8f4c266bc9be2bebe4424848757d10f13e76963af0b543a2357765d9f290410e919301ad065f0f2efa1233eb8634c93111f0e20";
        requiredSignatures = 1;

        address[] memory addresses = decoder.returnSigners(dataHash, data, signatures, requiredSignatures);
        for (uint256 i = 0; i < addresses.length; i++) {
            address signer = addresses[i];
            console.log("The signer is ", signer);
        }
        // Returns 0xfA54B4085811aef6ACf47D51B05FdA188DEAe28b
    }
}
