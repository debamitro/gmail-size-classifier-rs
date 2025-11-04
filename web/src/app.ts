// Gmail Size Classifier - TypeScript Web App UI (DOM Generated)

interface MessageItem {
    thread_id: string;
    title: string;
    size: number;
}

interface ProfileData {
    email: string;
}

interface CategorizedMessages {
    small: MessageItem[];
    medium: MessageItem[];
    large: MessageItem[];
}

class App {
    private profileData: ProfileData | null = null;
    private messages: MessageItem[] = [];
    private categorizedMessages: CategorizedMessages = {
        small: [],
        medium: [],
        large: []
    };
    private currentTab: string = 'small';

    private rootContainer: RootContainer;
    private headerComponent: HeaderComponent;
    private searchSection: SearchSectionComponent;
    private chartSection: ChartSectionComponent;
    private tabsSection: TabsSectionComponent;

    constructor() {
        this.headerComponent = new HeaderComponent();
        this.searchSection = new SearchSectionComponent();
        this.chartSection = new ChartSectionComponent();
        this.tabsSection = new TabsSectionComponent();
        this.rootContainer = new RootContainer();

        this.initializeApp();
    }

    private async initializeApp(): Promise<void> {
        await this.loadProfile();
        this.renderApp();
        this.setupEventListeners();
    }

    private async loadProfile(): Promise<void> {
        try {
            const response = await fetch('/api/profile');
            this.profileData = await response.json();
            if (this.profileData) {
                this.headerComponent.updateProfile(this.profileData);
            }
        } catch (error) {
            console.error('Profile fetch error:', error);
        }
    }

    private renderApp(): void {
        // Clear body and add CSS link
        document.body.innerHTML = '';
        document.body.className = 'font-Inter -apple-system BlinkMacSystemFont SegoeUI Roboto sans-serif max-w-5xl mx-auto px-5 py-5 bg-gradient-to-br from-purple-400 to-blue-600 min-h-screen';

        // Add Chart.js script
        const chartScript = document.createElement('script');
        chartScript.src = 'https://cdn.jsdelivr.net/npm/chart.js';
        document.head.appendChild(chartScript);

        // Add Tailwind CSS
        const tailwindLink = document.createElement('link');
        tailwindLink.href = 'https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css';
        tailwindLink.rel = 'stylesheet';
        document.head.appendChild(tailwindLink);

        // Add custom styles
        const style = document.createElement('style');
        style.textContent = `
            .message-card { background: white; border: 1px solid #e2e8f0; border-radius: 8px; padding: 1rem; margin-bottom: 0.75rem; transition: all 0.2s; box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1); }
            .message-card:hover { box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1); transform: translateY(-1px); }
            .size-badge { background: #3b82f6; color: white; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 600; }
            .loading { display: none; color: #64748b; }
            .btn-primary { background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); color: white; border: none; padding: 0.75rem 1.5rem; border-radius: 8px; font-weight: 600; cursor: pointer; transition: all 0.2s; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1); }
            .btn-primary:hover { transform: translateY(-1px); box-shadow: 0 6px 8px -1px rgba(0, 0, 0, 0.15); }
            .btn-primary:disabled { opacity: 0.6; cursor: not-allowed; transform: none; }
            .input-field { border: 2px solid #e2e8f0; border-radius: 8px; padding: 0.75rem; font-size: 1rem; transition: border-color 0.2s; }
            .input-field:focus { outline: none; border-color: #3b82f6; box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1); }
            .tab-button { padding: 1rem 1.5rem; cursor: pointer; transition: all 0.2s; border-bottom: 3px solid transparent; font-weight: 500; }
            .tab-button:hover { background: #e2e8f0; }
            .tab-button.active { background: white; border-bottom-color: #3b82f6; color: #3b82f6; }
            .tab-header { background: #f1f5f9; border-bottom: 2px solid #e2e8f0; }
        `;
        document.head.appendChild(style);

        // Render main container
        const mainContainer = this.rootContainer.render();
        document.body.appendChild(mainContainer);

        // Render components
        const headerElement = this.headerComponent.render();
        const searchElement = this.searchSection.render();
        const chartElement = this.chartSection.render();
        const tabsElement = this.tabsSection.render();

        mainContainer.appendChild(headerElement);
        mainContainer.appendChild(searchElement);
        mainContainer.appendChild(chartElement);
        mainContainer.appendChild(tabsElement);
    }

