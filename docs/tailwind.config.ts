/** @type {import('tailwindcss').Config} */
export default {
	content: ["./src/**/*.rs"],
	theme: {
		container: {
			center: true,
			padding: {
				DEFAULT: "1rem",
				sm: "2rem",
				lg: "4rem",
				xl: "5rem",
				"2xl": "6rem",
			},
		},
		extend: {
			colors: {
				primary: {
					DEFAULT: "#0070c2",
					50: "#f0f7ff",
					100: "#e0eefe",
					200: "#bae0fd",
					300: "#7cc6fb",
					400: "#36a9f7",
					500: "#0c8de4",
					600: "#0070c2",
					700: "#015a9e",
					800: "#064b81",
					900: "#0a406c",
					950: "#072a4a",
				},
				secondary: {
					50: "#f5f3ff",
					100: "#ede8ff",
					200: "#dcd5ff",
					300: "#c3b5fd",
					400: "#a48afb",
					500: "#8a5cf6",
					600: "#7c3aed",
					700: "#6d28d9",
					800: "#5b21b6",
					900: "#4c1d95",
					950: "#2e1065",
				},
				code: {
					background: "#1e293b",
					foreground: "#f8fafc",
				},
				background: "hsl(var(--background))",
				foreground: "hsl(var(--foreground))",
				card: {
					DEFAULT: "hsl(var(--card))",
					foreground: "hsl(var(--card-foreground))",
					hover: "hsl(var(--muted))",
				},
				muted: {
					DEFAULT: "hsl(var(--muted))",
					foreground: "hsl(var(--muted-foreground))",
				},
				accent: {
					DEFAULT: "hsl(var(--accent))",
					foreground: "hsl(var(--accent-foreground))",
				},
				border: "hsl(var(--border))",
				ring: "hsl(var(--ring))",
			},
			keyframes: {
				"accordion-down": {
					from: { height: 0 },
					to: { height: "var(--radix-accordion-content-height)" },
				},
				"accordion-up": {
					from: { height: "var(--radix-accordion-content-height)" },
					to: { height: 0 },
				},
				"fade-in": {
					from: { opacity: 0 },
					to: { opacity: 1 },
				},
				"fade-out": {
					from: { opacity: 1 },
					to: { opacity: 0 },
				},
				"slide-in": {
					from: { transform: "translateY(10px)", opacity: 0 },
					to: { transform: "translateY(0)", opacity: 1 },
				},
			},
			animation: {
				"accordion-down": "accordion-down 0.2s ease-out",
				"accordion-up": "accordion-up 0.2s ease-out",
				"fade-in": "fade-in 0.3s ease-out",
				"fade-out": "fade-out 0.3s ease-out",
				"slide-in": "slide-in 0.3s ease-out",
			},
			borderRadius: {
				lg: "var(--radius)",
				md: "calc(var(--radius) - 2px)",
				sm: "calc(var(--radius) - 4px)",
			},
		},
	},
	darkMode: "class",
	plugins: [],
};
