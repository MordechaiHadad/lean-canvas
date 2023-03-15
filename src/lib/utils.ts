import { invoke } from '@tauri-apps/api/tauri';

export class ThemeHandler {
	private isDark: boolean;

	constructor(isDark: boolean) {
		this.isDark = isDark;
		this.toggle = this.toggle.bind(this);
	}

	toggle() {
		console.log(this.isDark);
		this.isDark = !this.isDark;
		document.documentElement.classList.toggle('dark', this.isDark);
	}
}

import he from '../locales/he.json';
import en from '../locales/en.json';

export class LanguageHandler {
	private _currentLang: string;
	private _language_strings: any;

	constructor() {
		this._currentLang = 'en';
		this.toggle = this.toggle.bind(this);
	}

	toggle() {
		if (this._currentLang === 'en') {
			this._language_strings = he;
			this._currentLang = 'he';
			return;
		}
		this._language_strings = en;
		this._currentLang = 'en';
	}

	public get language_strings(): any {
		return this._language_strings;
	}
}

interface IDocumentFile {
	readonly name: string;
	readonly data: string;
}

export class DocumentFile implements IDocumentFile {
	readonly name: string;
	readonly data: string;

	constructor(name: string, data: string) {
		this.name = name;
		this.data = data;
	}

	static async readFile(): Promise<DocumentFile> {
		let file: IDocumentFile = await invoke('read_file');
		let newFile = new DocumentFile(file.name, file.data);

		const json = JSON.parse(newFile.data);
		const map = new Map(Object.entries(json));

		map.forEach((v, k) => {
			const parent = document.getElementById(k);

			if (!parent) {
				return;
			}

			let textarea = parent.querySelector('textarea');

			if (!textarea) {
				return;
			}

			textarea.value = v as string;
		});

		let name = document.getElementById('main_name');
		name!.textContent = newFile.name;

		return newFile;
	}

	async saveFile(ids: string[]) {
		let values: string[] = [];
		let existing_ids: string[] = [];

		ids.forEach((id) => {
			const parent = document.getElementById(id);

			if (!parent) {
				return;
			}

			const textarea = parent.querySelector('textarea');

			if (!textarea) {
				return;
			}

			existing_ids.push(id);
			values.push(textarea.value);
		});

		const prjName = document.getElementById('main_name');

		if (!prjName) {
			return;
		}

		await invoke('save_file', {
			values: values,
			ids: existing_ids,
			name: prjName.textContent
		});
	}
}
