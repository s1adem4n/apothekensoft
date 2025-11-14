export interface GS1Data {
	pzn: string;
	serial: string;
	expiry: string; // YYYY-MM-DD
	batch: string;
}

export function parseGS1(raw: string): GS1Data {
	let pos = 0;
	const result: GS1Data = {
		pzn: '',
		serial: '',
		expiry: '',
		batch: ''
	};

	const fixedAIs: Record<string, number> = {
		'01': 14, // GTIN/NTIN
		'17': 6 // Expiry YYMMDD
	};

	// variable-length AIs → stop at next AI or end
	const variableAIs = ['10', '21'];

	const knownAIs = ['01', '10', '17', '21'];
	const isAI = (s: string) => knownAIs.includes(s);

	while (pos < raw.length) {
		// Check if we have at least 2 characters for AI
		if (pos + 2 > raw.length) break;

		const ai = raw.substring(pos, pos + 2);

		// If not a known AI, stop parsing
		if (!isAI(ai)) break;

		pos += 2;

		if (fixedAIs[ai] !== undefined) {
			const len = fixedAIs[ai];
			const value = raw.substring(pos, pos + len);
			pos += len;

			if (ai === '01') {
				// Extract PZN from GTIN (04150 prefix + 7 digit PZN)
				if (value.startsWith('04150') && value.length === 14) {
					result.pzn = value.substring(5, 12);
				}
			}

			if (ai === '17') {
				const yy = value.substring(0, 2);
				const mm = value.substring(2, 4);
				const dd = value.substring(4, 6);
				result.expiry = `20${yy}-${mm}-${dd}`;
			}
		} else if (variableAIs.includes(ai)) {
			const start = pos;

			// Variable length: read until we find another AI or reach the end
			while (pos < raw.length) {
				// Look ahead for next AI
				if (pos + 2 <= raw.length && isAI(raw.substring(pos, pos + 2))) {
					break;
				}
				pos++;
			}

			const value = raw.substring(start, pos);

			if (ai === '10') result.batch = value;
			if (ai === '21') result.serial = value;
		}
	}

	return result;
}
