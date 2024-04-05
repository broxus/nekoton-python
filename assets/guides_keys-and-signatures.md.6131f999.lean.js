import{_ as a,v as s,b as e,R as n}from"./chunks/framework.f247d2cd.js";const D=JSON.parse('{"title":"Keys & Signatures","description":"","frontmatter":{"outline":[2,4]},"headers":[],"relativePath":"guides/keys-and-signatures.md","filePath":"src/pages/guides/keys-and-signatures.md"}'),o={name:"guides/keys-and-signatures.md"},l=n(`<h1 id="keys-signatures" tabindex="-1">Keys &amp; Signatures <a class="header-anchor" href="#keys-signatures" aria-label="Permalink to &quot;Keys &amp; Signatures&quot;">​</a></h1><p>Nekoton-Python provides a comprehensive set of tools for managing cryptographic keys and signatures, which are essential for interacting with the blockchain. This section will guide you through the usage of these tools.</p><h2 id="keypair" tabindex="-1">KeyPair <a class="header-anchor" href="#keypair" aria-label="Permalink to &quot;KeyPair&quot;">​</a></h2><p>A <code>KeyPair</code> in cryptographic systems pertains to a duo of a private (or secret) key and a public key. It plays a pivotal role in both encrypting and decrypting data, as well as in digital signatures. Specifically, for Ed25519, a widely-accepted and modern digital signature system, the <code>KeyPair</code> can be generated from a seed or directly from a secret key.</p><p>For Ed25519, the <code>KeyPair</code> can also be generated in two primary ways:</p><ul><li>Randomly, utilizing the <code>KeyPair.generate()</code> method.</li><li>Derivatively, from a seed using the <code>derive()</code> method, which is a feature of the respective seed class.</li></ul><p>In the context of Ed25519, the secret key&#39;s size is crucial, with the standard mandating a precise length of 32 bytes. This specification ensures the security and consistency of the cryptographic operations facilitated by the key pair.</p><h3 id="generating-a-keypair" tabindex="-1">Generating a KeyPair <a class="header-anchor" href="#generating-a-keypair" aria-label="Permalink to &quot;Generating a KeyPair&quot;">​</a></h3><p>A <code>KeyPair</code> can be generated randomly using the <code>KeyPair.generate()</code> method:</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">keypair </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> nt</span><span style="color:#89DDFF;">.</span><span style="color:#F07178;">KeyPair</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">generate</span><span style="color:#89DDFF;">()</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#C792EA;">f</span><span style="color:#C3E88D;">&quot;Public key: </span><span style="color:#F78C6C;">{</span><span style="color:#82AAFF;">keypair</span><span style="color:#89DDFF;">.</span><span style="color:#F07178;">public_key</span><span style="color:#F78C6C;">}</span><span style="color:#C3E88D;">&quot;</span><span style="color:#89DDFF;">)</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#C792EA;">f</span><span style="color:#C3E88D;">&quot;Secret key: </span><span style="color:#F78C6C;">{</span><span style="color:#82AAFF;">keypair</span><span style="color:#89DDFF;">.</span><span style="color:#F07178;">secret_key</span><span style="color:#F78C6C;">}</span><span style="color:#C3E88D;">&quot;</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result" tabindex="-1">Result <a class="header-anchor" href="#result" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">Public key</span><span style="color:#89DDFF;">:</span><span style="color:#A6ACCD;"> 6b6243e2fa88025d5f2fee051bd8b62ff3da730136c67bbb14033b72c60fb762</span></span>
<span class="line"></span>
<span class="line"><span style="color:#A6ACCD;">Secret key</span><span style="color:#89DDFF;">:</span><span style="color:#A6ACCD;"> e7f25e4cf517661a51081779dd7218d42c7db4bb85b18406e337a2ddb4359599</span></span></code></pre></div><h2 id="seeds" tabindex="-1">Seeds <a class="header-anchor" href="#seeds" aria-label="Permalink to &quot;Seeds&quot;">​</a></h2><p>A seed is a piece of data that can be used to generate deterministic keys. In Nekoton-Python, two types of seeds are supported: <code>Bip39</code> and <code>Legacy</code>.</p><h3 id="bip39-seeds" tabindex="-1">Bip39 Seeds <a class="header-anchor" href="#bip39-seeds" aria-label="Permalink to &quot;Bip39 Seeds&quot;">​</a></h3><p>Bip39 Seeds are 12-word seeds that can be used to derive a <code>KeyPair</code> following the BIP39 standard. They can be generated using the <code>Bip39Seed.generate()</code> method, and a <code>KeyPair</code> can be derived from a Bip39 seed using the <code>Bip39Seed.derive()</code> method.</p><p>Example of generating and using a Bip39 seed:</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">bip39_seed </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> nt</span><span style="color:#89DDFF;">.</span><span style="color:#F07178;">Bip39Seed</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">generate</span><span style="color:#89DDFF;">()</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">bip39_seed</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-1" tabindex="-1">Result <a class="header-anchor" href="#result-1" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">cricket prize gain hidden dragon fossil repeat blue dream already shaft</span></span>
<span class="line"><span style="color:#A6ACCD;">exclude</span></span></code></pre></div><p>Deriving a <code>KeyPair</code> from a Bip39 Seed:</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">keypair </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> bip39_seed</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">derive</span><span style="color:#89DDFF;">()</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">keypair</span><span style="color:#89DDFF;">.</span><span style="color:#F07178;">public_key</span><span style="color:#89DDFF;">,</span><span style="color:#82AAFF;"> keypair</span><span style="color:#89DDFF;">.</span><span style="color:#F07178;">secret_key</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-2" tabindex="-1">Result <a class="header-anchor" href="#result-2" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">3247bd75e041a03c9029297b89bb40132744df380596fb4bda4591f1dd9313d7</span></span>
<span class="line"></span>
<span class="line"><span style="color:#C792EA;">b</span><span style="color:#89DDFF;">&quot;</span><span style="color:#A6ACCD;">\\xbd\\xac\\xcd\\x0e\\x89</span><span style="color:#C3E88D;">c</span><span style="color:#A6ACCD;">\\xab\\xae</span><span style="color:#C3E88D;">Z:!</span><span style="color:#A6ACCD;">\\xf7\\xe6\\xd9\\x9f\\xe5</span><span style="color:#C3E88D;">a=</span><span style="color:#A6ACCD;">\\x06</span><span style="color:#C3E88D;">z0-n&#39;</span><span style="color:#A6ACCD;">\\xc5\\xd0\\xed\\x8e\\xee\\xb5\\x93\\x94</span><span style="color:#89DDFF;">&quot;</span></span></code></pre></div><h3 id="legacy-seeds" tabindex="-1">Legacy Seeds <a class="header-anchor" href="#legacy-seeds" aria-label="Permalink to &quot;Legacy Seeds&quot;">​</a></h3><p>Legacy Seeds are 24-word seeds that can be used to derive a <code>KeyPair</code>. They can be generated using the <code>LegacySeed.generate()</code> method, and a <code>KeyPair</code> can be derived from a legacy seed using the <code>LegacySeed.derive()</code> method.</p><p>Example of generating and using a Legacy seed:</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">legacy_seed </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> nt</span><span style="color:#89DDFF;">.</span><span style="color:#F07178;">LegacySeed</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">generate</span><span style="color:#89DDFF;">()</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">legacy_seed</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-3" tabindex="-1">Result <a class="header-anchor" href="#result-3" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">away october another abuse bridge woman local lottery ostrich genuine</span></span>
<span class="line"><span style="color:#A6ACCD;">obvious minor brand wall upper column response bus nose lonely question</span></span>
<span class="line"><span style="color:#A6ACCD;">useful grocery unable</span></span></code></pre></div><p>Deriving a <code>KeyPair</code> from a Legacy Seed:</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">keypair </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> legacy_seed</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">derive</span><span style="color:#89DDFF;">()</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">keypair</span><span style="color:#89DDFF;">.</span><span style="color:#F07178;">public_key</span><span style="color:#89DDFF;">,</span><span style="color:#82AAFF;"> keypair</span><span style="color:#89DDFF;">.</span><span style="color:#F07178;">secret_key</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-4" tabindex="-1">Result <a class="header-anchor" href="#result-4" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">7ec3e8c544c021808be23b10829440ea45175e76b9d5ede46a7e8d59085c3228</span></span>
<span class="line"></span>
<span class="line"><span style="color:#C792EA;">b</span><span style="color:#89DDFF;">&#39;</span><span style="color:#C3E88D;">{I</span><span style="color:#A6ACCD;">\\xd1\\x02</span><span style="color:#C3E88D;">*&gt;</span><span style="color:#A6ACCD;">\\x16\\x1c\\xa7</span><span style="color:#C3E88D;">Z</span><span style="color:#A6ACCD;">\\x97\\x01</span><span style="color:#C3E88D;">&lt;</span><span style="color:#A6ACCD;">\\x1a\\x07\\x0b\\xcc\\xb0\\x1d\\x18</span><span style="color:#C3E88D;">i</span><span style="color:#A6ACCD;">\\xba</span><span style="color:#C3E88D;">r</span><span style="color:#A6ACCD;">\\xe7</span><span style="color:#C3E88D;">aE</span><span style="color:#A6ACCD;">\\x1e\\x9d\\xb3\\xc3\\xd8\\xaf</span><span style="color:#89DDFF;">&#39;</span></span></code></pre></div><h3 id="derivation-path" tabindex="-1">Derivation Path <a class="header-anchor" href="#derivation-path" aria-label="Permalink to &quot;Derivation Path&quot;">​</a></h3><p>In the context of BIP39, a derivation path is used to derive different keys from the same seed phrase. It provides a hierarchical structure for generating and organizing keys. With Nekoton-Python, you can retrieve the default derivation path for a specified account number using the <code>path_for_account</code> method.</p><p>Here&#39;s how you can get the derivation path for a specified account number using a Bip39 seed:</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">path </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> bip39_seed</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">path_for_account</span><span style="color:#89DDFF;">(</span><span style="color:#F78C6C;">0</span><span style="color:#89DDFF;">)</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">path</span><span style="color:#89DDFF;">)</span><span style="color:#A6ACCD;"> </span><span style="color:#676E95;font-style:italic;"># m/44&#39;/396&#39;/0&#39;/0/1</span></span></code></pre></div><h2 id="public-key-operations" tabindex="-1">Public Key Operations <a class="header-anchor" href="#public-key-operations" aria-label="Permalink to &quot;Public Key Operations&quot;">​</a></h2><p>The <code>nekoton</code> library provides various methods to work with public keys. You can initialize, encode, and convert a <code>PublicKey</code> using the provided methods.</p><h3 id="initialization" tabindex="-1">Initialization <a class="header-anchor" href="#initialization" aria-label="Permalink to &quot;Initialization&quot;">​</a></h3><p>Public keys can be initialized from different formats:</p><h4 id="from-integer" tabindex="-1">From Integer <a class="header-anchor" href="#from-integer" aria-label="Permalink to &quot;From Integer&quot;">​</a></h4><p>You can initialize a public key from an integer using the <code>PublicKey.from_int()</code> method.</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">public_key </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> nt</span><span style="color:#89DDFF;">.</span><span style="color:#F07178;">PublicKey</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">from_int</span><span style="color:#89DDFF;">(</span><span style="color:#F78C6C;">63837483679490186262641015239053288982995430350508212654141177365814141551489</span><span style="color:#89DDFF;">)</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">public_key</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-5" tabindex="-1">Result <a class="header-anchor" href="#result-5" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381</span></span></code></pre></div><h4 id="from-bytes" tabindex="-1">From Bytes <a class="header-anchor" href="#from-bytes" aria-label="Permalink to &quot;From Bytes&quot;">​</a></h4><p>You can initialize a public key from bytes using the <code>PublicKey.from_bytes()</code> method.</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">public_key </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> PublicKey</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">from_bytes</span><span style="color:#89DDFF;">(</span><span style="color:#C792EA;">b</span><span style="color:#89DDFF;">&#39;</span><span style="color:#A6ACCD;">\\x8d</span><span style="color:#C3E88D;">&quot;</span><span style="color:#A6ACCD;">\\xbc</span><span style="color:#C3E88D;">?</span><span style="color:#A6ACCD;">\\x15</span><span style="color:#C3E88D;">o@</span><span style="color:#A6ACCD;">\\t</span><span style="color:#C3E88D;">44</span><span style="color:#A6ACCD;">\\x06\\x07\\xe3</span><span style="color:#C3E88D;">r</span><span style="color:#A6ACCD;">\\x07</span><span style="color:#C3E88D;">k</span><span style="color:#A6ACCD;">\\x9a\\x02</span><span style="color:#C3E88D;">&lt;n</span><span style="color:#A6ACCD;">\\xc5\\x91</span><span style="color:#C3E88D;">Z</span><span style="color:#A6ACCD;">\\xa2\\xf7\\x90\\xba\\x9b\\xce\\x08\\x83\\x81</span><span style="color:#89DDFF;">&#39;</span><span style="color:#89DDFF;">)</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">public_key</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-6" tabindex="-1">Result <a class="header-anchor" href="#result-6" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381</span></span></code></pre></div><h4 id="from-string" tabindex="-1">From String <a class="header-anchor" href="#from-string" aria-label="Permalink to &quot;From String&quot;">​</a></h4><p>You can initialize a public key from a string directly.</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">public_key </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> </span><span style="color:#82AAFF;">PublicKey</span><span style="color:#89DDFF;">(</span><span style="color:#89DDFF;">&quot;</span><span style="color:#C3E88D;">8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">)</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">public_key</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-7" tabindex="-1">Result <a class="header-anchor" href="#result-7" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381</span></span></code></pre></div><h3 id="encoding" tabindex="-1">Encoding <a class="header-anchor" href="#encoding" aria-label="Permalink to &quot;Encoding&quot;">​</a></h3><p>A <code>PublicKey</code> can be encoded to a string using the <code>PublicKey.encode()</code> method.</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">encoded </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> public_key</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">encode</span><span style="color:#89DDFF;">()</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">encoded</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-8" tabindex="-1">Result <a class="header-anchor" href="#result-8" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">8d22bc3f156f400934340607e372076b9a023c6ec5915aa2f790ba9bce088381</span></span></code></pre></div><h3 id="byte-conversion" tabindex="-1">Byte Conversion <a class="header-anchor" href="#byte-conversion" aria-label="Permalink to &quot;Byte Conversion&quot;">​</a></h3><p>Convert a <code>PublicKey</code> to bytes using the <code>PublicKey.to_bytes()</code> method.</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">bytes_representation </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> public_key</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">to_bytes</span><span style="color:#89DDFF;">()</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">bytes_representation</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-9" tabindex="-1">Result <a class="header-anchor" href="#result-9" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#C792EA;">b</span><span style="color:#89DDFF;">&#39;</span><span style="color:#A6ACCD;">\\x8d</span><span style="color:#C3E88D;">&quot;</span><span style="color:#A6ACCD;">\\xbc</span><span style="color:#C3E88D;">?</span><span style="color:#A6ACCD;">\\x15</span><span style="color:#C3E88D;">o@</span><span style="color:#A6ACCD;">\\t</span><span style="color:#C3E88D;">44</span><span style="color:#A6ACCD;">\\x06\\x07\\xe3</span><span style="color:#C3E88D;">r</span><span style="color:#A6ACCD;">\\x07</span><span style="color:#C3E88D;">k</span><span style="color:#A6ACCD;">\\x9a\\x02</span><span style="color:#C3E88D;">&lt;n</span><span style="color:#A6ACCD;">\\xc5\\x91</span><span style="color:#C3E88D;">Z</span><span style="color:#A6ACCD;">\\xa2\\xf7\\x90\\xba\\x9b\\xce\\x08\\x83\\x81</span><span style="color:#89DDFF;">&#39;</span></span></code></pre></div><p>This structure provides a clear distinction between the different methods of initializing a <code>PublicKey</code>, as well as its encoding and conversion functionalities.</p><h2 id="signing-data" tabindex="-1">Signing Data <a class="header-anchor" href="#signing-data" aria-label="Permalink to &quot;Signing Data&quot;">​</a></h2><p>Signing data is a fundamental cryptographic operation that provides both authentication and data integrity. By creating a digital signature for a specific set of data, you not only prove the origin of that data (authentication) but also confirm that the data hasn&#39;t been tampered with since the signature was created (integrity). This process can be accomplished using different methods, depending on the requirements of the specific application or system.</p><p>In this section, we&#39;ll explore two primary ways of signing data: with hashing (using the <code>sign()</code> method) and without hashing (using the <code>sign_raw()</code> method).</p><p>Additionally, we will touch upon the optional incorporation of a <code>signature_id</code> to further enhance the identification of the signed data.</p><h3 id="data-with-hashing" tabindex="-1">Data with Hashing <a class="header-anchor" href="#data-with-hashing" aria-label="Permalink to &quot;Data with Hashing&quot;">​</a></h3><p>When signing data using the <code>sign()</code> method, the data is first hashed before being signed. This is useful when you want to ensure the integrity of the data being signed.</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">data </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> </span><span style="color:#C792EA;">b</span><span style="color:#89DDFF;">&quot;</span><span style="color:#C3E88D;">Hello, World 42!</span><span style="color:#89DDFF;">&quot;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#A6ACCD;">signature_id </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;font-style:italic;">await</span><span style="color:#A6ACCD;"> transport</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">get_signature_id</span><span style="color:#89DDFF;">()</span><span style="color:#A6ACCD;"> </span><span style="color:#676E95;font-style:italic;"># Optional</span></span>
<span class="line"></span>
<span class="line"><span style="color:#A6ACCD;">signature </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> keypair</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">sign</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">data</span><span style="color:#89DDFF;">,</span><span style="color:#82AAFF;"> signature_id</span><span style="color:#89DDFF;">)</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">signature</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-10" tabindex="-1">Result <a class="header-anchor" href="#result-10" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#82AAFF;">Signature</span><span style="color:#89DDFF;">(</span><span style="color:#89DDFF;">&#39;</span><span style="color:#C3E88D;">b2bd3045b3ec3872bcccc96f58b71fe0fd60cba104249cc5e72c1a2ebad35cbbf1d82a631dda5cc7a8f07b540fb1564edfa0920ede751a59e08d3ed54f80e908</span><span style="color:#89DDFF;">&#39;</span><span style="color:#89DDFF;">)</span></span></code></pre></div><div class="tip custom-block"><p class="custom-block-title">TIP</p><p>The <code>signature_id</code> is an optional identifier for a signature, sourced directly from the transport layer using the <code>get_signature_id()</code> method. If this is your first time encountering <code>signature_id</code> or you&#39;re unfamiliar with our transport layer, it&#39;s recommended to <a href="./working-with-transport.html">read our guide on working with the transport</a> to get started.</p></div><h3 id="raw-data" tabindex="-1">Raw Data <a class="header-anchor" href="#raw-data" aria-label="Permalink to &quot;Raw Data&quot;">​</a></h3><p>The <code>sign_raw()</code> method signs the data directly without hashing it. This can be useful if you need to sign data that doesn&#39;t require hashing or in cases where the data has already been hashed:</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">data </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> </span><span style="color:#C792EA;">b</span><span style="color:#89DDFF;">&quot;</span><span style="color:#C3E88D;">Hello, World 42!</span><span style="color:#89DDFF;">&quot;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#A6ACCD;">signature_raw </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> keypair</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">sign_raw</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">data</span><span style="color:#89DDFF;">)</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">signature_raw</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h5 id="result-11" tabindex="-1">Result <a class="header-anchor" href="#result-11" aria-label="Permalink to &quot;Result&quot;">​</a></h5><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#82AAFF;">Signature</span><span style="color:#89DDFF;">(</span><span style="color:#89DDFF;">&#39;</span><span style="color:#C3E88D;">ce998c9cf3dcf0aecd2b3a372661e7d75946fd3f2dfa793a4f3944da985fe49ad9c6e23b2fbce2dee31b802a3bff646ff5ac268a4a54c7aa319882e4817b4504</span><span style="color:#89DDFF;">&#39;</span><span style="color:#89DDFF;">)</span></span></code></pre></div><h2 id="verifying-signatures" tabindex="-1">Verifying Signatures <a class="header-anchor" href="#verifying-signatures" aria-label="Permalink to &quot;Verifying Signatures&quot;">​</a></h2><p>To verify a signature, you need to use the correct input depending on the signing method. If the data was signed with <code>sign()</code>, you should use the hashed data.</p><p>If it was signed with <code>sign_raw()</code>, you should either use the hash of the original data (if you want to verify the signature against hashed data) or the original data itself (if you want to verify the signature against raw data).</p><p>When a <code>signature_id</code> was used during signing, the same <code>signature_id</code> should be used for verification.</p><h3 id="hashed-signature" tabindex="-1">Hashed Signature <a class="header-anchor" href="#hashed-signature" aria-label="Permalink to &quot;Hashed Signature&quot;">​</a></h3><p>To verify a hashed signature, you will first need to hash the original data using the SHA-256 algorithm, and then call the <code>check_signature()</code> method:</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#89DDFF;font-style:italic;">import</span><span style="color:#A6ACCD;"> hashlib</span></span>
<span class="line"></span>
<span class="line"><span style="color:#A6ACCD;">data </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> hashlib</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">sha256</span><span style="color:#89DDFF;">(</span><span style="color:#C792EA;">b</span><span style="color:#89DDFF;">&quot;</span><span style="color:#C3E88D;">Hello, World 42!</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">).</span><span style="color:#82AAFF;">digest</span><span style="color:#89DDFF;">()</span></span>
<span class="line"></span>
<span class="line"><span style="color:#A6ACCD;">is_valid </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> public_key</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">check_signature</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">data</span><span style="color:#89DDFF;">,</span><span style="color:#82AAFF;"> signature</span><span style="color:#89DDFF;">,</span><span style="color:#82AAFF;"> signature_id</span><span style="color:#89DDFF;">)</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">is_valid</span><span style="color:#89DDFF;">)</span><span style="color:#A6ACCD;"> </span><span style="color:#676E95;font-style:italic;"># True</span></span></code></pre></div><h3 id="raw-signature" tabindex="-1">Raw Signature <a class="header-anchor" href="#raw-signature" aria-label="Permalink to &quot;Raw Signature&quot;">​</a></h3><p>To verify a raw signature, you will need to call the <code>check_signature_raw()</code> method with the original raw data and the signature:</p><div class="language-python"><button title="Copy Code" class="copy"></button><span class="lang">python</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">data </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> hashlib</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">sha256</span><span style="color:#89DDFF;">(</span><span style="color:#C792EA;">b</span><span style="color:#89DDFF;">&quot;</span><span style="color:#C3E88D;">Hello, World 42!</span><span style="color:#89DDFF;">&quot;</span><span style="color:#89DDFF;">).</span><span style="color:#82AAFF;">digest</span><span style="color:#89DDFF;">()</span></span>
<span class="line"></span>
<span class="line"><span style="color:#A6ACCD;">signature_raw </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> keypair</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">sign_raw</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">data</span><span style="color:#89DDFF;">)</span></span>
<span class="line"><span style="color:#A6ACCD;">is_valid_raw </span><span style="color:#89DDFF;">=</span><span style="color:#A6ACCD;"> public_key</span><span style="color:#89DDFF;">.</span><span style="color:#82AAFF;">check_signature_raw</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">data</span><span style="color:#89DDFF;">,</span><span style="color:#82AAFF;"> signature_raw</span><span style="color:#89DDFF;">)</span></span>
<span class="line"></span>
<span class="line"><span style="color:#82AAFF;">print</span><span style="color:#89DDFF;">(</span><span style="color:#82AAFF;">is_valid_raw</span><span style="color:#89DDFF;">)</span><span style="color:#A6ACCD;"> </span><span style="color:#676E95;font-style:italic;"># True</span></span></code></pre></div>`,93),t=[l];function p(c,r,i,y,d,h){return s(),e("div",null,t)}const u=a(o,[["render",p]]);export{D as __pageData,u as default};