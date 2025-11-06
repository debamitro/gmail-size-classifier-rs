export default class SearchSectionComponent {
    private input: HTMLInputElement;
    private button: HTMLButtonElement;
    private loadingElement: HTMLSpanElement;
    private statusElement: HTMLDivElement;
    public onSearch?: (maxMessages: number) => void;

    constructor() {
        this.input = document.createElement('input');
        this.button = document.createElement('button');
        this.loadingElement = document.createElement('span');
        this.statusElement = document.createElement('div');
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

        this.button.textContent = 'Analyze first 20 emails';
        this.button.className = 'btn-primary';

        this.loadingElement.className = 'loading inline-flex items-center gap-2';
        this.loadingElement.innerHTML = `
            <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"></path>
            </svg>
            Analyzing emails...
        `;
        this.statusElement.className = 'mt-3 text-center text-sm text-gray-600';
        this.statusElement.textContent = 'No emails analyzed yet';
    }

    private setupEventListeners(): void {
        this.input.addEventListener('input', () => {
            const maxMessages = parseInt(this.input.value) || 20;
            const clampedValue = Math.min(Math.max(maxMessages, 1), 50);
            this.button.textContent = `Analyze first ${clampedValue} emails`;
        });

        this.button.addEventListener('click', () => {
            const maxMessages = parseInt(this.input.value) || 20;
            const clampedValue = Math.min(Math.max(maxMessages, 1), 50);
            this.input.value = clampedValue.toString();
            this.button.textContent = `Analyze next ${clampedValue} emails`;
            this.onSearch?.(clampedValue);
        });
    }

    public render(): HTMLDivElement {
        const section = document.createElement('div');
        section.className = 'search-container bg-gray-50 py-2 border-b border-gray-200';

        const container = document.createElement('div');
        container.className = 'max-w-md mx-auto flex flex-col';

        const label = document.createElement('label');
        label.htmlFor = 'maxMessages';
        label.className = 'block text-sm font-medium text-gray-700 mb-2';
        label.textContent = 'Maximum Messages to Analyze';

        const inputContainer = document.createElement('div');
        inputContainer.className = 'flex gap-3 flex-row';

        const loadingContainer = document.createElement('div');
        loadingContainer.className = 'mt-3 text-center';

        inputContainer.appendChild(label);
        inputContainer.appendChild(this.input);
        loadingContainer.appendChild(this.loadingElement);
        loadingContainer.appendChild(this.statusElement);

        container.appendChild(inputContainer);
        container.appendChild(this.button);
        container.appendChild(loadingContainer);
        section.appendChild(container);

        return section;
    }

    public setLoading(loading: boolean): void {
        this.loadingElement.style.display = loading ? 'inline-flex' : 'none';
        this.button.disabled = loading;
        this.input.disabled = loading;
    }

    public updateAnalyzedCount(count: number): void {
        if (count === 0) {
            this.statusElement.textContent = 'No emails analyzed yet';
        } else {
            this.statusElement.textContent = `Results from 1-${count} emails analyzed`;
        }
    }
}
