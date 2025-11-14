import { listen } from '@tauri-apps/api/event';

export interface KeyEvent {
	key_code: string;
	label: string | null;
	state: string;
}

export interface GS1Data {
	gtin: string | null;
	serialNumber: string | null;
	expirationDate: string | null;
	lotNumber: string | null;
}

export async function listenBuffered(callback: (data: GS1Data) => void) {
	let buffer: KeyEvent[] = [];
	let isScanning = false;

	return listen<KeyEvent>('global-key-event', (event) => {
		const { label, state } = event.payload;
		if (state !== 'down') return;

		// Check for start of scan (Ctrl+Shift+B = STX)
		if (!isScanning && label === '\u0002') {
			isScanning = true;
			buffer = [];
			return;
		}

		// Check for end of scan (ETX)
		if (isScanning && label === '\u0003') {
			isScanning = false;
			const gs1Data = parseGS1(buffer);
			callback(gs1Data);
			buffer = [];
			return;
		}

		// If scanning, add to buffer
		if (isScanning) {
			buffer.push(event.payload);
		}
	});
}

export function parseGS1(events: KeyEvent[]) {
	let barcodeData = '';

	for (const event of events) {
		if (event.state === 'down' && event.label) {
			// Handle special control characters
			if (event.label === '\u0002') {
				// Start of transmission - continue
				continue;
			} else if (event.label === '\u0003') {
				// End of transmission - stop processing
				break;
			} else if (event.label.length === 1) {
				barcodeData += event.label;
			}
		}
	}

	// Initialize result object
	const result: GS1Data = {
		gtin: null,
		serialNumber: null,
		expirationDate: null,
		lotNumber: null
	};

	if (!barcodeData) {
		return result;
	}

	console.log('Raw barcode data:', barcodeData); // For debugging

	let position = 0;

	while (position < barcodeData.length - 1) {
		// Get the next AI (2 digits)
		const ai = barcodeData.substring(position, position + 2);

		if (ai === '01') {
			// GTIN - fixed length 14 digits
			if (position + 16 <= barcodeData.length) {
				result.gtin = barcodeData.substring(position + 2, position + 16);
				position += 16;
			} else {
				break;
			}
		} else if (ai === '21') {
			// Serial Number - variable length, look for next AI or GS
			position += 2; // Skip the AI
			let endPos = position;

			// Look for the next AI (17 or 10) or end of string
			while (endPos < barcodeData.length - 1) {
				const nextAI = barcodeData.substring(endPos, endPos + 2);
				if (nextAI === '17' || nextAI === '10') {
					break;
				}
				endPos++;
			}

			result.serialNumber = barcodeData.substring(position, endPos);
			position = endPos;
		} else if (ai === '17') {
			// Expiration Date - fixed length 6 digits (YYMMDD)
			if (position + 8 <= barcodeData.length) {
				result.expirationDate = barcodeData.substring(position + 2, position + 8);
				position += 8;
			} else {
				break;
			}
		} else if (ai === '10') {
			// Lot/Batch Number - variable length, usually goes to end or next AI
			position += 2; // Skip the AI
			let endPos = barcodeData.length; // Default to end of string

			// Look for any other AI that might follow
			for (let i = position + 1; i < barcodeData.length - 1; i++) {
				const nextAI = barcodeData.substring(i, i + 2);
				if (['01', '21', '17'].includes(nextAI)) {
					endPos = i;
					break;
				}
			}

			result.lotNumber = barcodeData.substring(position, endPos);
			position = endPos;
		} else {
			// Unknown AI, skip one character and try again
			position++;
		}
	}

	return result;
}
