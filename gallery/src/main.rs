use dots_design::prelude::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn Section(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <section class="d-card" style="margin-bottom:var(--space-16)">
            <CardHead title=title/>
            <CardBody><div class="d-stack" style="gap:var(--space-12)">{children()}</div></CardBody>
        </section>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <DotsStyles/>
        <DotsProvider>
            <Gallery/>
        </DotsProvider>
    }
}

#[component]
fn Gallery() -> impl IntoView {
    let theme = use_theme();
    let toasts = use_toasts();
    let company = RwSignal::new("acme".to_string());
    let palette_open = RwSignal::new(false);
    let dialog_open = RwSignal::new(false);
    let sheet_open = RwSignal::new(false);
    let last_run = RwSignal::new(String::from("nothing yet"));
    let theme_name = RwSignal::new("dark".to_string());
    Effect::new(move |_| {
        let t = Theme::parse(&theme_name.get()).unwrap_or_default();
        if theme.get_untracked() != t {
            theme.set(t);
        }
    });

    let items = Signal::derive(|| {
        vec![
            PaletteItem::new("tokens", "go to tokens").group("navigate").hint(["g", "t"]),
            PaletteItem::new("components", "go to components").group("navigate").hint(["g", "c"]),
            PaletteItem::new("light", "switch to light theme").group("theme").keywords("appearance"),
            PaletteItem::new("rm", "delete workspace").group("danger").disabled(),
        ]
    });

    view! {
        <SkipLink/>
        <Shell persist="dots:gallery" height="100dvh">
            <Sidebar>
                <SidebarHead>
                    <SidebarSwitcher
                        label="companies"
                        value=company
                        items=vec![
                            SwitcherItem::new("acme", "acme inc", "ac", "enterprise"),
                            SwitcherItem::new("globex", "globex", "gl", "team"),
                        ]
                    >
                        <MenuItem>"add company"</MenuItem>
                    </SidebarSwitcher>
                </SidebarHead>
                <SidebarFilter/>
                <SidebarContent>
                    <SidebarGroup label="platform">
                        <SidebarItem href="#overview" icon="layout-dashboard" label="overview" current=true/>
                        <SidebarParent icon="rocket" label="deployments" open=true>
                            <SidebarSubItem href="#production">"production"</SidebarSubItem>
                            <SidebarSubItem href="#staging">"staging"</SidebarSubItem>
                        </SidebarParent>
                        <SidebarItem href="#servers" icon="server" label="servers" count="4"/>
                        <SidebarItem icon="scroll-text" label="audit log"/>
                        <SidebarItem
                            href="#dots-ui" icon="folder" label="dots-ui"
                            action=move || view! {
                                <SidebarAction row="dots-ui">
                                    <MenuItem>"rename"</MenuItem>
                                    <MenuSeparator/>
                                    <MenuItem danger=true>"delete project"</MenuItem>
                                </SidebarAction>
                            }
                        />
                    </SidebarGroup>
                </SidebarContent>
                <SidebarFoot>
                    <SidebarAccount label="account" initials="st" name="samin t" meta="samin@acme.example">
                        <MenuRadioGroup label="theme" value=theme_name>
                            <MenuRadio value="dark">"dark"</MenuRadio>
                            <MenuRadio value="light">"light"</MenuRadio>
                        </MenuRadioGroup>
                        <MenuSeparator/>
                        <MenuItem>"sign out"</MenuItem>
                    </SidebarAccount>
                </SidebarFoot>
            </Sidebar>

            <ShellMain>
                <AppBar>
                    <SidebarToggle/>
                    <AppBarSep/>
                    <Breadcrumbs>
                        <Crumb href="#">"dots"</Crumb>
                        <CrumbCurrent>"gallery"</CrumbCurrent>
                    </Breadcrumbs>
                    <AppBarSpacer/>
                    <button type="button" class="d-appbar__search" aria-keyshortcuts="Control+K" on:click=move |_| palette_open.set(true)>
                        <Icon name="search" small=true/><span>"search"</span><Keys keys=vec!["ctrl".to_string(), "k".to_string()]/>
                    </button>
                    <AppBarMenu label="status"/>
                    <AppBarCollapse>
                        <AppBarCluster label="system status">
                            <StatusChip on=true severity=Severity::Success>"all systems up"</StatusChip>
                            <AppBarClock/>
                        </AppBarCluster>
                    </AppBarCollapse>
                    <AppBarActions>
                        <Button variant=ButtonVariant::Ghost size=Size::Sm label="notifications, 3 unread">
                            <Icon name="bell"/><Badge count=true label="3">"3"</Badge>
                        </Button>
                        <Menu
                            label="account"
                            trigger=|| view! { <AvatarButton initials="st" label="account: samin t"/> }
                        >
                            <MenuHead>
                                <Avatar initials="st" decorative=true/>
                                <span class="d-menu__who"><b>"samin t"</b><small>"samin@acme.example"</small></span>
                            </MenuHead>
                            <MenuItem>"profile"</MenuItem>
                            <MenuSeparator/>
                            <MenuItem danger=true>"sign out"</MenuItem>
                        </Menu>
                    </AppBarActions>
                    <ShellToggle region=Region::Right label="toggle details" icon="panel-right"/>
                </AppBar>

                <Main>
                    <div class="d-container" style="padding:var(--space-16)">
                        <ComponentTour last_run palette_open dialog_open sheet_open toasts/>
                    </div>
                </Main>

                <BottomBar
                    label="logs"
                    line=|| view! {
                        <StatusChip on=true severity=Severity::Success>"connected"</StatusChip>
                        <BottomSpacer/>
                        <ShellToggle region=Region::Bottom label="toggle logs">"logs"</ShellToggle>
                    }
                >
                    <CodeBlock title="build.log" cursor=true code="$ cargo build\n   Compiling dots-design\n    Finished dev"/>
                </BottomBar>
            </ShellMain>

            <RightBar title="details">
                <p class="d-hint">"last command run: "{move || last_run.get()}</p>
            </RightBar>
        </Shell>

        <CommandPalette items=items open=palette_open on_run=move |i: PaletteItem| last_run.set(i.id)/>
    }
}

