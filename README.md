# Substrate Cumulus Parachain for BlockID®

1. What is the BlockID® dNFT?

The BlockID® dNFT is, "Data Management Protocol - DMP", the user’s interface allowing individuals and organizations to interact with and leverage their data. By connecting to the network, users gain instant access to a cryptocurrency wallet, allowing them to leverage the BlockIQ® DAO to control and monetize their most valuable asset, data. When users generate a new cryptocurrency wallet, they create a non-transferable unique dNFT (dynamic NFT) called a BlockID® dNFT. Using the BlockID® dNFT, the user decides which permission-based data the network can use to improve the scoring algorithms. As data is continually added to the network, the individual generates more tokens that serve as receipts for the provided data.

This is the secure global financial identity allowing lenders to provide compliant lending options to consumers and businesses without exposing the personal or sensitive data of the user. The BlockID® NFT is a dNFT (dynamic NFT) that can be updated and altered over time alongside the network and be used to self-collateralize loans. A BlockID® dNFT is the credit reporting registry protocol tracking current and historical debt obligations tied to the user’s BlockID® dNFT.

The user’s BlockID® is the key and access point for the user to control and customize data sharing options that give the individual the ability to interact with their BlockID® and the data ledger, storing hashed versions of all the data captured on the network. This allows them to customize the amount and type of data they allow for analysis when they would like to monetize their data utilizing the network. User-specific data earnings are calculated and fractionalized from the total earnings generated on the monetization layer and stored on the earnings ledger before being distributed to each individual’s BlockID® dNFT wallet.

2. How can we use the BlockID® dNFT?

This is the parachain for BlockID®, which is to mint the NFT, which is a kind of a card that includes the user's credit-related information after registering on the FreshCredit® platform.

The "ts-client" directory is for the interface, which lets the users communicate with the parachain.

    - Edit dummy data

        You can edit "ts-client/scripts/credits.ts to change the list of the dummy data of the users' information to be uploaded.

    - Register the credit information

        In the ts-client directory, run:

            yarn credits:register

        This will:

            Read credit information from credits.ts
            Submit a registration application for each user
            Approve the registration for each user (placeholder for KYC process)
    
    - Mint the NFTs

        yarn credits:mint

        This will mint various NFTs for each user. The NFTs will be initially owned by the system account.

    - View on-chain summary

        yarn credits:summary

        This will display a short summary about each registered users, the number of NFTs available for this user, and a random NFT, pulled from the blockchain state.