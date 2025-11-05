import { CategorizedMessages } from "./types.js";
import { calculateTotalSize } from "./utils.js";

export default class ChartSectionComponent {
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
