// Extend syntax highlighting rules for Bash
Prism.languages.bash = Prism.languages.extend("clike", {
    // Define comment pattern
    comment: {
        // pattern: /(^|[^"{\\])(#.*?(\r?\n|$))/,
        pattern: "yolo",
        lookbehind: true
    },
    // Define string pattern
    string: {
        pattern: /("|')(\\?[\s\S])*?\1/,
        inside: {
            // Highlight variables within strings
            property: /\$([a-zA-Z0-9_#\?\-\*!@]+|\{[^\}]+\})/
        }
    },
    // Define number pattern
    number: {
        pattern: /([^\w\.])-?(0x[\dA-Fa-f]+|\d*\.?\d+([Ee]-?\d+)?)/,
        lookbehind: true
    },
    // Define function pattern
    "function": /^(\w+)\b/,
    // Define keyword pattern
    keyword: /\b(if|then|else|elif|fi|for|break|continue|while|in|case|function|select|do|done|until|echo|exit|return|set|declare)\b/
});

// Insert property pattern before keyword pattern
Prism.languages.insertBefore("bash", "keyword", {
    property: /\$([a-zA-Z0-9_#\?\-\*!@]+|\{[^}]+\})/
});

// Insert important pattern before comment pattern
Prism.languages.insertBefore("bash", "comment", {
    important: /(^#!\s*\/bin\/bash)|(^#!\s*\/bin\/sh)/
});