    private setupEventListeners(): void {
        this.searchSection.onSearch = this.performSearch.bind(this);
        this.tabsSection.onTabChange = this.switchTab.bind(this);
    }

    private async performSearch(maxMessages: number): Promise<void> {
        this.searchSection.setLoading(true);

        try {
            // Clear previous results
            this.messages = [];
            this.categorizedMessages = { small: [], medium: [], large: [] };
            this.tabsSection.clearAllResults();

            const response = await fetch(`/api/summary?max=${encodeURIComponent(maxMessages.toString())}`);
            this.messages = await response.json();

            // Categorize messages
            this.categorizedMessages = {
                small: this.messages.filter(item => item.size < 100 * 1024),
                medium: this.messages.filter(item => item.size >= 100 * 1024 && item.size < 1024 * 1024),
                large: this.messages.filter(item => item.size >= 1024 * 1024)
            };

            // Update UI components
            this.tabsSection.updateResults(this.categorizedMessages);
            this.chartSection.updateChart(this.categorizedMessages);
            this.tabsSection.updateStats(this.categorizedMessages);

            // Switch to small tab by default
            this.switchTab('small');

        } catch (error) {
            console.error('Error:', error);
            this.tabsSection.showError('Error fetching results');
        } finally {
            this.searchSection.setLoading(false);
        }
    }

    private switchTab(category: string): void {
        this.currentTab = category;
        this.tabsSection.switchToTab(category);
    }
}

// Utility functions
function formatSize(sizeInB: number): string {
    const sizeInKB = sizeInB / 1024;
    if (sizeInKB >= 1024) {
        return `${(sizeInKB / 1024).toFixed(2)} MB`;
    }
    return `${sizeInKB.toFixed(2)} KB`;
}

function calculateTotalSize(messages: MessageItem[]): number {
    return messages.reduce((sum, item) => sum + item.size, 0);
}

// Component Classes

class RootContainer {
    public render(): HTMLDivElement {
        const container = document.createElement('div');
        container.className = 'main-container bg-white rounded-2xl shadow-2xl overflow-hidden';
        return container;
    }
}

class HeaderComponent {
    private profileText: HTMLDivElement;

    constructor() {
        this.profileText = document.createElement('div');
        this.profileText.className = 'absolute top-4 right-4 text-white/80 text-sm';
    }

    public render(): HTMLDivElement {
        const header = document.createElement('div');
        header.className = 'header-section bg-gradient-to-br from-purple-400 to-blue-600 text-white p-8 text-center relative';

        const title = document.createElement('h1');
        title.className = 'text-4xl font-bold mb-2';
        title.textContent = 'Gmail Cleaner';

        const subtitle = document.createElement('p');
        subtitle.className = 'text-white/80 text-lg';
        subtitle.textContent = 'Organize your inbox by email size';

        const version = document.createElement('p');
        version.className = 'text-white/60 text-sm mt-1';
        version.textContent = 'version 0.1-beta';

        header.appendChild(this.profileText);
        header.appendChild(title);
        header.appendChild(subtitle);
        header.appendChild(version);

        return header;
    }

    public updateProfile(profileData: ProfileData): void {
        this.profileText.innerHTML = `<a href="https://gmail.com" class="hover:underline">${profileData.email}</a>`;
    }
}

class SearchSectionComponent {
    private input: HTMLInputElement;
    private button: HTMLButtonElement;
    private loadingElement: HTMLSpanElement;
    public onSearch?: (maxMessages: number) => void;

    constructor() {
        this.input = document.createElement('input');
        this.button = document.createElement('button');
        this.loadingElement = document.createElement('span');
        this.setupElements();
        this.setupEventListeners();
    }

    private setupElements(): void {
        this.input.type = 'number';
        this.input.placeholder = 'Enter number (max 50)';
        this.input.min = '1';
        this.input.max = '50';
        this.input.value = '20';
        this.input.className = 'input-field flex-1';

        this.button.textContent = 'Classify Emails';
        this.button.className = 'btn-primary';

        this.loadingElement.className = 'loading inline-flex items-center gap-2';
        this.loadingElement.innerHTML = `
            <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"></path>
            </svg>
            Analyzing emails...
        `;
    }

    private setupEventListeners(): void {
        this.button.addEventListener('click', () => {
            const maxMessages = parseInt(this.input.value) || 20;
            const clampedValue = Math.min(Math.max(maxMessages, 1), 50);
            this.input.value = clampedValue.toString();
            this.onSearch?.(clampedValue);
        });
    }

