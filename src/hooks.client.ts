import posthog from 'posthog-js';
import type { HandleClientError } from '@sveltejs/kit';

export const handleError: HandleClientError = ({ error }) => {
	posthog.captureException(error);
};
