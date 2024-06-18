# Discmon
The discmon is a discord bot written in Rust.
It's main purpose its spawn Pokémons in the chat and let the users catch them.

## Usage
To use the bot, you need to invite it to your server. You can get the invite link by checking [Discord Developers](https://discord.com/developers/applications) and selecting your application.

When a user sends a message, in a channel the bot has access to, it has a chance from 0 to `BOT_POKE_SPAWN_RATE` to spawn a Pokémon. If the chances are met, a chance of 0 to `BOT_POKE_SHINY_RATE` will be used to determine if the Pokémon is shiny.
A pokémon will be spawned in the chat and the users can catch it by reacting to the message with the pokéball emoji.

The bot will store the caught pokémons in the database and the users can check their pokémons by using the slash command `/pokedex`.

## Configuration
The bot can be configured by setting the following environment variables: (A `.env` template is provided in the repository)

### BOT Configuration
- `BOT_POKE_FLEE_TIME_SECS`: The time in seconds that the Pokémon will stay in the chat before fleeing. (Default - 60);
- `BOT_POKE_SPAWN_RATE`: The chance of a Pokémon spawning in the chat. (Default - 100);
- `BOT_POKE_SHINY_RATE`: The chance of a Pokémon being shiny. (Default - 4096);
- `BOT_POKEBALL_EMOJI_ID`: The emoji id of the pokéball emoji. (Default - <:pokeball:1251387543090626623>);
- `BOT_POKEDEX_POKES_PER_PAGE`: The amount of pokémons that will be displayed per page in the pokedex. (Default - 6);
- `BOT_POKEDEX_TIMEOUT_SECS`: The time in seconds that the pokedex message will be available. (Default - 120).

### Database Configuration
- `DATABASE_HOST`: The host of the database. (Default - localhost:8000);
- `DATABASE_NAMESPACE`: The namespace of the database. (Default - discmon);
- `DATABASE_NAME`: The name of the database. (Default - discmon);
- `DATABASE_USER`: The user of the database. (Default - DATABASE_USER);
- `DATABASE_PASS`: The password of the database. (Default - DATABASE_PASS).

### Discord Configuration
- `DISCORD_TOKEN`: The token of the discord bot.

### PokeAPI Configuration
- `POKEAPI_URL`: The URL of the PokeAPI. (Default - https://pokeapi.co/api/v2).

### Cache Configuration
- `CACHE_HOST`: The host of the cache server. (Default - localhost:6379);
- `CACHE_NAMESPACE`: The namespace of the cache server. (Default - discmon);
- `CACHE_USER`: The user of the cache server. (Default - CACHE_USER);
- `CACHE_PASS`: The password of the cache server. (Default - CACHE_PASS).

### Other Configuration
`RUST_LOG`: The log level of the application. (Default - info).

## Deployment
In order to deploy the bot, you need to have a Redis server and a SurrealDB server running and accessible by the host machine.
A `Dockerfile` and a `docker-compose.yaml` file are provided to help with the deployment.

## Features
- `dev_commands`: A set of commands that are only available for the developers. Enable this feature by enabling the `dev_commands` feature.

## Dependencies
- `Discord` - The discord service. It uses the Serenity library to interact with the discord API.
- `Cache` - A application that stores the pokemons and the users. By default, it uses a Redis server.
- `Database` - A application that stores the pokemons and the users. By default, it uses a SurrealDB server.
