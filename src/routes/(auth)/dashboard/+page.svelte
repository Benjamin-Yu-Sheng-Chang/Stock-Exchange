<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { createAccountWindow } from './service';
	import type { Account } from '$lib/types/account';
	import * as Table from '$lib/components/ui/table';
	import * as Card from '$lib/components/ui/card';
	import { invoke } from '@tauri-apps/api/core';

	let accounts: Account[] = [];

	async function createAccount() {
		const account = await invoke<Account>('create_account', {
			name: 'John Doe',
			balance: 1000
		});
		accounts = [...accounts, account];
	}
</script>

<div class="min-h-screen bg-gray-100">
	<header class="bg-white shadow">
		<div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
			<h1 class="text-3xl font-bold text-gray-900">Account Dashboard</h1>
		</div>
	</header>
	<main>
		<div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
			<div class="px-4 py-6 sm:px-0">
				<div class="flex justify-between items-center mb-6">
					<Button onclick={createAccount}>Create Account</Button>
					<!-- <div class="w-64">
						<Input type="text" placeholder="Search accounts..." bind:value={searchTerm} />
					</div> -->
				</div>
				<Card.Root>
					<Card.Header>
						<Card.Title>Generated Accounts</Card.Title>
					</Card.Header>
					<Card.Content>
						<Table.Root>
							<Table.Header>
								<Table.Row>
									<Table.Head>ID</Table.Head>
									<Table.Head>Username</Table.Head>
									<Table.Head>Email</Table.Head>
									<Table.Head>Created At</Table.Head>
								</Table.Row>
							</Table.Header>
							<!-- <TableBody>
								{#each filteredAccounts as account (account.id)}
									<TableRow>
										<TableCell>{account.id}</TableCell>
										<TableCell>{account.username}</TableCell>
										<TableCell>{account.email}</TableCell>
										<TableCell>{new Date(account.createdAt).toLocaleString()}</TableCell>
									</TableRow>
								{/each}
							</TableBody> -->
							<Table.Body>
								{#each accounts as account (account.id)}
									<Table.Row onclick={() => createAccountWindow(account.id)}>
										<Table.Cell>{account.id}</Table.Cell>
										<Table.Cell>{account.name}</Table.Cell>
										<Table.Cell>{account.balance}</Table.Cell>
										<Table.Cell>{account.created_at}</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					</Card.Content>
				</Card.Root>
			</div>
		</div>
	</main>
</div>
