var _n=Array.isArray,cn=Array.prototype.indexOf,$n=Array.from,zn=Object.defineProperty,K=Object.getOwnPropertyDescriptor,vn=Object.getOwnPropertyDescriptors,dn=Object.prototype,pn=Array.prototype,Pt=Object.getPrototypeOf,It=Object.isExtensible;function Jn(t){return typeof t=="function"}const Wn=()=>{};function Xn(t){return t()}function Ct(t){for(var e=0;e<t.length;e++)t[e]()}const m=2,Ft=4,it=8,gt=16,N=32,B=64,nt=128,g=256,et=512,y=1024,R=2048,k=4096,Y=8192,ut=16384,hn=32768,Mt=65536,Qn=1<<17,yn=1<<19,Lt=1<<20,ht=1<<21,C=Symbol("$state"),te=Symbol("legacy props"),ne=Symbol("");function qt(t){return t===this.v}function wn(t,e){return t!=t?e==e:t!==e||t!==null&&typeof t=="object"||typeof t=="function"}function jt(t){return!wn(t,this.v)}function En(t){throw new Error("https://svelte.dev/e/effect_in_teardown")}function gn(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function Tn(t){throw new Error("https://svelte.dev/e/effect_orphan")}function xn(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function ee(){throw new Error("https://svelte.dev/e/hydration_failed")}function re(t){throw new Error("https://svelte.dev/e/props_invalid_value")}function mn(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function An(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function bn(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}let ot=!1;function ae(){ot=!0}const se=1,le=2,fe=4,ie=8,ue=16,oe=1,_e=2,ce=4,ve=8,de=16,pe=1,he=2,ye=4,we=1,Ee=2,In="[",On="[!",Rn="]",Yt={},w=Symbol(),ge="http://www.w3.org/1999/xhtml";let p=null;function Ot(t){p=t}function Te(t,e=!1,n){var r=p={p,c:null,d:!1,e:null,m:!1,s:t,x:null,l:null};ot&&!e&&(p.l={s:null,u:null,r1:[],r2:xt(!1)}),Pn(()=>{r.d=!0})}function xe(t){const e=p;if(e!==null){t!==void 0&&(e.x=t);const i=e.e;if(i!==null){var n=d,r=c;e.e=null;try{for(var a=0;a<i.length;a++){var s=i[a];st(s.effect),H(s.reaction),zt(s.fn)}}finally{st(n),H(r)}}p=e.p,e.m=!0}return t||{}}function _t(){return!ot||p!==null&&p.l===null}function j(t){if(typeof t!="object"||t===null||C in t)return t;const e=Pt(t);if(e!==dn&&e!==pn)return t;var n=new Map,r=_n(t),a=D(0),s=c,i=u=>{var l=c;H(s);var f=u();return H(l),f};return r&&n.set("length",D(t.length)),new Proxy(t,{defineProperty(u,l,f){(!("value"in f)||f.configurable===!1||f.enumerable===!1||f.writable===!1)&&mn();var _=n.get(l);return _===void 0?(_=i(()=>D(f.value)),n.set(l,_)):b(_,i(()=>j(f.value))),!0},deleteProperty(u,l){var f=n.get(l);if(f===void 0)l in u&&n.set(l,i(()=>D(w)));else{if(r&&typeof l=="string"){var _=n.get("length"),o=Number(l);Number.isInteger(o)&&o<_.v&&b(_,o)}b(f,w),Rt(a)}return!0},get(u,l,f){if(l===C)return t;var _=n.get(l),o=l in u;if(_===void 0&&(!o||K(u,l)?.writable)&&(_=i(()=>D(j(o?u[l]:w))),n.set(l,_)),_!==void 0){var v=P(_);return v===w?void 0:v}return Reflect.get(u,l,f)},getOwnPropertyDescriptor(u,l){var f=Reflect.getOwnPropertyDescriptor(u,l);if(f&&"value"in f){var _=n.get(l);_&&(f.value=P(_))}else if(f===void 0){var o=n.get(l),v=o?.v;if(o!==void 0&&v!==w)return{enumerable:!0,configurable:!0,value:v,writable:!0}}return f},has(u,l){if(l===C)return!0;var f=n.get(l),_=f!==void 0&&f.v!==w||Reflect.has(u,l);if(f!==void 0||d!==null&&(!_||K(u,l)?.writable)){f===void 0&&(f=i(()=>D(_?j(u[l]):w)),n.set(l,f));var o=P(f);if(o===w)return!1}return _},set(u,l,f,_){var o=n.get(l),v=l in u;if(r&&l==="length")for(var G=f;G<o.v;G+=1){var Q=n.get(G+"");Q!==void 0?b(Q,w):G in u&&(Q=i(()=>D(w)),n.set(G+"",Q))}o===void 0?(!v||K(u,l)?.writable)&&(o=i(()=>D(void 0)),b(o,i(()=>j(f))),n.set(l,o)):(v=o.v!==w,b(o,i(()=>j(f))));var At=Reflect.getOwnPropertyDescriptor(u,l);if(At?.set&&At.set.call(_,f),!v){if(r&&typeof l=="string"){var bt=n.get("length"),pt=Number(l);Number.isInteger(pt)&&pt>=bt.v&&b(bt,pt+1)}Rt(a)}return!0},ownKeys(u){P(a);var l=Reflect.ownKeys(u).filter(o=>{var v=n.get(o);return v===void 0||v.v!==w});for(var[f,_]of n)_.v!==w&&!(f in u)&&l.push(f);return l},setPrototypeOf(){An()}})}function Rt(t,e=1){b(t,t.v+e)}function Nt(t){try{if(t!==null&&typeof t=="object"&&C in t)return t[C]}catch{}return t}function me(t,e){return Object.is(Nt(t),Nt(e))}function Tt(t){var e=m|R,n=c!==null&&(c.f&m)!==0?c:null;return d===null||n!==null&&(n.f&g)!==0?e|=g:d.f|=Lt,{ctx:p,deps:null,effects:null,equals:qt,f:e,fn:t,reactions:null,rv:0,v:null,wv:0,parent:n??d}}function Ae(t){const e=Tt(t);return rn(e),e}function be(t){const e=Tt(t);return e.equals=jt,e}function Ht(t){var e=t.effects;if(e!==null){t.effects=null;for(var n=0;n<e.length;n+=1)L(e[n])}}function Nn(t){for(var e=t.parent;e!==null;){if((e.f&m)===0)return e;e=e.parent}return null}function Bt(t){var e,n=d;st(Nn(t));try{Ht(t),e=fn(t)}finally{st(n)}return e}function Ut(t){var e=Bt(t),n=(S||(t.f&g)!==0)&&t.deps!==null?k:y;x(t,n),t.equals(e)||(t.v=e,t.wv=sn())}const $=new Map;function xt(t,e){var n={f:0,v:t,reactions:null,equals:qt,rv:0,wv:0};return n}function D(t,e){const n=xt(t);return rn(n),n}function Ie(t,e=!1){const n=xt(t);return e||(n.equals=jt),ot&&p!==null&&p.l!==null&&(p.l.s??=[]).push(n),n}function Oe(t,e){return b(t,on(()=>P(t))),e}function b(t,e,n=!1){c!==null&&!I&&_t()&&(c.f&(m|gt))!==0&&!O?.includes(t)&&bn();let r=n?j(e):e;return Dn(t,r)}function Dn(t,e){if(!t.equals(e)){var n=t.v;X?$.set(t,e):$.set(t,n),t.v=e,(t.f&m)!==0&&((t.f&R)!==0&&Bt(t),x(t,(t.f&g)===0?y:k)),t.wv=sn(),Vt(t,R),_t()&&d!==null&&(d.f&y)!==0&&(d.f&(N|B))===0&&(T===null?jn([t]):T.push(t))}return e}function Vt(t,e){var n=t.reactions;if(n!==null)for(var r=_t(),a=n.length,s=0;s<a;s++){var i=n[s],u=i.f;(u&R)===0&&(!r&&i===d||(x(i,e),(u&(y|g))!==0&&((u&m)!==0?Vt(i,k):dt(i))))}}function Gt(t){console.warn("https://svelte.dev/e/hydration_mismatch")}let M=!1;function Re(t){M=t}let A;function z(t){if(t===null)throw Gt(),Yt;return A=t}function Ne(){return z(q(A))}function De(t){if(M){if(q(A)!==null)throw Gt(),Yt;A=t}}function Se(t=1){if(M){for(var e=t,n=A;e--;)n=q(n);A=n}}function ke(){for(var t=0,e=A;;){if(e.nodeType===8){var n=e.data;if(n===Rn){if(t===0)return e;t-=1}else(n===In||n===On)&&(t+=1)}var r=q(e);e.remove(),e=r}}var Dt,Sn,Kt,Zt;function Pe(){if(Dt===void 0){Dt=window,Sn=/Firefox/.test(navigator.userAgent);var t=Element.prototype,e=Node.prototype,n=Text.prototype;Kt=K(e,"firstChild").get,Zt=K(e,"nextSibling").get,It(t)&&(t.__click=void 0,t.__className=void 0,t.__attributes=null,t.__style=void 0,t.__e=void 0),It(n)&&(n.__t=void 0)}}function yt(t=""){return document.createTextNode(t)}function wt(t){return Kt.call(t)}function q(t){return Zt.call(t)}function Ce(t,e){if(!M)return wt(t);var n=wt(A);if(n===null)n=A.appendChild(yt());else if(e&&n.nodeType!==3){var r=yt();return n?.before(r),z(r),r}return z(n),n}function Fe(t,e){if(!M){var n=wt(t);return n instanceof Comment&&n.data===""?q(n):n}return A}function Me(t,e=1,n=!1){let r=M?A:t;for(var a;e--;)a=r,r=q(r);if(!M)return r;var s=r?.nodeType;if(n&&s!==3){var i=yt();return r===null?a?.after(i):r.before(i),z(i),i}return z(r),r}function Le(t){t.textContent=""}function $t(t){d===null&&c===null&&Tn(),c!==null&&(c.f&g)!==0&&d===null&&gn(),X&&En()}function kn(t,e){var n=e.last;n===null?e.last=e.first=t:(n.next=t,t.prev=n,e.last=t)}function U(t,e,n,r=!0){var a=d,s={ctx:p,deps:null,nodes_start:null,nodes_end:null,f:t|R,first:null,fn:e,last:null,next:null,parent:a,prev:null,teardown:null,transitions:null,wv:0};if(n)try{vt(s),s.f|=hn}catch(l){throw L(s),l}else e!==null&&dt(s);var i=n&&s.deps===null&&s.first===null&&s.nodes_start===null&&s.teardown===null&&(s.f&(Lt|nt))===0;if(!i&&r&&(a!==null&&kn(s,a),c!==null&&(c.f&m)!==0)){var u=c;(u.effects??=[]).push(s)}return s}function Pn(t){const e=U(it,null,!1);return x(e,y),e.teardown=t,e}function qe(t){$t();var e=d!==null&&(d.f&N)!==0&&p!==null&&!p.m;if(e){var n=p;(n.e??=[]).push({fn:t,effect:d,reaction:c})}else{var r=zt(t);return r}}function je(t){return $t(),mt(t)}function Ye(t){const e=U(B,t,!0);return(n={})=>new Promise(r=>{n.outro?Mn(e,()=>{L(e),r(void 0)}):(L(e),r(void 0))})}function zt(t){return U(Ft,t,!1)}function He(t,e){var n=p,r={effect:null,ran:!1};n.l.r1.push(r),r.effect=mt(()=>{t(),!r.ran&&(r.ran=!0,b(n.l.r2,!0),on(e))})}function Be(){var t=p;mt(()=>{if(P(t.l.r2)){for(var e of t.l.r1){var n=e.effect;(n.f&y)!==0&&x(n,k),V(n)&&vt(n),e.ran=!1}t.l.r2.v=!1}})}function mt(t){return U(it,t,!0)}function Ue(t,e=[],n=Tt){const r=e.map(n);return Cn(()=>t(...r.map(P)))}function Cn(t,e=0){return U(it|gt|e,t,!0)}function Ve(t,e=!0){return U(it|N,t,!0,e)}function Jt(t){var e=t.teardown;if(e!==null){const n=X,r=c;kt(!0),H(null);try{e.call(null)}finally{kt(n),H(r)}}}function Wt(t,e=!1){var n=t.first;for(t.first=t.last=null;n!==null;){var r=n.next;(n.f&B)!==0?n.parent=null:L(n,e),n=r}}function Fn(t){for(var e=t.first;e!==null;){var n=e.next;(e.f&N)===0&&L(e),e=n}}function L(t,e=!0){var n=!1;if((e||(t.f&yn)!==0)&&t.nodes_start!==null){for(var r=t.nodes_start,a=t.nodes_end;r!==null;){var s=r===a?null:q(r);r.remove(),r=s}n=!0}Wt(t,e&&!n),ft(t,0),x(t,ut);var i=t.transitions;if(i!==null)for(const l of i)l.stop();Jt(t);var u=t.parent;u!==null&&u.first!==null&&Xt(t),t.next=t.prev=t.teardown=t.ctx=t.deps=t.fn=t.nodes_start=t.nodes_end=null}function Xt(t){var e=t.parent,n=t.prev,r=t.next;n!==null&&(n.next=r),r!==null&&(r.prev=n),e!==null&&(e.first===t&&(e.first=r),e.last===t&&(e.last=n))}function Mn(t,e){var n=[];Qt(t,n,!0),Ln(n,()=>{L(t),e&&e()})}function Ln(t,e){var n=t.length;if(n>0){var r=()=>--n||e();for(var a of t)a.out(r)}else e()}function Qt(t,e,n){if((t.f&Y)===0){if(t.f^=Y,t.transitions!==null)for(const i of t.transitions)(i.is_global||n)&&e.push(i);for(var r=t.first;r!==null;){var a=r.next,s=(r.f&Mt)!==0||(r.f&N)!==0;Qt(r,e,s?n:!1),r=a}}}function Ge(t){tn(t,!0)}function tn(t,e){if((t.f&Y)!==0){t.f^=Y,(t.f&y)===0&&(t.f^=y),V(t)&&(x(t,R),dt(t));for(var n=t.first;n!==null;){var r=n.next,a=(n.f&Mt)!==0||(n.f&N)!==0;tn(n,a?e:!1),n=r}if(t.transitions!==null)for(const s of t.transitions)(s.is_global||e)&&s.in()}}const qn=typeof requestIdleCallback>"u"?t=>setTimeout(t,1):requestIdleCallback;let J=[],W=[];function nn(){var t=J;J=[],Ct(t)}function en(){var t=W;W=[],Ct(t)}function Ke(t){J.length===0&&queueMicrotask(nn),J.push(t)}function Ze(t){W.length===0&&qn(en),W.push(t)}function St(){J.length>0&&nn(),W.length>0&&en()}let tt=!1,rt=!1,at=null,F=!1,X=!1;function kt(t){X=t}let Z=[];let c=null,I=!1;function H(t){c=t}let d=null;function st(t){d=t}let O=null;function rn(t){c!==null&&c.f&ht&&(O===null?O=[t]:O.push(t))}let h=null,E=0,T=null;function jn(t){T=t}let an=1,lt=0,S=!1;function sn(){return++an}function V(t){var e=t.f;if((e&R)!==0)return!0;if((e&k)!==0){var n=t.deps,r=(e&g)!==0;if(n!==null){var a,s,i=(e&et)!==0,u=r&&d!==null&&!S,l=n.length;if(i||u){var f=t,_=f.parent;for(a=0;a<l;a++)s=n[a],(i||!s?.reactions?.includes(f))&&(s.reactions??=[]).push(f);i&&(f.f^=et),u&&_!==null&&(_.f&g)===0&&(f.f^=g)}for(a=0;a<l;a++)if(s=n[a],V(s)&&Ut(s),s.wv>t.wv)return!0}(!r||d!==null&&!S)&&x(t,y)}return!1}function Yn(t,e){for(var n=e;n!==null;){if((n.f&nt)!==0)try{n.fn(t);return}catch{n.f^=nt}n=n.parent}throw tt=!1,t}function Hn(t){return(t.f&ut)===0&&(t.parent===null||(t.parent.f&nt)===0)}function ct(t,e,n,r){if(tt){if(n===null&&(tt=!1),Hn(e))throw t;return}n!==null&&(tt=!0);{Yn(t,e);return}}function ln(t,e,n=!0){var r=t.reactions;if(r!==null)for(var a=0;a<r.length;a++){var s=r[a];O?.includes(t)||((s.f&m)!==0?ln(s,e,!1):e===s&&(n?x(s,R):(s.f&y)!==0&&x(s,k),dt(s)))}}function fn(t){var e=h,n=E,r=T,a=c,s=S,i=O,u=p,l=I,f=t.f;h=null,E=0,T=null,S=(f&g)!==0&&(I||!F||c===null),c=(f&(N|B))===0?t:null,O=null,Ot(t.ctx),I=!1,lt++,t.f|=ht;try{var _=(0,t.fn)(),o=t.deps;if(h!==null){var v;if(ft(t,E),o!==null&&E>0)for(o.length=E+h.length,v=0;v<h.length;v++)o[E+v]=h[v];else t.deps=o=h;if(!S)for(v=E;v<o.length;v++)(o[v].reactions??=[]).push(t)}else o!==null&&E<o.length&&(ft(t,E),o.length=E);if(_t()&&T!==null&&!I&&o!==null&&(t.f&(m|k|R))===0)for(v=0;v<T.length;v++)ln(T[v],t);return a!==t&&(lt++,T!==null&&(r===null?r=T:r.push(...T))),_}finally{h=e,E=n,T=r,c=a,S=s,O=i,Ot(u),I=l,t.f^=ht}}function Bn(t,e){let n=e.reactions;if(n!==null){var r=cn.call(n,t);if(r!==-1){var a=n.length-1;a===0?n=e.reactions=null:(n[r]=n[a],n.pop())}}n===null&&(e.f&m)!==0&&(h===null||!h.includes(e))&&(x(e,k),(e.f&(g|et))===0&&(e.f^=et),Ht(e),ft(e,0))}function ft(t,e){var n=t.deps;if(n!==null)for(var r=e;r<n.length;r++)Bn(t,n[r])}function vt(t){var e=t.f;if((e&ut)===0){x(t,y);var n=d,r=p,a=F;d=t,F=!0;try{(e&gt)!==0?Fn(t):Wt(t),Jt(t);var s=fn(t);t.teardown=typeof s=="function"?s:null,t.wv=an;var i=t.deps,u}catch(l){ct(l,t,n,r||t.ctx)}finally{F=a,d=n}}}function Un(){try{xn()}catch(t){if(at!==null)ct(t,at,null);else throw t}}function un(){var t=F;try{var e=0;for(F=!0;Z.length>0;){e++>1e3&&Un();var n=Z,r=n.length;Z=[];for(var a=0;a<r;a++){var s=Gn(n[a]);Vn(s)}$.clear()}}finally{rt=!1,F=t,at=null}}function Vn(t){var e=t.length;if(e!==0)for(var n=0;n<e;n++){var r=t[n];if((r.f&(ut|Y))===0)try{V(r)&&(vt(r),r.deps===null&&r.first===null&&r.nodes_start===null&&(r.teardown===null?Xt(r):r.fn=null))}catch(a){ct(a,r,null,r.ctx)}}}function dt(t){rt||(rt=!0,queueMicrotask(un));for(var e=at=t;e.parent!==null;){e=e.parent;var n=e.f;if((n&(B|N))!==0){if((n&y)===0)return;e.f^=y}}Z.push(e)}function Gn(t){for(var e=[],n=t;n!==null;){var r=n.f,a=(r&(N|B))!==0,s=a&&(r&y)!==0;if(!s&&(r&Y)===0){if((r&Ft)!==0)e.push(n);else if(a)n.f^=y;else{var i=c;try{c=n,V(n)&&vt(n)}catch(f){ct(f,n,null,n.ctx)}finally{c=i}}var u=n.first;if(u!==null){n=u;continue}}var l=n.parent;for(n=n.next;n===null&&l!==null;)n=l.next,l=l.parent}return e}function Kn(t){var e;for(St();Z.length>0;)rt=!0,un(),St();return e}async function $e(){await Promise.resolve(),Kn()}function P(t){var e=t.f,n=(e&m)!==0;if(c!==null&&!I){if(!O?.includes(t)){var r=c.deps;t.rv<lt&&(t.rv=lt,h===null&&r!==null&&r[E]===t?E++:h===null?h=[t]:(!S||!h.includes(t))&&h.push(t))}}else if(n&&t.deps===null&&t.effects===null){var a=t,s=a.parent;s!==null&&(s.f&g)===0&&(a.f^=g)}return n&&(a=t,V(a)&&Ut(a)),X&&$.has(t)?$.get(t):t.v}function on(t){var e=I;try{return I=!0,t()}finally{I=e}}const Zn=-7169;function x(t,e){t.f=t.f&Zn|e}function ze(t){if(!(typeof t!="object"||!t||t instanceof EventTarget)){if(C in t)Et(t);else if(!Array.isArray(t))for(let e in t){const n=t[e];typeof n=="object"&&n&&C in n&&Et(n)}}}function Et(t,e=new Set){if(typeof t=="object"&&t!==null&&!(t instanceof EventTarget)&&!e.has(t)){e.add(t),t instanceof Date&&t.getTime();for(let r in t)try{Et(t[r],e)}catch{}const n=Pt(t);if(n!==Object.prototype&&n!==Array.prototype&&n!==Map.prototype&&n!==Set.prototype&&n!==Date.prototype){const r=vn(n);for(let a in r){const s=r[a].get;if(s)try{s.call(t)}catch{}}}}}export{oe as $,z as A,Re as B,Ge as C,Mn as D,Mt as E,zt as F,mt as G,In as H,Ke as I,Ie as J,Pn as K,zn as L,b as M,K as N,re as O,Qn as P,be as Q,te as R,C as S,ot as T,w as U,_e as V,ve as W,ce as X,jt as Y,j as Z,de as _,qe as a,wn as a0,_n as a1,yt as a2,wt as a3,Sn as a4,d as a5,we as a6,Ee as a7,Le as a8,H as a9,vn as aA,Ze as aB,ne as aC,gt as aD,hn as aE,ye as aF,pe as aG,he as aH,Jn as aI,_t as aJ,me as aK,Se as aL,He as aM,Be as aN,Dt as aO,Oe as aP,st as aa,c as ab,Pe as ac,q as ad,Yt as ae,Rn as af,Gt as ag,ee as ah,$n as ai,Ye as aj,Kn as ak,D as al,$e as am,Ae as an,xt as ao,Y as ap,le as aq,ie as ar,se as as,Dn as at,Qt as au,Ln as av,ue as aw,fe as ax,ge as ay,Pt as az,on as b,p as c,Xn as d,ze as e,Tt as f,P as g,ae as h,Cn as i,Ve as j,L as k,M as l,A as m,Wn as n,Fe as o,Te as p,xe as q,Ct as r,Ce as s,Ue as t,je as u,De as v,Me as w,Ne as x,On as y,ke as z};
