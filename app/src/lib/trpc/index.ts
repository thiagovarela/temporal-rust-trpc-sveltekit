import type { Context } from '$lib/trpc/context';
import { initTRPC, TRPCError } from '@trpc/server';

const t = initTRPC.context<Context>().create();

export const auth = t.middleware(async ({ next, ctx }) => {
	if (!ctx.session) throw new TRPCError({ code: 'UNAUTHORIZED' });
	return next();
});

export const router = t.router;
export const publicProcedure = t.procedure;
export const protectedProcedure = t.procedure.use(auth);
