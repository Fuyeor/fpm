// @/composables/loader/useAsyncComponents.ts
import { defineAsyncComponent } from 'vue';

// define async components for better code splitting and performance
const asyncComponents = {
  RightSidebar: defineAsyncComponent(() => import('@/layout/Right.vue')),
  BottomSidebar: defineAsyncComponent(() => import('@/layout/Bottom.vue')),
} as const;

/**
 * Composable to provide globally used asynchronous components.
 * @returns An object containing the async components for easy destructuring.
 */
export function useAsyncComponents() {
  return asyncComponents;
}
