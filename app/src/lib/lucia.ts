
import { Lucia, TimeSpan } from 'lucia';
import { dev } from '$app/environment';


import { PostgresJsAdapter } from "@lucia-auth/adapter-postgresql";
import {sql} from '$lib/db'

// Lucia validates the session, that's why it needs to connect here. 
// Everything else is done via workflows.
const adapter = new PostgresJsAdapter(sql, {
	user: "users",
	session: "sessions"
});

export const lucia = new Lucia(adapter, {	
	sessionCookie: {
		attributes: {			
			secure: !dev
		}
	}
});

declare module 'lucia' {
	interface Register {
		Lucia: typeof lucia;
	}
}