    public render(): HTMLDivElement {
        const section = document.createElement('div');
        section.className = 'search-container bg-gray-50 p-8 border-b border-gray-200';

        const container = document.createElement('div');
        container.className = 'max-w-md mx-auto';

        const label = document.createElement('label');
        label.htmlFor = 'maxMessages';
        label.className = 'block text-sm font-medium text-gray-700 mb-2';
        label.textContent = 'Maximum Messages to Analyze';

        const inputContainer = document.createElement('div');
        inputContainer.className = 'flex gap-3';

        const loadingContainer = document.createElement('div');
        loadingContainer.className = 'mt-3 text-center';

        inputContainer.appendChild(this.input);
        inputContainer.appendChild(this.button);
        loadingContainer.appendChild(this.loadingElement);

        container.appendChild(label);
        container.appendChild(inputContainer);
        container.appendChild(loadingContainer);
        section.appendChild(container);

        return section;
    }

    public setLoading(loading: boolean): void {
        this.loadingElement.style.display = loading ? 'inline-flex' : 'none';
        this.button.disabled = loading;
        this.input.disabled = loading;
    }
}

class ChartSectionComponent {
    private chart: any = null;
    private canvas: HTMLCanvasElement;

    constructor() {
        this.canvas = document.createElement('canvas');
        this.canvas.height = 300;
    }

    public render(): HTMLDivElement {
        const section = document.createElement('div');
        section.className = 'grid grid-cols-1 md:grid-cols-2 gap-6 mb-8';

        // Chart container
        const chartContainer = document.createElement('div');
        chartContainer.className = 'bg-white rounded-lg shadow-lg p-6';
        chartContainer.appendChild(this.canvas);

        // Classifications container
        const classificationsContainer = document.createElement('div');
        classificationsContainer.className = 'bg-white rounded-lg shadow-lg p-6';

        const title = document.createElement('h2');
        title.className = 'text-xl font-semibold mb-4';
        title.textContent = 'Size Classifications';

        const list = document.createElement('ul');
        list.className = 'list-disc pl-6';

        const items = [
            { label: 'Small:', desc: 'Less than 100KB' },
            { label: 'Medium:', desc: 'Between 100KB and 1MB' },
            { label: 'Large:', desc: '1MB or larger' }
        ];

        items.forEach(item => {
            const li = document.createElement('li');
            li.innerHTML = `<span class="font-medium">${item.label}</span> ${item.desc}`;
            list.appendChild(li);
        });

        classificationsContainer.appendChild(title);
        classificationsContainer.appendChild(list);

        section.appendChild(chartContainer);
        section.appendChild(classificationsContainer);

        return section;
    }

    public updateChart(categorizedMessages: CategorizedMessages): void {
        const ctx = this.canvas.getContext('2d');
        if (!ctx) return;

        // Destroy existing chart if it exists
        if (this.chart) {
            this.chart.destroy();
        }

        const data = [
            calculateTotalSize(categorizedMessages.small) / 1024,
            calculateTotalSize(categorizedMessages.medium) / 1024,
            calculateTotalSize(categorizedMessages.large) / 1024
        ];

        this.chart = new (window as any).Chart(ctx, {
            type: 'pie',
            data: {
                labels: ['Small (<100KB)', 'Medium (100KB-1MB)', 'Large (>1MB)'],
                datasets: [{
                    data: data,
                    backgroundColor: ['#4CAF50', '#FFC107', '#F44336']
                }]
            },
            options: {
                responsive: true,
                plugins: {
                    legend: {
                        position: 'bottom'
                    }
                }
            }
        });
    }
}

class TabsSectionComponent {
    private tabContents: { [key: string]: HTMLDivElement } = {};
    private tabHeaders: { [key: string]: HTMLDivElement } = {};
    private resultContainers: { [key: string]: HTMLDivElement } = {};
    public onTabChange?: (category: string) => void;

    constructor() {
        this.setupEventListeners();
    }

    private setupEventListeners(): void {
        // Event listeners will be set up after rendering
    }

