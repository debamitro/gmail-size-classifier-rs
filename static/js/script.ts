// @ts-nocheck
function formatSize(sizeInB) {
    const sizeInKB = sizeInB / 1024;
    if (sizeInKB >= 1024) {
        return `${(sizeInKB / 1024).toFixed(2)} MB`;
    }
    return `${sizeInKB.toFixed(2)} KB`;
}

function createMessageElement(item) {
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

function updateStats(category, messages) {
    const countElement = document.getElementById(`${category}-count`);
    const sizeElement = document.getElementById(`${category}-size`);
    const totalSize = messages.reduce((sum, item) => sum + item.size, 0);
    countElement.textContent = `Count: ${messages.length}`;
    const formattedSize = formatSize(totalSize);
    sizeElement.textContent = `Total size: ${formattedSize}`;
    const headingElement = document.getElementById(`${category}-heading`);
    headingElement.textContent = `${category} (${formattedSize})`;
}

function switchTab(category) {
    ['small', 'medium', 'large'].forEach(cat => {
        const tabElement = document.getElementById(`${cat}-tab`);
        const headingElement = document.getElementById(`${cat}-heading`);

        if (cat === category) {
            tabElement.classList.remove('hidden');
            headingElement.classList.add('active');
        }
        else {
            tabElement.classList.add('hidden');
            headingElement.classList.remove('active');
        }
    })
}
function performSearch() {
    const searchInput = document.getElementById('maxMessages');
    const loading = document.getElementById('loading');
    loading.style.display = 'inline';
    const searchButton = document.getElementById('searchButton');
    searchButton.disabled = true;
    searchInput.disabled = true;

    // Clear all results
    document.getElementById('small-results').innerHTML = '';
    document.getElementById('medium-results').innerHTML = '';
    document.getElementById('large-results').innerHTML = '';

    fetch(`/api/summary?max=${encodeURIComponent(searchInput.value < 51 ? searchInput.value : 50)}`)
        .then(response => response.json())
        .then(data => {
            loading.style.display = 'none';

            // Categorize messages
            const categories = {
                small: data.filter(item => item.size < 100 * 1024),
                medium: data.filter(item => item.size >= 100 * 1024 && item.size < 1024 * 1024),
                large: data.filter(item => item.size >= 1024 * 1024)
            };

            // Update each category
            Object.entries(categories).forEach(([category, messages]) => {
                const container = document.getElementById(`${category}-results`);
                messages.forEach(item => {
                    container.appendChild(createMessageElement(item));
                });
                updateStats(category, messages);
            });

            // Update pie chart
            const ctx = document.getElementById('emailSizePieChart').getContext('2d');
            new Chart(ctx, {
                type: 'pie',
                data: {
                    labels: ['Small (<100KB)', 'Medium (100KB-1MB)', 'Large (>1MB)'],
                    datasets: [{
                        data: [
                            categories.small.reduce((prev, curr) => prev + curr.size, 0) / 1024,
                            categories.medium.reduce((prev, curr) => prev + curr.size, 0) / 1024,
                            categories.large.reduce((prev, curr) => prev + curr.size, 0) / 1024
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

            // Default to small tab and re-enable controls
            switchTab('small');
            searchButton.disabled = false;
            searchInput.disabled = false;
        })
        .catch(error => {
            loading.style.display = 'none';
            console.error('Error:', error);
            ['small', 'medium', 'large'].forEach(category => {
                document.getElementById(`${category}-results`).innerHTML = 'Error fetching results';
            });

            // Re-enable controls after error
            searchButton.disabled = false;
            searchInput.disabled = false;
        });
}

document.addEventListener('DOMContentLoaded', () => {
    ['small', 'medium', 'large'].forEach(category => {
        const heading = document.getElementById(`${category}-heading`);
        heading.addEventListener('click', () => {
            switchTab(category);
        });
    });
    const profileText = document.getElementById('profileText');
    fetch('/api/profile')
        .then(response => response.json())
        .then(data => {
            profileText.innerHTML = '<a href="https://gmail.com" class="hover:underline">' + data.email + '</a>';
        })
        .catch(error => {

        });
})
