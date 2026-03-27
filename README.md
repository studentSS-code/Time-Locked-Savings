<img width="1054" height="437" alt="Screenshot 2026-03-27 203950" src="https://github.com/user-attachments/assets/081f2c21-27f0-4af8-9786-b972690f77e0" />
<img width="1366" height="768" alt="Screenshot 2026-03-24 141348" src="https://github.com/user-attachments/assets/160fbcee-d2ab-42ef-831b-21edb6989a70" />

# ⏳ Time-Locked Savings (Soroban Smart Contract)

## 📌 Project Description

Time-Locked Savings is a decentralized smart contract built on the Stellar Soroban platform that allows users to lock their funds until a specified future time. This enables disciplined saving, delayed withdrawals, and simple trustless financial planning.

---

## 🚀 What It Does

This smart contract allows users to:

* Deposit funds into a contract
* Set a custom unlock time
* Prevent withdrawal until the specified time is reached
* Retrieve funds only after the lock period expires

The contract ensures:

* Full user ownership
* No intermediary control
* Transparent and immutable rules

---

## ✨ Features

* 🔐 **Time Locking** — Funds are locked until a user-defined timestamp
* 👤 **User Authorization** — Only the depositor can withdraw their funds
* ⛔ **Early Withdrawal Protection** — Prevents access before unlock time
* 📊 **Deposit Tracking** — Users can view their locked savings
* ⚡ **Lightweight & Efficient** — Built using Soroban SDK

---

## 🛠️ Functions Overview

### `deposit(user, amount, unlock_time)`

Stores a user's funds with a specified unlock timestamp.

### `withdraw(user)`

Allows users to withdraw funds **only after** the unlock time has passed.

### `get_deposit(user)`

Fetches deposit details including:

* Amount
* Unlock timestamp

---

## 🧪 Example Use Cases

* Personal savings lock
* Vesting schedules
* Delayed payments
* Trust-minimized escrow

---

## 🌐 Deployed Smart Contract Link

https://stellar.expert/explorer/testnet/contract/CDWJEPFBNZFVWICFSNXAKHYJKOFSOV7JJOBN77MGPDDJJBSS3S2M4OVR

```
https://stellar.expert/explorer/public/contract/XXXXXXXX
```

---

## ⚙️ Tech Stack

* Rust
* Soroban SDK
* Stellar Blockchain

---

## 📦 Future Improvements

* Support multiple deposits per user
* Add token transfers (integrate with Stellar assets)
* Interest/yield mechanisms
* Emergency unlock with penalty

---

## 🧾 License

MIT License
