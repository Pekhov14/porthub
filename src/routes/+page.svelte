<script lang="ts">
	import { onMount } from "svelte";
	import * as Avatar from "$lib/components/ui/avatar";
	import { Badge } from "$lib/components/ui/badge";
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import * as Collapsible from "$lib/components/ui/collapsible";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import * as Field from "$lib/components/ui/field";
	import { Input } from "$lib/components/ui/input";
	import * as Item from "$lib/components/ui/item";
	import { Separator } from "$lib/components/ui/separator";
	import * as Sidebar from "$lib/components/ui/sidebar";
	import { Textarea } from "$lib/components/ui/textarea";
	import { cn } from "$lib/utils";
	import {
		BriefcaseBusiness,
		CheckCircle2,
		ChevronRight,
		ChevronsUpDown,
		Download,
		Edit3,
		ExternalLink,
		FileJson,
		FolderKanban,
		House,
		Languages,
		Link2,
		Moon,
		Plus,
		Search,
		Settings,
		Sun,
		Trash2,
		Upload,
		X,
	} from "@lucide/svelte";

	type BookmarkLink = {
		id: string;
		title: string;
		url: string;
		description: string;
		tags: string[];
	};

	type Group = {
		id: string;
		name: string;
		description: string;
		links: BookmarkLink[];
	};

	type Space = {
		id: string;
		name: string;
		description: string;
		icon: "home" | "work" | "project";
		groups: Group[];
	};

	type Theme = "light" | "dark";
	type Language = "ru" | "en";

	const messages = {
		ru: {
			add: "Добавить",
			addGroup: "Группа",
			addLink: "Ссылка",
			addSpace: "Добавить пространство",
			addLinkTitle: "Добавить ссылку",
			addGroupTitle: "Добавить группу",
			addSpaceTitle: "Создать пространство",
			deleteGroup: "Удалить группу",
			deleteLink: "Удалить ссылку",
			deleteSpace: "Удалить пространство",
			confirmDeleteGroup: "Удалить группу",
			confirmDeleteLink: "Удалить ссылку",
			confirmDeleteSpace: "Удалить пространство",
			description: "Описание",
			dark: "Темная",
			editGroup: "Редактировать группу",
			editLink: "Редактировать ссылку",
			editSpace: "Редактировать пространство",
			export: "Экспорт",
			groupDialogDescription: "Группа появится внутри выбранного пространства.",
			import: "Импорт",
			importError: "Не удалось импортировать JSON. Проверьте структуру файла.",
			language: "Язык",
			light: "Светлая",
			linkDialogDescription: "Заполните ссылку, название, описание и теги.",
			linkPlaceholder: "https://example.com",
			links: "ссылок",
			name: "Название",
			noGroups: "Нет групп",
			noLinks: "В этой группе пока нет ссылок",
			openAll: "Открыть все",
			placeholderDescription: "Что находится по ссылке",
			placeholderGroup: "Документация",
			placeholderSpace: "Работа",
			placeholderTags: "Введите тег и нажмите Enter",
			russian: "Русский",
			save: "Сохранить",
			search: "Поиск...",
			selectGroup: "Выберите группу в боковой панели",
			settings: "Настройки",
			spaceDialogDescription: "Задайте короткое название для списка пространств.",
			spaces: "Пространства",
			tags: "Теги",
			theme: "Тема",
			title: "Название",
			url: "Ссылка",
			user: "Пользователь",
			works: "Работает",
			english: "English",
		},
		en: {
			add: "Add",
			addGroup: "Group",
			addLink: "Link",
			addSpace: "Add space",
			addLinkTitle: "Add link",
			addGroupTitle: "Add group",
			addSpaceTitle: "Create space",
			deleteGroup: "Delete group",
			deleteLink: "Delete link",
			deleteSpace: "Delete space",
			confirmDeleteGroup: "Delete group",
			confirmDeleteLink: "Delete link",
			confirmDeleteSpace: "Delete space",
			description: "Description",
			dark: "Dark",
			editGroup: "Edit group",
			editLink: "Edit link",
			editSpace: "Edit space",
			export: "Export",
			groupDialogDescription: "The group will be added to the selected space.",
			import: "Import",
			importError: "Could not import JSON. Check the file structure.",
			language: "Language",
			light: "Light",
			linkDialogDescription: "Fill in the URL, title, description, and tags.",
			linkPlaceholder: "https://example.com",
			links: "links",
			name: "Name",
			noGroups: "No groups",
			noLinks: "This group has no links yet",
			openAll: "Open all",
			placeholderDescription: "What this link contains",
			placeholderGroup: "Documentation",
			placeholderSpace: "Work",
			placeholderTags: "Type a tag and press Enter",
			russian: "Русский",
			save: "Save",
			search: "Search...",
			selectGroup: "Select a group in the sidebar",
			settings: "Settings",
			spaceDialogDescription: "Set a short name for the spaces list.",
			spaces: "Spaces",
			tags: "Tags",
			theme: "Theme",
			title: "Title",
			url: "URL",
			user: "User",
			works: "Works",
			english: "English",
		},
	} satisfies Record<Language, Record<string, string>>;

	const icons = {
		home: House,
		work: BriefcaseBusiness,
		project: FolderKanban,
	};

	let query = $state("");
	let theme = $state<Theme>("light");
	let language = $state<Language>("ru");
	let activeSpaceId = $state("work");
	let activeGroupId = $state("docs");
	let importInput = $state<HTMLInputElement | null>(null);
	let expandedSpaces = $state<Record<string, boolean>>({
		personal: true,
		work: true,
		project: true,
	});

	let spaceDialogOpen = $state(false);
	let editingSpaceId = $state<string | null>(null);
	let spaceName = $state("");

	let groupDialogOpen = $state(false);
	let editingGroupId = $state<string | null>(null);
	let groupName = $state("");

	let linkDialogOpen = $state(false);
	let editingLinkId = $state<string | null>(null);
	let linkTitle = $state("");
	let linkUrl = $state("");
	let linkDescription = $state("");
	let linkTagDraft = $state("");
	let linkTagList = $state<string[]>([]);

	let spaces = $state<Space[]>([
		{
			id: "personal",
			name: "Личное",
			description: "Идеи, сервисы и личные закладки",
			icon: "home",
			groups: [
				{
					id: "reading",
					name: "Чтение",
					description: "Материалы на потом",
					links: [
						{
							id: "hn",
							title: "Hacker News",
							url: "https://news.ycombinator.com",
							description: "Технические новости и обсуждения",
							tags: ["news", "tech"],
						},
					],
				},
			],
		},
		{
			id: "work",
			name: "Работа",
			description: "Документы, инструменты и внутренние ссылки",
			icon: "work",
			groups: [
				{
					id: "docs",
					name: "Документация",
					description: "Фреймворки, UI и справочники",
					links: [
						{
							id: "svelte",
							title: "Svelte Docs",
							url: "https://svelte.dev/docs",
							description: "Документация Svelte и SvelteKit",
							tags: ["docs", "svelte"],
						},
						{
							id: "tailwind",
							title: "Tailwind CSS",
							url: "https://tailwindcss.com",
							description: "CSS фреймворк и утилиты",
							tags: ["css", "docs"],
						},
						{
							id: "shadcn",
							title: "shadcn-svelte",
							url: "https://www.shadcn-svelte.com",
							description: "UI компоненты для Svelte",
							tags: ["ui", "svelte"],
						},
					],
				},
				{
					id: "tools",
					name: "Инструменты",
					description: "Рабочие сервисы",
					links: [
						{
							id: "github",
							title: "GitHub",
							url: "https://github.com",
							description: "Репозитории, issues и pull requests",
							tags: ["git", "code"],
						},
					],
				},
				{
					id: "internal",
					name: "Внутреннее",
					description: "Закрытые ресурсы команды",
					links: [],
				},
			],
		},
		{
			id: "project",
			name: "Проект",
			description: "Текущие проектные ссылки",
			icon: "project",
			groups: [
				{
					id: "design",
					name: "Дизайн",
					description: "Макеты и референсы",
					links: [],
				},
			],
		},
	]);

	const selectedSpace = $derived(spaces.find((space) => space.id === activeSpaceId) ?? spaces[0]);
	const selectedGroup = $derived(
		selectedSpace?.groups.find((group) => group.id === activeGroupId) ?? selectedSpace?.groups[0]
	);
	const visibleSpaces = $derived(
		query.trim() ? spaces.filter((space) => includesSpace(space, query)) : spaces
	);
	const ui = $derived(messages[language]);
	const allTags = $derived(
		Array.from(
			new Set(
				spaces.flatMap((space) =>
					space.groups.flatMap((group) => group.links.flatMap((link) => link.tags))
				)
			)
		).sort((a, b) => a.localeCompare(b))
	);

	onMount(() => {
		const storedTheme = localStorage.getItem("porthub-theme");
		const storedLanguage = localStorage.getItem("porthub-language");
		if (storedLanguage === "ru" || storedLanguage === "en") {
			language = storedLanguage;
		}
		setTheme(storedTheme === "dark" ? "dark" : "light");
	});

	function includesSpace(space: Space, value: string) {
		const needle = value.toLowerCase();
		return [space.name, space.description, ...space.groups.flatMap((group) => [
			group.name,
			group.description,
			...group.links.flatMap((link) => [link.title, link.url, link.description, ...link.tags]),
		])]
			.join(" ")
			.toLowerCase()
			.includes(needle);
	}

	function uid(prefix: string) {
		return `${prefix}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 7)}`;
	}

	function selectGroup(space: Space, group: Group) {
		activeSpaceId = space.id;
		activeGroupId = group.id;
		expandedSpaces[space.id] = true;
	}

	function openSpaceDialog(space?: Space) {
		editingSpaceId = space?.id ?? null;
		spaceName = space?.name ?? "";
		spaceDialogOpen = true;
	}

	function saveSpace(event: SubmitEvent) {
		event.preventDefault();
		if (!spaceName.trim()) return;

		const existing = spaces.find((space) => space.id === editingSpaceId);
		if (existing) {
			existing.name = spaceName.trim();
		} else {
			const space: Space = {
				id: uid("space"),
				name: spaceName.trim(),
				description: "Новое пространство",
				icon: "project",
				groups: [],
			};
			spaces.push(space);
			activeSpaceId = space.id;
			activeGroupId = "";
			expandedSpaces[space.id] = true;
		}

		spaceDialogOpen = false;
	}

	function deleteSpace(space: Space) {
		if (!window.confirm(`${ui.confirmDeleteSpace} "${space.name}"?`)) return;
		spaces = spaces.filter((item) => item.id !== space.id);
		activeSpaceId = spaces[0]?.id ?? "";
		activeGroupId = spaces[0]?.groups[0]?.id ?? "";
	}

	function openGroupDialog(group?: Group, space?: Space) {
		if (space) {
			activeSpaceId = space.id;
			expandedSpaces[space.id] = true;
		}
		editingGroupId = group?.id ?? null;
		groupName = group?.name ?? "";
		groupDialogOpen = true;
	}

	function saveGroup(event: SubmitEvent) {
		event.preventDefault();
		if (!selectedSpace || !groupName.trim()) return;

		const existing = selectedSpace.groups.find((group) => group.id === editingGroupId);
		if (existing) {
			existing.name = groupName.trim();
		} else {
			const group: Group = {
				id: uid("group"),
				name: groupName.trim(),
				description: "Новая группа ссылок",
				links: [],
			};
			selectedSpace.groups.push(group);
			activeGroupId = group.id;
			expandedSpaces[selectedSpace.id] = true;
		}

		groupDialogOpen = false;
	}

	function deleteGroup(group: Group, space = selectedSpace) {
		if (!space || !window.confirm(`${ui.confirmDeleteGroup} "${group.name}"?`)) return;
		space.groups = space.groups.filter((item) => item.id !== group.id);
		if (space.id === activeSpaceId) {
			activeGroupId = space.groups[0]?.id ?? "";
		}
	}

	function openLinkDialog(link?: BookmarkLink) {
		editingLinkId = link?.id ?? null;
		linkTitle = link?.title ?? "";
		linkUrl = link?.url ?? "https://";
		linkDescription = link?.description ?? "";
		linkTagDraft = "";
		linkTagList = [...(link?.tags ?? [])];
		linkDialogOpen = true;
	}

	function saveLink(event: SubmitEvent) {
		event.preventDefault();
		if (!selectedGroup || !linkTitle.trim() || !linkUrl.trim()) return;

		const values = {
			title: linkTitle.trim(),
			url: normalizeUrl(linkUrl.trim()),
			description: linkDescription.trim(),
			tags: normalizedTags([...linkTagList, linkTagDraft]),
		};
		const existing = selectedGroup.links.find((link) => link.id === editingLinkId);

		if (existing) {
			Object.assign(existing, values);
		} else {
			selectedGroup.links.push({ id: uid("link"), ...values });
		}

		linkDialogOpen = false;
	}

	function normalizedTags(tags: string[]) {
		return Array.from(
			new Set(
				tags
					.flatMap((tag) => tag.split(","))
					.map((tag) => tag.trim())
					.filter(Boolean)
			)
		);
	}

	function commitTag() {
		const tags = normalizedTags([linkTagDraft]);
		if (!tags.length) return;
		linkTagList = normalizedTags([...linkTagList, ...tags]);
		linkTagDraft = "";
	}

	function removeTag(tag: string) {
		linkTagList = linkTagList.filter((item) => item !== tag);
	}

	function handleTagKeydown(event: KeyboardEvent) {
		if (event.key !== "Enter" && event.key !== ",") return;
		event.preventDefault();
		commitTag();
	}

	function deleteLink(link: BookmarkLink) {
		if (!selectedGroup || !window.confirm(`${ui.confirmDeleteLink} "${link.title}"?`)) return;
		selectedGroup.links = selectedGroup.links.filter((item) => item.id !== link.id);
	}

	function normalizeUrl(value: string) {
		return /^https?:\/\//i.test(value) ? value : `https://${value}`;
	}

	function getDomain(url: string) {
		try {
			return new URL(normalizeUrl(url)).hostname.replace(/^www\./, "");
		} catch {
			return url;
		}
	}

	function favicon(url: string) {
		return `https://www.google.com/s2/favicons?domain=${getDomain(url)}&sz=64`;
	}

	function openLink(url: string) {
		window.open(normalizeUrl(url), "_blank", "noopener,noreferrer");
	}

	function exportJson() {
		const blob = new Blob([JSON.stringify({ spaces }, null, 2)], { type: "application/json" });
		const url = URL.createObjectURL(blob);
		const anchor = document.createElement("a");
		anchor.href = url;
		anchor.download = "porthub-export.json";
		anchor.click();
		URL.revokeObjectURL(url);
	}

	async function importJson(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;

		try {
			const parsed = JSON.parse(await file.text()) as { spaces?: Space[] } | Space[];
			const nextSpaces = Array.isArray(parsed) ? parsed : parsed.spaces;
			if (!Array.isArray(nextSpaces)) throw new Error("Invalid format");

			spaces = nextSpaces;
			activeSpaceId = spaces[0]?.id ?? "";
			activeGroupId = spaces[0]?.groups[0]?.id ?? "";
			expandedSpaces = Object.fromEntries(spaces.map((space) => [space.id, true]));
		} catch {
			window.alert(ui.importError);
		} finally {
			input.value = "";
		}
	}

	function setTheme(value: Theme) {
		theme = value;
		document.documentElement.classList.toggle("dark", value === "dark");
		localStorage.setItem("porthub-theme", value);
	}

	function setLanguage(value: Language) {
		language = value;
		localStorage.setItem("porthub-language", value);
	}
