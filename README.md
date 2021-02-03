# Telescope
Telescope intends to replace [Observatory](https://github.com/rcos/observatory-server) 
as the RCOS website.

### Installation:
1. Install dependencies:
    1. Rust (see [https://www.rust-lang.org/](https://www.rust-lang.org/) for more info)
        ```shell
        $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        $ source ~/.cargo/env
        ```
    2. OpenSSL and libssl (see [https://www.openssl.org/](https://www.openssl.org/) for more info)
        ```shell
        $ sudo apt update
        $ sudo apt install openssl libssl-dev libssl-doc
        ```
    3. DbMate to run database migrations. See [https://github.com/amacneil/dbmate](https://github.com/amacneil/dbmate) for more info.
        ```shell
        $ sudo curl -fsSL -o /usr/local/bin/dbmate https://github.com/amacneil/dbmate/releases/latest/download/dbmate-linux-amd64
        $ sudo chmod +x /usr/local/bin/dbmate
        ```
    4. Docker and docker-compose to run telescope and the database locally. 
       this can be a complicated process, but there are good instructions online 
       [here](https://docs.docker.com/get-docker/).
       Message me for help.
       
2. Clone this repository:
    ```shell script
    $ git clone --recurse-submodules https://github.com/rcos/Telescope.git
    ```
   You need to make sure you get all of the submodules here using 
   `--recurse-submodules` otherwise you won't have any of the RCOS branding
   logos or icons, or any of the database migrations and setup.
   
3. Generate self-signed TLS/SSL certificate and keys for testing: 
    ```shell script
    $ mkdir tls-ssl
    $ openssl req -x509 -newkey rsa:4096 -nodes -keyout tls-ssl/private-key.pem -out tls-ssl/certificate.pem -days 365
    ```
   If you are running this in production, do not do this. Instead, you should use
   a certificate signed by a trusted certificate authority. See 
   [https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs](https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs)
   for more details.
   
4. Copy the configuration templates as follows:
    - `config_example.toml` -> `config.toml`
    - `.env.example` -> `.env`
    
    Then modify them to match your environment.
   
5. Build and start the docker images.
    ```shell
    $ docker-compose up -d 
    ```

6. Run the database migrations.
    ```shell
    $ dbmate --no-dump-schema --migrations-dir ./rcos-data/db/migrations/ up
    ```

7. At this point Postgrest, the PostgREST API, the Swagger API explorer, and 
   Telescope should all be running on your system. To shut them down and make 
   changes, run
   ```shell
   $ docker-compose down
   ```
