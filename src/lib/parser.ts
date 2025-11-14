export interface GS1Data {
	gtin: string;
	pzn: string;
	serial: string;
	expiry: string; // YYYY-MM-DD
	batch: string;
}

export function parseGS1(raw: string): GS1Data {
	let pos = 0;
	const result: GS1Data = {
		gtin: '',
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

	const isAI = (s: string) => fixedAIs[s] !== undefined || variableAIs.includes(s);

	while (pos < raw.length) {
		const ai = raw.substring(pos, pos + 2);
		pos += 2;

		if (fixedAIs[ai] !== undefined) {
			const len = fixedAIs[ai];
			const value = raw.substring(pos, pos + len);
			pos += len;

			if (ai === '01') {
				result.gtin = value;

				// ❗ Extract PZN from GTIN/NTIN (Germany: digits 6..12)
				// Example: 04150 + PZN(7) + check digit
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

			// find next AI
			while (pos < raw.length && !isAI(raw.substring(pos, pos + 2))) {
				pos++;
			}

			const value = raw.substring(start, pos);

			if (ai === '10') result.batch = value;
			if (ai === '21') result.serial = value;
		} else {
			// unknown AI → fail-safe exit
			break;
		}
	}

	return result;
}