</script>

<Sidebar.Provider style="--sidebar-width: 19rem; --sidebar-width-mobile: 19rem;">
	<Sidebar.Root collapsible="offcanvas">
		<Sidebar.Header>
			<Sidebar.Menu>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton size="lg">
						<img src="/porthub-logo.png" alt="" class="size-8 rounded-md object-cover" />
						<span>PortHub</span>
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
			</Sidebar.Menu>
			<div class="relative px-2">
				<Search class="pointer-events-none absolute top-1/2 left-5 -translate-y-1/2 text-muted-foreground" />
				<Sidebar.Input class="pl-9" placeholder={ui.search} bind:value={query} />
			</div>
		</Sidebar.Header>

		<Sidebar.Content>
			<Sidebar.Group>
				<Sidebar.GroupLabel>{ui.spaces}</Sidebar.GroupLabel>
				<Sidebar.GroupContent>
					<div class="px-2 pb-3">
						<Button
							class="h-10 w-full justify-start shadow-sm"
							variant="secondary"
							size="sm"
							title={ui.addSpace}
							onclick={() => openSpaceDialog()}
						>
							<Plus data-icon="inline-start" />
							{ui.addSpace}
						</Button>
					</div>
					<Sidebar.Menu>
						{#each visibleSpaces as space (space.id)}
							{@const Icon = icons[space.icon]}
							<Collapsible.Root
								class="group/collapsible"
								bind:open={() => expandedSpaces[space.id] ?? false, (open) => (expandedSpaces[space.id] = open)}
							>
								<Sidebar.MenuItem>
									<Collapsible.Trigger>
										{#snippet child({ props })}
											<Sidebar.MenuButton {...props} isActive={space.id === activeSpaceId}>
												<ChevronRight class="transition-transform group-data-[state=open]/collapsible:rotate-90" />
												<Icon />
												<span>{space.name}</span>
											</Sidebar.MenuButton>
										{/snippet}
									</Collapsible.Trigger>

									<Collapsible.Content>
										<Sidebar.MenuSub>
											{#each space.groups as group (group.id)}
												<Sidebar.MenuSubItem class="w-full">
													<div class="group/group grid w-full grid-cols-[minmax(0,1fr)_auto] items-center gap-1 rounded-md hover:bg-sidebar-accent/60">
														<Sidebar.MenuSubButton
															href="#"
															class="min-w-0"
															isActive={space.id === activeSpaceId && group.id === activeGroupId}
															onclick={(event) => {
																event.preventDefault();
																selectGroup(space, group);
															}}
														>
															<Link2 />
															<span>{group.name}</span>
														</Sidebar.MenuSubButton>
														<div class="flex shrink-0 items-center gap-1 pr-1">
															<Button
																variant="ghost"
																size="icon-xs"
																class="opacity-75 group-hover/group:opacity-100"
																title={ui.editGroup}
																aria-label={ui.editGroup}
																onclick={() => openGroupDialog(group, space)}
															>
																<Edit3 />
															</Button>
															<Button
																variant="ghost"
																size="icon-xs"
																class="opacity-70 group-hover/group:opacity-100"
																title={ui.deleteGroup}
																aria-label={ui.deleteGroup}
																onclick={() => deleteGroup(group, space)}
															>
																<Trash2 />
															</Button>
														</div>
													</div>
												</Sidebar.MenuSubItem>
											{:else}
												<Sidebar.MenuSubItem>
													<div class="flex justify-end py-1 pr-1">
														<Button
															variant="ghost"
															size="xs"
															title={ui.addGroupTitle}
															onclick={() => openGroupDialog(undefined, space)}
														>
															<Plus data-icon="inline-start" />
															{ui.addGroup}
														</Button>
													</div>
												</Sidebar.MenuSubItem>
											{/each}
										</Sidebar.MenuSub>
									</Collapsible.Content>
								</Sidebar.MenuItem>
							</Collapsible.Root>
						{/each}
					</Sidebar.Menu>
				</Sidebar.GroupContent>
			</Sidebar.Group>
		</Sidebar.Content>

		<Sidebar.Footer>
			<Sidebar.Menu>
				<Sidebar.MenuItem>
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							{#snippet child({ props })}
								<Sidebar.MenuButton size="lg" {...props}>
									<Avatar.Root class="size-8 rounded-md">
										<Avatar.Fallback>PH</Avatar.Fallback>
									</Avatar.Root>
									<span>{ui.user}</span>
									<ChevronsUpDown class="ml-auto" />
								</Sidebar.MenuButton>
							{/snippet}
						</DropdownMenu.Trigger>
						<DropdownMenu.Content side="top" align="start" class="w-(--bits-dropdown-menu-anchor-width)">
							<DropdownMenu.Label>{ui.user}</DropdownMenu.Label>
							<DropdownMenu.Group>
								<DropdownMenu.Item onclick={exportJson}>
									<Download />
									<span>{ui.export}</span>
								</DropdownMenu.Item>
								<DropdownMenu.Item onclick={() => importInput?.click()}>
									<Upload />
									<span>{ui.import}</span>
								</DropdownMenu.Item>
							</DropdownMenu.Group>
							<DropdownMenu.Separator />
							<DropdownMenu.Sub>
								<DropdownMenu.SubTrigger>
									<Settings />
									<span>{ui.settings}</span>
								</DropdownMenu.SubTrigger>
								<DropdownMenu.SubContent>
									<DropdownMenu.Label>{ui.theme}</DropdownMenu.Label>
									<DropdownMenu.Item onclick={() => setTheme("light")}>
										<Sun />
										<span>{ui.light}</span>
										{#if theme === "light"}
											<CheckCircle2 class="ml-auto" />
										{/if}
									</DropdownMenu.Item>
									<DropdownMenu.Item onclick={() => setTheme("dark")}>
										<Moon />
										<span>{ui.dark}</span>
										{#if theme === "dark"}
											<CheckCircle2 class="ml-auto" />
										{/if}
									</DropdownMenu.Item>
									<DropdownMenu.Separator />
									<DropdownMenu.Label>{ui.language}</DropdownMenu.Label>
									<DropdownMenu.Item onclick={() => setLanguage("ru")}>
										<Languages />
										<span>{ui.russian}</span>
										{#if language === "ru"}
											<CheckCircle2 class="ml-auto" />
										{/if}
									</DropdownMenu.Item>
									<DropdownMenu.Item onclick={() => setLanguage("en")}>
										<Languages />
										<span>{ui.english}</span>
										{#if language === "en"}
											<CheckCircle2 class="ml-auto" />
										{/if}
									</DropdownMenu.Item>
								</DropdownMenu.SubContent>
							</DropdownMenu.Sub>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</Sidebar.MenuItem>
			</Sidebar.Menu>
		</Sidebar.Footer>
		<Sidebar.Rail />
	</Sidebar.Root>

	<Sidebar.Inset>
		<input
			bind:this={importInput}
			type="file"
			accept="application/json"
			class="hidden"
			onchange={importJson}
		/>

		<main class="flex min-h-screen min-w-0 flex-col bg-background">
			<header class="flex flex-col gap-4 border-border border-b p-5 lg:flex-row lg:items-center lg:justify-between">
				<div class="flex min-w-0 items-center gap-3">
					<Sidebar.Trigger />
					<div class="min-w-0">
						<p class="truncate text-sm text-muted-foreground">{selectedSpace?.name ?? ui.spaces}</p>
						<h1 class="truncate text-2xl font-semibold tracking-normal">
							{selectedGroup?.name ?? ui.selectGroup}
						</h1>
						<p class="mt-1 text-sm text-muted-foreground">{selectedGroup?.links.length ?? 0} {ui.links}</p>
					</div>
				</div>
				<div class="flex flex-wrap gap-2">
					<Button variant="outline" onclick={() => selectedGroup?.links.forEach((link) => openLink(link.url))}>
						<ExternalLink data-icon="inline-start" />
						{ui.openAll}
					</Button>
					<Button variant="outline" onclick={() => openGroupDialog()} disabled={!selectedSpace}>
						<Plus data-icon="inline-start" />
						{ui.addGroup}
					</Button>
					<Button onclick={() => openLinkDialog()} disabled={!selectedGroup}>
						<Plus data-icon="inline-start" />
						{ui.addLink}
					</Button>
				</div>
			</header>

			<div class="min-h-0 flex-1 overflow-auto p-5">
				{#if selectedGroup?.links.length}
					<div class="grid gap-3 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4">
						{#each selectedGroup.links as link (link.id)}
							<Card.Root size="sm" class="min-h-52">
								<Card.Header>
									<Item.Root size="sm" class="border-transparent px-0 py-0">
										<Item.Media variant="image">
											<img src={favicon(link.url)} alt="" />
										</Item.Media>
										<Item.Content>
											<Item.Title>{link.title}</Item.Title>
											<Item.Description>{getDomain(link.url)}</Item.Description>
										</Item.Content>
									</Item.Root>
								</Card.Header>
								<Card.Content class="flex flex-1 flex-col gap-4">
									<p class="line-clamp-2 text-sm text-muted-foreground">{link.description}</p>
									<div class="flex flex-wrap gap-1.5">
										{#each link.tags as tag (tag)}
											<Badge variant="secondary">{tag}</Badge>
										{/each}
									</div>
								</Card.Content>
								<Card.Footer class="mt-auto flex-col items-stretch gap-3 px-4">
									<Separator />
									<div class="flex items-center justify-between gap-2">
										<div class="flex items-center gap-1 text-sm text-muted-foreground">
											<CheckCircle2 class="text-primary" />
											{ui.works}
										</div>
										<div class="flex gap-1">
											<Button
												variant="ghost"
												size="icon-xs"
												aria-label={ui.editLink}
												onclick={() => openLinkDialog(link)}
											>
												<Edit3 />
											</Button>
											<Button variant="ghost" size="icon-xs" aria-label={ui.addLink} onclick={() => openLink(link.url)}>
												<ExternalLink />
											</Button>
											<Button variant="ghost" size="icon-xs" aria-label={ui.deleteLink} onclick={() => deleteLink(link)}>
												<Trash2 />
											</Button>
										</div>
									</div>
								</Card.Footer>
							</Card.Root>
						{/each}
					</div>
				{:else}
					<div class="flex min-h-80 flex-col items-center justify-center gap-3 rounded-md border border-dashed">
						<FileJson class="text-muted-foreground" />
						<p class="text-sm text-muted-foreground">
							{selectedGroup ? ui.noLinks : ui.selectGroup}
						</p>
						<Button onclick={() => openLinkDialog()} disabled={!selectedGroup}>
							<Plus data-icon="inline-start" />
							{ui.addLinkTitle}
						</Button>
					</div>
				{/if}
			</div>
		</main>
	</Sidebar.Inset>
</Sidebar.Provider>

<Dialog.Root bind:open={spaceDialogOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>{editingSpaceId ? ui.editSpace : ui.addSpaceTitle}</Dialog.Title>
			<Dialog.Description>{ui.spaceDialogDescription}</Dialog.Description>
		</Dialog.Header>
		<form class="flex flex-col gap-5" onsubmit={saveSpace}>
			<Field.Group>
				<Field.Field>
					<Field.Label for="space-name">{ui.name}</Field.Label>
					<Input id="space-name" bind:value={spaceName} placeholder={ui.placeholderSpace} />
				</Field.Field>
			</Field.Group>
			<Dialog.Footer>
				<Button type="submit">{editingSpaceId ? ui.save : ui.add}</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={groupDialogOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>{editingGroupId ? ui.editGroup : ui.addGroupTitle}</Dialog.Title>
			<Dialog.Description>{ui.groupDialogDescription}</Dialog.Description>
		</Dialog.Header>
		<form class="flex flex-col gap-5" onsubmit={saveGroup}>
			<Field.Group>
				<Field.Field>
					<Field.Label for="group-name">{ui.name}</Field.Label>
					<Input id="group-name" bind:value={groupName} placeholder={ui.placeholderGroup} />
				</Field.Field>
			</Field.Group>
			<Dialog.Footer>
				<Button type="submit">{editingGroupId ? ui.save : ui.add}</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={linkDialogOpen}>
	<Dialog.Content class="sm:max-w-lg">
		<Dialog.Header>
			<Dialog.Title>{editingLinkId ? ui.editLink : ui.addLinkTitle}</Dialog.Title>
			<Dialog.Description>{ui.linkDialogDescription}</Dialog.Description>
		</Dialog.Header>
		<form class="flex flex-col gap-5" onsubmit={saveLink}>
			<Field.Group>
				<Field.Field>
					<Field.Label for="link-url">{ui.url}</Field.Label>
					<Input id="link-url" bind:value={linkUrl} placeholder={ui.linkPlaceholder} />
				</Field.Field>
				<Field.Field>
					<Field.Label for="link-title">{ui.title}</Field.Label>
					<Input id="link-title" bind:value={linkTitle} placeholder={ui.placeholderGroup} />
				</Field.Field>
				<Field.Field>
					<Field.Label for="link-description">{ui.description}</Field.Label>
					<Textarea id="link-description" bind:value={linkDescription} placeholder={ui.placeholderDescription} />
				</Field.Field>
				<Field.Field>
					<Field.Label for="link-tags">{ui.tags}</Field.Label>
					{#if linkTagList.length}
						<div class="flex flex-wrap gap-2">
							{#each linkTagList as tag (tag)}
								<Badge variant="secondary" class="gap-1">
									{tag}
									<button
										type="button"
										class="rounded-sm text-muted-foreground hover:text-foreground"
										aria-label={`${ui.deleteLink}: ${tag}`}
										onclick={() => removeTag(tag)}
									>
										<X />
									</button>
								</Badge>
							{/each}
						</div>
					{/if}
					<Input
						id="link-tags"
						list="tag-suggestions"
						bind:value={linkTagDraft}
						placeholder={ui.placeholderTags}
						onkeydown={handleTagKeydown}
						onblur={commitTag}
					/>
					<datalist id="tag-suggestions">
						{#each allTags.filter((tag) => !linkTagList.includes(tag)) as tag (tag)}
							<option value={tag}></option>
						{/each}
					</datalist>
				</Field.Field>
			</Field.Group>
			<Dialog.Footer>
				<Button type="submit">{editingLinkId ? ui.save : ui.add}</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
