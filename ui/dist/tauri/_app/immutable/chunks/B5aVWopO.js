import{n as l,b as _,a0 as d,a as m,c as a,T as h,a1 as g}from"./o8uVwlLI.js";function p(e){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function v(e,n,t){if(e==null)return n(void 0),l;const s=_(()=>e.subscribe(n,t));return s.unsubscribe?()=>s.unsubscribe():s}const r=[];function k(e,n=l){let t=null;const s=new Set;function u(o){if(d(e,o)&&(e=o,t)){const b=!r.length;for(const c of s)c[1](),r.push(c,e);if(b){for(let c=0;c<r.length;c+=2)r[c][0](r[c+1]);r.length=0}}}function i(o){u(o(e))}function f(o,b=l){const c=[o,b];return s.add(c),s.size===1&&(t=n(u,i)||l),o(e),()=>{s.delete(c),s.size===0&&t&&(t(),t=null)}}return{set:u,update:i,subscribe:f}}function q(e){let n;return v(e,t=>n=t)(),n}function E(e){a===null&&p(),h&&a.l!==null?x(a).m.push(e):m(()=>{const n=_(e);if(typeof n=="function")return n})}function w(e,n,{bubbles:t=!1,cancelable:s=!1}={}){return new CustomEvent(e,{detail:n,bubbles:t,cancelable:s})}function z(){const e=a;return e===null&&p(),(n,t,s)=>{const u=e.s.$$events?.[n];if(u){const i=g(u)?u.slice():[u],f=w(n,t,s);for(const o of i)o.call(e.x,f);return!f.defaultPrevented}return!0}}function x(e){var n=e.l;return n.u??={a:[],b:[],m:[]}}export{z as c,q as g,E as o,v as s,k as w};
