import { ethers } from "ethers";
import auctionArtifact from "../artifacts/contracts/SimpleAuction.sol/SimpleAuction.json" with { type: "json" };
import crowdArtifact from "../artifacts/contracts/Crowdfunding.sol/Crowdfunding.json" with { type: "json" };

const provider = new ethers.JsonRpcProvider("http://127.0.0.1:8545");
const signer0 = await provider.getSigner(0);
const signer1 = await provider.getSigner(1);
const signer2 = await provider.getSigner(2);
const signer3 = await provider.getSigner(3);

const auctionFactory = new ethers.ContractFactory(auctionArtifact.abi, auctionArtifact.bytecode, signer0);
const auction = await auctionFactory.deploy(60, await signer0.getAddress());
await auction.waitForDeployment();
console.log("SimpleAuction:", await auction.getAddress());

await auction.connect(signer1).bid({ value: ethers.parseEther("1.0") });
await auction.connect(signer2).bid({ value: ethers.parseEther("2.0") });
console.log("highest bid:", ethers.formatEther(await auction.highestBid()), "ETH");
console.log("highest bidder:", await auction.highestBidder());

const crowdFactory = new ethers.ContractFactory(crowdArtifact.abi, crowdArtifact.bytecode, signer0);
const crowd = await crowdFactory.deploy(ethers.parseEther("5.0"), 30);
await crowd.waitForDeployment();
console.log("Crowdfunding:", await crowd.getAddress());

await crowd.connect(signer2).contribute({ value: ethers.parseEther("2.0") });
await crowd.connect(signer3).contribute({ value: ethers.parseEther("3.0") });
console.log("totalRaised:", ethers.formatEther(await crowd.totalRaised()), "ETH");
console.log("goal reached:", (await crowd.totalRaised()) >= (await crowd.goal()));