#[component]
fn ComponentTour(
    last_run: RwSignal<String>,
    palette_open: RwSignal<bool>,
    dialog_open: RwSignal<bool>,
    sheet_open: RwSignal<bool>,
    toasts: Toasts,
) -> impl IntoView {
    let busy = RwSignal::new(false);
    let pressed = RwSignal::new(true);
    let name = RwSignal::new(String::new());
    let host_error = Signal::derive(move || name.get().contains(' ').then(|| "no spaces in a hostname".to_string()));
    let checked = RwSignal::new(true);
    let autosave = RwSignal::new(true);
    let proto = RwSignal::new("tcp".to_string());
    let theme = RwSignal::new("dark".to_string());
    let vol = RwSignal::new(40.0);
    let tab = RwSignal::new("overview".to_string());
    let branch = RwSignal::new("main".to_string());
    let active_pane = RwSignal::new("main".to_string());
    let ws = RwSignal::new(0usize);
    let sel = RwSignal::new(Some(1usize));
    let sort = RwSignal::new(SortDirection::Ascending);
    let flag = RwSignal::new(true);
    let mut hosts = vec![("build-02", 7), ("build-01", 12), ("edge-1", 3)];
    hosts.sort_by_key(|h| h.0);
    let rows = RwSignal::new(hosts);
    let sorted = Memo::new(move |_| {
        let mut r = rows.get();
        match sort.get() {
            SortDirection::Descending => r.sort_by(|a, b| b.0.cmp(a.0)),
            _ => r.sort_by(|a, b| a.0.cmp(b.0)),
        }
        r
    });
    let progress = RwSignal::new(42.0);

    view! {
        <Section title="button">
            <div class="d-row" style="gap:var(--space-8)">
                <Button variant=ButtonVariant::Primary>"deploy"</Button>
                <Button>"cancel"</Button>
                <Button variant=ButtonVariant::Ghost>"details"</Button>
                <Button variant=ButtonVariant::Danger>"delete workspace"</Button>
                <Button variant=ButtonVariant::Primary busy=busy on:click=move |_| busy.update(|b| *b = !*b)>"deploying"</Button>
                <Button icon=true label="settings" tooltip="settings"><Icon name="settings"/></Button>
                <Button pressed=pressed on:click=move |_| pressed.update(|p| *p = !*p)>"watch"</Button>
                <Button disabled=true>"disabled"</Button>
            </div>
        </Section>
        <Section title="badge, status, kbd, avatar">
            <div class="d-row" style="gap:var(--space-8)">
                <Badge>"v1.0.0"</Badge>
                <Badge tone=Severity::Danger icon=true>"error"</Badge>
                <Badge tone=Severity::Success>"saved"</Badge>
                <Tag text="rust" on_remove=move |()| last_run.set("removed rust".into())/>
                <StatusChip on=true severity=Severity::Success>"connected"</StatusChip>
                <StatusChip on=false severity=Severity::Warning>"disconnected"</StatusChip>
                <StatusChip on=true severity=Severity::Success pulse=true>"charging"</StatusChip>
                <Keys keys=vec!["ctrl".to_string(), "k".to_string()]/>
                <Avatar initials="st" name="samin t"/>
                <Avatar initials="ab" name="ab" size=Size::Lg/>
            </div>
        </Section>
        <Section title="form">
            <TextField label="hostname" optional=true value=name hint="lowercase, no spaces" error=host_error/>
            <Checkbox checked=checked>"watch for changes"</Checkbox>
            <Switch checked=autosave>"autosave"</Switch>
            <RadioGroup label="protocol" value=proto>
                <Radio value="tcp">"tcp"</Radio>
                <Radio value="udp">"udp"</Radio>
            </RadioGroup>
            <Select label="theme" value=theme options=vec![SelectOption::new("dark", "dark"), SelectOption::new("light", "light")]/>
            <Slider value=vol label="volume"/>
            <ProgressRow label="uploading" value=progress/>
            <Progress label="indexing"/>
        </Section>
        <Section title="tabs, accordion, list">
            <Tabs value=tab>
                <TabList label="deployment">
                    <Tab value="overview">"overview"</Tab>
                    <Tab value="logs">"logs"</Tab>
                    <Tab value="settings">"settings"</Tab>
                </TabList>
                <TabPanel value="overview">"overview panel"</TabPanel>
                <TabPanel value="logs">"logs panel"</TabPanel>
                <TabPanel value="settings">"settings panel"</TabPanel>
            </Tabs>
            <Accordion>
                <AccordionItem title="what is a role step?" name="faq" open=true>"neutral-0 is always the ground."</AccordionItem>
                <AccordionItem title="why one warning colour?" name="faq">"one meaning, one colour."</AccordionItem>
            </Accordion>
            <Listbox label="branches" value=branch>
                <ListOption value="main" meta="2m ago">"main"</ListOption>
                <ListOption value="feat/palette">"feat/palette"</ListOption>
                <ListOption value="wip/old" disabled=true>"wip/old"</ListOption>
            </Listbox>
        </Section>
        <Section title="table">
            <Table label="hosts" max_height="200px">
                <thead>
                    <tr>
                        <Th sort=sort on_sort=move |()| sort.update(|s| *s = s.next())>"host"</Th>
                        <Th>"status"</Th>
                        <Th num=true>"jobs"</Th>
                    </tr>
                </thead>
                <tbody>
                    <For each=move || sorted.get().into_iter().enumerate() key=|(_, h)| h.0 children=move |(i, h)| view! {
                        <Tr selectable=true selected=Signal::derive(move || sel.get() == Some(i)) on_select=move |()| sel.set(Some(i))>
                            <Td>{h.0}</Td>
                            <Td><StatusChip on=true severity=Severity::Success>"up"</StatusChip></Td>
                            <Td num=true>{h.1}</Td>
                        </Tr>
                    }/>
                </tbody>
            </Table>
        </Section>
        <Section title="split pane, workspace dots, empty states">
            <SplitPane active=active_pane>
                <Pane id="main" title="main" context=PaneContext::Primary>"main pane"</Pane>
                <Pane id="aux" title="aux" context=PaneContext::Secondary>"aux pane"</Pane>
            </SplitPane>
            <WorkspaceDots
                label="workspaces" current=ws
                items=vec![
                    DotItem::new("workspace 1", DotState::Occupied),
                    DotItem::new("workspace 2", DotState::Occupied),
                    DotItem::new("workspace 3", DotState::Urgent),
                    DotItem::new("workspace 4", DotState::Empty),
                ]
            />
            <WorkspaceSteps label="setup steps" current=ws steps=vec!["account".to_string(), "keys".to_string()]/>
            <EmptyState title="no panes yet" hint="split the workspace to start."><Button>"new pane"</Button></EmptyState>
            <EmptyError title="could not load panes" hint="config is not valid toml (line 12)."><Button>"retry"</Button></EmptyError>
        </Section>
        <Section title="overlays">
            <div class="d-row" style="gap:var(--space-8)">
                <Button on:click=move |_| palette_open.set(true)>"open palette"</Button>
                <Button variant=ButtonVariant::Danger on:click=move |_| dialog_open.set(true)>"delete workspace"</Button>
                <Button on:click=move |_| sheet_open.set(true)>"settings"</Button>
                <Button on:click=move |_| { toasts.push(Toast::warning("disconnected").body("lost link to build-02. retrying in 5s.").action("retry now", || ())); }>"toast"</Button>
                <Button on:click=move |_| { toasts.push(Toast::success("saved")); }>"saved toast"</Button>
                <Menu label="pane" trigger=|| view! { <Button>"pane"</Button> }>
                    <MenuLabel>"pane"</MenuLabel>
                    <MenuItem meta="ctrl \\">"split right"</MenuItem>
                    <MenuSeparator/>
                    <MenuCheckbox checked=flag>"show line numbers"</MenuCheckbox>
                    <MenuItem disabled=true>"move to workspace"</MenuItem>
                    <MenuItem danger=true>"close pane"</MenuItem>
                </Menu>
                <ContextMenu label="pane actions" target=|| view! { <span class="d-badge">"right click me"</span> }>
                    <MenuItem>"copy"</MenuItem>
                </ContextMenu>
            </div>
            <HeroClock blink=true label="local time"/>
            <CodeBlock title="button.rs" numbered=true code="use leptos::prelude::*;\n// one token span never crosses a line break"/>
        </Section>
        <Dialog
            open=dialog_open title="delete workspace?" dismiss=false
            footer=move || view! {
                <DialogClose value="cancel">"cancel"</DialogClose>
                <DialogClose value="delete" variant=ButtonVariant::Danger>"delete workspace"</DialogClose>
            }
        >
            "this removes dots-staging and its 3 panes. it cannot be undone."
        </Dialog>
        <Sheet open=sheet_open title="settings">
            <Switch checked=flag>"autosave"</Switch>
        </Sheet>
    }
}
