    ((e,t)=>{const n="__q_context__",o=window,i=new Set,s=t=>e.querySelectorAll(t),r=(e,t,n=t.type)=>{s("[on"+e+"\\:"+n+"]").forEach((o=>f(o,e,t,n)))},a=(e,t)=>e.getAttribute(t),l=t=>{if(void 0===t._qwikjson_){let n=(t===e.documentElement?e.body:t).lastElementChild;for(;n;){if("SCRIPT"===n.tagName&&"qwik/json"===a(n,"type")){t._qwikjson_=JSON.parse(n.textContent.replace(/\\x3C(\/?script)/g,"<$1"));break}n=n.previousElementSibling}}},c=(e,t)=>new CustomEvent(e,{detail:t}),f=async(t,o,i,s=i.type)=>{const r="on"+o+":"+s;t.hasAttribute("preventdefault:"+s)&&i.preventDefault();const c=t._qc_,f=null==c?void 0:c.li.filter((e=>e[0]===r));if(f&&f.length>0){for(const e of f)await e[1].getFn([t,i],(()=>t.isConnected))(i,t);return}const d=a(t,r);if(d){const o=t.closest("[q\\:container]"),s=new URL(a(o,"q:base"),e.baseURI);for(const r of d.split("\n")){const a=new URL(r,s),c=a.hash.replace(/^#?([^?[|]*).*$/,"$1")||"default",f=performance.now(),d=import(a.href.split("#")[0]);l(o);const p=b(await d,c),w=e[n];if(t.isConnected)try{e[n]=[t,i,a],u("qsymbol",{symbol:c,element:t,reqTime:f}),await p(i,t)}finally{e[n]=w}}}},u=(t,n)=>{e.dispatchEvent(c(t,n))},b=(e,t)=>{if(t in e)return e[t];for(const n of Object.values(e))if("object"==typeof n&&n&&t in n)return n[t]},d=e=>e.replace(/([A-Z])/g,(e=>"-"+e.toLowerCase())),p=async e=>{let t=d(e.type),n=e.target;for(r("-document",e,t);n&&n.getAttribute;)await f(n,"",e,t),n=e.bubbles&&!0!==e.cancelBubble?n.parentElement:null},w=e=>{r("-window",e,d(e.type))},q=()=>{var n;const r=e.readyState;if(!t&&("interactive"==r||"complete"==r)&&(t=1,u("qinit"),(null!=(n=o.requestIdleCallback)?n:o.setTimeout).bind(o)((()=>u("qidle"))),i.has("qvisible"))){const e=s("[on\\:qvisible]"),t=new IntersectionObserver((e=>{for(const n of e)n.isIntersecting&&(t.unobserve(n.target),f(n.target,"",c("qvisible",n)))}));e.forEach((e=>t.observe(e)))}},v=(e,t,n,o=!1)=>e.addEventListener(t,n,{capture:o}),y=t=>{for(const n of t)i.has(n)||(v(e,n,p,!0),v(o,n,w),i.add(n))};if(!e.qR){const t=o.qwikevents;Array.isArray(t)&&y(t),o.qwikevents={push:(...e)=>y(e)},v(e,"readystatechange",q),q()}})(document);
