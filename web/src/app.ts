// Gmail Size Classifier - TypeScript Web App UI

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

    private searchComponent: SearchComponent;
    private tabComponent: TabComponent;
    private messageListComponent: MessageListComponent;
    private chartComponent: ChartComponent;

    constructor() {
        this.searchComponent = new SearchComponent();
        this.tabComponent = new TabComponent();
        this.messageListComponent = new MessageListComponent();
        this.chartComponent = new ChartComponent();

        this.initializeApp();
    }

    private async initializeApp(): Promise<void> {
        await this.loadProfile();
        this.setupEventListeners();
    }

    private async loadProfile(): Promise<void> {
        try {
            const response = await fetch('/api/profile');
            this.profileData = await response.json();
            this.updateProfileDisplay();
        } catch (error) {
            console.error('Profile fetch error:', error);
        }
    }

    private updateProfileDisplay(): void {
        const profileText = document.getElementById('profileText');
        if (profileText && this.profileData) {
            profileText.innerHTML = `<a href="https://gmail.com" class="hover:underline">${this.profileData.email}</a>`;
        }
    }

    private setupEventListeners(): void {
        this.searchComponent.onSearch = this.performSearch.bind(this);
        this.tabComponent.onTabChange = this.switchTab.bind(this);
    }

    private async performSearch(maxMessages: number): Promise<void> {
        this.searchComponent.setLoading(true);

        try {
            // Clear previous results
            this.messages = [];
            this.categorizedMessages = { small: [], medium: [], large: [] };
            this.messageListComponent.clearAllResults();

            const response = await fetch(`/api/summary?max=${encodeURIComponent(maxMessages.toString())}`);
            this.messages = await response.json();

            // Categorize messages
            this.categorizedMessages = {
                small: this.messages.filter(item => item.size < 100 * 1024),
                medium: this.messages.filter(item => item.size >= 100 * 1024 && item.size < 1024 * 1024),
                large: this.messages.filter(item => item.size >= 1024 * 1024)
            };

            // Update UI components
            this.messageListComponent.updateResults(this.categorizedMessages);
            this.chartComponent.updateChart(this.categorizedMessages);
            this.tabComponent.updateStats(this.categorizedMessages);

            // Switch to small tab by default
            this.switchTab('small');

        } catch (error) {
            console.error('Error:', error);
            this.messageListComponent.showError('Error fetching results');
        } finally {
            this.searchComponent.setLoading(false);
        }
    }

    private switchTab(category: string): void {
        this.currentTab = category;
        this.tabComponent.switchToTab(category);
        this.messageListComponent.showTab(category);
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

class SearchComponent {
    private input: HTMLInputElement;
    private button: HTMLButtonElement;
    private loadingElement: HTMLElement;
    public onSearch?: (maxMessages: number) => void;

    constructor() {
        this.input = document.getElementById('maxMessages') as HTMLInputElement;
        this.button = document.getElementById('searchButton') as HTMLButtonElement;
        this.loadingElement = document.getElementById('loading') as HTMLElement;

        this.setupEventListeners();
    }

    private setupEventListeners(): void {
        this.button.addEventListener('click', () => {
            const maxMessages = parseInt(this.input.value) || 20;
            const clampedValue = Math.min(Math.max(maxMessages, 1), 50);
            this.input.value = clampedValue.toString();
            this.onSearch?.(clampedValue);
        });
    }

    public setLoading(loading: boolean): void {
        this.loadingElement.style.display = loading ? 'inline' : 'none';
        this.button.disabled = loading;
        this.input.disabled = loading;
    }
}

class TabComponent {
    public onTabChange?: (category: string) => void;

    constructor() {
        this.setupEventListeners();
    }

    private setupEventListeners(): void {
        ['small', 'medium', 'large'].forEach(category => {
            const heading = document.getElementById(`${category}-heading`);
            if (heading) {
                heading.addEventListener('click', () => {
                    this.onTabChange?.(category);
                });
            }
        });
    }

    public switchToTab(category: string): void {
        ['small', 'medium', 'large'].forEach(cat => {
            const tabElement = document.getElementById(`${cat}-tab`);
            const headingElement = document.getElementById(`${cat}-heading`);

            if (cat === category) {
                if (tabElement) tabElement.classList.remove('hidden');
                if (headingElement) headingElement.classList.add('active');
            } else {
                if (tabElement) tabElement.classList.add('hidden');
                if (headingElement) headingElement.classList.remove('active');
            }
        });
    }

    public updateStats(categorizedMessages: CategorizedMessages): void {
        Object.entries(categorizedMessages).forEach(([category, messages]) => {
            const countElement = document.getElementById(`${category}-count`);
            const sizeElement = document.getElementById(`${category}-size`);
            const headingElement = document.getElementById(`${category}-heading`);

            const totalSize = calculateTotalSize(messages);
            const formattedSize = formatSize(totalSize);

            if (countElement) countElement.textContent = `Count: ${messages.length}`;
            if (sizeElement) sizeElement.textContent = `Total size: ${formattedSize}`;
            if (headingElement) headingElement.textContent = `${category.charAt(0).toUpperCase() + category.slice(1)} (${formattedSize})`;
        });
    }
}

class MessageListComponent {
    constructor() {
        // Component initialized
    }

    public updateResults(categorizedMessages: CategorizedMessages): void {
        Object.entries(categorizedMessages).forEach(([category, messages]) => {
            const container = document.getElementById(`${category}-results`);
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

    public showTab(category: string): void {
        ['small', 'medium', 'large'].forEach(cat => {
            const tabElement = document.getElementById(`${cat}-tab`);
            if (tabElement) {
                if (cat === category) {
                    tabElement.classList.remove('hidden');
                } else {
                    tabElement.classList.add('hidden');
                }
            }
        });
    }

    public clearAllResults(): void {
        ['small', 'medium', 'large'].forEach(category => {
            const container = document.getElementById(`${category}-results`);
            if (container) container.innerHTML = '';
        });
    }

    public showError(message: string): void {
        ['small', 'medium', 'large'].forEach(category => {
            const container = document.getElementById(`${category}-results`);
            if (container) container.innerHTML = message;
        });
    }
}

class ChartComponent {
    private chart: any = null;

    constructor() {
        // Component initialized
    }

    public updateChart(categorizedMessages: CategorizedMessages): void {
        const chartElement = document.getElementById('emailSizePieChart') as HTMLCanvasElement;
        if (!chartElement) return;

        const ctx = chartElement.getContext('2d');
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

// Initialize the app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new App();
});