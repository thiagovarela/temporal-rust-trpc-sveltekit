import { router, publicProcedure } from '$lib/trpc';
import { getClient } from '$lib/temporal';

import { lucia } from '$lib/lucia';
import { nanoid } from 'nanoid/non-secure'
import { loginWithEmailInputSchema, signUpInputSchema } from '$lib/schemas';


export const authRouter = router({
	signUp: publicProcedure.input(signUpInputSchema).mutation(async ({ input }) => {
		const client = await getClient();
		const handle = await client.workflow.start("sign-up-wf", {
			workflowId: `sign-up-with-email-${nanoid()}`,
			args: [input],
			taskQueue: 'default',
			workflowRunTimeout: '5 seconds',
				retry: {
					maximumAttempts: 1
				}
		});
		const userId = await handle.result();
		return { userId };
	}),
	login: publicProcedure
		.input(loginWithEmailInputSchema)
		.mutation(async ({ input, ctx }) => {
			const client = await getClient();
			const handle = await client.workflow.start("login-wf", {
				workflowId: `login-with-email-${nanoid()}`,
				args: [input],
				taskQueue: 'default',
				workflowRunTimeout: '5 seconds',
				retry: {
					maximumAttempts: 1
				}
			});
			const sessionId = await handle.result();
			const sessionCookie = lucia.createSessionCookie(sessionId);
			ctx.event.cookies.set(sessionCookie.name, sessionCookie.value, {
				path: '.',
				...sessionCookie.attributes
			});

			return { success: true };
		})
});
