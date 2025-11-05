import { ProfileData } from "./types.js";

export default class HeaderComponent {
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
        version.textContent = 'version 0.3.0-beta';

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
