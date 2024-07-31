import postgres from "postgres";
import { DATABASE_URL } from '$env/static/private';
import { dev } from '$app/environment'

export const sql = postgres(DATABASE_URL, {
    debug: dev 
});