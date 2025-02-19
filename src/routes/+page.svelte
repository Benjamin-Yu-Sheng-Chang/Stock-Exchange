<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import { Eye, EyeOff } from 'lucide-svelte';
	import { goto } from '$app/navigation';
	import { Label } from '$lib/components/ui/label/index.js';

	let username = '';
	let password = '';
	let showPassword = false;
	let confirmPassword = '';
	let showConfirmPassword = false;
	let name = '';

	function handleSignIn(e: Event) {
		e.preventDefault();
		if (!username || !password) return;

		// For testing purposes, redirect to dashboard
		goto('/dashboard');
	}

	function handleSignUp(e: Event) {
		e.preventDefault();
		if (!username || !password || !confirmPassword || password !== confirmPassword) return;
		console.log('Sign up with:', name, username, password);
	}
	function togglePasswordVisibility(field: 'password' | 'confirmPassword') {
		if (field === 'password') {
			showPassword = !showPassword;
		} else {
			showConfirmPassword = !showConfirmPassword;
		}
	}
</script>

<div class="flex items-center justify-center min-h-screen bg-gray-100">
	<Card.Root class="w-full max-w-md">
		<Card.Header>
			<Card.Title class="text-2xl font-bold text-center">
				<p>Stock Exchange</p>
			</Card.Title>
			<Card.Description class="text-center">
				Sign in to your account or create a new one
			</Card.Description>
		</Card.Header>
		<Card.Content class="p-6">
			<Tabs.Root value="signin" class="w-full">
				<Tabs.List class="grid w-full grid-cols-2">
					<Tabs.Trigger value="signin">Sign In</Tabs.Trigger>
					<Tabs.Trigger value="signup">Sign Up</Tabs.Trigger>
				</Tabs.List>
				<Tabs.Content value="signin">
					<form onsubmit={handleSignIn} class="space-y-4">
						<div class="space-y-2">
							<Label for="signin-username">Username</Label>
							<Input
								type="text"
								id="signin-username"
								placeholder="username"
								bind:value={username}
							/>
						</div>
						<div class="space-y-2">
							<Label for="signin-password">Password</Label>
							<div class="relative">
								<Input
									type={showPassword ? 'text' : 'password'}
									id="signin-password"
									bind:value={password}
									required
								/>
								{#if password !== ''}
									<button
										type="button"
										class="absolute inset-y-0 right-0 flex items-center pr-3"
										onclick={() => togglePasswordVisibility('password')}
									>
										{#if showPassword}
											<EyeOff class="h-4 w-4 text-gray-500" />
										{:else}
											<Eye class="h-4 w-4 text-gray-500" />
										{/if}
									</button>
								{/if}
							</div>
						</div>
						<Button disabled={!username || !password} type="submit" class="w-full">Sign In</Button>
					</form>
				</Tabs.Content>
				<Tabs.Content value="signup">
					<form onsubmit={handleSignUp} class="space-y-4">
						<div class="space-y-2">
							<Label for="signup-username">Username</Label>
							<Input
								type="text"
								id="signup-username"
								placeholder="username"
								bind:value={username}
							/>
						</div>
						<div class="space-y-2">
							<Label for="signup-password">Password</Label>
							<div class="relative">
								<Input
									type={showPassword ? 'text' : 'password'}
									id="signup-password"
									bind:value={password}
									required
								/>
								{#if password !== ''}
									<button
										type="button"
										class="absolute inset-y-0 right-0 flex items-center pr-3"
										onclick={() => togglePasswordVisibility('password')}
									>
										{#if showPassword}
											<EyeOff class="h-4 w-4 text-gray-500" />
										{:else}
											<Eye class="h-4 w-4 text-gray-500" />
										{/if}
									</button>
								{/if}
							</div>
						</div>
						<div class="space-y-2">
							<Label for="signup-confirm-password">Confirm Password</Label>
							<div class="relative">
								<Input
									type={showConfirmPassword ? 'text' : 'password'}
									id="signup-confirm-password"
									bind:value={confirmPassword}
								/>
								{#if confirmPassword !== ''}
									<button
										type="button"
										class="absolute inset-y-0 right-0 flex items-center pr-3"
										onclick={() => togglePasswordVisibility('confirmPassword')}
									>
										{#if showConfirmPassword}
											<EyeOff class="h-4 w-4 text-gray-500" />
										{:else}
											<Eye class="h-4 w-4 text-gray-500" />
										{/if}
									</button>
								{/if}
							</div>
						</div>
						<Button
							disabled={!username || !password || !confirmPassword || password !== confirmPassword}
							type="submit"
							class="w-full">Sign Up</Button
						>
					</form>
				</Tabs.Content>
			</Tabs.Root>
		</Card.Content>
	</Card.Root>
</div>
