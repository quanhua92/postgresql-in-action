-- name: GetMovieWithLimit :many
SELECT
    *
FROM
    movie
LIMIT $1;

