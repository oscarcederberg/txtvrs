# SVT Text

## URL
To retrieve page `<n>` from SVT Text, the page number is appended to one of the following URLs:\
`https://www.svt.se/text-tv/` for TV\
`https://www.svt.se/text-tv/webb/` for Webb

## HTML Layout
TV and Webb have different layouts, one might be more preferable to scrape over the other. Classes seem to be constant in-between days.

### TV
Each screen is contained within one single div-element. Multiple screens
```html
...

<!-- first screen -->
<div class="Content_screenreaderOnly__Gwyfj">
    <!-- screen content, with header and footer included -->
    <!-- several lines seperated by newlines -->
</div>

...

<!-- next screen -->
<div class="Content_screenreaderOnly__Gwyfj">
    ...
</div>

...
```

### Webb
Each screen consists of a header, content, and footer. If multiple screens, they are all contained within one main-element in a div-element. Page nubmers in content and footer are parsed at link-elements.

```html
...

<!-- first screen -->
<div class="TextContent_textWrapper__3s3Q0">
    <div class="TextContent_textContent__1Edsl TextContent_header__2agih">
        <!-- screen header -->
        <!-- single line -->
    </div>

    <div class="TextContent_textContent__1Edsl">
        <!-- screen content -->
        <!-- several lines seperated by newlines -->

        <a>
            <!-- page numbers are replaced with link tags -->
        </a>

    </div>

    <div class="TextContent_textContent__1Edsl TextContent_footer__28lhb">
        <!-- screen footer -->
        <!-- single line -->

        <a>
            <!-- can also contain link tags-->
        </a>

    </div>
</div>

...

<!-- next screen -->
<div class="TextContent_textWrapper__3s3Q0">
    ...
</div>

...
```

## Symbols
Less-than, and Greater-than-signs are replaced with `&lt;` and `&gt;` in scraped text. These are called "Named Character References", and should be replaced with their symbol-equivalent. **Are there other named character references that needs to be replaced after parsing?**
