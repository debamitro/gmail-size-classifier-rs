import { ProfileData } from "./types.js";

export default class HeaderComponent {
    private profileContainer: HTMLElement;
    private profileData: ProfileData | null = null;

    constructor() {
        this.profileContainer = document.createElement('div');
        this.profileContainer.className = 'profile-container';
    }

    render(): HTMLElement {
        const header = document.createElement('header');
        header.className = 'app-header';
        
        // Left section with logo and title
        const leftSection = document.createElement('div');
        leftSection.className = 'header-left';
        
        const logoContainer = document.createElement('div');
        logoContainer.className = 'logo-container';
        
        const logo = document.createElement('div');
        logo.className = 'app-logo';
        logo.innerHTML = '📧'; // Gmail-like icon
        
        const titleContainer = document.createElement('div');
        titleContainer.className = 'title-container';
        
        const title = document.createElement('h1');
        title.textContent = 'Gmail Cleaner';
        title.className = 'app-title';
        
        const subtitle = document.createElement('p');
        subtitle.textContent = 'Your inbox described by email sizes';
        subtitle.className = 'app-subtitle';
        
        logoContainer.appendChild(logo);
        titleContainer.appendChild(title);
        titleContainer.appendChild(subtitle);
        leftSection.appendChild(logoContainer);
        leftSection.appendChild(titleContainer);
        
        // Right section with profile
        const rightSection = document.createElement('div');
        rightSection.className = 'header-right';
        rightSection.appendChild(this.profileContainer);
        
        header.appendChild(leftSection);
        header.appendChild(rightSection);
        
        // Add professional styling
        this.addStyles();
        
        this.updateProfile(null);
        return header;
    }

    updateProfile(profileData: ProfileData | null) {
        this.profileData = profileData;
        
        if (profileData) {
            this.profileContainer.innerHTML = `
                <div class="profile-info">
                    <div class="profile-details" style="cursor: pointer;">
                        <div style="display: block;" id="profile-email">
                            <span class="profile-email">${profileData.email}</span>
                        </div>
                        <div style="display: none;" id="profile-dropdown">
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"></path>
                                <polyline points="10,17 15,12 10,7"></polyline>
                                <line x1="15" y1="12" x2="3" y2="12"></line>
                            </svg>
                            <a href="/login">Change Account</a>
                        </div>
                    </div>
                </div>`;
            
            // Add click event to toggle Change Account visibility
            const profileDetails = this.profileContainer.querySelector('.profile-details') as HTMLElement;
            profileDetails.addEventListener('click', () => {
                const changeAccountLink = profileDetails.querySelector('#profile-dropdown') as HTMLElement;
                changeAccountLink.style.display = changeAccountLink.style.display === 'none' ? 'block' : 'none';
            });
        } else {
            this.profileContainer.innerHTML = `
                <div class="sign-in-container">
                    <button class="sign-in-btn">
                        <a href="/login">Sign in</a>
                    </button>
                </div>
            `;           
        }
    }

    private addStyles() {
        const styleId = 'header-styles';
        if (document.getElementById(styleId)) return;
        
        const style = document.createElement('style');
        style.id = styleId;
        style.textContent = `
            .app-header {
                background: #667eea;
                color: white;
                padding: 1rem 2rem;
                display: flex;
                justify-content: space-between;
                align-items: center;
                box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
                border-bottom: 1px solid rgba(255, 255, 255, 0.1);
                min-height: 80px;
            }

            .header-left {
                display: flex;
                align-items: center;
                gap: 1rem;
            }

            .logo-container {
                display: flex;
                align-items: center;
                justify-content: center;
                width: 50px;
                height: 50px;
                background: rgba(255, 255, 255, 0.15);
                border-radius: 12px;
                backdrop-filter: blur(10px);
            }

            .app-logo {
                font-size: 24px;
                filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
            }

            .title-container {
                display: flex;
                flex-direction: column;
                gap: 0.25rem;
            }

            .app-title {
                margin: 0;
                font-size: 1.75rem;
                font-weight: 700;
                letter-spacing: -0.025em;
                text-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
            }

            .app-subtitle {
                margin: 0;
                font-size: 0.875rem;
                opacity: 0.9;
                font-weight: 400;
            }

            .header-right {
                display: flex;
                align-items: center;
            }

            .profile-container {
                display: flex;
                align-items: center;
                gap: 0.75rem;
            }

            .profile-info {
                display: flex;
                align-items: center;
                gap: 0.75rem;
                background: rgba(255, 255, 255, 0.1);
                padding: 0.5rem 1rem;
                border-radius: 50px;
                backdrop-filter: blur(10px);
                border: 1px solid rgba(255, 255, 255, 0.2);
            }

            .profile-details {
                display: flex;
                flex-direction: column;
                align-items: flex-end;
                gap: 0.125rem;
            }

            .profile-name {
                font-weight: 600;
                font-size: 0.875rem;
                line-height: 1.2;
            }

            .profile-email {
                font-size: 0.75rem;
                opacity: 0.8;
                line-height: 1.2;
            }

            .profile-avatar {
                display: flex;
                align-items: center;
                justify-content: center;
            }

            .profile-picture {
                width: 36px;
                height: 36px;
                border-radius: 50%;
                border: 2px solid rgba(255, 255, 255, 0.3);
                object-fit: cover;
            }

            .sign-out-btn {
                background: rgba(255, 255, 255, 0.1);
                border: 1px solid rgba(255, 255, 255, 0.2);
                color: white;
                padding: 0.5rem;
                border-radius: 8px;
                cursor: pointer;
                transition: all 0.2s ease;
                backdrop-filter: blur(10px);
                display: flex;
                align-items: center;
                justify-content: center;
            }

            .sign-out-btn:hover {
                background: rgba(255, 255, 255, 0.2);
                transform: translateY(-1px);
            }

            .sign-in-container {
                display: flex;
                align-items: center;
            }

            .sign-in-btn {
                background: rgba(255, 255, 255, 0.95);
                color: #374151;
                border: none;
                padding: 0.75rem 1.5rem;
                border-radius: 50px;
                font-weight: 600;
                font-size: 0.875rem;
                cursor: pointer;
                transition: all 0.2s ease;
                display: flex;
                align-items: center;
                gap: 0.5rem;
                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
            }

            .sign-in-btn:hover {
                background: white;
                transform: translateY(-1px);
                box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
            }

            .sign-in-btn svg {
                opacity: 0.7;
            }

            @media (max-width: 768px) {
                .app-header {
                    padding: 1rem;
                    flex-direction: column;
                    gap: 1rem;
                    min-height: auto;
                }

                .header-left {
                    flex-direction: column;
                    text-align: center;
                    gap: 0.5rem;
                }

                .app-title {
                    font-size: 1.5rem;
                }

                .app-subtitle {
                    font-size: 0.8rem;
                }

                .profile-info {
                    flex-direction: column;
                    text-align: center;
                    padding: 0.75rem;
                }

                .profile-details {
                    align-items: center;
                }
            }
        `;
        
        document.head.appendChild(style);
    }
}
