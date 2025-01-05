// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@fhenixprotocol/contracts/FHE.sol";
import "@fhenixprotocol/contracts/access/Permissioned.sol";

contract EncryptedToken is Ownable, Permissioned {
    mapping(string => address) public tokenAddresses;
    mapping(address => euint32) private _encryptedBalance;
    mapping(address => bool) public authorizedContracts;
    
    event ContractAuth(address indexed contractAddress);
    event TokensTransferred(address from, address to, euint32 amount);
    event Deposit(address indexed user, uint256 amount);
    event Withdrawal(address indexed user, uint256 amount);
    

    constructor() Ownable(msg.sender) {
    }

    function transferFromEncrypted(address from, address to, euint32 amount) public onlyAuthContract {
        FHE.req(FHE.gte(_encryptedBalance[from], amount));
        _encryptedBalance[from] = FHE.sub(_encryptedBalance[from], amount);
        _encryptedBalance[to] = FHE.add(_encryptedBalance[to], amount);
        emit TokensTransferred(from, to, amount);
    }

    function allowContract(address contractAddress) public onlyOwner {
        authorizedContracts[contractAddress] = true;
        emit ContractAuth(contractAddress);
    }
  
    function mint(address to, uint256 amount) public onlyOwner {
        if (amount > 100) {
            revert("Amount must be less than 100");
        }
        _encryptedBalance[to] = FHE.add(_encryptedBalance[to], FHE.asEuint32(amount));
    }
    
    function checkBalanceEncrypted(address account) public view returns (uint256) {
        if (msg.sender != account) {
            revert("Only the account owner can check their balance");
        }
        return FHE.decrypt(_encryptedBalance[account]);
    }

    modifier onlyAuthContract {
        require(authorizedContracts[msg.sender], "Only authorized contracts can call");
        _;
    }
}
