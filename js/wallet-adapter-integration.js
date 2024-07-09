document.addEventListener('DOMContentLoaded', (event) => {
    const { WalletAdapterNetwork } = solanaWeb3;
    const { Wallet } = solanaWalletAdapterWallets;
    const { BaseWalletAdapter } = solanaWalletAdapterBase;

    const connectWalletButton = document.getElementById('connect-wallet');
    const walletAddressDiv = document.getElementById('wallet-address');

    const wallet = new Wallet();

    async function connectWallet() {
        try {
            await wallet.connect();
            const address = wallet.publicKey.toString();
            walletAddressDiv.innerHTML = `Connected: ${address}`;
            
            htmx.ajax('POST', '/connect-wallet', {
                target: '#wallet-connection',
                swap: 'innerHTML',
                values: { address }
            });
        } catch (error) {
            console.error('Failed to connect wallet:', error);
        }
    }

    connectWalletButton.addEventListener('click', connectWallet);
});
