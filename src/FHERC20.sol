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
    event ContractAuth(address indexed contractAddress);
    event TokensTransferred(string tokenType, address from, address to, euint32 amount);

    bytes public fhePubKey;

    constructor(bytes memory _fhePubKey) Ownable(msg.sender){
        fhePubKey = _fhePubKey;
        createTokens();
    }

    function setFhePubKey(bytes memory _newPubKey) public onlyOwner {
        fhePubKey = _newPubKey;
    }

    function createTokens() internal {
        tokenAddresses["Yes"] = address(new YesToken());
        tokenAddresses["No"] = address(new NoToken());
        tokenAddresses["USD"] = address(new UsdToken());

        emit TokenCreated("Yes", tokenAddresses["Yes"]);
        emit TokenCreated("No", tokenAddresses["No"]);
        emit TokenCreated("USD", tokenAddresses["USD"]);
    }

 // function placeOrder(string memory buyToken, euint32 amount) public {
        
        require(keccak256(abi.encodePacked(buyToken)) != keccak256(abi.encodePacked("USD")), "Cannot buy USD with USD");

       
        transferFromEncrypted(msg.sender, address(this), "USD", amount);


        emit OrderPlaced(msg.sender, buyToken, "USD", amount);
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

// YesToken 
contract YesToken is ERC20, Ownable, MintableERC20 {
    constructor() ERC20("YesToken", "YES") Ownable(msg.sender) {
        _mint(msg.sender, 1000000);
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}

// NoToken 
contract NoToken is ERC20, Ownable, MintableERC20 {
    constructor() ERC20("NoToken", "NO") Ownable(msg.sender) {
        _mint(msg.sender, 1000000);
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
// NoToken 
contract UsdToken is ERC20, Ownable, MintableERC20 {
    constructor() ERC20("USDToken", "USD") Ownable(msg.sender) {
        _mint(msg.sender, 1000000);
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
