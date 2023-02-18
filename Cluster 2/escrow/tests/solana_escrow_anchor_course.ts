import { Program, BN, IdlAccounts } from "@project-serum/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import { Escrow } from "../programs/solana_escrow_anchor_course";

type EscrowAccount = IdlAccounts<Escrow>["escrowAccount"];