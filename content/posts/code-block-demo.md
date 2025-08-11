+++
title = "Code Block Styling Demo"
date = 2024-12-16
description = "A demonstration of the macOS-style code block styling"
draft = false
+++

This post demonstrates the beautiful macOS-style code block styling implemented in our blog theme.

## Python Example

<div class="highlight-wrap" data-lang="python">
<pre><code>def hello_world():
    """A simple greeting function"""
    print("Hello, World!")
    
    # Let's add some more interesting code
    for i in range(3):
        print(f"Count: {i}")

if __name__ == "__main__":
    hello_world()</code></pre>
</div>

## JavaScript Example

<div class="highlight-wrap" data-lang="javascript">
<pre><code>// A simple React component
const Greeting = ({ name }) => {
    const [count, setCount] = useState(0);
    
    return (
        <div className="greeting">
            <h1>Hello, {name}!</h1>
            <button onClick={() => setCount(count + 1)}>
                Clicked {count} times
            </button>
        </div>
    );
};</code></pre>
</div>

## CSS Example

<div class="highlight-wrap" data-lang="css">
<pre><code>.gradient-button {
    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
    color: white;
    padding: 12px 24px;
    border-radius: 6px;
    border: none;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s ease;
}

.gradient-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 8px rgba(0, 0, 0, 0.2);
}</code></pre>
</div>

## Features of the Code Blocks

1. **MacOS Window Style**: Notice the familiar red, yellow, and green buttons at the top-left corner
2. **Language Indicator**: Each code block shows the programming language in the title bar
3. **Smooth Scrolling**: For longer code blocks, a custom scrollbar appears
4. **Syntax Highlighting**: Code is properly formatted with syntax highlighting
5. **Modern Design**: Clean look with subtle shadows and rounded corners

Feel free to use these styled code blocks in your own posts by wrapping your code with:

<div class="highlight-wrap" data-lang="html">
<pre><code>&lt;div class="highlight-wrap" data-lang="language-name"&gt;
    &lt;pre&gt;&lt;code&gt;your code here&lt;/code&gt;&lt;/pre&gt;
&lt;/div&gt;</code></pre>
</div>

Replace `language-name` with the appropriate programming language (e.g., python, javascript, css, etc.).
