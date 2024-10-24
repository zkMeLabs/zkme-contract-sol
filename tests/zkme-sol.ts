import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { ZkmeSol } from "../target/types/zkme_sol";
import {it} from "mocha";
import {Connection, Keypair, sendAndConfirmTransaction, SystemProgram, Transaction,clusterApiUrl} from "@solana/web3.js";
import {
    createAccount,
    createAssociatedTokenAccount,
    createInitializeAccountInstruction,
    createInitializeImmutableOwnerInstruction,
    createInitializeMintInstruction,
    createInitializeNonTransferableMintInstruction,
    ExtensionType,
    getAccount,
    getAccountLen,
    getMintLen,
    mintTo,
    TOKEN_2022_PROGRAM_ID
} from "@solana/spl-token";


//anchor test
describe("zkme-sol", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  let program = anchor.workspace.ZkmeSol as Program<ZkmeSol>;
  const zkmeSeed = anchor.utils.bytes.utf8.encode("zkme_admin");
  let adminPubKey;
  const payer = web3.Keypair.generate();

  before( async () => {
    [adminPubKey] = await anchor.web3.PublicKey.findProgramAddress(
        [zkmeSeed, anchor.AnchorProvider.env().wallet.publicKey.toBytes()],
        program.programId
    );
      });
  console.log("programId:",program.programId)



  it( "Is createAdmin", async () => {

    //const payer = web3.Keypair.generate();


// Connection to devnet cluster
//     const connection = new Connection('http://127.0.0.1:8899', 'confirmed');
         const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

          // console.log("connection:",connection)
          //
          // const programInfo = await connection.getAccountInfo(adminPubKey);
          // console.log("adminPubKey",adminPubKey)
          // console.log("accountInfo:",programInfo)
          //
          // const programInfoB = await connection.getAccountInfo(anchor.AnchorProvider.env().wallet.publicKey);
          // console.log("anchor.AnchorProvider.env().wallet.publicKey",anchor.AnchorProvider.env().wallet.publicKey)
          // console.log("accountInfoB:",programInfoB)



// Transaction signature returned from sent transaction
    let transactionSignature: string;

    // Generate new keypair for Mint Account
    const mintKeypair = Keypair.generate();
// Address for Mint Account
    const mint = mintKeypair.publicKey;
// Decimals for Mint Account
    const decimals = 0;
// Authority that can mint new tokens
    const mintAuthority = payer.publicKey;

    const airdropSignature = await connection.requestAirdrop(
        payer.publicKey,
        web3.LAMPORTS_PER_SOL, // get 1 SOL
    );
    await connection.confirmTransaction(airdropSignature);
    const balanceA = await connection.getBalance(payer.publicKey)
    console.log("airdropSignature success", balanceA);


    const airdropSignatureB = await connection.requestAirdrop(
              adminPubKey,
              web3.LAMPORTS_PER_SOL, // get 1 SOL
    );
    await connection.confirmTransaction(airdropSignatureB);
    const balanceB = await connection.getBalance(adminPubKey)
          console.log("airdropSignature success", balanceB);

    const mintLen = getMintLen([ExtensionType.NonTransferable,ExtensionType.ImmutableOwner]);
// Minimum lamports required for Mint Account
    const lamports = await connection.getMinimumBalanceForRentExemption(mintLen);





    // Instruction to invoke System Program to create new account
    const createAccountInstruction = SystemProgram.createAccount({
      fromPubkey: payer.publicKey, // Account that will transfer lamports to created account
      newAccountPubkey: mint, // Address of the account to create
      space: mintLen, // Amount of bytes to allocate to the created account
      lamports, // Amount of lamports transferred to created account
      programId: TOKEN_2022_PROGRAM_ID, // Program assigned as owner of created account
    });
    console.log("createAccountInstruction",createAccountInstruction)

// Instruction to initialize the NonTransferable Extension
    const initializeNonTransferableMintInstruction =
        createInitializeNonTransferableMintInstruction(
            mint, // Mint Account address
            TOKEN_2022_PROGRAM_ID, // Token Extension Program ID
        );
          console.log("initializeNonTransferableMintInstruction",initializeNonTransferableMintInstruction)

    // Instruction to initialize Mint Account data
    const initializeMintInstruction = createInitializeMintInstruction(
        mint, // Mint Account Address
        decimals, // Decimals of Mint
        anchor.AnchorProvider.env().publicKey, // Designated Mint Authority
        null, // Optional Freeze Authority
        TOKEN_2022_PROGRAM_ID, // Token Extension Program ID
    );
          console.log("initializeMintInstruction",initializeMintInstruction)
          // Add instructions to new transaction
    const transaction = new Transaction().add(
        createAccountInstruction,
        initializeNonTransferableMintInstruction,
        initializeMintInstruction,
    );

// Send transaction
    transactionSignature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [payer, mintKeypair], // Signers
    );

    console.log(
        "transactionSignature",transactionSignature
    )
        // Create Token Account for Playground wallet
        // const sourceTokenAccount = await createAccount(
        //     connection,
        //     payer, // Payer to create Token Account
        //     mint, // Mint Account address
        //     payer.publicKey, // Token Account owner
        //     undefined, // Optional keypair, default to Associated Token Account
        //     undefined, // Confirmation options
        //     TOKEN_2022_PROGRAM_ID, // Token Extension Program ID
        // );

        // Random keypair to use as owner of Token Account
        const randomKeypair = new Keypair();
// Create Token Account for random keypair
        const userKey = Keypair.generate()
//         const destinationTokenAccountAddr = Keypair.generate();
//         // Size of Token Account with extensionconst
//           const accountLen = getAccountLen([ExtensionType.ImmutableOwner]);
//         // Minimum lamports required for Token Account
//           const lamportsAcc = await connection.getMinimumBalanceForRentExemption(accountLen);
//           // Instruction to invoke System Program to create new account
//           const createImmuAccountInstruction = SystemProgram.createAccount({
//               fromPubkey: payer.publicKey, // Account that will transfer lamports to created account
//               newAccountPubkey: destinationTokenAccountAddr.publicKey, // Address of the account to create
//               space: accountLen, // Amount of bytes to allocate to the created account
//               lamports:lamportsAcc, // Amount of lamports transferred to created account
//               programId: TOKEN_2022_PROGRAM_ID, // Program assigned as owner of created account
//           });
//           console.log("createImmuAccountInstruction:",createImmuAccountInstruction)
          // Instruction to initialize the ImmutableOwner Extension
          const initializeImmutableOwnerInstruction =
              createInitializeImmutableOwnerInstruction(
                  destinationTokenAccountAddr.publicKey, // Token Account address
                  TOKEN_2022_PROGRAM_ID, // Token Extension Program ID
              );
          console.log("initializeImmutableOwnerInstruction:",initializeImmutableOwnerInstruction)
          // Instruction to initialize Token Account data
          const initializeAccountInstruction = createInitializeAccountInstruction(
              destinationTokenAccountAddr.publicKey, // Token Account Address
              mint, // Mint Account
              userKey.publicKey, // Token Account Owner
              TOKEN_2022_PROGRAM_ID, // Token Extension Program ID
          );
          console.log("initializeAccountInstruction:",initializeAccountInstruction)

          const transactionImmu = new Transaction().add(
              createImmuAccountInstruction,
              initializeImmutableOwnerInstruction,
              initializeAccountInstruction,
          );

// Send transaction
          const transactionSignatureS = await sendAndConfirmTransaction(
              connection,
              transactionImmu,
              [payer, destinationTokenAccountAddr], // Signers
          );

          console.log(
              "\nCreate Token Account:",
              `https://solana.fm/tx/${transactionSignatureS}?cluster=devnet-solana`,
          );


        const destinationTokenAccount = await createAccount(
            connection,
            payer, // Payer to create Token Account
            mint, // Mint Account address
            userKey.publicKey, // Token Account owner
            undefined, // Optional keypair, default to Associated Token Account
            {}, // Confirmation options
            TOKEN_2022_PROGRAM_ID, // Token Extension Program ID
        );
        console.log("destinationTokenAccount:",destinationTokenAccount)
//
//         // Mint tokens to sourceTokenAccount
//         transactionSignature = await mintTo(
//             connection,
//             payer, // Transaction fee payer
//             mint, // Mint Account address
//             sourceTokenAccount, // Mint to
//             mintAuthority, // Mint Authority address
//             100, // Amount
//             undefined, // Additional signers
//             undefined, // Confirmation options
//             TOKEN_2022_PROGRAM_ID, // Token Extension Program ID
//         );
//         console.log(
//             "transactionSignature B",transactionSignature);
//
//
//
// console.log("adminPub owner:",anchor.AnchorProvider.env().wallet.publicKey)
//
        const tx = await program.methods.createAdmin().accounts({
            authority: anchor.AnchorProvider.env().wallet.publicKey,
        }).rpc();
        console.log("Your create Admin transcation signature", tx);



        const tx3 = await program.methods.mintSbt().accounts({
            admin: adminPubKey,
            authority: anchor.AnchorProvider.env().wallet.publicKey,
            user: userKey.publicKey,
            mint: mint,
            tokenAccount: destinationTokenAccount,

        }).rpc();
        console.log("Your transcation signature", tx3);


        let thresholdKey = "thresholdKey";
        const BN = require('bn.js');
        let expirationDate = new BN('20250101', 10);
        let s = "data";
        let valiq = ["validQ"];
        console.log("start create Kyc")


          const txKyc = await program.methods.createKyc(thresholdKey,expirationDate,s,valiq).accounts({
            admin: adminPubKey,
            authority:anchor.AnchorProvider.env().wallet.publicKey,
              user: userKey.publicKey,

              tokenAccount: destinationTokenAccount,
        }).rpc();
        console.log("Your kyc transcation signature",txKyc);
  }
  )
}
)
