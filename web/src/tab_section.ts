import { MessageItem, CategorizedMessages } from "./types.js";
import { calculateTotalSize, formatSize } from "./utils.js";

export default class TabsSectionComponent {
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

