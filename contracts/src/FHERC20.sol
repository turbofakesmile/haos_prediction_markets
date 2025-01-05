// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@fhenixprotocol/contracts/FHE.sol";
import "@fhenixprotocol/contracts/access/Permissioned.sol";

contract EncryptedTokens is Ownable, Permissioned {
    mapping(string => address) public tokenAddresses;
    mapping(address => mapping(string => euint32)) private _encryptedBalances;
    mapping(address => bool) public authorizedContracts;
    
    // Track token balances
    mapping(address => uint256) public userBalances;
    address public tokenContract; // The ERC20 token contract address
    
    event TokenCreated(string tokenType, address tokenAddress);
    event ContractAuth(address indexed contractAddress);
    event TokensTransferred(string tokenType, address from, address to, euint32 amount);
    event Deposit(address indexed user, uint256 amount);
    event Withdrawal(address indexed user, uint256 amount);
    
    bytes public fhePubKey;

    constructor(bytes memory _fhePubKey, address _tokenContract) Ownable(msg.sender) {
        fhePubKey = _fhePubKey;
        tokenContract = _tokenContract;
        createTokens();
    }

    function setFhePubKey(bytes memory _newPubKey) public onlyOwner {
        fhePubKey = _newPubKey;
    }

    function createTokens() internal {
        tokenAddresses["Yes"] = address(new YesToken());
        tokenAddresses["No"] = address(new NoToken());
        emit TokenCreated("Yes", tokenAddresses["Yes"]);
        emit TokenCreated("No", tokenAddresses["No"]);
    }

    // deposit
    function deposit(uint256 amount) external {
        require(amount > 0, "Amount must be greater than 0");
        require(IERC20(tokenContract).transferFrom(msg.sender, address(this), amount), "Transfer failed");
        userBalances[msg.sender] += amount;
        emit Deposit(msg.sender, amount);
    }

    // withdraw
    function withdraw(uint256 amount) external {
        require(amount > 0, "Amount must be greater than 0");
        require(userBalances[msg.sender] >= amount, "Insufficient balance");
        
        userBalances[msg.sender] -= amount;
        require(IERC20(tokenContract).transfer(msg.sender, amount), "Transfer failed");
        emit Withdrawal(msg.sender, amount);
    }

   // get balance
    function getBalance(address user) external view returns (uint256) {
        return userBalances[user];
    }

    function transferFromEncrypted(address from, address to, string memory tokenType, euint32 amount) public onlyAuthContract {
        address tokenAddress = tokenAddresses[tokenType];
        require(tokenAddress != address(0), "Token does not exist");
        FHE.req(FHE.gte(_encryptedBalances[from][tokenType], amount));
        _encryptedBalances[from][tokenType] = FHE.sub(_encryptedBalances[from][tokenType], amount);
        _encryptedBalances[to][tokenType] = FHE.add(_encryptedBalances[to][tokenType], amount);
        emit TokensTransferred(tokenType, from, to, amount);
    }

    function allowContract(address contractAddress) public onlyOwner {
        authorizedContracts[contractAddress] = true;
        emit ContractAuth(contractAddress);
    }
  
    function mint(string memory tokenType, address to, uint256 amount) public onlyOwner {
        MintableERC20(tokenAddresses[tokenType]).mint(to, amount);
    }
    
    function checkBalanceEncrypted(string memory tokenType, address account) public view returns (euint32) {
        return _encryptedBalances[account][tokenType];
    }

    modifier onlyAuthContract {
        require(authorizedContracts[msg.sender], "Only authorized contracts can call");
        _;
    }
}

interface MintableERC20 {
    function mint(address to, uint256 amount) external;
}

contract YesToken is ERC20, Ownable, MintableERC20 {
    constructor() ERC20("YesToken", "YES") Ownable(msg.sender) {
        _mint(msg.sender, 1000000);
    }
    
    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}

contract NoToken is ERC20, Ownable, MintableERC20 {
    constructor() ERC20("NoToken", "NO") Ownable(msg.sender) {
        _mint(msg.sender, 1000000);
    }
    
    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
