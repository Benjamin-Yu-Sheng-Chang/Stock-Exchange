import { invoke } from '@tauri-apps/api/core';

async function createAccountWindow(accountId: string) {
	console.log('createAccountWindow', accountId);
	try {
		const result = await invoke('create_account_window', {
			accountId
		});
		console.log('result', result);
	} catch (error) {
		console.error('Failed to create window:', error);
	}
}

export { createAccountWindow };
