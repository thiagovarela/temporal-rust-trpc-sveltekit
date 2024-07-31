import { lucia } from '$lib/lucia';
import type { RequestEvent } from '@sveltejs/kit';
import type { inferAsyncReturnType } from '@trpc/server';

// we're not using the event parameter is this example,
// hence the eslint-disable rule
// eslint-disable-next-line @typescript-eslint/no-unused-vars
export async function createContext(event: RequestEvent) {
	// it is important to pass the event downstream because a tRPC procedure is mutating the cookies.
	return {
		user: event.locals.user,
		session: event.locals.session,
		event
	};
}

export type Context = inferAsyncReturnType<typeof createContext>;
