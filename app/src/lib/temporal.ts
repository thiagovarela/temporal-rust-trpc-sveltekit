import { Connection, Client } from '@temporalio/client';

const globalTemporal = global as unknown as { client: Client };

export const getClient = async () => {
	if (globalTemporal.client) {
		return globalTemporal.client;
	}
	const connection = await Connection.connect();

	globalTemporal.client = new Client({
		connection,
		namespace: 'default'
	});
	return globalTemporal.client;
};
