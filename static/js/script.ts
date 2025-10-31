interface MessageItem {
    thread_id: string;
    title: string;
    size: number;
}

interface ProfileData {
    email: string;
}

declare const Chart: any;

function formatSize(sizeInB: number): string {
    const sizeInKB = sizeInB / 1024;
    if (sizeInKB >= 1024) {
        return `${(sizeInKB / 1024).toFixed(2)} MB`;
    }
    return `${sizeInKB.toFixed(2)} KB`;
}

function createMessageElement(item: MessageItem): HTMLDivElement {
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

    const message_subject = document.createElement('p');
    message_subject.className = 'font-semibold truncate text-lg';
    message_subject.textContent = item.title;

    const sizeSpan = document.createElement('span');
    sizeSpan.className = 'size-badge';
    sizeSpan.textContent = formatSize(item.size);

    link.appendChild(message_subject);
    contentDiv.appendChild(link);
    flexContainer.appendChild(contentDiv);
    flexContainer.appendChild(sizeSpan);
    div.appendChild(flexContainer);

    return div;
}

function updateStats(category: string, messages: MessageItem[]): void {
    const countElement = document.getElementById(`${category}-count`);
    const sizeElement = document.getElementById(`${category}-size`);
    const totalSize = messages.reduce((sum: number, item: MessageItem) => sum + item.size, 0);
    if (countElement) countElement.textContent = `Count: ${messages.length}`;
    const formattedSize = formatSize(totalSize);
    if (sizeElement) sizeElement.textContent = `Total size: ${formattedSize}`;
    const headingElement = document.getElementById(`${category}-heading`);
    if (headingElement) headingElement.textContent = `${category} (${formattedSize})`;
}

function switchTab(category: string): void {
    ['small', 'medium', 'large'].forEach(cat => {
        const tabElement = document.getElementById(`${cat}-tab`);
        const headingElement = document.getElementById(`${cat}-heading`);

        if (cat === category) {
            if (tabElement) tabElement.classList.remove('hidden');
            if (headingElement) headingElement.classList.add('active');
        }
        else {
            if (tabElement) tabElement.classList.add('hidden');
            if (headingElement) headingElement.classList.remove('active');
        }
    })
}
function performSearch(): void {
    const searchInput = document.getElementById('maxMessages') as HTMLInputElement | null;
    const loading = document.getElementById('loading');
    if (loading) loading.style.display = 'inline';
    const searchButton = document.getElementById('searchButton') as HTMLButtonElement | null;
    if (searchButton) searchButton.disabled = true;
    if (searchInput) searchInput.disabled = true;

    // Clear all results
    const smallResults = document.getElementById('small-results');
    const mediumResults = document.getElementById('medium-results');
    const largeResults = document.getElementById('large-results');
    if (smallResults) smallResults.innerHTML = '';
    if (mediumResults) mediumResults.innerHTML = '';
    if (largeResults) largeResults.innerHTML = '';

    const maxValue = searchInput?.value ? (parseInt(searchInput.value) < 51 ? searchInput.value : '50') : '50';
    fetch(`/api/summary?max=${encodeURIComponent(maxValue)}`)
        .then(response => response.json())
        .then((data: MessageItem[]) => {
            if (loading) loading.style.display = 'none';

            // Categorize messages
            const categories = {
                small: data.filter((item: MessageItem) => item.size < 100 * 1024),
                medium: data.filter((item: MessageItem) => item.size >= 100 * 1024 && item.size < 1024 * 1024),
                large: data.filter((item: MessageItem) => item.size >= 1024 * 1024)
            };

            // Update each category
            Object.entries(categories).forEach(([category, messages]) => {
                const container = document.getElementById(`${category}-results`);
                messages.forEach((item: MessageItem) => {
                    if (container) container.appendChild(createMessageElement(item));
                });
                updateStats(category, messages);
            });

            // Update pie chart
            const chartElement = document.getElementById('emailSizePieChart') as HTMLCanvasElement | null;
            if (chartElement) {
                const ctx = chartElement.getContext('2d');
                if (ctx) {
                    new Chart(ctx, {
                        type: 'pie',
                        data: {
                            labels: ['Small (<100KB)', 'Medium (100KB-1MB)', 'Large (>1MB)'],
                            datasets: [{
                                data: [
                                    categories.small.reduce((prev: number, curr: MessageItem) => prev + curr.size, 0) / 1024,
                                    categories.medium.reduce((prev: number, curr: MessageItem) => prev + curr.size, 0) / 1024,
                                    categories.large.reduce((prev: number, curr: MessageItem) => prev + curr.size, 0) / 1024
                                ],
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

            // Default to small tab and re-enable controls
            switchTab('small');
            if (searchButton) searchButton.disabled = false;
            if (searchInput) searchInput.disabled = false;
        })
        .catch(error => {
            if (loading) loading.style.display = 'none';
            console.error('Error:', error);
            ['small', 'medium', 'large'].forEach(category => {
                const results = document.getElementById(`${category}-results`);
                if (results) results.innerHTML = 'Error fetching results';
            });

            // Re-enable controls after error
            if (searchButton) searchButton.disabled = false;
            if (searchInput) searchInput.disabled = false;
        });
}

document.addEventListener('DOMContentLoaded', () => {
    ['small', 'medium', 'large'].forEach(category => {
        const heading = document.getElementById(`${category}-heading`);
        if (heading) {
            heading.addEventListener('click', () => {
                switchTab(category);
            });
        }
    });
    const profileText = document.getElementById('profileText');
    fetch('/api/profile')
        .then(response => response.json())
        .then((data: ProfileData) => {
            if (profileText) {
                profileText.innerHTML = '<a href="https://gmail.com" class="hover:underline">' + data.email + '</a>';
            }
        })
        .catch(error => {
            console.error('Profile fetch error:', error);
        });
})