    public render(): HTMLDivElement {
        const section = document.createElement('div');
        section.className = 'flex flex-col';

        // Tab headers
        const tabHeader = document.createElement('div');
        tabHeader.className = 'tab-header flex';

        const categories = ['small', 'medium', 'large'];
        categories.forEach(category => {
            const header = document.createElement('div');
            header.className = 'tab-button flex-1 text-center';
            header.id = `${category}-heading`;
            header.textContent = `${category.charAt(0).toUpperCase() + category.slice(1)} Emails`;
            header.addEventListener('click', () => {
                this.onTabChange?.(category);
            });
            this.tabHeaders[category] = header;
            tabHeader.appendChild(header);
        });

        // Tab contents
        const tabContentContainer = document.createElement('div');
        categories.forEach(category => {
            const tabContent = document.createElement('div');
            tabContent.className = 'bg-white rounded-lg shadow-lg p-6 hidden';
            tabContent.id = `${category}-tab`;

            const title = document.createElement('h2');
            title.className = 'text-xl font-semibold mb-4';
            title.textContent = `${category.charAt(0).toUpperCase() + category.slice(1)} Emails`;

            const stats = document.createElement('div');
            stats.className = 'mb-4 text-gray-600';

            const count = document.createElement('p');
            count.id = `${category}-count`;
            count.textContent = 'Count: 0';

            const size = document.createElement('p');
            size.id = `${category}-size`;
            size.textContent = 'Total size: 0 KB';

            const results = document.createElement('div');
            results.className = 'space-y-3';
            results.id = `${category}-results`;

            stats.appendChild(count);
            stats.appendChild(size);
            tabContent.appendChild(title);
            tabContent.appendChild(stats);
            tabContent.appendChild(results);

            this.tabContents[category] = tabContent;
            this.resultContainers[category] = results;
            tabContentContainer.appendChild(tabContent);
        });

        section.appendChild(tabHeader);
        section.appendChild(tabContentContainer);

        return section;
    }

    public switchToTab(category: string): void {
        Object.entries(this.tabContents).forEach(([cat, element]) => {
            if (cat === category) {
                element.classList.remove('hidden');
                this.tabHeaders[cat].classList.add('active');
            } else {
                element.classList.add('hidden');
                this.tabHeaders[cat].classList.remove('active');
            }
        });
    }

    public updateStats(categorizedMessages: CategorizedMessages): void {
        Object.entries(categorizedMessages).forEach(([category, messages]) => {
            const countElement = document.getElementById(`${category}-count`);
            const sizeElement = document.getElementById(`${category}-size`);
            const headingElement = this.tabHeaders[category];

            const totalSize = calculateTotalSize(messages);
            const formattedSize = formatSize(totalSize);

            if (countElement) countElement.textContent = `Count: ${messages.length}`;
            if (sizeElement) sizeElement.textContent = `Total size: ${formattedSize}`;
            if (headingElement) headingElement.textContent = `${category.charAt(0).toUpperCase() + category.slice(1)} (${formattedSize})`;
        });
    }

    public updateResults(categorizedMessages: CategorizedMessages): void {
        Object.entries(categorizedMessages).forEach(([category, messages]) => {
            const container = this.resultContainers[category];
            if (container) {
                container.innerHTML = '';
                messages.forEach((item: MessageItem) => {
                    container.appendChild(this.createMessageElement(item));
                });
            }
        });
    }

    private createMessageElement(item: MessageItem): HTMLDivElement {
        const div = document.createElement('div');
        div.className = 'message-card';

        const flexContainer = document.createElement('div');
        flexContainer.className = 'flex items-center justify-between';

        const contentDiv = document.createElement('div');
        contentDiv.className = 'flex-1 min-w-0';

        const link = document.createElement('a');
        link.className = 'text-gray-900 hover:text-blue-600 transition-colors';
        link.href = `https://mail.google.com/mail/u/0/#all/${item.thread_id}`;
        link.target = '_blank';

        const messageSubject = document.createElement('p');
        messageSubject.className = 'font-semibold truncate text-lg';
        messageSubject.textContent = item.title;

        const sizeSpan = document.createElement('span');
        sizeSpan.className = 'size-badge';
        sizeSpan.textContent = formatSize(item.size);

        link.appendChild(messageSubject);
        contentDiv.appendChild(link);
        flexContainer.appendChild(contentDiv);
        flexContainer.appendChild(sizeSpan);
        div.appendChild(flexContainer);

        return div;
    }

    public clearAllResults(): void {
        Object.values(this.resultContainers).forEach(container => {
            container.innerHTML = '';
        });
    }

    public showError(message: string): void {
        Object.values(this.resultContainers).forEach(container => {
            container.innerHTML = message;
        });
    }
}

// Initialize the app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new App();
});