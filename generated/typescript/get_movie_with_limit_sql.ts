import { QueryArrayConfig, QueryArrayResult } from "pg";

interface Client {
    query: (config: QueryArrayConfig) => Promise<QueryArrayResult>;
}

export const getMovieWithLimitQuery = `-- name: GetMovieWithLimit :many
SELECT
    id, created_date, modified_date, available_globally, locale, original_title, release_date, runtime, title
FROM
    movie
LIMIT $1`;

export interface GetMovieWithLimitArgs {
    limit: string;
}

export interface GetMovieWithLimitRow {
    id: string;
    createdDate: Date;
    modifiedDate: Date;
    availableGlobally: boolean | null;
    locale: string | null;
    originalTitle: string | null;
    releaseDate: Date | null;
    runtime: string | null;
    title: string;
}

export async function getMovieWithLimit(client: Client, args: GetMovieWithLimitArgs): Promise<GetMovieWithLimitRow[]> {
    const result = await client.query({
        text: getMovieWithLimitQuery,
        values: [args.limit],
        rowMode: "array"
    });
    return result.rows.map(row => {
        return {
            id: row[0],
            createdDate: row[1],
            modifiedDate: row[2],
            availableGlobally: row[3],
            locale: row[4],
            originalTitle: row[5],
            releaseDate: row[6],
            runtime: row[7],
            title: row[8]
        };
    });
}

