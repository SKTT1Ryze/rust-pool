<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="API documentation for the Rust `rust_pool` crate."><meta name="keywords" content="rust, rustlang, rust-lang, rust_pool"><title>rust_pool - Rust</title><link rel="stylesheet" type="text/css" href="../normalize.css"><link rel="stylesheet" type="text/css" href="../rustdoc.css" id="mainThemeStyle"><link rel="stylesheet" type="text/css" href="../dark.css"><link rel="stylesheet" type="text/css" href="../light.css" id="themeStyle"><script src="../storage.js"></script><noscript><link rel="stylesheet" href="../noscript.css"></noscript><link rel="shortcut icon" href="../favicon.ico"><style type="text/css">#crate-search{background-image:url("../down-arrow.svg");}</style></head><body class="rustdoc mod"><!--[if lte IE 8]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="sidebar-menu">&#9776;</div><a href='../rust_pool/index.html'><div class='logo-container'><img src='../rust-logo.png' alt='logo'></div></a><p class='location'>Crate rust_pool</p><div class="sidebar-elems"><a id='all-types' href='all.html'><p>See all rust_pool's items</p></a><div class="block items"><ul><li><a href="#reexports">Re-exports</a></li><li><a href="#modules">Modules</a></li></ul></div><p class='location'></p><script>window.sidebarCurrent = {name: 'rust_pool', ty: 'mod', relpath: '../'};</script></div></nav><div class="theme-picker"><button id="theme-picker" aria-label="Pick another theme!"><img src="../brush.svg" width="18" alt="Pick another theme!"></button><div id="theme-choices"></div></div><script src="../theme.js"></script><nav class="sub"><form class="search-form"><div class="search-container"><div><select id="crate-search"><option value="All crates">All crates</option></select><input class="search-input" name="search" disabled autocomplete="off" spellcheck="false" placeholder="Click or press ‘S’ to search, ‘?’ for more options…" type="search"></div><a id="settings-menu" href="../settings.html"><img src="../wheel.svg" width="18" alt="Change settings"></a></div></form></nav><section id="main" class="content"><h1 class='fqn'><span class='out-of-band'><span id='render-detail'><a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">[<span class='inner'>&#x2212;</span>]</a></span><a class='srclink' href='../src/rust_pool/lib.rs.html#2-95' title='goto source code'>[src]</a></span><span class='in-band'>Crate <a class="mod" href=''>rust_pool</a></span></h1><div class='docblock'><h1 id="a-thread-pool-written-with-rust" class="section-header"><a href="#a-thread-pool-written-with-rust">A thread pool written with Rust</a></h1>
<ul>
<li>Create a thread pool to run your code</li>
<li>[TODO] Support different scheduling algorithm</li>
<li>[TODO] Use log crate to print log</li>
</ul>
<h2 id="example" class="section-header"><a href="#example">Example</a></h2>
<pre><code class="language-Rust">use rust_pool::ThreadPool;
 
// Create new ThreadPool
let mut new_pool = match ThreadPool::new(5) {
    Ok(pool) =&gt; pool,
    Err(_) =&gt; {
        panic!(&quot;Create new ThreadPool error&quot;);
    },
};
 
// Add your code to the pool
for i in 0..8 {
    match new_pool.push_task(move ||
    {
           println!(&quot;Task {} is running...&quot;, i);
    }) {
        Err(err) =&gt; {
            panic!(err);
        },
        _ =&gt; {},
    }
}
 
// Execute the pool
match new_pool.execute() {
    Ok(_) =&gt; {},
    Err(err) =&gt; panic!(err),
}
</code></pre>
<p>See more details in example/src/main.rs</p>
<h2 id="dependencies" class="section-header"><a href="#dependencies">Dependencies</a></h2>
<ul>
<li>random-number = &quot;0.1.3&quot;</li>
</ul>
<h2 id="todo" class="section-header"><a href="#todo">TODO</a></h2>
<ul>
<li>Add algorithm module for supporting different scheduling algorithm</li>
<li>Use log crate from crate.io to manage log</li>
</ul>
<h2 id="welcome-to-contribute" class="section-header"><a href="#welcome-to-contribute"><strong>Welcome to contribute!</strong></a></h2></div><h2 id='reexports' class='section-header'><a href="#reexports">Re-exports</a></h2>
<table><tr><td><code>pub use <a class="mod" href="../rust_pool/pool/index.html" title="mod rust_pool::pool">pool</a>::*;</code></td></tr></table><h2 id='modules' class='section-header'><a href="#modules">Modules</a></h2>
<table><tr class='module-item'><td><a class="mod" href="config/index.html" title='rust_pool::config mod'>config</a></td><td class='docblock-short'><p>Config of Rust Thread Pool</p>
</td></tr><tr class='module-item'><td><a class="mod" href="cookie_generator/index.html" title='rust_pool::cookie_generator mod'>cookie_generator</a></td><td class='docblock-short'><p>Cookie Generator to generate a unique cookie</p>
</td></tr><tr class='module-item'><td><a class="mod" href="error/index.html" title='rust_pool::error mod'>error</a></td><td class='docblock-short'><p>Error Handler for Rust Thread Pool</p>
</td></tr><tr class='module-item'><td><a class="mod" href="pool/index.html" title='rust_pool::pool mod'>pool</a></td><td class='docblock-short'><p>Thread Pool Implement</p>
</td></tr><tr class='module-item'><td><a class="mod" href="signal/index.html" title='rust_pool::signal mod'>signal</a></td><td class='docblock-short'><p>Signal Implement</p>
</td></tr><tr class='module-item'><td><a class="mod" href="task/index.html" title='rust_pool::task mod'>task</a></td><td class='docblock-short'><p>Task Implement</p>
</td></tr><tr class='module-item'><td><a class="mod" href="worker/index.html" title='rust_pool::worker mod'>worker</a></td><td class='docblock-short'><p>Worker Implement</p>
</td></tr></table></section><section id="search" class="content hidden"></section><section class="footer"></section><script>window.rootPath = "../";window.currentCrate = "rust_pool";</script><script src="../aliases.js"></script><script src="../main.js"></script><script defer src="../search-index.js"></script></body></html>