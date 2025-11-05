export interface ProfileData {
    email: string;
}

export interface MessageItem {
    thread_id: string;
    title: string;
    size: number;
}

export interface CategorizedMessages {
    small: MessageItem[];
    medium: MessageItem[];
    large: MessageItem[];
}

