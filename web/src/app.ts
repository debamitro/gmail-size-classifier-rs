// Gmail Size Classifier - TypeScript Web App UI (DOM Generated)

import HeaderComponent from './header.js'
import SearchSectionComponent from './search.js';
import ChartSectionComponent from './chart_section.js';
import TabsSectionComponent from './tab_section.js';
import {ProfileData, MessageItem, CategorizedMessages} from './types.js';

export class App {
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
    private chartElement: HTMLDivElement | null = null;
    private tabsElement: HTMLDivElement | null = null;

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
        document.body.className = 'font-Inter -apple-system BlinkMacSystemFont SegoeUI Roboto sans-serif max-w-5xl mx-auto px-5 py-5 bg-gray-100 min-h-screen';

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
            .input-field { border: 2px solid #e2e8f0; border-radius: 8px; font-size: 1rem; transition: border-color 0.2s; }
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
        this.chartElement = this.chartSection.render();
        this.tabsElement = this.tabsSection.render();

        this.chartElement.style.display = "none";
        this.tabsElement.style.display = "none";
        mainContainer.appendChild(headerElement);
        mainContainer.appendChild(searchElement);
        mainContainer.appendChild(this.chartElement);
        mainContainer.appendChild(this.tabsElement);
    }

    private setupEventListeners(): void {
        this.searchSection.onSearch = this.performSearch.bind(this);
        this.tabsSection.onTabChange = this.switchTab.bind(this);
    }

    private async performSearch(maxMessages: number): Promise<void> {
        this.searchSection.setLoading(true);

        try {
            const response = await fetch(`/api/summary?max=${encodeURIComponent(maxMessages.toString())}`);
            const newMessages = await response.json();

            if (this.chartElement) {
                this.chartElement.style.display = "grid";
            }
            if (this.tabsElement) {
                this.tabsElement.style.display = "grid";
            }
            
            // Append new messages to existing ones
            this.messages.push(...newMessages);

            // Re-categorize all messages
            this.categorizedMessages = {
                small: this.messages.filter(item => item.size < 100 * 1024),
                medium: this.messages.filter(item => item.size >= 100 * 1024 && item.size < 1024 * 1024),
                large: this.messages.filter(item => item.size >= 1024 * 1024)
            };

            // Update tabs with all results
            this.tabsSection.clearAllResults();
            this.tabsSection.updateResults(this.categorizedMessages);

            // Update UI components
            this.chartSection.updateChart(this.categorizedMessages);
            this.tabsSection.updateStats(this.categorizedMessages);

            // Update analyzed count
            this.searchSection.updateAnalyzedCount(this.messages.length);

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

// Component Classes

class RootContainer {
    public render(): HTMLDivElement {
        const container = document.createElement('div');
        container.className = 'main-container bg-white rounded-2xl shadow-2xl overflow-hidden';
        return container;
    }
}
