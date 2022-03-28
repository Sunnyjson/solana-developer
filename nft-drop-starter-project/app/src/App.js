import React, { useEffect, useState } from "react";
import "./App.css";
import twitterLogo from "./assets/twitter-logo.svg";

// Constants
const TWITTER_HANDLE = "_buildspace";
const TWITTER_LINK = `https://twitter.com/${TWITTER_HANDLE}`;

const App = () => {
  const [walletAddress, setWalletAddress] = useState(null)

  const checkWalletIsConnect = async () => {
    try {
      const { solana } = window;
      if (!!solana && !!solana.isPhantom) {
        console.log("Phantom wallet found!");
        // onlyIfTrusted 設置為true, 使phantom暫時拒絕用戶的連接請求, 只允許已授權的網站連接
        const response = await solana.connect({ onlyIfTrusted: true });
        console.log("Connect with public Key:", response.publicKey.toString());

        // set wallet address
        setWalletAddress(response.publicKey.toString())
      }
    } catch (error) {
      console.error(error);
    }
  };

  const connectWallet = async () => {
    const { solana } = window
    if(!!solana){
      const response = await solana.connect()
      console.log("Connect with public Key:", response.publicKey.toString());
      setWalletAddress(response.publicKey.toString())
    }
  };

  const renderNotConnectedContainer = () => {
    return (
      <button
        className="cta-button connect-wallet-button"
        onClick={connectWallet}
      >
        Connect to Wallet
      </button>
    );
  };

  useEffect(() => {
    const onLoad = async () => {
      await checkWalletIsConnect();
    };
    window.addEventListener("load", onLoad);
    return () => window.removeEventListener("load", onLoad);
  }, []);

  return (
    <div className="App">
      <div className="container">
        <div className="header-container">
          <p className="header">🍭 Candy Drop</p>
          <p className="sub-text">NFT drop machine with fair mint</p>
          {!walletAddress && renderNotConnectedContainer()}
        </div>
        <div className="footer-container">
          <img alt="Twitter Logo" className="twitter-logo" src={twitterLogo} />
          <a
            className="footer-text"
            href={TWITTER_LINK}
            target="_blank"
            rel="noreferrer"
          >{`built on @${TWITTER_HANDLE}`}</a>
        </div>
      </div>
    </div>
  );
};

export default App;
