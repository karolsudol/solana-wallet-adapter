// js/wallet-adapter-integration.js
document.addEventListener('DOMContentLoaded', (event) => {
    const { PhantomWalletAdapter } = solanaWalletAdapterWallets;

    const connectWalletButton = document.getElementById('connect-wallet');
    const walletAddressDiv = document.getElementById('wallet-address');

    const wallet = new PhantomWalletAdapter();

    async function connectWallet() {
        try {
            await wallet.connect();
            const address = wallet.publicKey.toString();
            walletAddressDiv.innerHTML = `Connected: ${address}`;
            
            const response = await fetch('/connect-wallet', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                },
                body: `address=${encodeURIComponent(address)}`,
            });

            if (response.ok) {
                const html = await response.text();
                document.body.innerHTML = html;
                // Re-attach event listener to the new connect button
                document.getElementById('connect-wallet').addEventListener('click', connectWallet);
            } else {
                console.error('Failed to connect wallet on server');
            }
        } catch (error) {
            console.error('Failed to connect wallet:', error);
        }
    }

    connectWalletButton.addEventListener('click', connectWallet);
});