rusthtml_macro::rusthtml_view_macro! {
    @name "home_index"
@{
    // Layout = "_Layout_Home_Index";
    ViewData.insert("Title", "Home");
}

<div class="answercell post-layout--right">
    
    <div class="s-prose js-post-body" itemprop="text">
<p><em><a href="https://blog.rust-lang.org/2017/04/27/Rust-1.17.html" rel="noreferrer">2017</a> stabilization update (in 2020).</em></p>

<p>In Rust 1.17 and forward, you can use <a href="https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq" rel="noreferrer"><code>Rc::ptr_eq</code></a>. It does the same as <a href="https://doc.rust-lang.org/stable/std/ptr/fn.eq.html" rel="noreferrer"><code>ptr::eq</code></a>, without the need of converting the <code>Rc</code> to a reference or pointer.</p>

<h3>Reference Equality</h3>

<p>As the other answers mention <code>Rc::ptr_eq</code> (and <code>ptr::eq</code>) checks for reference equality, i.e. whether the two references "point" to the same address.</p>

<pre class="lang-rust s-code-block"><code class="hljs language-rust"><span class="hljs-keyword">let</span> <span class="hljs-variable">five</span> = Rc::<span class="hljs-title function_ invoke__">new</span>(<span class="hljs-number">5</span>);
<span class="hljs-keyword">let</span> <span class="hljs-variable">same_five</span> = Rc::<span class="hljs-title function_ invoke__">clone</span>(&amp;five);
<span class="hljs-keyword">let</span> <span class="hljs-variable">other_five</span> = Rc::<span class="hljs-title function_ invoke__">new</span>(<span class="hljs-number">5</span>);

<span class="hljs-comment">// five and same_five reference the same value in memory</span>
<span class="hljs-built_in">assert!</span>(Rc::<span class="hljs-title function_ invoke__">ptr_eq</span>(&amp;five, &amp;same_five));

<span class="hljs-comment">// five and other_five does not reference the same value in memory</span>
<span class="hljs-built_in">assert!</span>(!Rc::<span class="hljs-title function_ invoke__">ptr_eq</span>(&amp;five, &amp;other_five));
</code></pre>

<p><em>The example is from the Rust <code>Rc::ptr_eq</code> docs.</em></p>

<h3>Value Equality</h3>

<p><code>Rc</code> implements <code>PartialEq</code>, so simply use <code>==</code> as always, to perform value equality, i.e. whether the values are equal, irrelevant of whether they reference the same address in memory.</p>

<pre class="lang-rust s-code-block"><code class="hljs language-rust"><span class="hljs-keyword">use</span> std::rc::Rc;

<span class="hljs-keyword">let</span> <span class="hljs-variable">five</span> = Rc::<span class="hljs-title function_ invoke__">new</span>(<span class="hljs-number">5</span>);
<span class="hljs-keyword">let</span> <span class="hljs-variable">other_five</span> = Rc::<span class="hljs-title function_ invoke__">new</span>(<span class="hljs-number">5</span>);

<span class="hljs-keyword">let</span> <span class="hljs-variable">ten</span> = Rc::<span class="hljs-title function_ invoke__">new</span>(<span class="hljs-number">10</span>);

<span class="hljs-built_in">assert!</span>(five == other_five);

<span class="hljs-built_in">assert!</span>(ten != five);
<span class="hljs-built_in">assert!</span>(ten != other_five);
</code></pre>
    </div>
    <div class="mt24">
        <div class="d-flex fw-wrap ai-start jc-end gs8 gsy">
            <time itemprop="dateCreated" datetime="2020-02-15T18:01:28"></time>
            <div class="flex--item mr16" style="flex: 1 1 100px;">
                


<div class="js-post-menu pt2" data-post-id="60241585" data-post-type-id="2">

    <div class="d-flex gs8 s-anchors s-anchors__muted fw-wrap">

            <div class="flex--item">
                <a href="/a/60241585" rel="nofollow" itemprop="url" class="js-share-link js-gps-track" title="Short permalink to this answer" data-gps-track="post.click({ item: 2, priv: 0, post_type: 2 })" data-controller="se-share-sheet s-popover" data-se-share-sheet-title="Share a link to this answer" data-se-share-sheet-subtitle="" data-se-share-sheet-post-type="answer" data-se-share-sheet-social="facebook twitter devto" data-se-share-sheet-location="2" data-se-share-sheet-license-url="https%3a%2f%2fcreativecommons.org%2flicenses%2fby-sa%2f4.0%2f" data-se-share-sheet-license-name="CC BY-SA 4.0" data-s-popover-placement="bottom-start" aria-controls="se-share-sheet-1" data-action=" s-popover#toggle se-share-sheet#preventNavigation s-popover:show->se-share-sheet#willShow s-popover:shown->se-share-sheet#didShow" aria-expanded="false">Share</a><div class="s-popover z-dropdown s-anchors s-anchors__default" style="width: unset; max-width: 28em;" id="se-share-sheet-1"><div class="s-popover--arrow"></div><div><label class="js-title fw-bold" for="share-sheet-input-se-share-sheet-1">Share a link to this answer</label> <span class="js-subtitle"></span></div><div class="my8"><input type="text" id="share-sheet-input-se-share-sheet-1" class="js-input s-input wmn3 sm:wmn-initial bc-black-200 bg-white fc-dark" readonly=""></div><div class="d-flex jc-space-between ai-center mbn4"><button class="js-copy-link-btn s-btn s-btn__link js-gps-track" data-gps-track="">Copy link</button><a href="https://creativecommons.org/licenses/by-sa/4.0/" rel="license" class="js-license s-block-link w-auto" target="_blank" title="The current license for this post: CC BY-SA 4.0">CC BY-SA 4.0</a><div class="js-social-container d-none"></div></div></div>
            </div>


                    <div class="flex--item">
                        <a href="/posts/60241585/edit" class="js-suggest-edit-post js-gps-track" data-gps-track="post.click({ item: 6, priv: 0, post_type: 2 })" title="">Improve this answer</a>
                    </div>

            <div class="flex--item">
                <button type="button" id="btnFollowPost-60241585" class="s-btn s-btn__link js-follow-post js-follow-answer js-gps-track" data-gps-track="post.click({ item: 14, priv: 0, post_type: 2 })" data-controller="s-tooltip " data-s-tooltip-placement="bottom" data-s-popover-placement="bottom" aria-controls="" aria-describedby="--stacks-s-tooltip-2ghb18pm">
                    Follow
                </button><div id="--stacks-s-tooltip-2ghb18pm" class="s-popover s-popover__tooltip" role="tooltip">Follow this answer to receive notifications<div class="s-popover--arrow"></div></div>
            </div>






    </div>
    <div class="js-menu-popup-container"></div>
</div>
            </div>


            <div class="post-signature flex--item fl0">
                <div class="user-info user-hover">
    <div class="user-action-time">
        answered <span title="2020-02-15 18:01:28Z" class="relativetime">Feb 15, 2020 at 18:01</span>
    </div>
    <div class="user-gravatar32">
    </div>
    <div class="user-details" itemprop="author" itemscope="" itemtype="http://schema.org/Person">
        <a href="/users/2470818/vallentin">vallentin</a><span class="d-none" itemprop="name">vallentin</span>
        <div class="-flair">
            <span class="reputation-score" title="reputation score 22,772" dir="ltr">22.8k</span><span title="6 gold badges" aria-hidden="true"><span class="badge1"></span><span class="badgecount">6</span></span><span class="v-visible-sr">6 gold badges</span><span title="58 silver badges" aria-hidden="true"><span class="badge2"></span><span class="badgecount">58</span></span><span class="v-visible-sr">58 silver badges</span><span title="80 bronze badges" aria-hidden="true"><span class="badge3"></span><span class="badgecount">80</span></span><span class="v-visible-sr">80 bronze badges</span>
        </div>
    </div>
</div>


            </div>
        </div>
        
    
    </div>
    
</div>
}