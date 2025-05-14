/** @type {import('tailwindcss').Config} */
module.exports = {
	mode: "jit",
	content: [
		"./css/*.{js,ts,jsx,tsx,css,scss,html}",
		"./css/**/*.{js,ts,jsx,tsx}",
		"./src/**/*.{js,ts,jsx,tsx,rs,scss,css,html}",
		"./index.html",
		"./src/main.rs",
	]
};
