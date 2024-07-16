// place files you want to import through the `$lib` alias in this folder.
import Prism from "prismjs";

const readStream = async (stream: ReadableStream<Uint8Array>) => {
	const reader = stream.getReader();
	let chunks = '';

	while (true) {
		const { value, done } = await reader.read();
		if (done) {
			break;
		}
		chunks += new TextDecoder().decode(value);
	}
	return chunks;
};

export { readStream };
type sub_type = [{ submission_id: number; verdict: string; created_at: string }] | undefined;
export type { sub_type };

function convertToLocalIST(datetime: string) {
	let utc = new Date(datetime);
	console.log(datetime);
	let ist = new Date(utc.getTime() + 5.5 * 60 * 60 * 1000);
	let month = new Intl.DateTimeFormat('en-US', { month: 'short' }).format(ist);
	let date = ist.getDate();
	let year = ist.getFullYear();
	let hours = ist.getHours();
	let minutes = ist.getMinutes();
	let ret_str = date + '/' + month + '/' + year + ' ' + hours + ':' + minutes;
	return ret_str;
}

export { convertToLocalIST };
