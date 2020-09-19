</a></div></form></nav><section id="main" class="content"><h1 class='fqn'><span class='out-of-band'><span id='render-detail'><a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">[<span class='inner'>&#x2212;</span>]</a></span><a class='srclink' href='../src/rust_pool/lib.rs.html#2-95' title='goto source code'>[src]</a></span><span class='in-band'>Crate <a class="mod" href=''>rust_pool</a></span></h1><div class='docblock'><h1 id="a-thread-pool-written-with-rust" class="section-header"><a href="#a-thread-pool-written-with-rust">A thread pool written with Rust</a></h1>
<ul>
<li>Create a thread pool to run your code</li>
<li>Use log crate to print log</li>
<li>[TODO] Support different scheduling algorithm</li>
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
<li>log = &quot;0.4.11&quot;</li>
</ul>
<h2 id="todo" class="section-header"><a href="#todo">TODO</a></h2>
<ul>
<li>Add algorithm module for supporting different scheduling algorithm</li>
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
</td></tr></table></section><section id="search" class="content hidden"></section><section class="footer"></section></body></html>  

## Document
Run `cargo doc --open` to read the docment.  
