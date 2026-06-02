// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.26;

contract SimpleAuctionOptimized {
    address payable public immutable beneficiary;
    uint public immutable auctionEndTime;

    address public highestBidder;
    uint96 public highestBid;

    mapping(address => uint) public pendingReturns;
    bool public ended;

    error AuctionAlreadyEnded();
    error BidNotHighEnough(uint highestBid);
    error AuctionNotYetEnded();
    error AuctionEndAlreadyCalled();

    event HighestBidIncreased(address bidder, uint amount);
    event AuctionEnded(address winner, uint amount);

    constructor(uint biddingTime, address payable beneficiaryAddress) {
        beneficiary = beneficiaryAddress;
        auctionEndTime = block.timestamp + biddingTime;
    }

    function bid() external payable {
        if (block.timestamp > auctionEndTime) revert AuctionAlreadyEnded();
        if (msg.value <= highestBid) revert BidNotHighEnough(highestBid);
        if (highestBid != 0) pendingReturns[highestBidder] += highestBid;
        highestBidder = msg.sender;
        highestBid = uint96(msg.value);
        emit HighestBidIncreased(msg.sender, msg.value);
    }

    function withdraw() external returns (bool) {
        uint amount = pendingReturns[msg.sender];
        if (amount > 0) {
            pendingReturns[msg.sender] = 0;
            if (!payable(msg.sender).send(amount)) {
                pendingReturns[msg.sender] = amount;
                return false;
            }
        }
        return true;
    }

    function auctionEnd() external {
        if (block.timestamp < auctionEndTime) revert AuctionNotYetEnded();
        if (ended) revert AuctionEndAlreadyCalled();
        ended = true;
        emit AuctionEnded(highestBidder, highestBid);
        beneficiary.transfer(highestBid);
    }
}
