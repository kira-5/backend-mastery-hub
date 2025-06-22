import sys
import os
import asyncio
import markdown
import re
from pyppeteer import launch
from pygments import highlight
from pygments.lexers import Python3Lexer, get_lexer_by_name
from pygments.formatters import HtmlFormatter
from datetime import datetime


async def convert_to_pdf(input_file, output_file):
    if not os.path.exists(input_file):
        print(f"Error: Input file '{input_file}' does not exist")
        return

    # Ensure the output directory exists
    output_dir = os.path.dirname(output_file)
    if output_dir and not os.path.exists(output_dir):
        os.makedirs(output_dir)
        print(f"Created output directory: {output_dir}")

    file_extension = os.path.splitext(input_file)[1].lower()
    browser = None

    try:
        browser = await launch()
        page = await browser.newPage()

        # Get current date for footer
        current_date = datetime.now().strftime("%B %d, %Y")

        # Enhanced dark theme CSS with better typography and layout
        css = f"""
            <style>
                @page {{
                    margin: 0mm 0mm 0mm 0mm;
                    @bottom-center {{
                        content: "{current_date} • Page " counter(page) " of " counter(pages);
                        font-family: 'Inter', sans-serif;
                        font-size: 10px;
                        color: #666;
                    }}
                }}
                
                html, body {{
                    margin: 0;
                    padding: 0;
                    background: #161618;
                }}
                
                body {{
                    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                    margin: 10px 10px 10px 10px;
                    padding: 0;
                    color: #e0e0e0;
                    background: #161618;
                    line-height: 1.6;
                    font-size: 15px;
                    max-width: 800px;
                }}
                
                .document-container {{
                    background: #161618;
                    padding: 40px 50px;
                    margin: 0 auto;
                }}
                
                .content-wrapper {{
                    padding: 10px;  /* Added 10px margin around content */
                }}
                
                
                .title {{
                    color: #ffffff;
                    font-size: 32px;
                    font-weight: 700;
                    margin-bottom: 10px;
                    letter-spacing: -0.5px;
                }}
                
                .subtitle {{
                    color: #a0a0a0;
                    font-size: 16px;
                    font-weight: 400;
                    margin-top: 0;
                }}

                .text-orange-500 {{
                    background-color: #292220;
                    color: #F7AA7D;
                }}
                
                h1, h2, h3, h4 {{
                    color: #ffffff;
                    font-weight: 600;
                    margin-top: 1.5em;
                    margin-bottom: 0.8em;
                    letter-spacing: -0.3px;
                }}
                
                h1 {{
                    font-size: 26px;
                    border-bottom: 1px solid #2a2a2e;
                    padding-bottom: 10px;
                }}
                
                h2 {{
                    font-size: 22px;
                    border-bottom: 1px solid #2a2a2e;
                    padding-bottom: 8px;
                }}
                
                h3 {{
                    font-size: 19px;
                }}
                
                h4 {{
                    font-size: 16px;
                    color: #b0b0b0;
                }}
                
                p {{
                    font-size: 15px;
                    margin-bottom: 1.2em;
                    line-height: 1.7;
                }}
                
                ul, ol {{
                    margin-bottom: 1.5em;
                    padding-left: 25px;
                }}
                
                li {{
                    margin-bottom: 0.6em;
                    line-height: 1.6;
                }}
                
                a {{
                    color: #4da8ff;
                    text-decoration: none;
                    transition: color 0.2s;
                }}
                
                a:hover {{
                    color: #7fc1ff;
                    text-decoration: underline;
                }}
                
                blockquote {{
                    border-left: 4px solid #4da8ff;
                    background: #1e1e2e;
                    padding: 15px 20px;
                    margin: 1.5em 0;
                    font-style: italic;
                    color: #c0c0c0;
                }}
                
                blockquote p {{
                    margin: 0;
                }}
                
                pre, code {{
                    font-family: 'Fira Code', 'Consolas', monospace;
                }}
                
                pre {{
                    background: #111113;
                    padding: 18px;
                    border-radius: 6px;
                    overflow-x: auto;
                    font-size: 14px;
                    line-height: 1.5;
                    border-left: 4px solid #7f5af0;
                    margin: 1.5em 0;
                    page-break-inside: avoid;
                }}
                
                code:not(pre code) {{
                    background: #1e1e2e;
                    padding: 2px 6px;
                    border-radius: 3px;
                    font-size: 14px;
                    color: #e0e0e0;
                }}
                
                table {{
                    width: 100%;
                    border-collapse: collapse;
                    margin: 1.5em 0;
                    page-break-inside: avoid;
                }}
                
                th, td {{
                    padding: 12px 15px;
                    text-align: left;
                    border-bottom: 1px solid #2a2a2e;
                }}
                
                th {{
                    background: #1e1e2e;
                    font-weight: 600;
                    color: #ffffff;
                }}
                
                tr:hover td {{
                    background: #1e1e2e;
                }}
                
                img {{
                    max-width: 100%;
                    height: auto;
                    border-radius: 4px;
                    margin: 1.5em 0;
                }}
                
                .highlight {{
                    margin: 1.5em 0;
                    border-radius: 6px;
                    overflow: hidden;
                    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
                }}
                
                /* Pygments syntax highlighting styles - Enhanced */
                .highlight .c {{ color: #6a9955; font-style: italic; }} /* Comment */
                .highlight .k {{ color: #ff79c6; font-weight: bold; }} /* Keyword */
                .highlight .o {{ color: #ff79c6; font-weight: bold; }} /* Operator */
                .highlight .cm {{ color: #6a9955; font-style: italic; }} /* Comment.Multiline */
                .highlight .cp {{ color: #6a9955; font-weight: bold; }} /* Comment.Preproc */
                .highlight .c1 {{ color: #6a9955; font-style: italic; }} /* Comment.Single */
                .highlight .cs {{ color: #6a9955; font-weight: bold; font-style: italic; }} /* Comment.Special */
                .highlight .kc {{ color: #ff79c6; font-weight: bold; }} /* Keyword.Constant */
                .highlight .kd {{ color: #ff79c6; font-weight: bold; }} /* Keyword.Declaration */
                .highlight .kn {{ color: #ff79c6; font-weight: bold; }} /* Keyword.Namespace */
                .highlight .kp {{ color: #ff79c6; }} /* Keyword.Pseudo */
                .highlight .kr {{ color: #ff79c6; font-weight: bold; }} /* Keyword.Reserved */
                .highlight .kt {{ color: #8be9fd; }} /* Keyword.Type */
                .highlight .s {{ color: #f1fa8c; }} /* String */
                .highlight .sb {{ color: #f1fa8c; }} /* String.Backtick */
                .highlight .sc {{ color: #f1fa8c; }} /* String.Char */
                .highlight .sd {{ color: #f1fa8c; }} /* String.Doc */
                .highlight .s2 {{ color: #f1fa8c; }} /* String.Double */
                .highlight .se {{ color: #f1fa8c; }} /* String.Escape */
                .highlight .sh {{ color: #f1fa8c; }} /* String.Heredoc */
                .highlight .si {{ color: #f1fa8c; }} /* String.Interpol */
                .highlight .sx {{ color: #f1fa8c; }} /* String.Other */
                .highlight .sr {{ color: #f1fa8c; }} /* String.Regex */
                .highlight .s1 {{ color: #f1fa8c; }} /* String.Single */
                .highlight .ss {{ color: #f1fa8c; }} /* String.Symbol */
                .highlight .m {{ color: #bd93f9; }} /* Number */
                .highlight .mf {{ color: #bd93f9; }} /* Number.Float */
                .highlight .mh {{ color: #bd93f9; }} /* Number.Hex */
                .highlight .mi {{ color: #bd93f9; }} /* Number.Integer */
                .highlight .il {{ color: #bd93f9; }} /* Number.Integer.Long */
                .highlight .mo {{ color: #bd93f9; }} /* Number.Oct */
                .highlight .n {{ color: #f8f8f2; }} /* Name */
                .highlight .na {{ color: #50fa7b; }} /* Name.Attribute */
                .highlight .nb {{ color: #8be9fd; }} /* Name.Builtin */
                .highlight .nc {{ color: #8be9fd; }} /* Name.Class */
                .highlight .no {{ color: #bd93f9; }} /* Name.Constant */
                .highlight .nd {{ color: #50fa7b; }} /* Name.Decorator */
                .highlight .ni {{ color: #ff79c6; }} /* Name.Entity */
                .highlight .ne {{ color: #ff79c6; }} /* Name.Exception */
                .highlight .nf {{ color: #50fa7b; }} /* Name.Function */
                .highlight .nl {{ color: #8be9fd; }} /* Name.Label */
                .highlight .nn {{ color: #8be9fd; }} /* Name.Namespace */
                .highlight .nx {{ color: #50fa7b; }} /* Name.Other */
                .highlight .py {{ color: #8be9fd; }} /* Name.Property */
                .highlight .nt {{ color: #ff79c6; }} /* Name.Tag */
                .highlight .ow {{ color: #ff79c6; font-weight: bold; }} /* Operator.Word */
                .highlight .p {{ color: #f8f8f2; }} /* Punctuation */
                
                /* Callouts */
                .note {{
                    background: rgba(77, 168, 255, 0.1);
                    border-left: 4px solid #4da8ff;
                    padding: 15px;
                    margin: 1.5em 0;
                    border-radius: 0 4px 4px 0;
                }}
                
                .warning {{
                    background: rgba(255, 184, 108, 0.1);
                    border-left: 4px solid #ffb86c;
                    padding: 15px;
                    margin: 1.5em 0;
                    border-radius: 0 4px 4px 0;
                }}
                
                .important {{
                    background: rgba(255, 121, 198, 0.1);
                    border-left: 4px solid #ff79c6;
                    padding: 15px;
                    margin: 1.5em 0;
                    border-radius: 0 4px 4px 0;
                }}

                /* Style for text between single asterisks */
                span.highlight-asterisk {{
                    background-color: #292220;
                    color: #F7AA7D;
                    font-style: normal;
                    padding: 0.1em 0.3em;
                    border-radius: 3px;
                }}

                .custom-highlight {{
                    background-color: #292220;
                    color: #F7AA7D;
                    padding: 0.1em 0.3em;
                    border-radius: 3px;
                    font-style: normal;
                    display: inline;
                }}

                /* Style for text between backticks */
                code {{
                    background-color: #292220;
                    color: #F7AA7D;
                    padding: 0.1em 0.3em;
                    border-radius: 3px;
                    font-family: 'Fira Code', monospace;
                }}

                
                /* Print-specific styles */
                @media print {{
                    body {{
                        font-size: 13pt;
                        line-height: 1.5;
                        color-adjust: exact;
                        -webkit-print-color-adjust: exact;
                    }}
                    
                    .document-container {{
                        padding: 0;
                    }}
                    
                    .content-wrapper {{
                        padding: 10px;  /* Maintain 10px margin in print */
                    }}
                    
                    h1, h2, h3, h4 {{
                        page-break-after: avoid;
                    }}
                    
                    pre, code, blockquote, table, img {{
                        page-break-inside: avoid;
                    }}
                    
                    ul, ol {{
                        page-break-inside: avoid;
                    }}
                    
                    p {{
                        orphans: 3;
                        widows: 3;
                    }}
                }}
            </style>
        """

        def render_code_block(code, lang="python"):
            try:
                lexer = get_lexer_by_name(lang, stripall=True)
            except:
                lexer = Python3Lexer()
            formatter = HtmlFormatter(style="material")
            return highlight(code, lexer, formatter)

        if file_extension == ".html":
            # Read HTML content and wrap in styled container
            with open(input_file, "r", encoding="utf-8") as f:
                html_content = f.read()

            # Process <pre><code> blocks with Pygments

            def replace_code_blocks(match):
                code = match.group(1)
                return render_code_block(code)

            html_content = re.sub(
                r"<pre><code>(.*?)</code></pre>",
                replace_code_blocks,
                html_content,
                flags=re.DOTALL,
            )

            styled_html = f"""
            <html>
            <head>
                {css}
                <link href="https://fonts.googleapis.com/css2?family=Fira+Code:wght@400;500&family=Inter:wght@400;500;600&display=swap" rel="stylesheet">
            </head>
            <body>
                <div class="document-container">
                    <div class="content-wrapper">
                        {html_content}
                    </div>
                </div>
            </body>
            </html>
            """
            await page.setContent(styled_html)
        elif file_extension == ".md":
            import re
            # Convert Markdown to HTML and apply styling
            with open(input_file, "r", encoding="utf-8") as f:
                md_content = f.read()


            # In the markdown processing section (around line 150), modify the extensions:
            extensions = [
                "fenced_code",
                "codehilite",
                "tables",
                "footnotes",
                "toc",
                "nl2br",  # This extension converts newlines to <br> tags
                "md_in_html"  # Allows HTML in markdown
            ]

            # And add this function to preserve your line breaks:
            def preprocess_markdown(content):
                # Ensure each product starts on a new line
                content = content.replace("* **", "\n* **")
                return content

            # Then modify where the markdown is processed:
            md_content = preprocess_markdown(md_content)

            # def preprocess_special_formatting(content):
            #     # Process in two passes to handle all cases perfectly
            #     lines = content.split('\n')
            #     processed_lines = []
                
            #     for line in lines:
            #         # Skip processing for bullet points (lines starting with *)
            #         if line.strip().startswith('* '):
            #             processed_lines.append(line)
            #             continue
                        
            #         # Process highlighting only in non-bullet lines
            #         line = re.sub(
            #             r'(?<!\*)\*([^*\n]+)\*(?!\*)',  # Match single *text* but not **text**
            #             r'<span class="highlight-asterisk">\1</span>',
            #             line
            #         )
            #         processed_lines.append(line)
                
            #     return '\n'.join(processed_lines)

            def preprocess_special_formatting(content):
                """
                Convert @@highlight@@ to orange-styled spans
                while preserving all Markdown formatting
                """
                return re.sub(
                    r'@@([^@]+)@@',  # Match @@text@@
                    r'<span class="custom-highlight">\1</span>',
                    content
                )

            # # Use it before markdown conversion:
            md_content = preprocess_special_formatting(md_content)
            
            html_content = markdown.markdown(
                md_content,
                extensions=extensions,
                extension_configs={
                    "codehilite": {
                        "use_pygments": True,
                        "noclasses": True,
                        "pygments_style": "material",
                    }
                },
            )

            # Process code blocks that might have been missed
            import re

            def process_code_blocks(html):
                def replacer(match):
                    lang = match.group(1) or "python"
                    code = match.group(2)
                    return render_code_block(code, lang)

                # Handle both ```lang and <pre><code> formats
                html = re.sub(
                    r'<pre><code class="language-(.*?)">(.*?)</code></pre>',
                    replacer,
                    html,
                    flags=re.DOTALL,
                )
                return re.sub(
                    r"<pre><code>(.*?)</code></pre>",
                    lambda m: replacer(("python", m.group(1))),
                    html,
                    flags=re.DOTALL,
                )

            html_content = process_code_blocks(html_content)

            styled_html = f"""
            <html>
            <head>
                {css}
                <link href="https://fonts.googleapis.com/css2?family=Fira+Code:wght@400;500&family=Inter:wght@400;500;600&display=swap" rel="stylesheet">
            </head>
            <body>
                <div class="document-container">
                    <div class="content-wrapper">
                        <div class="header">
                            <p class="subtitle">{current_date}</p>
                        </div>
                        {html_content}
                    </div>
                </div>
            </body>
            </html>
            """
            await page.setContent(styled_html)
        else:
            print("Error: Unsupported file type. Use .html or .md files")
            return

        # Generate PDF with custom options
        await page.pdf(
            {
                "path": output_file,
                "format": "A4",
                "margin": {
                    "top": "0mm",
                    "bottom": "0mm",
                    "left": "0mm",
                    "right": "0mm",
                },
                "printBackground": True,
                "preferCSSPageSize": True,
                "headerTemplate": "<div></div>",
                "footerTemplate": "<div style='font-size: 10px; color: #666; width: 100%; text-align: center; font-family: \"Inter\";'></div>",
            }
        )
        print(f"PDF successfully created at: {output_file}")

    except Exception as e:
        print(f"Error during conversion: {str(e)}")
    finally:
        if browser:
            await browser.close()


def main():
    if len(sys.argv) != 3:
        print("Usage: python convert_to_pdf_pyppeteer.py <input_file> <output_file>")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    # Append .pdf extension if not present
    if not output_file.lower().endswith(".pdf"):
        output_file += ".pdf"

    asyncio.get_event_loop().run_until_complete(convert_to_pdf(input_file, output_file))


if __name__ == "__main__":
    main()