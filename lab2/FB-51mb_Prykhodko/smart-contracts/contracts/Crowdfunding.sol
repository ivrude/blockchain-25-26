// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.26;

contract Crowdfunding {
    address public immutable owner;
    uint public immutable goal;
    uint public immutable deadline;

    mapping(address => uint) public contributions;
    uint public totalRaised;
    bool public withdrawn;

    event Contributed(address contributor, uint amount);
    event GoalReached(uint total);
    event Refunded(address contributor, uint amount);

    constructor(uint _goal, uint _durationDays) {
        owner = msg.sender;
        goal = _goal;
        deadline = block.timestamp + _durationDays * 1 days;
    }

    function contribute() external payable {
        require(block.timestamp <= deadline, "Deadline passed");
        contributions[msg.sender] += msg.value;
        totalRaised += msg.value;
        emit Contributed(msg.sender, msg.value);
        if (totalRaised >= goal) emit GoalReached(totalRaised);
    }

    function withdraw() external {
        require(msg.sender == owner, "Not owner");
        require(totalRaised >= goal, "Goal not reached");
        require(!withdrawn, "Already withdrawn");
        withdrawn = true;
        payable(owner).transfer(totalRaised);
    }

    function refund() external {
        require(block.timestamp > deadline, "Deadline not passed");
        require(totalRaised < goal, "Goal was reached");
        uint amount = contributions[msg.sender];
        require(amount > 0, "Nothing to refund");
        contributions[msg.sender] = 0;
        payable(msg.sender).transfer(amount);
        emit Refunded(msg.sender, amount);
    }
}
