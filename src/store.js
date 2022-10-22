import { reactive } from 'vue';

export const store = reactive({
    content: '',
    updateContent(text) {
        this.content = text;
    }
});