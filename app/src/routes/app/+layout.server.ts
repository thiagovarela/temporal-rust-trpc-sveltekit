import type { LayoutServerLoad } from './$types';
import {sql} from '$lib/db'
import type { User } from '$lib/types';

export const load: LayoutServerLoad = async ({ locals }) => {
	
    if(!locals.user) {
        return {}
    }
    
	const [user]: [User?] = await sql`SELECT * FROM users WHERE id = ${locals.user.id}`		
	
	return {
		user		
	};
};
