import { ethers } from "ethers";
import auctionArtifact from "../artifacts/contracts/SimpleAuction.sol/SimpleAuction.json" with { type: "json" };
import auctionOptArtifact from "../artifacts/contracts/SimpleAuctionOptimized.sol/SimpleAuctionOptimized.json" with { type: "json" };

const provider = new ethers.JsonRpcProvider("http://127.0.0.1:8545");
const signer0 = await provider.getSigner(0);
const signer1 = await provider.getSigner(1);
const signer2 = await provider.getSigner(2);

const f1 = new ethers.ContractFactory(auctionArtifact.abi, auctionArtifact.bytecode, signer0);
const c1 = await f1.deploy(60, await signer0.getAddress());
await c1.waitForDeployment();
const r1 = await provider.getTransactionReceipt(c1.deploymentTransaction().hash);
console.log("original deploy gas:", r1.gasUsed.toString());

const tx1a = await c1.connect(signer1).bid({ value: ethers.parseEther("1.0") });
const r1a = await tx1a.wait();
console.log("original bid 1 gas:", r1a.gasUsed.toString());

const tx1b = await c1.connect(signer2).bid({ value: ethers.parseEther("2.0") });
const r1b = await tx1b.wait();
console.log("original bid 2 gas:", r1b.gasUsed.toString());

const f2 = new ethers.ContractFactory(auctionOptArtifact.abi, auctionOptArtifact.bytecode, signer0);
const c2 = await f2.deploy(60, await signer0.getAddress());
await c2.waitForDeployment();
const r2 = await provider.getTransactionReceipt(c2.deploymentTransaction().hash);
console.log("optimized deploy gas:", r2.gasUsed.toString());

const tx2a = await c2.connect(signer1).bid({ value: ethers.parseEther("1.0") });
const r2a = await tx2a.wait();
console.log("optimized bid 1 gas:", r2a.gasUsed.toString());

const tx2b = await c2.connect(signer2).bid({ value: ethers.parseEther("2.0") });
const r2b = await tx2b.wait();
console.log("optimized bid 2 gas:", r2b.gasUsed.toString());
