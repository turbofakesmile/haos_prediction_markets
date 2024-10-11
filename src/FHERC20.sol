// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@fhenixprotocol/contracts/FHE.sol";
import "@fhenixprotocol/contracts/access/Permissioned.sol";

contract EncryptedTokens is Ownable, Permissioned {
    mapping(string => address) public tokenAddresses;
    mapping(address => mapping(string => euint32)) private _encryptedBalances;
    mapping(address => bool) public authorizedContracts;

    event TokenCreated(string tokenType, address tokenAddress);
    event BalanceChecked(address indexed checker, address indexed target, string tokenType, uint256 amount);
    event ContractAuth(address indexed contractAddress);
    event TokensMinted(string tokenType, address to, uint256 amount);

    bytes public fhePubKey;

    constructor(bytes memory _fhePubKey) {
        fhePubKey = _fhePubKey;
        createTokens();
    }

    function setFhePubKey(bytes memory _newPubKey) public onlyOwner {
        fhePubKey = _newPubKey;
    }

    function createTokens() internal {
        YesToken yesToken = new YesToken();
        NoToken noToken = new NoToken();

        tokenAddresses["Yes"] = address(yesToken);
        tokenAddresses["No"] = address(noToken);

        emit TokenCreated("Yes", address(yesToken));
        emit TokenCreated("No", address(noToken));
    }

    function mintEncrypted(string memory tokenType, address to, inEuint32 calldata amount, Permission calldata perm) public onlySender(perm) {
        ERC20 token = ERC20(tokenAddresses[tokenType]);
        token.mint(to, FHE.decrypt(amount));
        _encryptedBalances[to][tokenType] += amount;
        emit TokensMinted(tokenType, to, FHE.decrypt(amount));
    }

    function transferEncrypted(string memory tokenType, address to, inEuint32 calldata amount, Permission calldata perm) public onlySender(perm) {
        require(tokenAddresses[tokenType] != address(0), "Token does not exist");
        require(FHE.decrypt(_encryptedBalances[msg.sender][tokenType]) >= FHE.decrypt(amount), "Insufficient encrypted balance");

        _encryptedBalances[msg.sender][tokenType] -= amount;
        _encryptedBalances[to][tokenType] += amount;
    }

    function allowContract(address contractAddress) public onlyOwner {
        authorizedContracts[contractAddress] = true;
        emit ContractAuth(contractAddress);
    }

    function checkBalanceEncrypted(address account, string memory tokenType) public view onlyAuthContract returns (inEuint32 memory) {
        return FHE.asEuint32(_encryptedBalances[account][tokenType]);
    }

    function revealBalanceEncrypted(address account, string memory tokenType, Permission calldata perm) public onlyOwner onlySender(perm) view returns (uint256) {
        euint32 encryptedBalance = _encryptedBalances[account][tokenType];
        return FHE.decrypt(encryptedBalance);
    }

    function wrap(string memory tokenType, uint256 amount) public {
        address tokenAddress = tokenAddresses[tokenType];
        require(tokenAddress != address(0), "Token type does not exist");

        ERC20 token = ERC20(tokenAddress);
        require(token.balanceOf(msg.sender) >= amount, "Not enough tokens to wrap");

        token.transferFrom(msg.sender, address(this), amount);
        euint32 encryptedAmount = FHE.asEuint32(amount);
        _encryptedBalances[msg.sender][tokenType] += encryptedAmount;
    }

    function unwrap(string memory tokenType, inEuint32 calldata encryptedAmount, Permission calldata perm) public onlySender(perm) {
        euint32 amount = encryptedAmount;
        require(FHE.decrypt(_encryptedBalances[msg.sender][tokenType]) >= FHE.decrypt(amount), "Insufficient encrypted balance");

        _encryptedBalances[msg.sender][tokenType] -= amount;
        ERC20(tokenAddresses[tokenType]).transfer(msg.sender, FHE.decrypt(amount));
    }

    function mintTokens(string memory tokenType, address to, uint256 amount) public onlyOwner {
        ERC20 token = ERC20(tokenAddresses[tokenType]);
        token.mint(to, amount);
        emit TokensMinted(tokenType, to, amount);
    }

    modifier onlyAuthContract {
        require(authorizedContracts[tx.origin], "Only authorized contracts can query");
        _;
    }
}

contract YesToken is ERC20 {
    constructor() ERC20("Yes", "YES") {
        _mint(msg.sender, 1000000 * 10 ** uint(decimals()));
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}

contract NoToken is ERC20 {
    constructor() ERC20("No", "NO") {
        _mint(msg.sender, 1000000 * 10 ** uint(decimals()));
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
