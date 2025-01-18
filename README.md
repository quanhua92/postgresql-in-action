# PostgreSQL in Action


## Prepare data

This section shows how to load the [Netflix Sample Database](https://github.com/lerocha/netflixdb), a sample database with movies and TV shows into our PostgreSQL database.

+ Download the Netflix Sample Database

    ```bash
    wget https://github.com/lerocha/netflixdb/releases/download/v1.0.0/netflixdb-postgres.zip
    unzip netflixdb-postgres.zip
    ```
+ Run `psql` to load from the file `netflixdb-postgres.sql`. (Change parameters if needed)

    ```bash
    psql -h localhost -p 5432 -d postgresql-in-action -U postgres -f netflixdb-postgres.sql
    ```
+ Use `psql` to connect to the PostgreSQL database and run `\dt+` to see the list of tables

    ```
    psql -h localhost -p 5432 -d postgresql-in-action -U postgres

    \dt+
    ```
+ You should see the results similar to:

    ```
    postgresql-in-action=# \dt+
                                             List of relations
     Schema |     Name     | Type  |  Owner   | Persistence | Access method |    Size    | Description
    --------+--------------+-------+----------+-------------+---------------+------------+-------------
     public | episode      | table | postgres | permanent   | heap          | 8192 bytes |
     public | movie        | table | postgres | permanent   | heap          | 1240 kB    |
     public | season       | table | postgres | permanent   | heap          | 1048 kB    |
     public | tv_show      | table | postgres | permanent   | heap          | 424 kB     |
     public | view_summary | table | postgres | permanent   | heap          | 3488 kB    |
    (5 rows)
    ```
