# SVT Text TV

## HTML Layout
Each screen consists of a header, content, and footer. If multiple screens, they are all contained within one main-element in a div-element.

```html
...

<div class="ContentWrapper_content__lZ54y">
    <main aria-live="polite">
        <h1>
            <!-- SKIP: page number -->
        </h1>
        
        <div>
            <!-- first screen -->
            <div class="TextContent_textWrapper__3s3Q0">
                <div class="TextContent_textContent__1Edsl TextContent_header__2agih">
                    <!-- screen header -->
                    <!-- single line -->
                </div>

                <div class="TextContent_textContent__1Edsl">
                    <!-- screen content -->
                    <!-- several lines seperated by newlines -->
                </div>

                <div class="TextContent_textContent__1Edsl TextContent_footer__28lhb">
                    <!-- screen footer -->
                    <!-- single line -->
                </div>
            </div>
        </div>

        <div>
            <!-- next screen -->
            <div class="TextContent_textWrapper__3s3Q0">
                ...
            </div>
        </div>

        ...

    </main>
</div>

...
```