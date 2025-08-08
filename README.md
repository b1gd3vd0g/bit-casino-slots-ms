# Bit Casino -- Slots Microservice

> [!NOTE]
> This service is currently **stable** but under development.

A **REST API** written in **Rust** handling **placing bets** and **game logic** for slot machines in **Bit Casino** - a virtual gambling simulator.

### Features

- A custom slot machine game based off of two's complement binary numbers - **Byte Builder**.
- Special events (free spins and bonuses) based on spins.
- Thorough tested with customized rules to maintain a slight **house edge** while staying fun for players.

## How to use this repository

This service is not very useful on its own. It relies upon the [**Player Microservice**](https://github.com/b1gd3vd0g/bit-casino-player-ms) and the [**Currency Microservice**](https://github.com/b1gd3vd0g/bit-casino-currency-ms).

To test this API alongside the whole environment, you can follow the instructions in the [Infrastructure](https://github.com/b1gd3vd0g/bit-casino-infra) repository to test all services locally using **Docker Compose**.

You can then interact via the frontend at `localhost:60000` or call the integrated slots microservice directly at `localhost:60603`.

## Functionality

The slots microservice supports the functionality to spin a virtual slot machine (provided you have the bits to wager) for a chance to win more bits.

It has the potential to host multiple different **machines** for the player to spin, but for now, it only supports **Byte Builder**.

## Related Repositories

- [Player Microservice](https://github.com/b1gd3vd0g/bit-casino-player-ms) - Handles account creation and player authentication.
- [Currency Microservice](https://github.com/b1gd3vd0g/bit-casino-currency-ms) - Handles bit wallet creation and safe transactions.
- [Reward Microservice](https://github.com/b1gd3vd0g/bit-casino-reward-ms) - Handles daily bonus claims and streaks.
- [Frontend](https://github.com/b1gd3vd0g/bit-casino-frontend) - A react app creating a user-friendly interface with which to interact with the backend.
- [Infrastructure](https://github.com/b1gd3vd0g/bit-casino-infra) - Allows for integration testing locally using **docker compose**.
