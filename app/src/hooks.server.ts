import { createContext } from '$lib/trpc/context';
import { appRouter } from '$lib/trpc/router';
import type { Handle } from '@sveltejs/kit';
import { createTRPCHandle } from 'trpc-sveltekit';
import { sequence } from '@sveltejs/kit/hooks';

import { lucia } from '$lib/lucia';

const trpcContext: Handle = createTRPCHandle({ router: appRouter, createContext });
const validateSession: Handle = async ({ event, resolve }) => {
	let user, session;
	const sessionId = event.cookies.get(lucia.sessionCookieName);
	if (!sessionId) {
		user = null;
		session = null;
	} else {
		({ session, user } = await lucia.validateSession(sessionId));
	}

	if (!session) {
		const sessionCookie = lucia.createBlankSessionCookie();
		event.cookies.set(sessionCookie.name, sessionCookie.value, {
			path: '.',
			...sessionCookie.attributes
		});
	}
	event.locals.user = user;
	event.locals.session = session;
	const response = await resolve(event);
	return response;
};
export const handle: Handle = sequence(validateSession, trpcContext);
