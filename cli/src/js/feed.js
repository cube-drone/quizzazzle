(() => {
  // node_modules/preact/dist/preact.module.js
  var n;
  var l;
  var u;
  var t;
  var i;
  var o;
  var r;
  var f;
  var e;
  var c = {};
  var s = [];
  var a = /acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;
  var v = Array.isArray;
  function h(n3, l3) {
    for (var u3 in l3)
      n3[u3] = l3[u3];
    return n3;
  }
  function p(n3) {
    var l3 = n3.parentNode;
    l3 && l3.removeChild(n3);
  }
  function y(l3, u3, t4) {
    var i3, o3, r3, f3 = {};
    for (r3 in u3)
      "key" == r3 ? i3 = u3[r3] : "ref" == r3 ? o3 = u3[r3] : f3[r3] = u3[r3];
    if (arguments.length > 2 && (f3.children = arguments.length > 3 ? n.call(arguments, 2) : t4), "function" == typeof l3 && null != l3.defaultProps)
      for (r3 in l3.defaultProps)
        void 0 === f3[r3] && (f3[r3] = l3.defaultProps[r3]);
    return d(l3, f3, i3, o3, null);
  }
  function d(n3, t4, i3, o3, r3) {
    var f3 = { type: n3, props: t4, key: i3, ref: o3, __k: null, __: null, __b: 0, __e: null, __d: void 0, __c: null, __h: null, constructor: void 0, __v: null == r3 ? ++u : r3 };
    return null == r3 && null != l.vnode && l.vnode(f3), f3;
  }
  function k(n3) {
    return n3.children;
  }
  function b(n3, l3) {
    this.props = n3, this.context = l3;
  }
  function g(n3, l3) {
    if (null == l3)
      return n3.__ ? g(n3.__, n3.__.__k.indexOf(n3) + 1) : null;
    for (var u3; l3 < n3.__k.length; l3++)
      if (null != (u3 = n3.__k[l3]) && null != u3.__e)
        return u3.__d || u3.__e;
    return "function" == typeof n3.type ? g(n3) : null;
  }
  function m(n3) {
    var l3, u3;
    if (null != (n3 = n3.__) && null != n3.__c) {
      for (n3.__e = n3.__c.base = null, l3 = 0; l3 < n3.__k.length; l3++)
        if (null != (u3 = n3.__k[l3]) && null != u3.__e) {
          n3.__e = n3.__c.base = u3.__e;
          break;
        }
      return m(n3);
    }
  }
  function w(n3) {
    (!n3.__d && (n3.__d = true) && i.push(n3) && !x.__r++ || o !== l.debounceRendering) && ((o = l.debounceRendering) || r)(x);
  }
  function x() {
    var n3, l3, u3, t4, o3, r3, e3, c3, s3;
    for (i.sort(f); n3 = i.shift(); )
      n3.__d && (l3 = i.length, t4 = void 0, o3 = void 0, r3 = void 0, c3 = (e3 = (u3 = n3).__v).__e, (s3 = u3.__P) && (t4 = [], o3 = [], (r3 = h({}, e3)).__v = e3.__v + 1, z(s3, e3, r3, u3.__n, void 0 !== s3.ownerSVGElement, null != e3.__h ? [c3] : null, t4, null == c3 ? g(e3) : c3, e3.__h, o3), L(t4, e3, o3), e3.__e != c3 && m(e3)), i.length > l3 && i.sort(f));
    x.__r = 0;
  }
  function P(n3, l3, u3, t4, i3, o3, r3, f3, e3, a3, h3) {
    var p3, y2, _, b3, m3, w3, x2, P2, C, D2 = 0, H2 = t4 && t4.__k || s, I2 = H2.length, T2 = I2, j3 = l3.length;
    for (u3.__k = [], p3 = 0; p3 < j3; p3++)
      null != (b3 = u3.__k[p3] = null == (b3 = l3[p3]) || "boolean" == typeof b3 || "function" == typeof b3 ? null : "string" == typeof b3 || "number" == typeof b3 || "bigint" == typeof b3 ? d(null, b3, null, null, b3) : v(b3) ? d(k, { children: b3 }, null, null, null) : b3.__b > 0 ? d(b3.type, b3.props, b3.key, b3.ref ? b3.ref : null, b3.__v) : b3) ? (b3.__ = u3, b3.__b = u3.__b + 1, -1 === (P2 = A(b3, H2, x2 = p3 + D2, T2)) ? _ = c : (_ = H2[P2] || c, H2[P2] = void 0, T2--), z(n3, b3, _, i3, o3, r3, f3, e3, a3, h3), m3 = b3.__e, (y2 = b3.ref) && _.ref != y2 && (_.ref && N(_.ref, null, b3), h3.push(y2, b3.__c || m3, b3)), null != m3 && (null == w3 && (w3 = m3), (C = _ === c || null === _.__v) ? -1 == P2 && D2-- : P2 !== x2 && (P2 === x2 + 1 ? D2++ : P2 > x2 ? T2 > j3 - x2 ? D2 += P2 - x2 : D2-- : D2 = P2 < x2 && P2 == x2 - 1 ? P2 - x2 : 0), x2 = p3 + D2, "function" != typeof b3.type || P2 === x2 && _.__k !== b3.__k ? "function" == typeof b3.type || P2 === x2 && !C ? void 0 !== b3.__d ? (e3 = b3.__d, b3.__d = void 0) : e3 = m3.nextSibling : e3 = S(n3, m3, e3) : e3 = $(b3, e3, n3), "function" == typeof u3.type && (u3.__d = e3))) : (_ = H2[p3]) && null == _.key && _.__e && (_.__e == e3 && (_.__ = t4, e3 = g(_)), O(_, _, false), H2[p3] = null);
    for (u3.__e = w3, p3 = I2; p3--; )
      null != H2[p3] && ("function" == typeof u3.type && null != H2[p3].__e && H2[p3].__e == u3.__d && (u3.__d = H2[p3].__e.nextSibling), O(H2[p3], H2[p3]));
  }
  function $(n3, l3, u3) {
    for (var t4, i3 = n3.__k, o3 = 0; i3 && o3 < i3.length; o3++)
      (t4 = i3[o3]) && (t4.__ = n3, l3 = "function" == typeof t4.type ? $(t4, l3, u3) : S(u3, t4.__e, l3));
    return l3;
  }
  function S(n3, l3, u3) {
    return null == u3 || u3.parentNode !== n3 ? n3.insertBefore(l3, null) : l3 == u3 && null != l3.parentNode || n3.insertBefore(l3, u3), l3.nextSibling;
  }
  function A(n3, l3, u3, t4) {
    var i3 = n3.key, o3 = n3.type, r3 = u3 - 1, f3 = u3 + 1, e3 = l3[u3];
    if (null === e3 || e3 && i3 == e3.key && o3 === e3.type)
      return u3;
    if (t4 > (null != e3 ? 1 : 0))
      for (; r3 >= 0 || f3 < l3.length; ) {
        if (r3 >= 0) {
          if ((e3 = l3[r3]) && i3 == e3.key && o3 === e3.type)
            return r3;
          r3--;
        }
        if (f3 < l3.length) {
          if ((e3 = l3[f3]) && i3 == e3.key && o3 === e3.type)
            return f3;
          f3++;
        }
      }
    return -1;
  }
  function D(n3, l3, u3, t4, i3) {
    var o3;
    for (o3 in u3)
      "children" === o3 || "key" === o3 || o3 in l3 || I(n3, o3, null, u3[o3], t4);
    for (o3 in l3)
      i3 && "function" != typeof l3[o3] || "children" === o3 || "key" === o3 || "value" === o3 || "checked" === o3 || u3[o3] === l3[o3] || I(n3, o3, l3[o3], u3[o3], t4);
  }
  function H(n3, l3, u3) {
    "-" === l3[0] ? n3.setProperty(l3, null == u3 ? "" : u3) : n3[l3] = null == u3 ? "" : "number" != typeof u3 || a.test(l3) ? u3 : u3 + "px";
  }
  function I(n3, l3, u3, t4, i3) {
    var o3;
    n:
      if ("style" === l3)
        if ("string" == typeof u3)
          n3.style.cssText = u3;
        else {
          if ("string" == typeof t4 && (n3.style.cssText = t4 = ""), t4)
            for (l3 in t4)
              u3 && l3 in u3 || H(n3.style, l3, "");
          if (u3)
            for (l3 in u3)
              t4 && u3[l3] === t4[l3] || H(n3.style, l3, u3[l3]);
        }
      else if ("o" === l3[0] && "n" === l3[1])
        o3 = l3 !== (l3 = l3.replace(/(PointerCapture)$|Capture$/, "$1")), l3 = l3.toLowerCase() in n3 ? l3.toLowerCase().slice(2) : l3.slice(2), n3.l || (n3.l = {}), n3.l[l3 + o3] = u3, u3 ? t4 ? u3.u = t4.u : (u3.u = Date.now(), n3.addEventListener(l3, o3 ? j : T, o3)) : n3.removeEventListener(l3, o3 ? j : T, o3);
      else if ("dangerouslySetInnerHTML" !== l3) {
        if (i3)
          l3 = l3.replace(/xlink(H|:h)/, "h").replace(/sName$/, "s");
        else if ("width" !== l3 && "height" !== l3 && "href" !== l3 && "list" !== l3 && "form" !== l3 && "tabIndex" !== l3 && "download" !== l3 && "rowSpan" !== l3 && "colSpan" !== l3 && "role" !== l3 && l3 in n3)
          try {
            n3[l3] = null == u3 ? "" : u3;
            break n;
          } catch (n4) {
          }
        "function" == typeof u3 || (null == u3 || false === u3 && "-" !== l3[4] ? n3.removeAttribute(l3) : n3.setAttribute(l3, u3));
      }
  }
  function T(n3) {
    var u3 = this.l[n3.type + false];
    if (n3.t) {
      if (n3.t <= u3.u)
        return;
    } else
      n3.t = Date.now();
    return u3(l.event ? l.event(n3) : n3);
  }
  function j(n3) {
    return this.l[n3.type + true](l.event ? l.event(n3) : n3);
  }
  function z(n3, u3, t4, i3, o3, r3, f3, e3, c3, s3) {
    var a3, p3, y2, d3, _, g3, m3, w3, x2, $2, C, S2, A2, D2, H2, I2 = u3.type;
    if (void 0 !== u3.constructor)
      return null;
    null != t4.__h && (c3 = t4.__h, e3 = u3.__e = t4.__e, u3.__h = null, r3 = [e3]), (a3 = l.__b) && a3(u3);
    n:
      if ("function" == typeof I2)
        try {
          if (w3 = u3.props, x2 = (a3 = I2.contextType) && i3[a3.__c], $2 = a3 ? x2 ? x2.props.value : a3.__ : i3, t4.__c ? m3 = (p3 = u3.__c = t4.__c).__ = p3.__E : ("prototype" in I2 && I2.prototype.render ? u3.__c = p3 = new I2(w3, $2) : (u3.__c = p3 = new b(w3, $2), p3.constructor = I2, p3.render = q), x2 && x2.sub(p3), p3.props = w3, p3.state || (p3.state = {}), p3.context = $2, p3.__n = i3, y2 = p3.__d = true, p3.__h = [], p3._sb = []), null == p3.__s && (p3.__s = p3.state), null != I2.getDerivedStateFromProps && (p3.__s == p3.state && (p3.__s = h({}, p3.__s)), h(p3.__s, I2.getDerivedStateFromProps(w3, p3.__s))), d3 = p3.props, _ = p3.state, p3.__v = u3, y2)
            null == I2.getDerivedStateFromProps && null != p3.componentWillMount && p3.componentWillMount(), null != p3.componentDidMount && p3.__h.push(p3.componentDidMount);
          else {
            if (null == I2.getDerivedStateFromProps && w3 !== d3 && null != p3.componentWillReceiveProps && p3.componentWillReceiveProps(w3, $2), !p3.__e && (null != p3.shouldComponentUpdate && false === p3.shouldComponentUpdate(w3, p3.__s, $2) || u3.__v === t4.__v)) {
              for (u3.__v !== t4.__v && (p3.props = w3, p3.state = p3.__s, p3.__d = false), u3.__e = t4.__e, u3.__k = t4.__k, u3.__k.forEach(function(n4) {
                n4 && (n4.__ = u3);
              }), C = 0; C < p3._sb.length; C++)
                p3.__h.push(p3._sb[C]);
              p3._sb = [], p3.__h.length && f3.push(p3);
              break n;
            }
            null != p3.componentWillUpdate && p3.componentWillUpdate(w3, p3.__s, $2), null != p3.componentDidUpdate && p3.__h.push(function() {
              p3.componentDidUpdate(d3, _, g3);
            });
          }
          if (p3.context = $2, p3.props = w3, p3.__P = n3, p3.__e = false, S2 = l.__r, A2 = 0, "prototype" in I2 && I2.prototype.render) {
            for (p3.state = p3.__s, p3.__d = false, S2 && S2(u3), a3 = p3.render(p3.props, p3.state, p3.context), D2 = 0; D2 < p3._sb.length; D2++)
              p3.__h.push(p3._sb[D2]);
            p3._sb = [];
          } else
            do {
              p3.__d = false, S2 && S2(u3), a3 = p3.render(p3.props, p3.state, p3.context), p3.state = p3.__s;
            } while (p3.__d && ++A2 < 25);
          p3.state = p3.__s, null != p3.getChildContext && (i3 = h(h({}, i3), p3.getChildContext())), y2 || null == p3.getSnapshotBeforeUpdate || (g3 = p3.getSnapshotBeforeUpdate(d3, _)), P(n3, v(H2 = null != a3 && a3.type === k && null == a3.key ? a3.props.children : a3) ? H2 : [H2], u3, t4, i3, o3, r3, f3, e3, c3, s3), p3.base = u3.__e, u3.__h = null, p3.__h.length && f3.push(p3), m3 && (p3.__E = p3.__ = null);
        } catch (n4) {
          u3.__v = null, (c3 || null != r3) && (u3.__e = e3, u3.__h = !!c3, r3[r3.indexOf(e3)] = null), l.__e(n4, u3, t4);
        }
      else
        null == r3 && u3.__v === t4.__v ? (u3.__k = t4.__k, u3.__e = t4.__e) : u3.__e = M(t4.__e, u3, t4, i3, o3, r3, f3, c3, s3);
    (a3 = l.diffed) && a3(u3);
  }
  function L(n3, u3, t4) {
    for (var i3 = 0; i3 < t4.length; i3++)
      N(t4[i3], t4[++i3], t4[++i3]);
    l.__c && l.__c(u3, n3), n3.some(function(u4) {
      try {
        n3 = u4.__h, u4.__h = [], n3.some(function(n4) {
          n4.call(u4);
        });
      } catch (n4) {
        l.__e(n4, u4.__v);
      }
    });
  }
  function M(l3, u3, t4, i3, o3, r3, f3, e3, s3) {
    var a3, h3, y2, d3 = t4.props, _ = u3.props, k3 = u3.type, b3 = 0;
    if ("svg" === k3 && (o3 = true), null != r3) {
      for (; b3 < r3.length; b3++)
        if ((a3 = r3[b3]) && "setAttribute" in a3 == !!k3 && (k3 ? a3.localName === k3 : 3 === a3.nodeType)) {
          l3 = a3, r3[b3] = null;
          break;
        }
    }
    if (null == l3) {
      if (null === k3)
        return document.createTextNode(_);
      l3 = o3 ? document.createElementNS("http://www.w3.org/2000/svg", k3) : document.createElement(k3, _.is && _), r3 = null, e3 = false;
    }
    if (null === k3)
      d3 === _ || e3 && l3.data === _ || (l3.data = _);
    else {
      if (r3 = r3 && n.call(l3.childNodes), h3 = (d3 = t4.props || c).dangerouslySetInnerHTML, y2 = _.dangerouslySetInnerHTML, !e3) {
        if (null != r3)
          for (d3 = {}, b3 = 0; b3 < l3.attributes.length; b3++)
            d3[l3.attributes[b3].name] = l3.attributes[b3].value;
        (y2 || h3) && (y2 && (h3 && y2.__html == h3.__html || y2.__html === l3.innerHTML) || (l3.innerHTML = y2 && y2.__html || ""));
      }
      if (D(l3, _, d3, o3, e3), y2)
        u3.__k = [];
      else if (P(l3, v(b3 = u3.props.children) ? b3 : [b3], u3, t4, i3, o3 && "foreignObject" !== k3, r3, f3, r3 ? r3[0] : t4.__k && g(t4, 0), e3, s3), null != r3)
        for (b3 = r3.length; b3--; )
          null != r3[b3] && p(r3[b3]);
      e3 || ("value" in _ && void 0 !== (b3 = _.value) && (b3 !== l3.value || "progress" === k3 && !b3 || "option" === k3 && b3 !== d3.value) && I(l3, "value", b3, d3.value, false), "checked" in _ && void 0 !== (b3 = _.checked) && b3 !== l3.checked && I(l3, "checked", b3, d3.checked, false));
    }
    return l3;
  }
  function N(n3, u3, t4) {
    try {
      "function" == typeof n3 ? n3(u3) : n3.current = u3;
    } catch (n4) {
      l.__e(n4, t4);
    }
  }
  function O(n3, u3, t4) {
    var i3, o3;
    if (l.unmount && l.unmount(n3), (i3 = n3.ref) && (i3.current && i3.current !== n3.__e || N(i3, null, u3)), null != (i3 = n3.__c)) {
      if (i3.componentWillUnmount)
        try {
          i3.componentWillUnmount();
        } catch (n4) {
          l.__e(n4, u3);
        }
      i3.base = i3.__P = null, n3.__c = void 0;
    }
    if (i3 = n3.__k)
      for (o3 = 0; o3 < i3.length; o3++)
        i3[o3] && O(i3[o3], u3, t4 || "function" != typeof n3.type);
    t4 || null == n3.__e || p(n3.__e), n3.__ = n3.__e = n3.__d = void 0;
  }
  function q(n3, l3, u3) {
    return this.constructor(n3, u3);
  }
  function B(u3, t4, i3) {
    var o3, r3, f3, e3;
    l.__ && l.__(u3, t4), r3 = (o3 = "function" == typeof i3) ? null : i3 && i3.__k || t4.__k, f3 = [], e3 = [], z(t4, u3 = (!o3 && i3 || t4).__k = y(k, null, [u3]), r3 || c, c, void 0 !== t4.ownerSVGElement, !o3 && i3 ? [i3] : r3 ? null : t4.firstChild ? n.call(t4.childNodes) : null, f3, !o3 && i3 ? i3 : r3 ? r3.__e : t4.firstChild, o3, e3), L(f3, u3, e3);
  }
  n = s.slice, l = { __e: function(n3, l3, u3, t4) {
    for (var i3, o3, r3; l3 = l3.__; )
      if ((i3 = l3.__c) && !i3.__)
        try {
          if ((o3 = i3.constructor) && null != o3.getDerivedStateFromError && (i3.setState(o3.getDerivedStateFromError(n3)), r3 = i3.__d), null != i3.componentDidCatch && (i3.componentDidCatch(n3, t4 || {}), r3 = i3.__d), r3)
            return i3.__E = i3;
        } catch (l4) {
          n3 = l4;
        }
    throw n3;
  } }, u = 0, t = function(n3) {
    return null != n3 && void 0 === n3.constructor;
  }, b.prototype.setState = function(n3, l3) {
    var u3;
    u3 = null != this.__s && this.__s !== this.state ? this.__s : this.__s = h({}, this.state), "function" == typeof n3 && (n3 = n3(h({}, u3), this.props)), n3 && h(u3, n3), null != n3 && this.__v && (l3 && this._sb.push(l3), w(this));
  }, b.prototype.forceUpdate = function(n3) {
    this.__v && (this.__e = true, n3 && this.__h.push(n3), w(this));
  }, b.prototype.render = k, i = [], r = "function" == typeof Promise ? Promise.prototype.then.bind(Promise.resolve()) : setTimeout, f = function(n3, l3) {
    return n3.__v.__b - l3.__v.__b;
  }, x.__r = 0, e = 0;

  // node_modules/preact/hooks/dist/hooks.module.js
  var t2;
  var r2;
  var u2;
  var i2;
  var o2 = 0;
  var f2 = [];
  var c2 = [];
  var e2 = l.__b;
  var a2 = l.__r;
  var v2 = l.diffed;
  var l2 = l.__c;
  var m2 = l.unmount;
  function d2(t4, u3) {
    l.__h && l.__h(r2, t4, o2 || u3), o2 = 0;
    var i3 = r2.__H || (r2.__H = { __: [], __h: [] });
    return t4 >= i3.__.length && i3.__.push({ __V: c2 }), i3.__[t4];
  }
  function h2(n3) {
    return o2 = 1, s2(B2, n3);
  }
  function s2(n3, u3, i3) {
    var o3 = d2(t2++, 2);
    if (o3.t = n3, !o3.__c && (o3.__ = [i3 ? i3(u3) : B2(void 0, u3), function(n4) {
      var t4 = o3.__N ? o3.__N[0] : o3.__[0], r3 = o3.t(t4, n4);
      t4 !== r3 && (o3.__N = [r3, o3.__[1]], o3.__c.setState({}));
    }], o3.__c = r2, !r2.u)) {
      var f3 = function(n4, t4, r3) {
        if (!o3.__c.__H)
          return true;
        var u4 = o3.__c.__H.__.filter(function(n5) {
          return n5.__c;
        });
        if (u4.every(function(n5) {
          return !n5.__N;
        }))
          return !c3 || c3.call(this, n4, t4, r3);
        var i4 = false;
        return u4.forEach(function(n5) {
          if (n5.__N) {
            var t5 = n5.__[0];
            n5.__ = n5.__N, n5.__N = void 0, t5 !== n5.__[0] && (i4 = true);
          }
        }), !(!i4 && o3.__c.props === n4) && (!c3 || c3.call(this, n4, t4, r3));
      };
      r2.u = true;
      var c3 = r2.shouldComponentUpdate, e3 = r2.componentWillUpdate;
      r2.componentWillUpdate = function(n4, t4, r3) {
        if (this.__e) {
          var u4 = c3;
          c3 = void 0, f3(n4, t4, r3), c3 = u4;
        }
        e3 && e3.call(this, n4, t4, r3);
      }, r2.shouldComponentUpdate = f3;
    }
    return o3.__N || o3.__;
  }
  function p2(u3, i3) {
    var o3 = d2(t2++, 3);
    !l.__s && z2(o3.__H, i3) && (o3.__ = u3, o3.i = i3, r2.__H.__h.push(o3));
  }
  function b2() {
    for (var t4; t4 = f2.shift(); )
      if (t4.__P && t4.__H)
        try {
          t4.__H.__h.forEach(k2), t4.__H.__h.forEach(w2), t4.__H.__h = [];
        } catch (r3) {
          t4.__H.__h = [], l.__e(r3, t4.__v);
        }
  }
  l.__b = function(n3) {
    r2 = null, e2 && e2(n3);
  }, l.__r = function(n3) {
    a2 && a2(n3), t2 = 0;
    var i3 = (r2 = n3.__c).__H;
    i3 && (u2 === r2 ? (i3.__h = [], r2.__h = [], i3.__.forEach(function(n4) {
      n4.__N && (n4.__ = n4.__N), n4.__V = c2, n4.__N = n4.i = void 0;
    })) : (i3.__h.forEach(k2), i3.__h.forEach(w2), i3.__h = [], t2 = 0)), u2 = r2;
  }, l.diffed = function(t4) {
    v2 && v2(t4);
    var o3 = t4.__c;
    o3 && o3.__H && (o3.__H.__h.length && (1 !== f2.push(o3) && i2 === l.requestAnimationFrame || ((i2 = l.requestAnimationFrame) || j2)(b2)), o3.__H.__.forEach(function(n3) {
      n3.i && (n3.__H = n3.i), n3.__V !== c2 && (n3.__ = n3.__V), n3.i = void 0, n3.__V = c2;
    })), u2 = r2 = null;
  }, l.__c = function(t4, r3) {
    r3.some(function(t5) {
      try {
        t5.__h.forEach(k2), t5.__h = t5.__h.filter(function(n3) {
          return !n3.__ || w2(n3);
        });
      } catch (u3) {
        r3.some(function(n3) {
          n3.__h && (n3.__h = []);
        }), r3 = [], l.__e(u3, t5.__v);
      }
    }), l2 && l2(t4, r3);
  }, l.unmount = function(t4) {
    m2 && m2(t4);
    var r3, u3 = t4.__c;
    u3 && u3.__H && (u3.__H.__.forEach(function(n3) {
      try {
        k2(n3);
      } catch (n4) {
        r3 = n4;
      }
    }), u3.__H = void 0, r3 && l.__e(r3, u3.__v));
  };
  var g2 = "function" == typeof requestAnimationFrame;
  function j2(n3) {
    var t4, r3 = function() {
      clearTimeout(u3), g2 && cancelAnimationFrame(t4), setTimeout(n3);
    }, u3 = setTimeout(r3, 100);
    g2 && (t4 = requestAnimationFrame(r3));
  }
  function k2(n3) {
    var t4 = r2, u3 = n3.__c;
    "function" == typeof u3 && (n3.__c = void 0, u3()), r2 = t4;
  }
  function w2(n3) {
    var t4 = r2;
    n3.__c = n3.__(), r2 = t4;
  }
  function z2(n3, t4) {
    return !n3 || n3.length !== t4.length || t4.some(function(t5, r3) {
      return t5 !== n3[r3];
    });
  }
  function B2(n3, t4) {
    return "function" == typeof t4 ? t4(n3) : t4;
  }

  // node_modules/htm/dist/htm.module.js
  var n2 = function(t4, s3, r3, e3) {
    var u3;
    s3[0] = 0;
    for (var h3 = 1; h3 < s3.length; h3++) {
      var p3 = s3[h3++], a3 = s3[h3] ? (s3[0] |= p3 ? 1 : 2, r3[s3[h3++]]) : s3[++h3];
      3 === p3 ? e3[0] = a3 : 4 === p3 ? e3[1] = Object.assign(e3[1] || {}, a3) : 5 === p3 ? (e3[1] = e3[1] || {})[s3[++h3]] = a3 : 6 === p3 ? e3[1][s3[++h3]] += a3 + "" : p3 ? (u3 = t4.apply(a3, n2(t4, a3, r3, ["", null])), e3.push(u3), a3[0] ? s3[0] |= 2 : (s3[h3 - 2] = 0, s3[h3] = u3)) : e3.push(a3);
    }
    return e3;
  };
  var t3 = /* @__PURE__ */ new Map();
  function htm_module_default(s3) {
    var r3 = t3.get(this);
    return r3 || (r3 = /* @__PURE__ */ new Map(), t3.set(this, r3)), (r3 = n2(this, r3.get(s3) || (r3.set(s3, r3 = function(n3) {
      for (var t4, s4, r4 = 1, e3 = "", u3 = "", h3 = [0], p3 = function(n4) {
        1 === r4 && (n4 || (e3 = e3.replace(/^\s*\n\s*|\s*\n\s*$/g, ""))) ? h3.push(0, n4, e3) : 3 === r4 && (n4 || e3) ? (h3.push(3, n4, e3), r4 = 2) : 2 === r4 && "..." === e3 && n4 ? h3.push(4, n4, 0) : 2 === r4 && e3 && !n4 ? h3.push(5, 0, true, e3) : r4 >= 5 && ((e3 || !n4 && 5 === r4) && (h3.push(r4, 0, e3, s4), r4 = 6), n4 && (h3.push(r4, n4, 0, s4), r4 = 6)), e3 = "";
      }, a3 = 0; a3 < n3.length; a3++) {
        a3 && (1 === r4 && p3(), p3(a3));
        for (var l3 = 0; l3 < n3[a3].length; l3++)
          t4 = n3[a3][l3], 1 === r4 ? "<" === t4 ? (p3(), h3 = [h3], r4 = 3) : e3 += t4 : 4 === r4 ? "--" === e3 && ">" === t4 ? (r4 = 1, e3 = "") : e3 = t4 + e3[0] : u3 ? t4 === u3 ? u3 = "" : e3 += t4 : '"' === t4 || "'" === t4 ? u3 = t4 : ">" === t4 ? (p3(), r4 = 1) : r4 && ("=" === t4 ? (r4 = 5, s4 = e3, e3 = "") : "/" === t4 && (r4 < 5 || ">" === n3[a3][l3 + 1]) ? (p3(), 3 === r4 && (h3 = h3[0]), r4 = h3, (h3 = h3[0]).push(2, 0, r4), r4 = 0) : " " === t4 || "	" === t4 || "\n" === t4 || "\r" === t4 ? (p3(), r4 = 2) : e3 += t4), 3 === r4 && "!--" === e3 && (r4 = 4, h3 = h3[0]);
      }
      return p3(), h3;
    }(s3)), r3), arguments, [])).length > 1 ? r3 : r3[0];
  }

  // src/assert.js
  function assert(boolean, message) {
    if (!boolean) {
      throw new Error(message);
    }
  }

  // src/data.js
  var PAGE_SIZE = 100;
  var RealServer = class {
    constructor({ serverUrl: serverUrl2 }) {
      this.serverUrl = serverUrl2;
      this.index = null;
    }
    async getIndexId({ userSlug, contentSlug }) {
      console.log(`getting index id for ${userSlug}/${contentSlug}`);
      if (userSlug == null || contentSlug == null) {
        const response = await fetch(`${this.serverUrl}/index`, {});
        this.index = await response.json();
        return `/s/default/default`;
      }
      return `/s/${userSlug}/${contentSlug}`;
    }
    indexTransform(serverIndex) {
      console.dir(serverIndex);
      let appIndex = {
        id: serverIndex.id,
        userSlug: serverIndex.metadata.author_slug,
        authorSlug: serverIndex.metadata.author_slug,
        contentSlug: serverIndex.metadata.slug,
        author: serverIndex.metadata.author,
        authorLink: serverIndex.metadata.author_link,
        name: serverIndex.metadata.title,
        description: serverIndex.metadata.description,
        thumbnailImageUrl: serverIndex.metadata.image_url,
        locale: serverIndex.metadata.locale,
        contentIds: serverIndex.deck_ids || [],
        toc: serverIndex.toc || [],
        mp3: serverIndex.metadata.mp3,
        audioGuide: serverIndex.metadata.audio_guide,
        containerClass: serverIndex.metadata.container_class,
        extraClass: serverIndex.metadata.extra_class,
        updatedAt: new Date(serverIndex?.metadata?.last_update_time?.secs_since_epoch * 1e3),
        updatedAtTimestamp: serverIndex?.metadata?.last_update_time?.secs_since_epoch
      };
      return appIndex;
    }
    async getIndex({ indexId }) {
      if (this.index == null) {
        const response = await fetch(`${this.serverUrl}${indexId}/index`, {});
        this.index = await response.json();
      }
      return this.indexTransform(this.index);
    }
    cardTransform(card) {
      let appCard = {
        id: card.id,
        title: card.title,
        type: card.card_type || "title",
        extraClass: card.extra_class || [],
        containerClass: card.container_class || [],
        content: card.content,
        footnote: card.footnote,
        imageUrl: card.image_url,
        videoUrl: card.video_url,
        videoHasSound: card.video_has_sound,
        videoControls: card.video_controls,
        loop: card.is_loop,
        pngs: card.pngs,
        pngsFps: card.pngs_fps,
        fadeIn: card.fade_in,
        fadeOut: card.fade_out,
        shake: card.shake,
        verticalShake: card.vertical_shake,
        jitter: card.jitter,
        verticalJitter: card.vertical_jitter,
        panLeft: card.pan_left,
        panRight: card.pan_right,
        panUp: card.pan_up,
        panDown: card.pan_down,
        dollyIn: card.dolly_in,
        dollyOut: card.dolly_out,
        spinClockwise: card.spin_clockwise,
        duration: card.duration,
        amount: card.amount,
        delay: card.delay,
        easing: card.easing,
        animateContainer: card.animate_container,
        next: card.next,
        stack: card.stack.map(this.cardTransform.bind(this)),
        tocDepth: card.toc_depth
      };
      return appCard;
    }
    async getRange({ indexId, startId, endId }) {
      if (startId == null) {
        startId = 0;
      }
      if (endId == null) {
        endId = 0;
      }
      const response = await fetch(`${this.serverUrl}${indexId}/range/${startId}/${endId}`, {});
      let cards = await response.json();
      return cards.map(this.cardTransform.bind(this));
    }
    async getContent({ indexId, contentId }) {
      const response = await fetch(`${this.serverUrl}${indexId}/content/${contentId}`, {});
      let card = await response.json();
      return this.cardTransform.bind(this)(card);
    }
    async getContents({ indexId, contentIds }) {
      let contents = [];
      for (let contentId of contentIds) {
        let content = await this.getContent({ indexId, contentId });
        contents.push(content);
      }
      return contents;
    }
    async getSitemap() {
      let response = await fetch(`${this.serverUrl}/sitemap`, {});
      return await response.json();
    }
  };
  var Data = class {
    /*
        Okay, so, "Data" is kind of a special thing:
        It's not the same thing as RealServer or StubServer, which are classes which are used to interact with the server.
        "Data's" job is to keep track of the state of the data that we've loaded from the server -
            like, if we've got 30 nodes loaded, and there are 10 blank nodes after that, and then there are 30 more nodes loaded?
            the RealServer/StubServer system is where we go to find out what's ON those nodes,
            but Data is responsible for keeping track of what nodes we've loaded and what nodes we haven't loaded.
    */
    constructor({ server }) {
      this.server = server;
      this.index = null;
      this.indexId = null;
      this.fullyLoadedBakedPotato = false;
      this.content = {};
      this.currentLocation = 0;
      this.currentId = null;
      setTimeout(this.ping.bind(this), 2e3);
      this.server.getSitemap().then((sitemap) => {
        this.sitemap = sitemap;
      });
    }
    async _addItem({ node }) {
      if (this.index?.containerClass != null) {
        node.containerClass = [...this.index.containerClass, ...node.containerClass];
      } else {
        console.warn("no container class");
        console.dir(this.index);
      }
      if (this.index?.extraClass != null) {
        node.extraClass = [...this.index.extraClass, ...node.extraClass];
      }
      this.content[node.id] = node;
    }
    async _addItems(nodes) {
      for (let node of nodes.filter((node2) => node2 != null)) {
        this._addItem({ node });
      }
    }
    async _loadEndCapItems() {
      let firstNodeId = this.index.contentIds[0];
      let lastNodeId = this.index.contentIds[this.index.contentIds.length - 1];
      if (this.content[lastNodeId] != null && this.content[lastNodeId] != null) {
        return;
      }
      let [firstNode, secondNode, penultimateNode, lastNode] = await this.server.getContents({
        indexId: this.indexId,
        contentIds: [
          firstNodeId,
          this.index.contentIds[1],
          this.index.contentIds[this.index.contentIds.length - 2],
          lastNodeId
        ]
      });
      this._addItems([firstNode, secondNode, penultimateNode, lastNode]);
      assert(this.content[firstNodeId] != null, `first node ${firstNodeId} not loaded`);
      assert(this.content[firstNodeId].id === firstNodeId, `first node ${firstNodeId} not loaded properly`);
      assert(this.index.contentIds[0] == this.content[firstNodeId].id);
    }
    async _loadIndexFromBeginning({ indexId }) {
      let index = this.index;
      let afterRange = await this.server.getRange({ indexId });
      if (index == null) {
        throw new Error(`Index ${indexId} not found`);
      }
      this.index = index;
      this.fullyLoadedBakedPotato = false;
      this._addItems([...afterRange]);
      if (index.count < PAGE_SIZE) {
        console.log("index is small enough to load all at once");
        this.bakePotato();
      } else {
        await this._loadEndCapItems();
      }
    }
    async _loadIndexFromMiddle({ indexId, contentId }) {
      contentId = contentId.replace("#", "");
      let index = this.index;
      let indexOfContent = index.contentIds.indexOf(contentId);
      let startOfPageIndex = Math.max(0, indexOfContent - PAGE_SIZE / 2);
      let startId = index.contentIds[startOfPageIndex];
      let [beforeRange, afterRange] = await Promise.all([
        this.server.getRange({ indexId, startId, endId: contentId }),
        this.server.getRange({ indexId, startId: contentId })
      ]);
      this.fullyLoadedBakedPotato = false;
      this._addItems([...beforeRange, ...afterRange]);
      if (this.index.count < PAGE_SIZE / 2) {
        this.fullyLoadedBakedPotato = true;
      } else {
        await this._loadEndCapItems();
      }
      this.currentLocation = 0;
      for (let i3 = 0; i3 < this.index.contentIds.length; i3++) {
        if (this.index.contentIds[i3] === contentId) {
          this.currentLocation = i3;
          break;
        }
      }
      this.currentId = contentId;
    }
    async loadIndex({ userSlug, contentSlug, contentId }) {
      let indexId = await this.server.getIndexId({ userSlug, contentSlug });
      this.indexId = indexId;
      console.log(`loading index id ${indexId}`);
      this.index = await this.server.getIndex({ indexId });
      if (contentId == null || contentId == "") {
        return this._loadIndexFromBeginning({ indexId });
      } else {
        return this._loadIndexFromMiddle({ indexId, contentId });
      }
    }
    async loadMoreContent({ user, indexId, contentId }) {
      if (this.fullyLoadedBakedPotato) {
        return;
      }
      let index = this.index;
      let indexOfContent = index.contentIds.indexOf(contentId);
      let startOfPageIndex = Math.max(0, indexOfContent - PAGE_SIZE / 2);
      let startId = index.contentIds[startOfPageIndex];
      let [beforeRange, afterRange] = await Promise.all([
        this.server.getRange({ user, indexId, startId, endId: contentId }),
        this.server.getRange({ user, indexId, startId: contentId })
      ]);
      this._addItems([...beforeRange, ...afterRange]);
    }
    async setCurrentLocation(n3) {
      this.currentLocation = n3;
      this.currentId = this.index.contentIds[n3];
    }
    async getCurrentLocation() {
      return this.currentLocation ?? 0;
    }
    bakePotato() {
      this.fullyLoadedBakedPotato = true;
      console.log("all content has been loaded");
    }
    async loadSomeNearbyContent() {
      if (this.fullyLoadedBakedPotato) {
        return;
      }
      for (let i3 = 0; i3 < this.index.contentIds.length; i3++) {
        let id = this.index.contentIds[i3];
        if (this.content[id] == null) {
          console.log(`loading content ${id}`);
          await this.loadMoreContent({ indexId: this.indexId, contentId: id });
          return;
        }
      }
      this.bakePotato();
    }
    async ping() {
      await this.loadSomeNearbyContent();
      setTimeout(this.ping.bind(this), 2e3);
    }
    getIndex() {
      if (this.index == null) {
        throw new Error("Index not loaded");
      }
      return this.index;
    }
    async getContent({ id }) {
      if (this.content[id] == null) {
        await this.loadMoreContent({ indexId: this.indexId, contentId: id });
      }
      let content = this.content[id];
      assert(content != null, `content ${id} not found`);
      assert(content.id === id, `content ${id} not found properly`);
      assert(content.type != null, `content ${id} has no type`);
      return this.content[id];
    }
    getContentOrder(id) {
      return this.index.contentIds.indexOf(id);
    }
    getNextContentId() {
      return this.index.contentIds[this.currentLocation + 1];
    }
    getPreviousContentId() {
      return this.index.contentIds[this.currentLocation - 1];
    }
    getSitemap() {
      return this.sitemap;
    }
  };
  function initialize({ serverUrl: serverUrl2 } = {}) {
    let server = new RealServer({ serverUrl: serverUrl2 });
    return new Data({ server });
  }

  // node_modules/animejs/lib/anime.es.js
  var defaultInstanceSettings = {
    update: null,
    begin: null,
    loopBegin: null,
    changeBegin: null,
    change: null,
    changeComplete: null,
    loopComplete: null,
    complete: null,
    loop: 1,
    direction: "normal",
    autoplay: true,
    timelineOffset: 0
  };
  var defaultTweenSettings = {
    duration: 1e3,
    delay: 0,
    endDelay: 0,
    easing: "easeOutElastic(1, .5)",
    round: 0
  };
  var validTransforms = ["translateX", "translateY", "translateZ", "rotate", "rotateX", "rotateY", "rotateZ", "scale", "scaleX", "scaleY", "scaleZ", "skew", "skewX", "skewY", "perspective", "matrix", "matrix3d"];
  var cache = {
    CSS: {},
    springs: {}
  };
  function minMax(val, min, max) {
    return Math.min(Math.max(val, min), max);
  }
  function stringContains(str, text) {
    return str.indexOf(text) > -1;
  }
  function applyArguments(func, args) {
    return func.apply(null, args);
  }
  var is = {
    arr: function(a3) {
      return Array.isArray(a3);
    },
    obj: function(a3) {
      return stringContains(Object.prototype.toString.call(a3), "Object");
    },
    pth: function(a3) {
      return is.obj(a3) && a3.hasOwnProperty("totalLength");
    },
    svg: function(a3) {
      return a3 instanceof SVGElement;
    },
    inp: function(a3) {
      return a3 instanceof HTMLInputElement;
    },
    dom: function(a3) {
      return a3.nodeType || is.svg(a3);
    },
    str: function(a3) {
      return typeof a3 === "string";
    },
    fnc: function(a3) {
      return typeof a3 === "function";
    },
    und: function(a3) {
      return typeof a3 === "undefined";
    },
    nil: function(a3) {
      return is.und(a3) || a3 === null;
    },
    hex: function(a3) {
      return /(^#[0-9A-F]{6}$)|(^#[0-9A-F]{3}$)/i.test(a3);
    },
    rgb: function(a3) {
      return /^rgb/.test(a3);
    },
    hsl: function(a3) {
      return /^hsl/.test(a3);
    },
    col: function(a3) {
      return is.hex(a3) || is.rgb(a3) || is.hsl(a3);
    },
    key: function(a3) {
      return !defaultInstanceSettings.hasOwnProperty(a3) && !defaultTweenSettings.hasOwnProperty(a3) && a3 !== "targets" && a3 !== "keyframes";
    }
  };
  function parseEasingParameters(string) {
    var match = /\(([^)]+)\)/.exec(string);
    return match ? match[1].split(",").map(function(p3) {
      return parseFloat(p3);
    }) : [];
  }
  function spring(string, duration) {
    var params = parseEasingParameters(string);
    var mass = minMax(is.und(params[0]) ? 1 : params[0], 0.1, 100);
    var stiffness = minMax(is.und(params[1]) ? 100 : params[1], 0.1, 100);
    var damping = minMax(is.und(params[2]) ? 10 : params[2], 0.1, 100);
    var velocity = minMax(is.und(params[3]) ? 0 : params[3], 0.1, 100);
    var w0 = Math.sqrt(stiffness / mass);
    var zeta = damping / (2 * Math.sqrt(stiffness * mass));
    var wd = zeta < 1 ? w0 * Math.sqrt(1 - zeta * zeta) : 0;
    var a3 = 1;
    var b3 = zeta < 1 ? (zeta * w0 + -velocity) / wd : -velocity + w0;
    function solver(t4) {
      var progress = duration ? duration * t4 / 1e3 : t4;
      if (zeta < 1) {
        progress = Math.exp(-progress * zeta * w0) * (a3 * Math.cos(wd * progress) + b3 * Math.sin(wd * progress));
      } else {
        progress = (a3 + b3 * progress) * Math.exp(-progress * w0);
      }
      if (t4 === 0 || t4 === 1) {
        return t4;
      }
      return 1 - progress;
    }
    function getDuration() {
      var cached = cache.springs[string];
      if (cached) {
        return cached;
      }
      var frame = 1 / 6;
      var elapsed = 0;
      var rest = 0;
      while (true) {
        elapsed += frame;
        if (solver(elapsed) === 1) {
          rest++;
          if (rest >= 16) {
            break;
          }
        } else {
          rest = 0;
        }
      }
      var duration2 = elapsed * frame * 1e3;
      cache.springs[string] = duration2;
      return duration2;
    }
    return duration ? solver : getDuration;
  }
  function steps(steps2) {
    if (steps2 === void 0)
      steps2 = 10;
    return function(t4) {
      return Math.ceil(minMax(t4, 1e-6, 1) * steps2) * (1 / steps2);
    };
  }
  var bezier = function() {
    var kSplineTableSize = 11;
    var kSampleStepSize = 1 / (kSplineTableSize - 1);
    function A2(aA1, aA2) {
      return 1 - 3 * aA2 + 3 * aA1;
    }
    function B3(aA1, aA2) {
      return 3 * aA2 - 6 * aA1;
    }
    function C(aA1) {
      return 3 * aA1;
    }
    function calcBezier(aT, aA1, aA2) {
      return ((A2(aA1, aA2) * aT + B3(aA1, aA2)) * aT + C(aA1)) * aT;
    }
    function getSlope(aT, aA1, aA2) {
      return 3 * A2(aA1, aA2) * aT * aT + 2 * B3(aA1, aA2) * aT + C(aA1);
    }
    function binarySubdivide(aX, aA, aB, mX1, mX2) {
      var currentX, currentT, i3 = 0;
      do {
        currentT = aA + (aB - aA) / 2;
        currentX = calcBezier(currentT, mX1, mX2) - aX;
        if (currentX > 0) {
          aB = currentT;
        } else {
          aA = currentT;
        }
      } while (Math.abs(currentX) > 1e-7 && ++i3 < 10);
      return currentT;
    }
    function newtonRaphsonIterate(aX, aGuessT, mX1, mX2) {
      for (var i3 = 0; i3 < 4; ++i3) {
        var currentSlope = getSlope(aGuessT, mX1, mX2);
        if (currentSlope === 0) {
          return aGuessT;
        }
        var currentX = calcBezier(aGuessT, mX1, mX2) - aX;
        aGuessT -= currentX / currentSlope;
      }
      return aGuessT;
    }
    function bezier2(mX1, mY1, mX2, mY2) {
      if (!(0 <= mX1 && mX1 <= 1 && 0 <= mX2 && mX2 <= 1)) {
        return;
      }
      var sampleValues = new Float32Array(kSplineTableSize);
      if (mX1 !== mY1 || mX2 !== mY2) {
        for (var i3 = 0; i3 < kSplineTableSize; ++i3) {
          sampleValues[i3] = calcBezier(i3 * kSampleStepSize, mX1, mX2);
        }
      }
      function getTForX(aX) {
        var intervalStart = 0;
        var currentSample = 1;
        var lastSample = kSplineTableSize - 1;
        for (; currentSample !== lastSample && sampleValues[currentSample] <= aX; ++currentSample) {
          intervalStart += kSampleStepSize;
        }
        --currentSample;
        var dist = (aX - sampleValues[currentSample]) / (sampleValues[currentSample + 1] - sampleValues[currentSample]);
        var guessForT = intervalStart + dist * kSampleStepSize;
        var initialSlope = getSlope(guessForT, mX1, mX2);
        if (initialSlope >= 1e-3) {
          return newtonRaphsonIterate(aX, guessForT, mX1, mX2);
        } else if (initialSlope === 0) {
          return guessForT;
        } else {
          return binarySubdivide(aX, intervalStart, intervalStart + kSampleStepSize, mX1, mX2);
        }
      }
      return function(x2) {
        if (mX1 === mY1 && mX2 === mY2) {
          return x2;
        }
        if (x2 === 0 || x2 === 1) {
          return x2;
        }
        return calcBezier(getTForX(x2), mY1, mY2);
      };
    }
    return bezier2;
  }();
  var penner = function() {
    var eases = { linear: function() {
      return function(t4) {
        return t4;
      };
    } };
    var functionEasings = {
      Sine: function() {
        return function(t4) {
          return 1 - Math.cos(t4 * Math.PI / 2);
        };
      },
      Circ: function() {
        return function(t4) {
          return 1 - Math.sqrt(1 - t4 * t4);
        };
      },
      Back: function() {
        return function(t4) {
          return t4 * t4 * (3 * t4 - 2);
        };
      },
      Bounce: function() {
        return function(t4) {
          var pow2, b3 = 4;
          while (t4 < ((pow2 = Math.pow(2, --b3)) - 1) / 11) {
          }
          return 1 / Math.pow(4, 3 - b3) - 7.5625 * Math.pow((pow2 * 3 - 2) / 22 - t4, 2);
        };
      },
      Elastic: function(amplitude, period) {
        if (amplitude === void 0)
          amplitude = 1;
        if (period === void 0)
          period = 0.5;
        var a3 = minMax(amplitude, 1, 10);
        var p3 = minMax(period, 0.1, 2);
        return function(t4) {
          return t4 === 0 || t4 === 1 ? t4 : -a3 * Math.pow(2, 10 * (t4 - 1)) * Math.sin((t4 - 1 - p3 / (Math.PI * 2) * Math.asin(1 / a3)) * (Math.PI * 2) / p3);
        };
      }
    };
    var baseEasings = ["Quad", "Cubic", "Quart", "Quint", "Expo"];
    baseEasings.forEach(function(name, i3) {
      functionEasings[name] = function() {
        return function(t4) {
          return Math.pow(t4, i3 + 2);
        };
      };
    });
    Object.keys(functionEasings).forEach(function(name) {
      var easeIn = functionEasings[name];
      eases["easeIn" + name] = easeIn;
      eases["easeOut" + name] = function(a3, b3) {
        return function(t4) {
          return 1 - easeIn(a3, b3)(1 - t4);
        };
      };
      eases["easeInOut" + name] = function(a3, b3) {
        return function(t4) {
          return t4 < 0.5 ? easeIn(a3, b3)(t4 * 2) / 2 : 1 - easeIn(a3, b3)(t4 * -2 + 2) / 2;
        };
      };
      eases["easeOutIn" + name] = function(a3, b3) {
        return function(t4) {
          return t4 < 0.5 ? (1 - easeIn(a3, b3)(1 - t4 * 2)) / 2 : (easeIn(a3, b3)(t4 * 2 - 1) + 1) / 2;
        };
      };
    });
    return eases;
  }();
  function parseEasings(easing, duration) {
    if (is.fnc(easing)) {
      return easing;
    }
    var name = easing.split("(")[0];
    var ease = penner[name];
    var args = parseEasingParameters(easing);
    switch (name) {
      case "spring":
        return spring(easing, duration);
      case "cubicBezier":
        return applyArguments(bezier, args);
      case "steps":
        return applyArguments(steps, args);
      default:
        return applyArguments(ease, args);
    }
  }
  function selectString(str) {
    try {
      var nodes = document.querySelectorAll(str);
      return nodes;
    } catch (e3) {
      return;
    }
  }
  function filterArray(arr, callback) {
    var len = arr.length;
    var thisArg = arguments.length >= 2 ? arguments[1] : void 0;
    var result = [];
    for (var i3 = 0; i3 < len; i3++) {
      if (i3 in arr) {
        var val = arr[i3];
        if (callback.call(thisArg, val, i3, arr)) {
          result.push(val);
        }
      }
    }
    return result;
  }
  function flattenArray(arr) {
    return arr.reduce(function(a3, b3) {
      return a3.concat(is.arr(b3) ? flattenArray(b3) : b3);
    }, []);
  }
  function toArray(o3) {
    if (is.arr(o3)) {
      return o3;
    }
    if (is.str(o3)) {
      o3 = selectString(o3) || o3;
    }
    if (o3 instanceof NodeList || o3 instanceof HTMLCollection) {
      return [].slice.call(o3);
    }
    return [o3];
  }
  function arrayContains(arr, val) {
    return arr.some(function(a3) {
      return a3 === val;
    });
  }
  function cloneObject(o3) {
    var clone = {};
    for (var p3 in o3) {
      clone[p3] = o3[p3];
    }
    return clone;
  }
  function replaceObjectProps(o1, o22) {
    var o3 = cloneObject(o1);
    for (var p3 in o1) {
      o3[p3] = o22.hasOwnProperty(p3) ? o22[p3] : o1[p3];
    }
    return o3;
  }
  function mergeObjects(o1, o22) {
    var o3 = cloneObject(o1);
    for (var p3 in o22) {
      o3[p3] = is.und(o1[p3]) ? o22[p3] : o1[p3];
    }
    return o3;
  }
  function rgbToRgba(rgbValue) {
    var rgb = /rgb\((\d+,\s*[\d]+,\s*[\d]+)\)/g.exec(rgbValue);
    return rgb ? "rgba(" + rgb[1] + ",1)" : rgbValue;
  }
  function hexToRgba(hexValue) {
    var rgx = /^#?([a-f\d])([a-f\d])([a-f\d])$/i;
    var hex = hexValue.replace(rgx, function(m3, r4, g4, b4) {
      return r4 + r4 + g4 + g4 + b4 + b4;
    });
    var rgb = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    var r3 = parseInt(rgb[1], 16);
    var g3 = parseInt(rgb[2], 16);
    var b3 = parseInt(rgb[3], 16);
    return "rgba(" + r3 + "," + g3 + "," + b3 + ",1)";
  }
  function hslToRgba(hslValue) {
    var hsl = /hsl\((\d+),\s*([\d.]+)%,\s*([\d.]+)%\)/g.exec(hslValue) || /hsla\((\d+),\s*([\d.]+)%,\s*([\d.]+)%,\s*([\d.]+)\)/g.exec(hslValue);
    var h3 = parseInt(hsl[1], 10) / 360;
    var s3 = parseInt(hsl[2], 10) / 100;
    var l3 = parseInt(hsl[3], 10) / 100;
    var a3 = hsl[4] || 1;
    function hue2rgb(p4, q3, t4) {
      if (t4 < 0) {
        t4 += 1;
      }
      if (t4 > 1) {
        t4 -= 1;
      }
      if (t4 < 1 / 6) {
        return p4 + (q3 - p4) * 6 * t4;
      }
      if (t4 < 1 / 2) {
        return q3;
      }
      if (t4 < 2 / 3) {
        return p4 + (q3 - p4) * (2 / 3 - t4) * 6;
      }
      return p4;
    }
    var r3, g3, b3;
    if (s3 == 0) {
      r3 = g3 = b3 = l3;
    } else {
      var q2 = l3 < 0.5 ? l3 * (1 + s3) : l3 + s3 - l3 * s3;
      var p3 = 2 * l3 - q2;
      r3 = hue2rgb(p3, q2, h3 + 1 / 3);
      g3 = hue2rgb(p3, q2, h3);
      b3 = hue2rgb(p3, q2, h3 - 1 / 3);
    }
    return "rgba(" + r3 * 255 + "," + g3 * 255 + "," + b3 * 255 + "," + a3 + ")";
  }
  function colorToRgb(val) {
    if (is.rgb(val)) {
      return rgbToRgba(val);
    }
    if (is.hex(val)) {
      return hexToRgba(val);
    }
    if (is.hsl(val)) {
      return hslToRgba(val);
    }
  }
  function getUnit(val) {
    var split = /[+-]?\d*\.?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?(%|px|pt|em|rem|in|cm|mm|ex|ch|pc|vw|vh|vmin|vmax|deg|rad|turn)?$/.exec(val);
    if (split) {
      return split[1];
    }
  }
  function getTransformUnit(propName) {
    if (stringContains(propName, "translate") || propName === "perspective") {
      return "px";
    }
    if (stringContains(propName, "rotate") || stringContains(propName, "skew")) {
      return "deg";
    }
  }
  function getFunctionValue(val, animatable) {
    if (!is.fnc(val)) {
      return val;
    }
    return val(animatable.target, animatable.id, animatable.total);
  }
  function getAttribute(el, prop) {
    return el.getAttribute(prop);
  }
  function convertPxToUnit(el, value, unit) {
    var valueUnit = getUnit(value);
    if (arrayContains([unit, "deg", "rad", "turn"], valueUnit)) {
      return value;
    }
    var cached = cache.CSS[value + unit];
    if (!is.und(cached)) {
      return cached;
    }
    var baseline = 100;
    var tempEl = document.createElement(el.tagName);
    var parentEl = el.parentNode && el.parentNode !== document ? el.parentNode : document.body;
    parentEl.appendChild(tempEl);
    tempEl.style.position = "absolute";
    tempEl.style.width = baseline + unit;
    var factor = baseline / tempEl.offsetWidth;
    parentEl.removeChild(tempEl);
    var convertedUnit = factor * parseFloat(value);
    cache.CSS[value + unit] = convertedUnit;
    return convertedUnit;
  }
  function getCSSValue(el, prop, unit) {
    if (prop in el.style) {
      var uppercasePropName = prop.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase();
      var value = el.style[prop] || getComputedStyle(el).getPropertyValue(uppercasePropName) || "0";
      return unit ? convertPxToUnit(el, value, unit) : value;
    }
  }
  function getAnimationType(el, prop) {
    if (is.dom(el) && !is.inp(el) && (!is.nil(getAttribute(el, prop)) || is.svg(el) && el[prop])) {
      return "attribute";
    }
    if (is.dom(el) && arrayContains(validTransforms, prop)) {
      return "transform";
    }
    if (is.dom(el) && (prop !== "transform" && getCSSValue(el, prop))) {
      return "css";
    }
    if (el[prop] != null) {
      return "object";
    }
  }
  function getElementTransforms(el) {
    if (!is.dom(el)) {
      return;
    }
    var str = el.style.transform || "";
    var reg = /(\w+)\(([^)]*)\)/g;
    var transforms = /* @__PURE__ */ new Map();
    var m3;
    while (m3 = reg.exec(str)) {
      transforms.set(m3[1], m3[2]);
    }
    return transforms;
  }
  function getTransformValue(el, propName, animatable, unit) {
    var defaultVal = stringContains(propName, "scale") ? 1 : 0 + getTransformUnit(propName);
    var value = getElementTransforms(el).get(propName) || defaultVal;
    if (animatable) {
      animatable.transforms.list.set(propName, value);
      animatable.transforms["last"] = propName;
    }
    return unit ? convertPxToUnit(el, value, unit) : value;
  }
  function getOriginalTargetValue(target, propName, unit, animatable) {
    switch (getAnimationType(target, propName)) {
      case "transform":
        return getTransformValue(target, propName, animatable, unit);
      case "css":
        return getCSSValue(target, propName, unit);
      case "attribute":
        return getAttribute(target, propName);
      default:
        return target[propName] || 0;
    }
  }
  function getRelativeValue(to, from) {
    var operator = /^(\*=|\+=|-=)/.exec(to);
    if (!operator) {
      return to;
    }
    var u3 = getUnit(to) || 0;
    var x2 = parseFloat(from);
    var y2 = parseFloat(to.replace(operator[0], ""));
    switch (operator[0][0]) {
      case "+":
        return x2 + y2 + u3;
      case "-":
        return x2 - y2 + u3;
      case "*":
        return x2 * y2 + u3;
    }
  }
  function validateValue(val, unit) {
    if (is.col(val)) {
      return colorToRgb(val);
    }
    if (/\s/g.test(val)) {
      return val;
    }
    var originalUnit = getUnit(val);
    var unitLess = originalUnit ? val.substr(0, val.length - originalUnit.length) : val;
    if (unit) {
      return unitLess + unit;
    }
    return unitLess;
  }
  function getDistance(p1, p22) {
    return Math.sqrt(Math.pow(p22.x - p1.x, 2) + Math.pow(p22.y - p1.y, 2));
  }
  function getCircleLength(el) {
    return Math.PI * 2 * getAttribute(el, "r");
  }
  function getRectLength(el) {
    return getAttribute(el, "width") * 2 + getAttribute(el, "height") * 2;
  }
  function getLineLength(el) {
    return getDistance(
      { x: getAttribute(el, "x1"), y: getAttribute(el, "y1") },
      { x: getAttribute(el, "x2"), y: getAttribute(el, "y2") }
    );
  }
  function getPolylineLength(el) {
    var points = el.points;
    var totalLength = 0;
    var previousPos;
    for (var i3 = 0; i3 < points.numberOfItems; i3++) {
      var currentPos = points.getItem(i3);
      if (i3 > 0) {
        totalLength += getDistance(previousPos, currentPos);
      }
      previousPos = currentPos;
    }
    return totalLength;
  }
  function getPolygonLength(el) {
    var points = el.points;
    return getPolylineLength(el) + getDistance(points.getItem(points.numberOfItems - 1), points.getItem(0));
  }
  function getTotalLength(el) {
    if (el.getTotalLength) {
      return el.getTotalLength();
    }
    switch (el.tagName.toLowerCase()) {
      case "circle":
        return getCircleLength(el);
      case "rect":
        return getRectLength(el);
      case "line":
        return getLineLength(el);
      case "polyline":
        return getPolylineLength(el);
      case "polygon":
        return getPolygonLength(el);
    }
  }
  function setDashoffset(el) {
    var pathLength = getTotalLength(el);
    el.setAttribute("stroke-dasharray", pathLength);
    return pathLength;
  }
  function getParentSvgEl(el) {
    var parentEl = el.parentNode;
    while (is.svg(parentEl)) {
      if (!is.svg(parentEl.parentNode)) {
        break;
      }
      parentEl = parentEl.parentNode;
    }
    return parentEl;
  }
  function getParentSvg(pathEl, svgData) {
    var svg = svgData || {};
    var parentSvgEl = svg.el || getParentSvgEl(pathEl);
    var rect = parentSvgEl.getBoundingClientRect();
    var viewBoxAttr = getAttribute(parentSvgEl, "viewBox");
    var width = rect.width;
    var height = rect.height;
    var viewBox = svg.viewBox || (viewBoxAttr ? viewBoxAttr.split(" ") : [0, 0, width, height]);
    return {
      el: parentSvgEl,
      viewBox,
      x: viewBox[0] / 1,
      y: viewBox[1] / 1,
      w: width,
      h: height,
      vW: viewBox[2],
      vH: viewBox[3]
    };
  }
  function getPath(path, percent) {
    var pathEl = is.str(path) ? selectString(path)[0] : path;
    var p3 = percent || 100;
    return function(property) {
      return {
        property,
        el: pathEl,
        svg: getParentSvg(pathEl),
        totalLength: getTotalLength(pathEl) * (p3 / 100)
      };
    };
  }
  function getPathProgress(path, progress, isPathTargetInsideSVG) {
    function point(offset) {
      if (offset === void 0)
        offset = 0;
      var l3 = progress + offset >= 1 ? progress + offset : 0;
      return path.el.getPointAtLength(l3);
    }
    var svg = getParentSvg(path.el, path.svg);
    var p3 = point();
    var p0 = point(-1);
    var p1 = point(1);
    var scaleX = isPathTargetInsideSVG ? 1 : svg.w / svg.vW;
    var scaleY = isPathTargetInsideSVG ? 1 : svg.h / svg.vH;
    switch (path.property) {
      case "x":
        return (p3.x - svg.x) * scaleX;
      case "y":
        return (p3.y - svg.y) * scaleY;
      case "angle":
        return Math.atan2(p1.y - p0.y, p1.x - p0.x) * 180 / Math.PI;
    }
  }
  function decomposeValue(val, unit) {
    var rgx = /[+-]?\d*\.?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?/g;
    var value = validateValue(is.pth(val) ? val.totalLength : val, unit) + "";
    return {
      original: value,
      numbers: value.match(rgx) ? value.match(rgx).map(Number) : [0],
      strings: is.str(val) || unit ? value.split(rgx) : []
    };
  }
  function parseTargets(targets) {
    var targetsArray = targets ? flattenArray(is.arr(targets) ? targets.map(toArray) : toArray(targets)) : [];
    return filterArray(targetsArray, function(item, pos, self) {
      return self.indexOf(item) === pos;
    });
  }
  function getAnimatables(targets) {
    var parsed = parseTargets(targets);
    return parsed.map(function(t4, i3) {
      return { target: t4, id: i3, total: parsed.length, transforms: { list: getElementTransforms(t4) } };
    });
  }
  function normalizePropertyTweens(prop, tweenSettings) {
    var settings = cloneObject(tweenSettings);
    if (/^spring/.test(settings.easing)) {
      settings.duration = spring(settings.easing);
    }
    if (is.arr(prop)) {
      var l3 = prop.length;
      var isFromTo = l3 === 2 && !is.obj(prop[0]);
      if (!isFromTo) {
        if (!is.fnc(tweenSettings.duration)) {
          settings.duration = tweenSettings.duration / l3;
        }
      } else {
        prop = { value: prop };
      }
    }
    var propArray = is.arr(prop) ? prop : [prop];
    return propArray.map(function(v3, i3) {
      var obj = is.obj(v3) && !is.pth(v3) ? v3 : { value: v3 };
      if (is.und(obj.delay)) {
        obj.delay = !i3 ? tweenSettings.delay : 0;
      }
      if (is.und(obj.endDelay)) {
        obj.endDelay = i3 === propArray.length - 1 ? tweenSettings.endDelay : 0;
      }
      return obj;
    }).map(function(k3) {
      return mergeObjects(k3, settings);
    });
  }
  function flattenKeyframes(keyframes) {
    var propertyNames = filterArray(flattenArray(keyframes.map(function(key) {
      return Object.keys(key);
    })), function(p3) {
      return is.key(p3);
    }).reduce(function(a3, b3) {
      if (a3.indexOf(b3) < 0) {
        a3.push(b3);
      }
      return a3;
    }, []);
    var properties = {};
    var loop = function(i4) {
      var propName = propertyNames[i4];
      properties[propName] = keyframes.map(function(key) {
        var newKey = {};
        for (var p3 in key) {
          if (is.key(p3)) {
            if (p3 == propName) {
              newKey.value = key[p3];
            }
          } else {
            newKey[p3] = key[p3];
          }
        }
        return newKey;
      });
    };
    for (var i3 = 0; i3 < propertyNames.length; i3++)
      loop(i3);
    return properties;
  }
  function getProperties(tweenSettings, params) {
    var properties = [];
    var keyframes = params.keyframes;
    if (keyframes) {
      params = mergeObjects(flattenKeyframes(keyframes), params);
    }
    for (var p3 in params) {
      if (is.key(p3)) {
        properties.push({
          name: p3,
          tweens: normalizePropertyTweens(params[p3], tweenSettings)
        });
      }
    }
    return properties;
  }
  function normalizeTweenValues(tween, animatable) {
    var t4 = {};
    for (var p3 in tween) {
      var value = getFunctionValue(tween[p3], animatable);
      if (is.arr(value)) {
        value = value.map(function(v3) {
          return getFunctionValue(v3, animatable);
        });
        if (value.length === 1) {
          value = value[0];
        }
      }
      t4[p3] = value;
    }
    t4.duration = parseFloat(t4.duration);
    t4.delay = parseFloat(t4.delay);
    return t4;
  }
  function normalizeTweens(prop, animatable) {
    var previousTween;
    return prop.tweens.map(function(t4) {
      var tween = normalizeTweenValues(t4, animatable);
      var tweenValue = tween.value;
      var to = is.arr(tweenValue) ? tweenValue[1] : tweenValue;
      var toUnit = getUnit(to);
      var originalValue = getOriginalTargetValue(animatable.target, prop.name, toUnit, animatable);
      var previousValue = previousTween ? previousTween.to.original : originalValue;
      var from = is.arr(tweenValue) ? tweenValue[0] : previousValue;
      var fromUnit = getUnit(from) || getUnit(originalValue);
      var unit = toUnit || fromUnit;
      if (is.und(to)) {
        to = previousValue;
      }
      tween.from = decomposeValue(from, unit);
      tween.to = decomposeValue(getRelativeValue(to, from), unit);
      tween.start = previousTween ? previousTween.end : 0;
      tween.end = tween.start + tween.delay + tween.duration + tween.endDelay;
      tween.easing = parseEasings(tween.easing, tween.duration);
      tween.isPath = is.pth(tweenValue);
      tween.isPathTargetInsideSVG = tween.isPath && is.svg(animatable.target);
      tween.isColor = is.col(tween.from.original);
      if (tween.isColor) {
        tween.round = 1;
      }
      previousTween = tween;
      return tween;
    });
  }
  var setProgressValue = {
    css: function(t4, p3, v3) {
      return t4.style[p3] = v3;
    },
    attribute: function(t4, p3, v3) {
      return t4.setAttribute(p3, v3);
    },
    object: function(t4, p3, v3) {
      return t4[p3] = v3;
    },
    transform: function(t4, p3, v3, transforms, manual) {
      transforms.list.set(p3, v3);
      if (p3 === transforms.last || manual) {
        var str = "";
        transforms.list.forEach(function(value, prop) {
          str += prop + "(" + value + ") ";
        });
        t4.style.transform = str;
      }
    }
  };
  function setTargetsValue(targets, properties) {
    var animatables = getAnimatables(targets);
    animatables.forEach(function(animatable) {
      for (var property in properties) {
        var value = getFunctionValue(properties[property], animatable);
        var target = animatable.target;
        var valueUnit = getUnit(value);
        var originalValue = getOriginalTargetValue(target, property, valueUnit, animatable);
        var unit = valueUnit || getUnit(originalValue);
        var to = getRelativeValue(validateValue(value, unit), originalValue);
        var animType = getAnimationType(target, property);
        setProgressValue[animType](target, property, to, animatable.transforms, true);
      }
    });
  }
  function createAnimation(animatable, prop) {
    var animType = getAnimationType(animatable.target, prop.name);
    if (animType) {
      var tweens = normalizeTweens(prop, animatable);
      var lastTween = tweens[tweens.length - 1];
      return {
        type: animType,
        property: prop.name,
        animatable,
        tweens,
        duration: lastTween.end,
        delay: tweens[0].delay,
        endDelay: lastTween.endDelay
      };
    }
  }
  function getAnimations(animatables, properties) {
    return filterArray(flattenArray(animatables.map(function(animatable) {
      return properties.map(function(prop) {
        return createAnimation(animatable, prop);
      });
    })), function(a3) {
      return !is.und(a3);
    });
  }
  function getInstanceTimings(animations, tweenSettings) {
    var animLength = animations.length;
    var getTlOffset = function(anim) {
      return anim.timelineOffset ? anim.timelineOffset : 0;
    };
    var timings = {};
    timings.duration = animLength ? Math.max.apply(Math, animations.map(function(anim) {
      return getTlOffset(anim) + anim.duration;
    })) : tweenSettings.duration;
    timings.delay = animLength ? Math.min.apply(Math, animations.map(function(anim) {
      return getTlOffset(anim) + anim.delay;
    })) : tweenSettings.delay;
    timings.endDelay = animLength ? timings.duration - Math.max.apply(Math, animations.map(function(anim) {
      return getTlOffset(anim) + anim.duration - anim.endDelay;
    })) : tweenSettings.endDelay;
    return timings;
  }
  var instanceID = 0;
  function createNewInstance(params) {
    var instanceSettings = replaceObjectProps(defaultInstanceSettings, params);
    var tweenSettings = replaceObjectProps(defaultTweenSettings, params);
    var properties = getProperties(tweenSettings, params);
    var animatables = getAnimatables(params.targets);
    var animations = getAnimations(animatables, properties);
    var timings = getInstanceTimings(animations, tweenSettings);
    var id = instanceID;
    instanceID++;
    return mergeObjects(instanceSettings, {
      id,
      children: [],
      animatables,
      animations,
      duration: timings.duration,
      delay: timings.delay,
      endDelay: timings.endDelay
    });
  }
  var activeInstances = [];
  var engine = function() {
    var raf;
    function play() {
      if (!raf && (!isDocumentHidden() || !anime.suspendWhenDocumentHidden) && activeInstances.length > 0) {
        raf = requestAnimationFrame(step);
      }
    }
    function step(t4) {
      var activeInstancesLength = activeInstances.length;
      var i3 = 0;
      while (i3 < activeInstancesLength) {
        var activeInstance = activeInstances[i3];
        if (!activeInstance.paused) {
          activeInstance.tick(t4);
          i3++;
        } else {
          activeInstances.splice(i3, 1);
          activeInstancesLength--;
        }
      }
      raf = i3 > 0 ? requestAnimationFrame(step) : void 0;
    }
    function handleVisibilityChange() {
      if (!anime.suspendWhenDocumentHidden) {
        return;
      }
      if (isDocumentHidden()) {
        raf = cancelAnimationFrame(raf);
      } else {
        activeInstances.forEach(
          function(instance) {
            return instance._onDocumentVisibility();
          }
        );
        engine();
      }
    }
    if (typeof document !== "undefined") {
      document.addEventListener("visibilitychange", handleVisibilityChange);
    }
    return play;
  }();
  function isDocumentHidden() {
    return !!document && document.hidden;
  }
  function anime(params) {
    if (params === void 0)
      params = {};
    var startTime = 0, lastTime = 0, now = 0;
    var children, childrenLength = 0;
    var resolve = null;
    function makePromise(instance2) {
      var promise2 = window.Promise && new Promise(function(_resolve) {
        return resolve = _resolve;
      });
      instance2.finished = promise2;
      return promise2;
    }
    var instance = createNewInstance(params);
    var promise = makePromise(instance);
    function toggleInstanceDirection() {
      var direction = instance.direction;
      if (direction !== "alternate") {
        instance.direction = direction !== "normal" ? "normal" : "reverse";
      }
      instance.reversed = !instance.reversed;
      children.forEach(function(child) {
        return child.reversed = instance.reversed;
      });
    }
    function adjustTime(time) {
      return instance.reversed ? instance.duration - time : time;
    }
    function resetTime() {
      startTime = 0;
      lastTime = adjustTime(instance.currentTime) * (1 / anime.speed);
    }
    function seekChild(time, child) {
      if (child) {
        child.seek(time - child.timelineOffset);
      }
    }
    function syncInstanceChildren(time) {
      if (!instance.reversePlayback) {
        for (var i3 = 0; i3 < childrenLength; i3++) {
          seekChild(time, children[i3]);
        }
      } else {
        for (var i$1 = childrenLength; i$1--; ) {
          seekChild(time, children[i$1]);
        }
      }
    }
    function setAnimationsProgress(insTime) {
      var i3 = 0;
      var animations = instance.animations;
      var animationsLength = animations.length;
      while (i3 < animationsLength) {
        var anim = animations[i3];
        var animatable = anim.animatable;
        var tweens = anim.tweens;
        var tweenLength = tweens.length - 1;
        var tween = tweens[tweenLength];
        if (tweenLength) {
          tween = filterArray(tweens, function(t4) {
            return insTime < t4.end;
          })[0] || tween;
        }
        var elapsed = minMax(insTime - tween.start - tween.delay, 0, tween.duration) / tween.duration;
        var eased = isNaN(elapsed) ? 1 : tween.easing(elapsed);
        var strings = tween.to.strings;
        var round = tween.round;
        var numbers = [];
        var toNumbersLength = tween.to.numbers.length;
        var progress = void 0;
        for (var n3 = 0; n3 < toNumbersLength; n3++) {
          var value = void 0;
          var toNumber = tween.to.numbers[n3];
          var fromNumber = tween.from.numbers[n3] || 0;
          if (!tween.isPath) {
            value = fromNumber + eased * (toNumber - fromNumber);
          } else {
            value = getPathProgress(tween.value, eased * toNumber, tween.isPathTargetInsideSVG);
          }
          if (round) {
            if (!(tween.isColor && n3 > 2)) {
              value = Math.round(value * round) / round;
            }
          }
          numbers.push(value);
        }
        var stringsLength = strings.length;
        if (!stringsLength) {
          progress = numbers[0];
        } else {
          progress = strings[0];
          for (var s3 = 0; s3 < stringsLength; s3++) {
            var a3 = strings[s3];
            var b3 = strings[s3 + 1];
            var n$1 = numbers[s3];
            if (!isNaN(n$1)) {
              if (!b3) {
                progress += n$1 + " ";
              } else {
                progress += n$1 + b3;
              }
            }
          }
        }
        setProgressValue[anim.type](animatable.target, anim.property, progress, animatable.transforms);
        anim.currentValue = progress;
        i3++;
      }
    }
    function setCallback(cb) {
      if (instance[cb] && !instance.passThrough) {
        instance[cb](instance);
      }
    }
    function countIteration() {
      if (instance.remaining && instance.remaining !== true) {
        instance.remaining--;
      }
    }
    function setInstanceProgress(engineTime) {
      var insDuration = instance.duration;
      var insDelay = instance.delay;
      var insEndDelay = insDuration - instance.endDelay;
      var insTime = adjustTime(engineTime);
      instance.progress = minMax(insTime / insDuration * 100, 0, 100);
      instance.reversePlayback = insTime < instance.currentTime;
      if (children) {
        syncInstanceChildren(insTime);
      }
      if (!instance.began && instance.currentTime > 0) {
        instance.began = true;
        setCallback("begin");
      }
      if (!instance.loopBegan && instance.currentTime > 0) {
        instance.loopBegan = true;
        setCallback("loopBegin");
      }
      if (insTime <= insDelay && instance.currentTime !== 0) {
        setAnimationsProgress(0);
      }
      if (insTime >= insEndDelay && instance.currentTime !== insDuration || !insDuration) {
        setAnimationsProgress(insDuration);
      }
      if (insTime > insDelay && insTime < insEndDelay) {
        if (!instance.changeBegan) {
          instance.changeBegan = true;
          instance.changeCompleted = false;
          setCallback("changeBegin");
        }
        setCallback("change");
        setAnimationsProgress(insTime);
      } else {
        if (instance.changeBegan) {
          instance.changeCompleted = true;
          instance.changeBegan = false;
          setCallback("changeComplete");
        }
      }
      instance.currentTime = minMax(insTime, 0, insDuration);
      if (instance.began) {
        setCallback("update");
      }
      if (engineTime >= insDuration) {
        lastTime = 0;
        countIteration();
        if (!instance.remaining) {
          instance.paused = true;
          if (!instance.completed) {
            instance.completed = true;
            setCallback("loopComplete");
            setCallback("complete");
            if (!instance.passThrough && "Promise" in window) {
              resolve();
              promise = makePromise(instance);
            }
          }
        } else {
          startTime = now;
          setCallback("loopComplete");
          instance.loopBegan = false;
          if (instance.direction === "alternate") {
            toggleInstanceDirection();
          }
        }
      }
    }
    instance.reset = function() {
      var direction = instance.direction;
      instance.passThrough = false;
      instance.currentTime = 0;
      instance.progress = 0;
      instance.paused = true;
      instance.began = false;
      instance.loopBegan = false;
      instance.changeBegan = false;
      instance.completed = false;
      instance.changeCompleted = false;
      instance.reversePlayback = false;
      instance.reversed = direction === "reverse";
      instance.remaining = instance.loop;
      children = instance.children;
      childrenLength = children.length;
      for (var i3 = childrenLength; i3--; ) {
        instance.children[i3].reset();
      }
      if (instance.reversed && instance.loop !== true || direction === "alternate" && instance.loop === 1) {
        instance.remaining++;
      }
      setAnimationsProgress(instance.reversed ? instance.duration : 0);
    };
    instance._onDocumentVisibility = resetTime;
    instance.set = function(targets, properties) {
      setTargetsValue(targets, properties);
      return instance;
    };
    instance.tick = function(t4) {
      now = t4;
      if (!startTime) {
        startTime = now;
      }
      setInstanceProgress((now + (lastTime - startTime)) * anime.speed);
    };
    instance.seek = function(time) {
      setInstanceProgress(adjustTime(time));
    };
    instance.pause = function() {
      instance.paused = true;
      resetTime();
    };
    instance.play = function() {
      if (!instance.paused) {
        return;
      }
      if (instance.completed) {
        instance.reset();
      }
      instance.paused = false;
      activeInstances.push(instance);
      resetTime();
      engine();
    };
    instance.reverse = function() {
      toggleInstanceDirection();
      instance.completed = instance.reversed ? false : true;
      resetTime();
    };
    instance.restart = function() {
      instance.reset();
      instance.play();
    };
    instance.remove = function(targets) {
      var targetsArray = parseTargets(targets);
      removeTargetsFromInstance(targetsArray, instance);
    };
    instance.reset();
    if (instance.autoplay) {
      instance.play();
    }
    return instance;
  }
  function removeTargetsFromAnimations(targetsArray, animations) {
    for (var a3 = animations.length; a3--; ) {
      if (arrayContains(targetsArray, animations[a3].animatable.target)) {
        animations.splice(a3, 1);
      }
    }
  }
  function removeTargetsFromInstance(targetsArray, instance) {
    var animations = instance.animations;
    var children = instance.children;
    removeTargetsFromAnimations(targetsArray, animations);
    for (var c3 = children.length; c3--; ) {
      var child = children[c3];
      var childAnimations = child.animations;
      removeTargetsFromAnimations(targetsArray, childAnimations);
      if (!childAnimations.length && !child.children.length) {
        children.splice(c3, 1);
      }
    }
    if (!animations.length && !children.length) {
      instance.pause();
    }
  }
  function removeTargetsFromActiveInstances(targets) {
    var targetsArray = parseTargets(targets);
    for (var i3 = activeInstances.length; i3--; ) {
      var instance = activeInstances[i3];
      removeTargetsFromInstance(targetsArray, instance);
    }
  }
  function stagger(val, params) {
    if (params === void 0)
      params = {};
    var direction = params.direction || "normal";
    var easing = params.easing ? parseEasings(params.easing) : null;
    var grid = params.grid;
    var axis = params.axis;
    var fromIndex = params.from || 0;
    var fromFirst = fromIndex === "first";
    var fromCenter = fromIndex === "center";
    var fromLast = fromIndex === "last";
    var isRange = is.arr(val);
    var val1 = isRange ? parseFloat(val[0]) : parseFloat(val);
    var val2 = isRange ? parseFloat(val[1]) : 0;
    var unit = getUnit(isRange ? val[1] : val) || 0;
    var start = params.start || 0 + (isRange ? val1 : 0);
    var values = [];
    var maxValue = 0;
    return function(el, i3, t4) {
      if (fromFirst) {
        fromIndex = 0;
      }
      if (fromCenter) {
        fromIndex = (t4 - 1) / 2;
      }
      if (fromLast) {
        fromIndex = t4 - 1;
      }
      if (!values.length) {
        for (var index = 0; index < t4; index++) {
          if (!grid) {
            values.push(Math.abs(fromIndex - index));
          } else {
            var fromX = !fromCenter ? fromIndex % grid[0] : (grid[0] - 1) / 2;
            var fromY = !fromCenter ? Math.floor(fromIndex / grid[0]) : (grid[1] - 1) / 2;
            var toX = index % grid[0];
            var toY = Math.floor(index / grid[0]);
            var distanceX = fromX - toX;
            var distanceY = fromY - toY;
            var value = Math.sqrt(distanceX * distanceX + distanceY * distanceY);
            if (axis === "x") {
              value = -distanceX;
            }
            if (axis === "y") {
              value = -distanceY;
            }
            values.push(value);
          }
          maxValue = Math.max.apply(Math, values);
        }
        if (easing) {
          values = values.map(function(val3) {
            return easing(val3 / maxValue) * maxValue;
          });
        }
        if (direction === "reverse") {
          values = values.map(function(val3) {
            return axis ? val3 < 0 ? val3 * -1 : -val3 : Math.abs(maxValue - val3);
          });
        }
      }
      var spacing = isRange ? (val2 - val1) / maxValue : val1;
      return start + spacing * (Math.round(values[i3] * 100) / 100) + unit;
    };
  }
  function timeline(params) {
    if (params === void 0)
      params = {};
    var tl = anime(params);
    tl.duration = 0;
    tl.add = function(instanceParams, timelineOffset) {
      var tlIndex = activeInstances.indexOf(tl);
      var children = tl.children;
      if (tlIndex > -1) {
        activeInstances.splice(tlIndex, 1);
      }
      function passThrough(ins2) {
        ins2.passThrough = true;
      }
      for (var i3 = 0; i3 < children.length; i3++) {
        passThrough(children[i3]);
      }
      var insParams = mergeObjects(instanceParams, replaceObjectProps(defaultTweenSettings, params));
      insParams.targets = insParams.targets || params.targets;
      var tlDuration = tl.duration;
      insParams.autoplay = false;
      insParams.direction = tl.direction;
      insParams.timelineOffset = is.und(timelineOffset) ? tlDuration : getRelativeValue(timelineOffset, tlDuration);
      passThrough(tl);
      tl.seek(insParams.timelineOffset);
      var ins = anime(insParams);
      passThrough(ins);
      children.push(ins);
      var timings = getInstanceTimings(children, params);
      tl.delay = timings.delay;
      tl.endDelay = timings.endDelay;
      tl.duration = timings.duration;
      tl.seek(0);
      tl.reset();
      if (tl.autoplay) {
        tl.play();
      }
      return tl;
    };
    return tl;
  }
  anime.version = "3.2.1";
  anime.speed = 1;
  anime.suspendWhenDocumentHidden = true;
  anime.running = activeInstances;
  anime.remove = removeTargetsFromActiveInstances;
  anime.get = getOriginalTargetValue;
  anime.set = setTargetsValue;
  anime.convertPx = convertPxToUnit;
  anime.path = getPath;
  anime.setDashoffset = setDashoffset;
  anime.stagger = stagger;
  anime.timeline = timeline;
  anime.easing = parseEasings;
  anime.penner = penner;
  anime.random = function(min, max) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  };
  var anime_es_default = anime;

  // node_modules/marked/lib/marked.esm.js
  function _getDefaults() {
    return {
      async: false,
      breaks: false,
      extensions: null,
      gfm: true,
      hooks: null,
      pedantic: false,
      renderer: null,
      silent: false,
      tokenizer: null,
      walkTokens: null
    };
  }
  var _defaults = _getDefaults();
  function changeDefaults(newDefaults) {
    _defaults = newDefaults;
  }
  var escapeTest = /[&<>"']/;
  var escapeReplace = new RegExp(escapeTest.source, "g");
  var escapeTestNoEncode = /[<>"']|&(?!(#\d{1,7}|#[Xx][a-fA-F0-9]{1,6}|\w+);)/;
  var escapeReplaceNoEncode = new RegExp(escapeTestNoEncode.source, "g");
  var escapeReplacements = {
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    '"': "&quot;",
    "'": "&#39;"
  };
  var getEscapeReplacement = (ch) => escapeReplacements[ch];
  function escape(html9, encode) {
    if (encode) {
      if (escapeTest.test(html9)) {
        return html9.replace(escapeReplace, getEscapeReplacement);
      }
    } else {
      if (escapeTestNoEncode.test(html9)) {
        return html9.replace(escapeReplaceNoEncode, getEscapeReplacement);
      }
    }
    return html9;
  }
  var unescapeTest = /&(#(?:\d+)|(?:#x[0-9A-Fa-f]+)|(?:\w+));?/ig;
  function unescape(html9) {
    return html9.replace(unescapeTest, (_, n3) => {
      n3 = n3.toLowerCase();
      if (n3 === "colon")
        return ":";
      if (n3.charAt(0) === "#") {
        return n3.charAt(1) === "x" ? String.fromCharCode(parseInt(n3.substring(2), 16)) : String.fromCharCode(+n3.substring(1));
      }
      return "";
    });
  }
  var caret = /(^|[^\[])\^/g;
  function edit(regex, opt) {
    regex = typeof regex === "string" ? regex : regex.source;
    opt = opt || "";
    const obj = {
      replace: (name, val) => {
        val = typeof val === "object" && "source" in val ? val.source : val;
        val = val.replace(caret, "$1");
        regex = regex.replace(name, val);
        return obj;
      },
      getRegex: () => {
        return new RegExp(regex, opt);
      }
    };
    return obj;
  }
  function cleanUrl(href) {
    try {
      href = encodeURI(href).replace(/%25/g, "%");
    } catch (e3) {
      return null;
    }
    return href;
  }
  var noopTest = { exec: () => null };
  function splitCells(tableRow, count) {
    const row = tableRow.replace(/\|/g, (match, offset, str) => {
      let escaped = false;
      let curr = offset;
      while (--curr >= 0 && str[curr] === "\\")
        escaped = !escaped;
      if (escaped) {
        return "|";
      } else {
        return " |";
      }
    }), cells = row.split(/ \|/);
    let i3 = 0;
    if (!cells[0].trim()) {
      cells.shift();
    }
    if (cells.length > 0 && !cells[cells.length - 1].trim()) {
      cells.pop();
    }
    if (count) {
      if (cells.length > count) {
        cells.splice(count);
      } else {
        while (cells.length < count)
          cells.push("");
      }
    }
    for (; i3 < cells.length; i3++) {
      cells[i3] = cells[i3].trim().replace(/\\\|/g, "|");
    }
    return cells;
  }
  function rtrim(str, c3, invert) {
    const l3 = str.length;
    if (l3 === 0) {
      return "";
    }
    let suffLen = 0;
    while (suffLen < l3) {
      const currChar = str.charAt(l3 - suffLen - 1);
      if (currChar === c3 && !invert) {
        suffLen++;
      } else if (currChar !== c3 && invert) {
        suffLen++;
      } else {
        break;
      }
    }
    return str.slice(0, l3 - suffLen);
  }
  function findClosingBracket(str, b3) {
    if (str.indexOf(b3[1]) === -1) {
      return -1;
    }
    let level = 0;
    for (let i3 = 0; i3 < str.length; i3++) {
      if (str[i3] === "\\") {
        i3++;
      } else if (str[i3] === b3[0]) {
        level++;
      } else if (str[i3] === b3[1]) {
        level--;
        if (level < 0) {
          return i3;
        }
      }
    }
    return -1;
  }
  function outputLink(cap, link, raw, lexer2) {
    const href = link.href;
    const title = link.title ? escape(link.title) : null;
    const text = cap[1].replace(/\\([\[\]])/g, "$1");
    if (cap[0].charAt(0) !== "!") {
      lexer2.state.inLink = true;
      const token = {
        type: "link",
        raw,
        href,
        title,
        text,
        tokens: lexer2.inlineTokens(text)
      };
      lexer2.state.inLink = false;
      return token;
    }
    return {
      type: "image",
      raw,
      href,
      title,
      text: escape(text)
    };
  }
  function indentCodeCompensation(raw, text) {
    const matchIndentToCode = raw.match(/^(\s+)(?:```)/);
    if (matchIndentToCode === null) {
      return text;
    }
    const indentToCode = matchIndentToCode[1];
    return text.split("\n").map((node) => {
      const matchIndentInNode = node.match(/^\s+/);
      if (matchIndentInNode === null) {
        return node;
      }
      const [indentInNode] = matchIndentInNode;
      if (indentInNode.length >= indentToCode.length) {
        return node.slice(indentToCode.length);
      }
      return node;
    }).join("\n");
  }
  var _Tokenizer = class {
    options;
    // TODO: Fix this rules type
    rules;
    lexer;
    constructor(options2) {
      this.options = options2 || _defaults;
    }
    space(src) {
      const cap = this.rules.block.newline.exec(src);
      if (cap && cap[0].length > 0) {
        return {
          type: "space",
          raw: cap[0]
        };
      }
    }
    code(src) {
      const cap = this.rules.block.code.exec(src);
      if (cap) {
        const text = cap[0].replace(/^ {1,4}/gm, "");
        return {
          type: "code",
          raw: cap[0],
          codeBlockStyle: "indented",
          text: !this.options.pedantic ? rtrim(text, "\n") : text
        };
      }
    }
    fences(src) {
      const cap = this.rules.block.fences.exec(src);
      if (cap) {
        const raw = cap[0];
        const text = indentCodeCompensation(raw, cap[3] || "");
        return {
          type: "code",
          raw,
          lang: cap[2] ? cap[2].trim().replace(this.rules.inline._escapes, "$1") : cap[2],
          text
        };
      }
    }
    heading(src) {
      const cap = this.rules.block.heading.exec(src);
      if (cap) {
        let text = cap[2].trim();
        if (/#$/.test(text)) {
          const trimmed = rtrim(text, "#");
          if (this.options.pedantic) {
            text = trimmed.trim();
          } else if (!trimmed || / $/.test(trimmed)) {
            text = trimmed.trim();
          }
        }
        return {
          type: "heading",
          raw: cap[0],
          depth: cap[1].length,
          text,
          tokens: this.lexer.inline(text)
        };
      }
    }
    hr(src) {
      const cap = this.rules.block.hr.exec(src);
      if (cap) {
        return {
          type: "hr",
          raw: cap[0]
        };
      }
    }
    blockquote(src) {
      const cap = this.rules.block.blockquote.exec(src);
      if (cap) {
        const text = rtrim(cap[0].replace(/^ *>[ \t]?/gm, ""), "\n");
        const top = this.lexer.state.top;
        this.lexer.state.top = true;
        const tokens = this.lexer.blockTokens(text);
        this.lexer.state.top = top;
        return {
          type: "blockquote",
          raw: cap[0],
          tokens,
          text
        };
      }
    }
    list(src) {
      let cap = this.rules.block.list.exec(src);
      if (cap) {
        let bull = cap[1].trim();
        const isordered = bull.length > 1;
        const list = {
          type: "list",
          raw: "",
          ordered: isordered,
          start: isordered ? +bull.slice(0, -1) : "",
          loose: false,
          items: []
        };
        bull = isordered ? `\\d{1,9}\\${bull.slice(-1)}` : `\\${bull}`;
        if (this.options.pedantic) {
          bull = isordered ? bull : "[*+-]";
        }
        const itemRegex = new RegExp(`^( {0,3}${bull})((?:[	 ][^\\n]*)?(?:\\n|$))`);
        let raw = "";
        let itemContents = "";
        let endsWithBlankLine = false;
        while (src) {
          let endEarly = false;
          if (!(cap = itemRegex.exec(src))) {
            break;
          }
          if (this.rules.block.hr.test(src)) {
            break;
          }
          raw = cap[0];
          src = src.substring(raw.length);
          let line = cap[2].split("\n", 1)[0].replace(/^\t+/, (t4) => " ".repeat(3 * t4.length));
          let nextLine = src.split("\n", 1)[0];
          let indent = 0;
          if (this.options.pedantic) {
            indent = 2;
            itemContents = line.trimStart();
          } else {
            indent = cap[2].search(/[^ ]/);
            indent = indent > 4 ? 1 : indent;
            itemContents = line.slice(indent);
            indent += cap[1].length;
          }
          let blankLine = false;
          if (!line && /^ *$/.test(nextLine)) {
            raw += nextLine + "\n";
            src = src.substring(nextLine.length + 1);
            endEarly = true;
          }
          if (!endEarly) {
            const nextBulletRegex = new RegExp(`^ {0,${Math.min(3, indent - 1)}}(?:[*+-]|\\d{1,9}[.)])((?:[ 	][^\\n]*)?(?:\\n|$))`);
            const hrRegex = new RegExp(`^ {0,${Math.min(3, indent - 1)}}((?:- *){3,}|(?:_ *){3,}|(?:\\* *){3,})(?:\\n+|$)`);
            const fencesBeginRegex = new RegExp(`^ {0,${Math.min(3, indent - 1)}}(?:\`\`\`|~~~)`);
            const headingBeginRegex = new RegExp(`^ {0,${Math.min(3, indent - 1)}}#`);
            while (src) {
              const rawLine = src.split("\n", 1)[0];
              nextLine = rawLine;
              if (this.options.pedantic) {
                nextLine = nextLine.replace(/^ {1,4}(?=( {4})*[^ ])/g, "  ");
              }
              if (fencesBeginRegex.test(nextLine)) {
                break;
              }
              if (headingBeginRegex.test(nextLine)) {
                break;
              }
              if (nextBulletRegex.test(nextLine)) {
                break;
              }
              if (hrRegex.test(src)) {
                break;
              }
              if (nextLine.search(/[^ ]/) >= indent || !nextLine.trim()) {
                itemContents += "\n" + nextLine.slice(indent);
              } else {
                if (blankLine) {
                  break;
                }
                if (line.search(/[^ ]/) >= 4) {
                  break;
                }
                if (fencesBeginRegex.test(line)) {
                  break;
                }
                if (headingBeginRegex.test(line)) {
                  break;
                }
                if (hrRegex.test(line)) {
                  break;
                }
                itemContents += "\n" + nextLine;
              }
              if (!blankLine && !nextLine.trim()) {
                blankLine = true;
              }
              raw += rawLine + "\n";
              src = src.substring(rawLine.length + 1);
              line = nextLine.slice(indent);
            }
          }
          if (!list.loose) {
            if (endsWithBlankLine) {
              list.loose = true;
            } else if (/\n *\n *$/.test(raw)) {
              endsWithBlankLine = true;
            }
          }
          let istask = null;
          let ischecked;
          if (this.options.gfm) {
            istask = /^\[[ xX]\] /.exec(itemContents);
            if (istask) {
              ischecked = istask[0] !== "[ ] ";
              itemContents = itemContents.replace(/^\[[ xX]\] +/, "");
            }
          }
          list.items.push({
            type: "list_item",
            raw,
            task: !!istask,
            checked: ischecked,
            loose: false,
            text: itemContents,
            tokens: []
          });
          list.raw += raw;
        }
        list.items[list.items.length - 1].raw = raw.trimEnd();
        list.items[list.items.length - 1].text = itemContents.trimEnd();
        list.raw = list.raw.trimEnd();
        for (let i3 = 0; i3 < list.items.length; i3++) {
          this.lexer.state.top = false;
          list.items[i3].tokens = this.lexer.blockTokens(list.items[i3].text, []);
          if (!list.loose) {
            const spacers = list.items[i3].tokens.filter((t4) => t4.type === "space");
            const hasMultipleLineBreaks = spacers.length > 0 && spacers.some((t4) => /\n.*\n/.test(t4.raw));
            list.loose = hasMultipleLineBreaks;
          }
        }
        if (list.loose) {
          for (let i3 = 0; i3 < list.items.length; i3++) {
            list.items[i3].loose = true;
          }
        }
        return list;
      }
    }
    html(src) {
      const cap = this.rules.block.html.exec(src);
      if (cap) {
        const token = {
          type: "html",
          block: true,
          raw: cap[0],
          pre: cap[1] === "pre" || cap[1] === "script" || cap[1] === "style",
          text: cap[0]
        };
        return token;
      }
    }
    def(src) {
      const cap = this.rules.block.def.exec(src);
      if (cap) {
        const tag = cap[1].toLowerCase().replace(/\s+/g, " ");
        const href = cap[2] ? cap[2].replace(/^<(.*)>$/, "$1").replace(this.rules.inline._escapes, "$1") : "";
        const title = cap[3] ? cap[3].substring(1, cap[3].length - 1).replace(this.rules.inline._escapes, "$1") : cap[3];
        return {
          type: "def",
          tag,
          raw: cap[0],
          href,
          title
        };
      }
    }
    table(src) {
      const cap = this.rules.block.table.exec(src);
      if (cap) {
        if (!/[:|]/.test(cap[2])) {
          return;
        }
        const item = {
          type: "table",
          raw: cap[0],
          header: splitCells(cap[1]).map((c3) => {
            return { text: c3, tokens: [] };
          }),
          align: cap[2].replace(/^\||\| *$/g, "").split("|"),
          rows: cap[3] && cap[3].trim() ? cap[3].replace(/\n[ \t]*$/, "").split("\n") : []
        };
        if (item.header.length === item.align.length) {
          let l3 = item.align.length;
          let i3, j3, k3, row;
          for (i3 = 0; i3 < l3; i3++) {
            const align = item.align[i3];
            if (align) {
              if (/^ *-+: *$/.test(align)) {
                item.align[i3] = "right";
              } else if (/^ *:-+: *$/.test(align)) {
                item.align[i3] = "center";
              } else if (/^ *:-+ *$/.test(align)) {
                item.align[i3] = "left";
              } else {
                item.align[i3] = null;
              }
            }
          }
          l3 = item.rows.length;
          for (i3 = 0; i3 < l3; i3++) {
            item.rows[i3] = splitCells(item.rows[i3], item.header.length).map((c3) => {
              return { text: c3, tokens: [] };
            });
          }
          l3 = item.header.length;
          for (j3 = 0; j3 < l3; j3++) {
            item.header[j3].tokens = this.lexer.inline(item.header[j3].text);
          }
          l3 = item.rows.length;
          for (j3 = 0; j3 < l3; j3++) {
            row = item.rows[j3];
            for (k3 = 0; k3 < row.length; k3++) {
              row[k3].tokens = this.lexer.inline(row[k3].text);
            }
          }
          return item;
        }
      }
    }
    lheading(src) {
      const cap = this.rules.block.lheading.exec(src);
      if (cap) {
        return {
          type: "heading",
          raw: cap[0],
          depth: cap[2].charAt(0) === "=" ? 1 : 2,
          text: cap[1],
          tokens: this.lexer.inline(cap[1])
        };
      }
    }
    paragraph(src) {
      const cap = this.rules.block.paragraph.exec(src);
      if (cap) {
        const text = cap[1].charAt(cap[1].length - 1) === "\n" ? cap[1].slice(0, -1) : cap[1];
        return {
          type: "paragraph",
          raw: cap[0],
          text,
          tokens: this.lexer.inline(text)
        };
      }
    }
    text(src) {
      const cap = this.rules.block.text.exec(src);
      if (cap) {
        return {
          type: "text",
          raw: cap[0],
          text: cap[0],
          tokens: this.lexer.inline(cap[0])
        };
      }
    }
    escape(src) {
      const cap = this.rules.inline.escape.exec(src);
      if (cap) {
        return {
          type: "escape",
          raw: cap[0],
          text: escape(cap[1])
        };
      }
    }
    tag(src) {
      const cap = this.rules.inline.tag.exec(src);
      if (cap) {
        if (!this.lexer.state.inLink && /^<a /i.test(cap[0])) {
          this.lexer.state.inLink = true;
        } else if (this.lexer.state.inLink && /^<\/a>/i.test(cap[0])) {
          this.lexer.state.inLink = false;
        }
        if (!this.lexer.state.inRawBlock && /^<(pre|code|kbd|script)(\s|>)/i.test(cap[0])) {
          this.lexer.state.inRawBlock = true;
        } else if (this.lexer.state.inRawBlock && /^<\/(pre|code|kbd|script)(\s|>)/i.test(cap[0])) {
          this.lexer.state.inRawBlock = false;
        }
        return {
          type: "html",
          raw: cap[0],
          inLink: this.lexer.state.inLink,
          inRawBlock: this.lexer.state.inRawBlock,
          block: false,
          text: cap[0]
        };
      }
    }
    link(src) {
      const cap = this.rules.inline.link.exec(src);
      if (cap) {
        const trimmedUrl = cap[2].trim();
        if (!this.options.pedantic && /^</.test(trimmedUrl)) {
          if (!/>$/.test(trimmedUrl)) {
            return;
          }
          const rtrimSlash = rtrim(trimmedUrl.slice(0, -1), "\\");
          if ((trimmedUrl.length - rtrimSlash.length) % 2 === 0) {
            return;
          }
        } else {
          const lastParenIndex = findClosingBracket(cap[2], "()");
          if (lastParenIndex > -1) {
            const start = cap[0].indexOf("!") === 0 ? 5 : 4;
            const linkLen = start + cap[1].length + lastParenIndex;
            cap[2] = cap[2].substring(0, lastParenIndex);
            cap[0] = cap[0].substring(0, linkLen).trim();
            cap[3] = "";
          }
        }
        let href = cap[2];
        let title = "";
        if (this.options.pedantic) {
          const link = /^([^'"]*[^\s])\s+(['"])(.*)\2/.exec(href);
          if (link) {
            href = link[1];
            title = link[3];
          }
        } else {
          title = cap[3] ? cap[3].slice(1, -1) : "";
        }
        href = href.trim();
        if (/^</.test(href)) {
          if (this.options.pedantic && !/>$/.test(trimmedUrl)) {
            href = href.slice(1);
          } else {
            href = href.slice(1, -1);
          }
        }
        return outputLink(cap, {
          href: href ? href.replace(this.rules.inline._escapes, "$1") : href,
          title: title ? title.replace(this.rules.inline._escapes, "$1") : title
        }, cap[0], this.lexer);
      }
    }
    reflink(src, links) {
      let cap;
      if ((cap = this.rules.inline.reflink.exec(src)) || (cap = this.rules.inline.nolink.exec(src))) {
        let link = (cap[2] || cap[1]).replace(/\s+/g, " ");
        link = links[link.toLowerCase()];
        if (!link) {
          const text = cap[0].charAt(0);
          return {
            type: "text",
            raw: text,
            text
          };
        }
        return outputLink(cap, link, cap[0], this.lexer);
      }
    }
    emStrong(src, maskedSrc, prevChar = "") {
      let match = this.rules.inline.emStrong.lDelim.exec(src);
      if (!match)
        return;
      if (match[3] && prevChar.match(/[\p{L}\p{N}]/u))
        return;
      const nextChar = match[1] || match[2] || "";
      if (!nextChar || !prevChar || this.rules.inline.punctuation.exec(prevChar)) {
        const lLength = [...match[0]].length - 1;
        let rDelim, rLength, delimTotal = lLength, midDelimTotal = 0;
        const endReg = match[0][0] === "*" ? this.rules.inline.emStrong.rDelimAst : this.rules.inline.emStrong.rDelimUnd;
        endReg.lastIndex = 0;
        maskedSrc = maskedSrc.slice(-1 * src.length + match[0].length - 1);
        while ((match = endReg.exec(maskedSrc)) != null) {
          rDelim = match[1] || match[2] || match[3] || match[4] || match[5] || match[6];
          if (!rDelim)
            continue;
          rLength = [...rDelim].length;
          if (match[3] || match[4]) {
            delimTotal += rLength;
            continue;
          } else if (match[5] || match[6]) {
            if (lLength % 3 && !((lLength + rLength) % 3)) {
              midDelimTotal += rLength;
              continue;
            }
          }
          delimTotal -= rLength;
          if (delimTotal > 0)
            continue;
          rLength = Math.min(rLength, rLength + delimTotal + midDelimTotal);
          const raw = [...src].slice(0, lLength + match.index + rLength + 1).join("");
          if (Math.min(lLength, rLength) % 2) {
            const text2 = raw.slice(1, -1);
            return {
              type: "em",
              raw,
              text: text2,
              tokens: this.lexer.inlineTokens(text2)
            };
          }
          const text = raw.slice(2, -2);
          return {
            type: "strong",
            raw,
            text,
            tokens: this.lexer.inlineTokens(text)
          };
        }
      }
    }
    codespan(src) {
      const cap = this.rules.inline.code.exec(src);
      if (cap) {
        let text = cap[2].replace(/\n/g, " ");
        const hasNonSpaceChars = /[^ ]/.test(text);
        const hasSpaceCharsOnBothEnds = /^ /.test(text) && / $/.test(text);
        if (hasNonSpaceChars && hasSpaceCharsOnBothEnds) {
          text = text.substring(1, text.length - 1);
        }
        text = escape(text, true);
        return {
          type: "codespan",
          raw: cap[0],
          text
        };
      }
    }
    br(src) {
      const cap = this.rules.inline.br.exec(src);
      if (cap) {
        return {
          type: "br",
          raw: cap[0]
        };
      }
    }
    del(src) {
      const cap = this.rules.inline.del.exec(src);
      if (cap) {
        return {
          type: "del",
          raw: cap[0],
          text: cap[2],
          tokens: this.lexer.inlineTokens(cap[2])
        };
      }
    }
    autolink(src) {
      const cap = this.rules.inline.autolink.exec(src);
      if (cap) {
        let text, href;
        if (cap[2] === "@") {
          text = escape(cap[1]);
          href = "mailto:" + text;
        } else {
          text = escape(cap[1]);
          href = text;
        }
        return {
          type: "link",
          raw: cap[0],
          text,
          href,
          tokens: [
            {
              type: "text",
              raw: text,
              text
            }
          ]
        };
      }
    }
    url(src) {
      let cap;
      if (cap = this.rules.inline.url.exec(src)) {
        let text, href;
        if (cap[2] === "@") {
          text = escape(cap[0]);
          href = "mailto:" + text;
        } else {
          let prevCapZero;
          do {
            prevCapZero = cap[0];
            cap[0] = this.rules.inline._backpedal.exec(cap[0])[0];
          } while (prevCapZero !== cap[0]);
          text = escape(cap[0]);
          if (cap[1] === "www.") {
            href = "http://" + cap[0];
          } else {
            href = cap[0];
          }
        }
        return {
          type: "link",
          raw: cap[0],
          text,
          href,
          tokens: [
            {
              type: "text",
              raw: text,
              text
            }
          ]
        };
      }
    }
    inlineText(src) {
      const cap = this.rules.inline.text.exec(src);
      if (cap) {
        let text;
        if (this.lexer.state.inRawBlock) {
          text = cap[0];
        } else {
          text = escape(cap[0]);
        }
        return {
          type: "text",
          raw: cap[0],
          text
        };
      }
    }
  };
  var block = {
    newline: /^(?: *(?:\n|$))+/,
    code: /^( {4}[^\n]+(?:\n(?: *(?:\n|$))*)?)+/,
    fences: /^ {0,3}(`{3,}(?=[^`\n]*(?:\n|$))|~{3,})([^\n]*)(?:\n|$)(?:|([\s\S]*?)(?:\n|$))(?: {0,3}\1[~`]* *(?=\n|$)|$)/,
    hr: /^ {0,3}((?:-[\t ]*){3,}|(?:_[ \t]*){3,}|(?:\*[ \t]*){3,})(?:\n+|$)/,
    heading: /^ {0,3}(#{1,6})(?=\s|$)(.*)(?:\n+|$)/,
    blockquote: /^( {0,3}> ?(paragraph|[^\n]*)(?:\n|$))+/,
    list: /^( {0,3}bull)([ \t][^\n]+?)?(?:\n|$)/,
    html: "^ {0,3}(?:<(script|pre|style|textarea)[\\s>][\\s\\S]*?(?:</\\1>[^\\n]*\\n+|$)|comment[^\\n]*(\\n+|$)|<\\?[\\s\\S]*?(?:\\?>\\n*|$)|<![A-Z][\\s\\S]*?(?:>\\n*|$)|<!\\[CDATA\\[[\\s\\S]*?(?:\\]\\]>\\n*|$)|</?(tag)(?: +|\\n|/?>)[\\s\\S]*?(?:(?:\\n *)+\\n|$)|<(?!script|pre|style|textarea)([a-z][\\w-]*)(?:attribute)*? */?>(?=[ \\t]*(?:\\n|$))[\\s\\S]*?(?:(?:\\n *)+\\n|$)|</(?!script|pre|style|textarea)[a-z][\\w-]*\\s*>(?=[ \\t]*(?:\\n|$))[\\s\\S]*?(?:(?:\\n *)+\\n|$))",
    def: /^ {0,3}\[(label)\]: *(?:\n *)?([^<\s][^\s]*|<.*?>)(?:(?: +(?:\n *)?| *\n *)(title))? *(?:\n+|$)/,
    table: noopTest,
    lheading: /^(?!bull )((?:.|\n(?!\s*?\n|bull ))+?)\n {0,3}(=+|-+) *(?:\n+|$)/,
    // regex template, placeholders will be replaced according to different paragraph
    // interruption rules of commonmark and the original markdown spec:
    _paragraph: /^([^\n]+(?:\n(?!hr|heading|lheading|blockquote|fences|list|html|table| +\n)[^\n]+)*)/,
    text: /^[^\n]+/
  };
  block._label = /(?!\s*\])(?:\\.|[^\[\]\\])+/;
  block._title = /(?:"(?:\\"?|[^"\\])*"|'[^'\n]*(?:\n[^'\n]+)*\n?'|\([^()]*\))/;
  block.def = edit(block.def).replace("label", block._label).replace("title", block._title).getRegex();
  block.bullet = /(?:[*+-]|\d{1,9}[.)])/;
  block.listItemStart = edit(/^( *)(bull) */).replace("bull", block.bullet).getRegex();
  block.list = edit(block.list).replace(/bull/g, block.bullet).replace("hr", "\\n+(?=\\1?(?:(?:- *){3,}|(?:_ *){3,}|(?:\\* *){3,})(?:\\n+|$))").replace("def", "\\n+(?=" + block.def.source + ")").getRegex();
  block._tag = "address|article|aside|base|basefont|blockquote|body|caption|center|col|colgroup|dd|details|dialog|dir|div|dl|dt|fieldset|figcaption|figure|footer|form|frame|frameset|h[1-6]|head|header|hr|html|iframe|legend|li|link|main|menu|menuitem|meta|nav|noframes|ol|optgroup|option|p|param|section|source|summary|table|tbody|td|tfoot|th|thead|title|tr|track|ul";
  block._comment = /<!--(?!-?>)[\s\S]*?(?:-->|$)/;
  block.html = edit(block.html, "i").replace("comment", block._comment).replace("tag", block._tag).replace("attribute", / +[a-zA-Z:_][\w.:-]*(?: *= *"[^"\n]*"| *= *'[^'\n]*'| *= *[^\s"'=<>`]+)?/).getRegex();
  block.lheading = edit(block.lheading).replace(/bull/g, block.bullet).getRegex();
  block.paragraph = edit(block._paragraph).replace("hr", block.hr).replace("heading", " {0,3}#{1,6} ").replace("|lheading", "").replace("|table", "").replace("blockquote", " {0,3}>").replace("fences", " {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n").replace("list", " {0,3}(?:[*+-]|1[.)]) ").replace("html", "</?(?:tag)(?: +|\\n|/?>)|<(?:script|pre|style|textarea|!--)").replace("tag", block._tag).getRegex();
  block.blockquote = edit(block.blockquote).replace("paragraph", block.paragraph).getRegex();
  block.normal = { ...block };
  block.gfm = {
    ...block.normal,
    table: "^ *([^\\n ].*)\\n {0,3}((?:\\| *)?:?-+:? *(?:\\| *:?-+:? *)*(?:\\| *)?)(?:\\n((?:(?! *\\n|hr|heading|blockquote|code|fences|list|html).*(?:\\n|$))*)\\n*|$)"
    // Cells
  };
  block.gfm.table = edit(block.gfm.table).replace("hr", block.hr).replace("heading", " {0,3}#{1,6} ").replace("blockquote", " {0,3}>").replace("code", " {4}[^\\n]").replace("fences", " {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n").replace("list", " {0,3}(?:[*+-]|1[.)]) ").replace("html", "</?(?:tag)(?: +|\\n|/?>)|<(?:script|pre|style|textarea|!--)").replace("tag", block._tag).getRegex();
  block.gfm.paragraph = edit(block._paragraph).replace("hr", block.hr).replace("heading", " {0,3}#{1,6} ").replace("|lheading", "").replace("table", block.gfm.table).replace("blockquote", " {0,3}>").replace("fences", " {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n").replace("list", " {0,3}(?:[*+-]|1[.)]) ").replace("html", "</?(?:tag)(?: +|\\n|/?>)|<(?:script|pre|style|textarea|!--)").replace("tag", block._tag).getRegex();
  block.pedantic = {
    ...block.normal,
    html: edit(`^ *(?:comment *(?:\\n|\\s*$)|<(tag)[\\s\\S]+?</\\1> *(?:\\n{2,}|\\s*$)|<tag(?:"[^"]*"|'[^']*'|\\s[^'"/>\\s]*)*?/?> *(?:\\n{2,}|\\s*$))`).replace("comment", block._comment).replace(/tag/g, "(?!(?:a|em|strong|small|s|cite|q|dfn|abbr|data|time|code|var|samp|kbd|sub|sup|i|b|u|mark|ruby|rt|rp|bdi|bdo|span|br|wbr|ins|del|img)\\b)\\w+(?!:|[^\\w\\s@]*@)\\b").getRegex(),
    def: /^ *\[([^\]]+)\]: *<?([^\s>]+)>?(?: +(["(][^\n]+[")]))? *(?:\n+|$)/,
    heading: /^(#{1,6})(.*)(?:\n+|$)/,
    fences: noopTest,
    lheading: /^(.+?)\n {0,3}(=+|-+) *(?:\n+|$)/,
    paragraph: edit(block.normal._paragraph).replace("hr", block.hr).replace("heading", " *#{1,6} *[^\n]").replace("lheading", block.lheading).replace("blockquote", " {0,3}>").replace("|fences", "").replace("|list", "").replace("|html", "").getRegex()
  };
  var inline = {
    escape: /^\\([!"#$%&'()*+,\-./:;<=>?@\[\]\\^_`{|}~])/,
    autolink: /^<(scheme:[^\s\x00-\x1f<>]*|email)>/,
    url: noopTest,
    tag: "^comment|^</[a-zA-Z][\\w:-]*\\s*>|^<[a-zA-Z][\\w-]*(?:attribute)*?\\s*/?>|^<\\?[\\s\\S]*?\\?>|^<![a-zA-Z]+\\s[\\s\\S]*?>|^<!\\[CDATA\\[[\\s\\S]*?\\]\\]>",
    link: /^!?\[(label)\]\(\s*(href)(?:\s+(title))?\s*\)/,
    reflink: /^!?\[(label)\]\[(ref)\]/,
    nolink: /^!?\[(ref)\](?:\[\])?/,
    reflinkSearch: "reflink|nolink(?!\\()",
    emStrong: {
      lDelim: /^(?:\*+(?:((?!\*)[punct])|[^\s*]))|^_+(?:((?!_)[punct])|([^\s_]))/,
      //         (1) and (2) can only be a Right Delimiter. (3) and (4) can only be Left.  (5) and (6) can be either Left or Right.
      //         | Skip orphan inside strong      | Consume to delim | (1) #***              | (2) a***#, a***                    | (3) #***a, ***a                  | (4) ***#                 | (5) #***#                         | (6) a***a
      rDelimAst: /^[^_*]*?__[^_*]*?\*[^_*]*?(?=__)|[^*]+(?=[^*])|(?!\*)[punct](\*+)(?=[\s]|$)|[^punct\s](\*+)(?!\*)(?=[punct\s]|$)|(?!\*)[punct\s](\*+)(?=[^punct\s])|[\s](\*+)(?!\*)(?=[punct])|(?!\*)[punct](\*+)(?!\*)(?=[punct])|[^punct\s](\*+)(?=[^punct\s])/,
      rDelimUnd: /^[^_*]*?\*\*[^_*]*?_[^_*]*?(?=\*\*)|[^_]+(?=[^_])|(?!_)[punct](_+)(?=[\s]|$)|[^punct\s](_+)(?!_)(?=[punct\s]|$)|(?!_)[punct\s](_+)(?=[^punct\s])|[\s](_+)(?!_)(?=[punct])|(?!_)[punct](_+)(?!_)(?=[punct])/
      // ^- Not allowed for _
    },
    code: /^(`+)([^`]|[^`][\s\S]*?[^`])\1(?!`)/,
    br: /^( {2,}|\\)\n(?!\s*$)/,
    del: noopTest,
    text: /^(`+|[^`])(?:(?= {2,}\n)|[\s\S]*?(?:(?=[\\<!\[`*_]|\b_|$)|[^ ](?= {2,}\n)))/,
    punctuation: /^((?![*_])[\spunctuation])/
  };
  inline._punctuation = "\\p{P}$+<=>`^|~";
  inline.punctuation = edit(inline.punctuation, "u").replace(/punctuation/g, inline._punctuation).getRegex();
  inline.blockSkip = /\[[^[\]]*?\]\([^\(\)]*?\)|`[^`]*?`|<[^<>]*?>/g;
  inline.anyPunctuation = /\\[punct]/g;
  inline._escapes = /\\([punct])/g;
  inline._comment = edit(block._comment).replace("(?:-->|$)", "-->").getRegex();
  inline.emStrong.lDelim = edit(inline.emStrong.lDelim, "u").replace(/punct/g, inline._punctuation).getRegex();
  inline.emStrong.rDelimAst = edit(inline.emStrong.rDelimAst, "gu").replace(/punct/g, inline._punctuation).getRegex();
  inline.emStrong.rDelimUnd = edit(inline.emStrong.rDelimUnd, "gu").replace(/punct/g, inline._punctuation).getRegex();
  inline.anyPunctuation = edit(inline.anyPunctuation, "gu").replace(/punct/g, inline._punctuation).getRegex();
  inline._escapes = edit(inline._escapes, "gu").replace(/punct/g, inline._punctuation).getRegex();
  inline._scheme = /[a-zA-Z][a-zA-Z0-9+.-]{1,31}/;
  inline._email = /[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+(@)[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+(?![-_])/;
  inline.autolink = edit(inline.autolink).replace("scheme", inline._scheme).replace("email", inline._email).getRegex();
  inline._attribute = /\s+[a-zA-Z:_][\w.:-]*(?:\s*=\s*"[^"]*"|\s*=\s*'[^']*'|\s*=\s*[^\s"'=<>`]+)?/;
  inline.tag = edit(inline.tag).replace("comment", inline._comment).replace("attribute", inline._attribute).getRegex();
  inline._label = /(?:\[(?:\\.|[^\[\]\\])*\]|\\.|`[^`]*`|[^\[\]\\`])*?/;
  inline._href = /<(?:\\.|[^\n<>\\])+>|[^\s\x00-\x1f]*/;
  inline._title = /"(?:\\"?|[^"\\])*"|'(?:\\'?|[^'\\])*'|\((?:\\\)?|[^)\\])*\)/;
  inline.link = edit(inline.link).replace("label", inline._label).replace("href", inline._href).replace("title", inline._title).getRegex();
  inline.reflink = edit(inline.reflink).replace("label", inline._label).replace("ref", block._label).getRegex();
  inline.nolink = edit(inline.nolink).replace("ref", block._label).getRegex();
  inline.reflinkSearch = edit(inline.reflinkSearch, "g").replace("reflink", inline.reflink).replace("nolink", inline.nolink).getRegex();
  inline.normal = { ...inline };
  inline.pedantic = {
    ...inline.normal,
    strong: {
      start: /^__|\*\*/,
      middle: /^__(?=\S)([\s\S]*?\S)__(?!_)|^\*\*(?=\S)([\s\S]*?\S)\*\*(?!\*)/,
      endAst: /\*\*(?!\*)/g,
      endUnd: /__(?!_)/g
    },
    em: {
      start: /^_|\*/,
      middle: /^()\*(?=\S)([\s\S]*?\S)\*(?!\*)|^_(?=\S)([\s\S]*?\S)_(?!_)/,
      endAst: /\*(?!\*)/g,
      endUnd: /_(?!_)/g
    },
    link: edit(/^!?\[(label)\]\((.*?)\)/).replace("label", inline._label).getRegex(),
    reflink: edit(/^!?\[(label)\]\s*\[([^\]]*)\]/).replace("label", inline._label).getRegex()
  };
  inline.gfm = {
    ...inline.normal,
    escape: edit(inline.escape).replace("])", "~|])").getRegex(),
    _extended_email: /[A-Za-z0-9._+-]+(@)[a-zA-Z0-9-_]+(?:\.[a-zA-Z0-9-_]*[a-zA-Z0-9])+(?![-_])/,
    url: /^((?:ftp|https?):\/\/|www\.)(?:[a-zA-Z0-9\-]+\.?)+[^\s<]*|^email/,
    _backpedal: /(?:[^?!.,:;*_'"~()&]+|\([^)]*\)|&(?![a-zA-Z0-9]+;$)|[?!.,:;*_'"~)]+(?!$))+/,
    del: /^(~~?)(?=[^\s~])([\s\S]*?[^\s~])\1(?=[^~]|$)/,
    text: /^([`~]+|[^`~])(?:(?= {2,}\n)|(?=[a-zA-Z0-9.!#$%&'*+\/=?_`{\|}~-]+@)|[\s\S]*?(?:(?=[\\<!\[`*~_]|\b_|https?:\/\/|ftp:\/\/|www\.|$)|[^ ](?= {2,}\n)|[^a-zA-Z0-9.!#$%&'*+\/=?_`{\|}~-](?=[a-zA-Z0-9.!#$%&'*+\/=?_`{\|}~-]+@)))/
  };
  inline.gfm.url = edit(inline.gfm.url, "i").replace("email", inline.gfm._extended_email).getRegex();
  inline.breaks = {
    ...inline.gfm,
    br: edit(inline.br).replace("{2,}", "*").getRegex(),
    text: edit(inline.gfm.text).replace("\\b_", "\\b_| {2,}\\n").replace(/\{2,\}/g, "*").getRegex()
  };
  var _Lexer = class __Lexer {
    tokens;
    options;
    state;
    tokenizer;
    inlineQueue;
    constructor(options2) {
      this.tokens = [];
      this.tokens.links = /* @__PURE__ */ Object.create(null);
      this.options = options2 || _defaults;
      this.options.tokenizer = this.options.tokenizer || new _Tokenizer();
      this.tokenizer = this.options.tokenizer;
      this.tokenizer.options = this.options;
      this.tokenizer.lexer = this;
      this.inlineQueue = [];
      this.state = {
        inLink: false,
        inRawBlock: false,
        top: true
      };
      const rules = {
        block: block.normal,
        inline: inline.normal
      };
      if (this.options.pedantic) {
        rules.block = block.pedantic;
        rules.inline = inline.pedantic;
      } else if (this.options.gfm) {
        rules.block = block.gfm;
        if (this.options.breaks) {
          rules.inline = inline.breaks;
        } else {
          rules.inline = inline.gfm;
        }
      }
      this.tokenizer.rules = rules;
    }
    /**
     * Expose Rules
     */
    static get rules() {
      return {
        block,
        inline
      };
    }
    /**
     * Static Lex Method
     */
    static lex(src, options2) {
      const lexer2 = new __Lexer(options2);
      return lexer2.lex(src);
    }
    /**
     * Static Lex Inline Method
     */
    static lexInline(src, options2) {
      const lexer2 = new __Lexer(options2);
      return lexer2.inlineTokens(src);
    }
    /**
     * Preprocessing
     */
    lex(src) {
      src = src.replace(/\r\n|\r/g, "\n");
      this.blockTokens(src, this.tokens);
      let next;
      while (next = this.inlineQueue.shift()) {
        this.inlineTokens(next.src, next.tokens);
      }
      return this.tokens;
    }
    blockTokens(src, tokens = []) {
      if (this.options.pedantic) {
        src = src.replace(/\t/g, "    ").replace(/^ +$/gm, "");
      } else {
        src = src.replace(/^( *)(\t+)/gm, (_, leading, tabs) => {
          return leading + "    ".repeat(tabs.length);
        });
      }
      let token;
      let lastToken;
      let cutSrc;
      let lastParagraphClipped;
      while (src) {
        if (this.options.extensions && this.options.extensions.block && this.options.extensions.block.some((extTokenizer) => {
          if (token = extTokenizer.call({ lexer: this }, src, tokens)) {
            src = src.substring(token.raw.length);
            tokens.push(token);
            return true;
          }
          return false;
        })) {
          continue;
        }
        if (token = this.tokenizer.space(src)) {
          src = src.substring(token.raw.length);
          if (token.raw.length === 1 && tokens.length > 0) {
            tokens[tokens.length - 1].raw += "\n";
          } else {
            tokens.push(token);
          }
          continue;
        }
        if (token = this.tokenizer.code(src)) {
          src = src.substring(token.raw.length);
          lastToken = tokens[tokens.length - 1];
          if (lastToken && (lastToken.type === "paragraph" || lastToken.type === "text")) {
            lastToken.raw += "\n" + token.raw;
            lastToken.text += "\n" + token.text;
            this.inlineQueue[this.inlineQueue.length - 1].src = lastToken.text;
          } else {
            tokens.push(token);
          }
          continue;
        }
        if (token = this.tokenizer.fences(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.heading(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.hr(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.blockquote(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.list(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.html(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.def(src)) {
          src = src.substring(token.raw.length);
          lastToken = tokens[tokens.length - 1];
          if (lastToken && (lastToken.type === "paragraph" || lastToken.type === "text")) {
            lastToken.raw += "\n" + token.raw;
            lastToken.text += "\n" + token.raw;
            this.inlineQueue[this.inlineQueue.length - 1].src = lastToken.text;
          } else if (!this.tokens.links[token.tag]) {
            this.tokens.links[token.tag] = {
              href: token.href,
              title: token.title
            };
          }
          continue;
        }
        if (token = this.tokenizer.table(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.lheading(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        cutSrc = src;
        if (this.options.extensions && this.options.extensions.startBlock) {
          let startIndex = Infinity;
          const tempSrc = src.slice(1);
          let tempStart;
          this.options.extensions.startBlock.forEach((getStartIndex) => {
            tempStart = getStartIndex.call({ lexer: this }, tempSrc);
            if (typeof tempStart === "number" && tempStart >= 0) {
              startIndex = Math.min(startIndex, tempStart);
            }
          });
          if (startIndex < Infinity && startIndex >= 0) {
            cutSrc = src.substring(0, startIndex + 1);
          }
        }
        if (this.state.top && (token = this.tokenizer.paragraph(cutSrc))) {
          lastToken = tokens[tokens.length - 1];
          if (lastParagraphClipped && lastToken.type === "paragraph") {
            lastToken.raw += "\n" + token.raw;
            lastToken.text += "\n" + token.text;
            this.inlineQueue.pop();
            this.inlineQueue[this.inlineQueue.length - 1].src = lastToken.text;
          } else {
            tokens.push(token);
          }
          lastParagraphClipped = cutSrc.length !== src.length;
          src = src.substring(token.raw.length);
          continue;
        }
        if (token = this.tokenizer.text(src)) {
          src = src.substring(token.raw.length);
          lastToken = tokens[tokens.length - 1];
          if (lastToken && lastToken.type === "text") {
            lastToken.raw += "\n" + token.raw;
            lastToken.text += "\n" + token.text;
            this.inlineQueue.pop();
            this.inlineQueue[this.inlineQueue.length - 1].src = lastToken.text;
          } else {
            tokens.push(token);
          }
          continue;
        }
        if (src) {
          const errMsg = "Infinite loop on byte: " + src.charCodeAt(0);
          if (this.options.silent) {
            console.error(errMsg);
            break;
          } else {
            throw new Error(errMsg);
          }
        }
      }
      this.state.top = true;
      return tokens;
    }
    inline(src, tokens = []) {
      this.inlineQueue.push({ src, tokens });
      return tokens;
    }
    /**
     * Lexing/Compiling
     */
    inlineTokens(src, tokens = []) {
      let token, lastToken, cutSrc;
      let maskedSrc = src;
      let match;
      let keepPrevChar, prevChar;
      if (this.tokens.links) {
        const links = Object.keys(this.tokens.links);
        if (links.length > 0) {
          while ((match = this.tokenizer.rules.inline.reflinkSearch.exec(maskedSrc)) != null) {
            if (links.includes(match[0].slice(match[0].lastIndexOf("[") + 1, -1))) {
              maskedSrc = maskedSrc.slice(0, match.index) + "[" + "a".repeat(match[0].length - 2) + "]" + maskedSrc.slice(this.tokenizer.rules.inline.reflinkSearch.lastIndex);
            }
          }
        }
      }
      while ((match = this.tokenizer.rules.inline.blockSkip.exec(maskedSrc)) != null) {
        maskedSrc = maskedSrc.slice(0, match.index) + "[" + "a".repeat(match[0].length - 2) + "]" + maskedSrc.slice(this.tokenizer.rules.inline.blockSkip.lastIndex);
      }
      while ((match = this.tokenizer.rules.inline.anyPunctuation.exec(maskedSrc)) != null) {
        maskedSrc = maskedSrc.slice(0, match.index) + "++" + maskedSrc.slice(this.tokenizer.rules.inline.anyPunctuation.lastIndex);
      }
      while (src) {
        if (!keepPrevChar) {
          prevChar = "";
        }
        keepPrevChar = false;
        if (this.options.extensions && this.options.extensions.inline && this.options.extensions.inline.some((extTokenizer) => {
          if (token = extTokenizer.call({ lexer: this }, src, tokens)) {
            src = src.substring(token.raw.length);
            tokens.push(token);
            return true;
          }
          return false;
        })) {
          continue;
        }
        if (token = this.tokenizer.escape(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.tag(src)) {
          src = src.substring(token.raw.length);
          lastToken = tokens[tokens.length - 1];
          if (lastToken && token.type === "text" && lastToken.type === "text") {
            lastToken.raw += token.raw;
            lastToken.text += token.text;
          } else {
            tokens.push(token);
          }
          continue;
        }
        if (token = this.tokenizer.link(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.reflink(src, this.tokens.links)) {
          src = src.substring(token.raw.length);
          lastToken = tokens[tokens.length - 1];
          if (lastToken && token.type === "text" && lastToken.type === "text") {
            lastToken.raw += token.raw;
            lastToken.text += token.text;
          } else {
            tokens.push(token);
          }
          continue;
        }
        if (token = this.tokenizer.emStrong(src, maskedSrc, prevChar)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.codespan(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.br(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.del(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (token = this.tokenizer.autolink(src)) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        if (!this.state.inLink && (token = this.tokenizer.url(src))) {
          src = src.substring(token.raw.length);
          tokens.push(token);
          continue;
        }
        cutSrc = src;
        if (this.options.extensions && this.options.extensions.startInline) {
          let startIndex = Infinity;
          const tempSrc = src.slice(1);
          let tempStart;
          this.options.extensions.startInline.forEach((getStartIndex) => {
            tempStart = getStartIndex.call({ lexer: this }, tempSrc);
            if (typeof tempStart === "number" && tempStart >= 0) {
              startIndex = Math.min(startIndex, tempStart);
            }
          });
          if (startIndex < Infinity && startIndex >= 0) {
            cutSrc = src.substring(0, startIndex + 1);
          }
        }
        if (token = this.tokenizer.inlineText(cutSrc)) {
          src = src.substring(token.raw.length);
          if (token.raw.slice(-1) !== "_") {
            prevChar = token.raw.slice(-1);
          }
          keepPrevChar = true;
          lastToken = tokens[tokens.length - 1];
          if (lastToken && lastToken.type === "text") {
            lastToken.raw += token.raw;
            lastToken.text += token.text;
          } else {
            tokens.push(token);
          }
          continue;
        }
        if (src) {
          const errMsg = "Infinite loop on byte: " + src.charCodeAt(0);
          if (this.options.silent) {
            console.error(errMsg);
            break;
          } else {
            throw new Error(errMsg);
          }
        }
      }
      return tokens;
    }
  };
  var _Renderer = class {
    options;
    constructor(options2) {
      this.options = options2 || _defaults;
    }
    code(code, infostring, escaped) {
      const lang = (infostring || "").match(/^\S*/)?.[0];
      code = code.replace(/\n$/, "") + "\n";
      if (!lang) {
        return "<pre><code>" + (escaped ? code : escape(code, true)) + "</code></pre>\n";
      }
      return '<pre><code class="language-' + escape(lang) + '">' + (escaped ? code : escape(code, true)) + "</code></pre>\n";
    }
    blockquote(quote) {
      return `<blockquote>
${quote}</blockquote>
`;
    }
    html(html9, block2) {
      return html9;
    }
    heading(text, level, raw) {
      return `<h${level}>${text}</h${level}>
`;
    }
    hr() {
      return "<hr>\n";
    }
    list(body, ordered, start) {
      const type = ordered ? "ol" : "ul";
      const startatt = ordered && start !== 1 ? ' start="' + start + '"' : "";
      return "<" + type + startatt + ">\n" + body + "</" + type + ">\n";
    }
    listitem(text, task, checked) {
      return `<li>${text}</li>
`;
    }
    checkbox(checked) {
      return "<input " + (checked ? 'checked="" ' : "") + 'disabled="" type="checkbox">';
    }
    paragraph(text) {
      return `<p>${text}</p>
`;
    }
    table(header, body) {
      if (body)
        body = `<tbody>${body}</tbody>`;
      return "<table>\n<thead>\n" + header + "</thead>\n" + body + "</table>\n";
    }
    tablerow(content) {
      return `<tr>
${content}</tr>
`;
    }
    tablecell(content, flags) {
      const type = flags.header ? "th" : "td";
      const tag = flags.align ? `<${type} align="${flags.align}">` : `<${type}>`;
      return tag + content + `</${type}>
`;
    }
    /**
     * span level renderer
     */
    strong(text) {
      return `<strong>${text}</strong>`;
    }
    em(text) {
      return `<em>${text}</em>`;
    }
    codespan(text) {
      return `<code>${text}</code>`;
    }
    br() {
      return "<br>";
    }
    del(text) {
      return `<del>${text}</del>`;
    }
    link(href, title, text) {
      const cleanHref = cleanUrl(href);
      if (cleanHref === null) {
        return text;
      }
      href = cleanHref;
      let out = '<a href="' + href + '"';
      if (title) {
        out += ' title="' + title + '"';
      }
      out += ">" + text + "</a>";
      return out;
    }
    image(href, title, text) {
      const cleanHref = cleanUrl(href);
      if (cleanHref === null) {
        return text;
      }
      href = cleanHref;
      let out = `<img src="${href}" alt="${text}"`;
      if (title) {
        out += ` title="${title}"`;
      }
      out += ">";
      return out;
    }
    text(text) {
      return text;
    }
  };
  var _TextRenderer = class {
    // no need for block level renderers
    strong(text) {
      return text;
    }
    em(text) {
      return text;
    }
    codespan(text) {
      return text;
    }
    del(text) {
      return text;
    }
    html(text) {
      return text;
    }
    text(text) {
      return text;
    }
    link(href, title, text) {
      return "" + text;
    }
    image(href, title, text) {
      return "" + text;
    }
    br() {
      return "";
    }
  };
  var _Parser = class __Parser {
    options;
    renderer;
    textRenderer;
    constructor(options2) {
      this.options = options2 || _defaults;
      this.options.renderer = this.options.renderer || new _Renderer();
      this.renderer = this.options.renderer;
      this.renderer.options = this.options;
      this.textRenderer = new _TextRenderer();
    }
    /**
     * Static Parse Method
     */
    static parse(tokens, options2) {
      const parser2 = new __Parser(options2);
      return parser2.parse(tokens);
    }
    /**
     * Static Parse Inline Method
     */
    static parseInline(tokens, options2) {
      const parser2 = new __Parser(options2);
      return parser2.parseInline(tokens);
    }
    /**
     * Parse Loop
     */
    parse(tokens, top = true) {
      let out = "";
      for (let i3 = 0; i3 < tokens.length; i3++) {
        const token = tokens[i3];
        if (this.options.extensions && this.options.extensions.renderers && this.options.extensions.renderers[token.type]) {
          const genericToken = token;
          const ret = this.options.extensions.renderers[genericToken.type].call({ parser: this }, genericToken);
          if (ret !== false || !["space", "hr", "heading", "code", "table", "blockquote", "list", "html", "paragraph", "text"].includes(genericToken.type)) {
            out += ret || "";
            continue;
          }
        }
        switch (token.type) {
          case "space": {
            continue;
          }
          case "hr": {
            out += this.renderer.hr();
            continue;
          }
          case "heading": {
            const headingToken = token;
            out += this.renderer.heading(this.parseInline(headingToken.tokens), headingToken.depth, unescape(this.parseInline(headingToken.tokens, this.textRenderer)));
            continue;
          }
          case "code": {
            const codeToken = token;
            out += this.renderer.code(codeToken.text, codeToken.lang, !!codeToken.escaped);
            continue;
          }
          case "table": {
            const tableToken = token;
            let header = "";
            let cell = "";
            for (let j3 = 0; j3 < tableToken.header.length; j3++) {
              cell += this.renderer.tablecell(this.parseInline(tableToken.header[j3].tokens), { header: true, align: tableToken.align[j3] });
            }
            header += this.renderer.tablerow(cell);
            let body = "";
            for (let j3 = 0; j3 < tableToken.rows.length; j3++) {
              const row = tableToken.rows[j3];
              cell = "";
              for (let k3 = 0; k3 < row.length; k3++) {
                cell += this.renderer.tablecell(this.parseInline(row[k3].tokens), { header: false, align: tableToken.align[k3] });
              }
              body += this.renderer.tablerow(cell);
            }
            out += this.renderer.table(header, body);
            continue;
          }
          case "blockquote": {
            const blockquoteToken = token;
            const body = this.parse(blockquoteToken.tokens);
            out += this.renderer.blockquote(body);
            continue;
          }
          case "list": {
            const listToken = token;
            const ordered = listToken.ordered;
            const start = listToken.start;
            const loose = listToken.loose;
            let body = "";
            for (let j3 = 0; j3 < listToken.items.length; j3++) {
              const item = listToken.items[j3];
              const checked = item.checked;
              const task = item.task;
              let itemBody = "";
              if (item.task) {
                const checkbox = this.renderer.checkbox(!!checked);
                if (loose) {
                  if (item.tokens.length > 0 && item.tokens[0].type === "paragraph") {
                    item.tokens[0].text = checkbox + " " + item.tokens[0].text;
                    if (item.tokens[0].tokens && item.tokens[0].tokens.length > 0 && item.tokens[0].tokens[0].type === "text") {
                      item.tokens[0].tokens[0].text = checkbox + " " + item.tokens[0].tokens[0].text;
                    }
                  } else {
                    item.tokens.unshift({
                      type: "text",
                      text: checkbox + " "
                    });
                  }
                } else {
                  itemBody += checkbox + " ";
                }
              }
              itemBody += this.parse(item.tokens, loose);
              body += this.renderer.listitem(itemBody, task, !!checked);
            }
            out += this.renderer.list(body, ordered, start);
            continue;
          }
          case "html": {
            const htmlToken = token;
            out += this.renderer.html(htmlToken.text, htmlToken.block);
            continue;
          }
          case "paragraph": {
            const paragraphToken = token;
            out += this.renderer.paragraph(this.parseInline(paragraphToken.tokens));
            continue;
          }
          case "text": {
            let textToken = token;
            let body = textToken.tokens ? this.parseInline(textToken.tokens) : textToken.text;
            while (i3 + 1 < tokens.length && tokens[i3 + 1].type === "text") {
              textToken = tokens[++i3];
              body += "\n" + (textToken.tokens ? this.parseInline(textToken.tokens) : textToken.text);
            }
            out += top ? this.renderer.paragraph(body) : body;
            continue;
          }
          default: {
            const errMsg = 'Token with "' + token.type + '" type was not found.';
            if (this.options.silent) {
              console.error(errMsg);
              return "";
            } else {
              throw new Error(errMsg);
            }
          }
        }
      }
      return out;
    }
    /**
     * Parse Inline Tokens
     */
    parseInline(tokens, renderer) {
      renderer = renderer || this.renderer;
      let out = "";
      for (let i3 = 0; i3 < tokens.length; i3++) {
        const token = tokens[i3];
        if (this.options.extensions && this.options.extensions.renderers && this.options.extensions.renderers[token.type]) {
          const ret = this.options.extensions.renderers[token.type].call({ parser: this }, token);
          if (ret !== false || !["escape", "html", "link", "image", "strong", "em", "codespan", "br", "del", "text"].includes(token.type)) {
            out += ret || "";
            continue;
          }
        }
        switch (token.type) {
          case "escape": {
            const escapeToken = token;
            out += renderer.text(escapeToken.text);
            break;
          }
          case "html": {
            const tagToken = token;
            out += renderer.html(tagToken.text);
            break;
          }
          case "link": {
            const linkToken = token;
            out += renderer.link(linkToken.href, linkToken.title, this.parseInline(linkToken.tokens, renderer));
            break;
          }
          case "image": {
            const imageToken = token;
            out += renderer.image(imageToken.href, imageToken.title, imageToken.text);
            break;
          }
          case "strong": {
            const strongToken = token;
            out += renderer.strong(this.parseInline(strongToken.tokens, renderer));
            break;
          }
          case "em": {
            const emToken = token;
            out += renderer.em(this.parseInline(emToken.tokens, renderer));
            break;
          }
          case "codespan": {
            const codespanToken = token;
            out += renderer.codespan(codespanToken.text);
            break;
          }
          case "br": {
            out += renderer.br();
            break;
          }
          case "del": {
            const delToken = token;
            out += renderer.del(this.parseInline(delToken.tokens, renderer));
            break;
          }
          case "text": {
            const textToken = token;
            out += renderer.text(textToken.text);
            break;
          }
          default: {
            const errMsg = 'Token with "' + token.type + '" type was not found.';
            if (this.options.silent) {
              console.error(errMsg);
              return "";
            } else {
              throw new Error(errMsg);
            }
          }
        }
      }
      return out;
    }
  };
  var _Hooks = class {
    options;
    constructor(options2) {
      this.options = options2 || _defaults;
    }
    static passThroughHooks = /* @__PURE__ */ new Set([
      "preprocess",
      "postprocess"
    ]);
    /**
     * Process markdown before marked
     */
    preprocess(markdown) {
      return markdown;
    }
    /**
     * Process HTML after marked is finished
     */
    postprocess(html9) {
      return html9;
    }
  };
  var Marked = class {
    defaults = _getDefaults();
    options = this.setOptions;
    parse = this.#parseMarkdown(_Lexer.lex, _Parser.parse);
    parseInline = this.#parseMarkdown(_Lexer.lexInline, _Parser.parseInline);
    Parser = _Parser;
    parser = _Parser.parse;
    Renderer = _Renderer;
    TextRenderer = _TextRenderer;
    Lexer = _Lexer;
    lexer = _Lexer.lex;
    Tokenizer = _Tokenizer;
    Hooks = _Hooks;
    constructor(...args) {
      this.use(...args);
    }
    /**
     * Run callback for every token
     */
    walkTokens(tokens, callback) {
      let values = [];
      for (const token of tokens) {
        values = values.concat(callback.call(this, token));
        switch (token.type) {
          case "table": {
            const tableToken = token;
            for (const cell of tableToken.header) {
              values = values.concat(this.walkTokens(cell.tokens, callback));
            }
            for (const row of tableToken.rows) {
              for (const cell of row) {
                values = values.concat(this.walkTokens(cell.tokens, callback));
              }
            }
            break;
          }
          case "list": {
            const listToken = token;
            values = values.concat(this.walkTokens(listToken.items, callback));
            break;
          }
          default: {
            const genericToken = token;
            if (this.defaults.extensions?.childTokens?.[genericToken.type]) {
              this.defaults.extensions.childTokens[genericToken.type].forEach((childTokens) => {
                values = values.concat(this.walkTokens(genericToken[childTokens], callback));
              });
            } else if (genericToken.tokens) {
              values = values.concat(this.walkTokens(genericToken.tokens, callback));
            }
          }
        }
      }
      return values;
    }
    use(...args) {
      const extensions = this.defaults.extensions || { renderers: {}, childTokens: {} };
      args.forEach((pack) => {
        const opts = { ...pack };
        opts.async = this.defaults.async || opts.async || false;
        if (pack.extensions) {
          pack.extensions.forEach((ext) => {
            if (!ext.name) {
              throw new Error("extension name required");
            }
            if ("renderer" in ext) {
              const prevRenderer = extensions.renderers[ext.name];
              if (prevRenderer) {
                extensions.renderers[ext.name] = function(...args2) {
                  let ret = ext.renderer.apply(this, args2);
                  if (ret === false) {
                    ret = prevRenderer.apply(this, args2);
                  }
                  return ret;
                };
              } else {
                extensions.renderers[ext.name] = ext.renderer;
              }
            }
            if ("tokenizer" in ext) {
              if (!ext.level || ext.level !== "block" && ext.level !== "inline") {
                throw new Error("extension level must be 'block' or 'inline'");
              }
              const extLevel = extensions[ext.level];
              if (extLevel) {
                extLevel.unshift(ext.tokenizer);
              } else {
                extensions[ext.level] = [ext.tokenizer];
              }
              if (ext.start) {
                if (ext.level === "block") {
                  if (extensions.startBlock) {
                    extensions.startBlock.push(ext.start);
                  } else {
                    extensions.startBlock = [ext.start];
                  }
                } else if (ext.level === "inline") {
                  if (extensions.startInline) {
                    extensions.startInline.push(ext.start);
                  } else {
                    extensions.startInline = [ext.start];
                  }
                }
              }
            }
            if ("childTokens" in ext && ext.childTokens) {
              extensions.childTokens[ext.name] = ext.childTokens;
            }
          });
          opts.extensions = extensions;
        }
        if (pack.renderer) {
          const renderer = this.defaults.renderer || new _Renderer(this.defaults);
          for (const prop in pack.renderer) {
            const rendererFunc = pack.renderer[prop];
            const rendererKey = prop;
            const prevRenderer = renderer[rendererKey];
            renderer[rendererKey] = (...args2) => {
              let ret = rendererFunc.apply(renderer, args2);
              if (ret === false) {
                ret = prevRenderer.apply(renderer, args2);
              }
              return ret || "";
            };
          }
          opts.renderer = renderer;
        }
        if (pack.tokenizer) {
          const tokenizer = this.defaults.tokenizer || new _Tokenizer(this.defaults);
          for (const prop in pack.tokenizer) {
            const tokenizerFunc = pack.tokenizer[prop];
            const tokenizerKey = prop;
            const prevTokenizer = tokenizer[tokenizerKey];
            tokenizer[tokenizerKey] = (...args2) => {
              let ret = tokenizerFunc.apply(tokenizer, args2);
              if (ret === false) {
                ret = prevTokenizer.apply(tokenizer, args2);
              }
              return ret;
            };
          }
          opts.tokenizer = tokenizer;
        }
        if (pack.hooks) {
          const hooks = this.defaults.hooks || new _Hooks();
          for (const prop in pack.hooks) {
            const hooksFunc = pack.hooks[prop];
            const hooksKey = prop;
            const prevHook = hooks[hooksKey];
            if (_Hooks.passThroughHooks.has(prop)) {
              hooks[hooksKey] = (arg) => {
                if (this.defaults.async) {
                  return Promise.resolve(hooksFunc.call(hooks, arg)).then((ret2) => {
                    return prevHook.call(hooks, ret2);
                  });
                }
                const ret = hooksFunc.call(hooks, arg);
                return prevHook.call(hooks, ret);
              };
            } else {
              hooks[hooksKey] = (...args2) => {
                let ret = hooksFunc.apply(hooks, args2);
                if (ret === false) {
                  ret = prevHook.apply(hooks, args2);
                }
                return ret;
              };
            }
          }
          opts.hooks = hooks;
        }
        if (pack.walkTokens) {
          const walkTokens2 = this.defaults.walkTokens;
          const packWalktokens = pack.walkTokens;
          opts.walkTokens = function(token) {
            let values = [];
            values.push(packWalktokens.call(this, token));
            if (walkTokens2) {
              values = values.concat(walkTokens2.call(this, token));
            }
            return values;
          };
        }
        this.defaults = { ...this.defaults, ...opts };
      });
      return this;
    }
    setOptions(opt) {
      this.defaults = { ...this.defaults, ...opt };
      return this;
    }
    #parseMarkdown(lexer2, parser2) {
      return (src, options2) => {
        const origOpt = { ...options2 };
        const opt = { ...this.defaults, ...origOpt };
        if (this.defaults.async === true && origOpt.async === false) {
          if (!opt.silent) {
            console.warn("marked(): The async option was set to true by an extension. The async: false option sent to parse will be ignored.");
          }
          opt.async = true;
        }
        const throwError = this.#onError(!!opt.silent, !!opt.async);
        if (typeof src === "undefined" || src === null) {
          return throwError(new Error("marked(): input parameter is undefined or null"));
        }
        if (typeof src !== "string") {
          return throwError(new Error("marked(): input parameter is of type " + Object.prototype.toString.call(src) + ", string expected"));
        }
        if (opt.hooks) {
          opt.hooks.options = opt;
        }
        if (opt.async) {
          return Promise.resolve(opt.hooks ? opt.hooks.preprocess(src) : src).then((src2) => lexer2(src2, opt)).then((tokens) => opt.walkTokens ? Promise.all(this.walkTokens(tokens, opt.walkTokens)).then(() => tokens) : tokens).then((tokens) => parser2(tokens, opt)).then((html9) => opt.hooks ? opt.hooks.postprocess(html9) : html9).catch(throwError);
        }
        try {
          if (opt.hooks) {
            src = opt.hooks.preprocess(src);
          }
          const tokens = lexer2(src, opt);
          if (opt.walkTokens) {
            this.walkTokens(tokens, opt.walkTokens);
          }
          let html9 = parser2(tokens, opt);
          if (opt.hooks) {
            html9 = opt.hooks.postprocess(html9);
          }
          return html9;
        } catch (e3) {
          return throwError(e3);
        }
      };
    }
    #onError(silent, async) {
      return (e3) => {
        e3.message += "\nPlease report this to https://github.com/markedjs/marked.";
        if (silent) {
          const msg = "<p>An error occurred:</p><pre>" + escape(e3.message + "", true) + "</pre>";
          if (async) {
            return Promise.resolve(msg);
          }
          return msg;
        }
        if (async) {
          return Promise.reject(e3);
        }
        throw e3;
      };
    }
  };
  var markedInstance = new Marked();
  function marked(src, opt) {
    return markedInstance.parse(src, opt);
  }
  marked.options = marked.setOptions = function(options2) {
    markedInstance.setOptions(options2);
    marked.defaults = markedInstance.defaults;
    changeDefaults(marked.defaults);
    return marked;
  };
  marked.getDefaults = _getDefaults;
  marked.defaults = _defaults;
  marked.use = function(...args) {
    markedInstance.use(...args);
    marked.defaults = markedInstance.defaults;
    changeDefaults(marked.defaults);
    return marked;
  };
  marked.walkTokens = function(tokens, callback) {
    return markedInstance.walkTokens(tokens, callback);
  };
  marked.parseInline = markedInstance.parseInline;
  marked.Parser = _Parser;
  marked.parser = _Parser.parse;
  marked.Renderer = _Renderer;
  marked.TextRenderer = _TextRenderer;
  marked.Lexer = _Lexer;
  marked.lexer = _Lexer.lex;
  marked.Tokenizer = _Tokenizer;
  marked.Hooks = _Hooks;
  marked.parse = marked;
  var options = marked.options;
  var setOptions = marked.setOptions;
  var use = marked.use;
  var walkTokens = marked.walkTokens;
  var parseInline = marked.parseInline;
  var parser = _Parser.parse;
  var lexer = _Lexer.lex;

  // src/components/RenderedContent.js
  var html = htm_module_default.bind(y);
  function markdownify(md) {
    let markyMark = new Marked();
    return markyMark.parse(md);
  }
  function AnyCard({ card, cardType, stackIndex, primary, visible, children }) {
    let [animation, setAnimation] = h2(null);
    let style = stackIndex != "" ? `z-index:${stackIndex};` : "";
    let isAnimation = false;
    if (card.fadeIn || card.shake) {
      animation = true;
    }
    let opacity = null;
    let translateX = null;
    let translateY = null;
    let rotation = null;
    let scale = null;
    let easing = card.easing ?? "easeInOutQuad";
    let duration = card.duration ?? 500;
    let delay = card.delay ?? 0;
    let restrictMaxWidth = true;
    let restrictMaxHeight = true;
    let animStyle = [];
    if (card.fadeIn) {
      isAnimation = true;
      if (!isNaN(card.fadeIn)) {
        delay = card.fadeIn;
      }
      animStyle.push(`opacity: 0;`);
      opacity = [0, 1];
    }
    if (card.fadeOut) {
      isAnimation = true;
      if (!isNaN(card.fadeOut)) {
        delay = card.fadeOut;
      }
      if (!card.fadeIn) {
        animStyle.push(`opacity: 1;`);
      }
      if (opacity == null) {
        opacity = [];
      }
      opacity = opacity.concat([1, 0]);
    }
    if (card.shake) {
      isAnimation = true;
      if (!isNaN(card.shake)) {
        duration = card.shake;
      }
      let amount2 = card.amount ?? 5;
      translateX = [];
      translateX.push(0);
      for (let i3 = 0; i3 < duration / 100; i3++) {
        translateX.push(i3 % 2 === 0 ? amount2 : -amount2);
      }
      translateX.push(0);
    }
    if (card.verticalShake) {
      isAnimation = true;
      if (!isNaN(card.shakeY)) {
        duration = card.shakeY;
      }
      let amount2 = card.amount ?? 5;
      translateY = [];
      translateY.push(0);
      for (let i3 = 0; i3 < duration / 100; i3++) {
        translateY.push(i3 % 2 === 0 ? amount2 : -amount2);
      }
      translateY.push(0);
    }
    if (card.jitter) {
      isAnimation = true;
      if (!isNaN(card.jitter)) {
        duration = card.jitter;
      }
      let amount2 = card.amount ?? 5;
      translateX = [];
      translateX.push(0);
      for (let i3 = 0; i3 < duration / 50; i3++) {
        translateX.push(Math.random() * amount2 * 2 - amount2);
      }
      translateX.push(0);
    }
    if (card.verticalJitter) {
      isAnimation = true;
      if (!isNaN(card.jitter)) {
        duration = card.verticalJitter;
      }
      let amount2 = card.amount ?? 5;
      translateY = [];
      translateY.push(0);
      for (let i3 = 0; i3 < duration / 50; i3++) {
        translateY.push(Math.random() * amount2 * 2 - amount2);
      }
      translateY.push(0);
    }
    if (card.panLeft) {
      isAnimation = true;
      translateX = -card.panLeft;
      duration = card.duration ?? 5e3;
      restrictMaxWidth = false;
    }
    if (card.panRight) {
      isAnimation = true;
      translateX = 0;
      duration = card.duration ?? 5e3;
      amount = card.panRight ?? 300;
      animStyle.push(`transform: translateX(-${amount}px);`);
      restrictMaxWidth = false;
    }
    if (card.panDown) {
      isAnimation = true;
      translateY = -card.panDown;
      duration = card.duration ?? 5e3;
      restrictMaxHeight = false;
    }
    if (card.panUp) {
      isAnimation = true;
      translateY = 0;
      duration = card.duration ?? 5e3;
      amount = card.panUp ?? 400;
      animStyle.push(`${style} transform: translateY(-${amount}px);`);
      restrictMaxHeight = false;
    }
    if (card.dollyIn) {
      isAnimation = true;
      scale = card.dollyIn;
    }
    if (card.dollyOut) {
      isAnimation = true;
      scale = card.dollyOut;
    }
    if (card.spinClockwise) {
      isAnimation = true;
      rotation = card.spinClockwise;
      animStyle.push(`${style} transform: rotate(${rotation});`);
    }
    if (isAnimation) {
      p2(() => {
        if (primary) {
          let el;
          if (card.animateContainer) {
            el = this.base;
          } else {
            el = this.base.querySelector(".animation-frame");
          }
          if (animation && animation.remove != null) {
            animation?.remove(el);
          }
          let anim = { targets: el, duration, delay, easing };
          if (opacity != null) {
            anim.opacity = opacity;
          }
          if (translateX != null) {
            anim.translateX = translateX;
          }
          if (translateY != null) {
            anim.translateY = translateY;
          }
          if (scale != null) {
            anim.scale = scale;
          }
          if (rotation != null) {
            anim.rotate = rotation;
          }
          console.dir(anim);
          let createdAnimation = anime_es_default(anim);
          setAnimation(createdAnimation);
          createdAnimation?.play();
        } else {
          if (animation && animation.restart != null && animation.pause != null) {
            animation?.restart();
            animation?.pause();
          }
        }
      }, [primary]);
    }
    let restrictions = [];
    if (restrictMaxWidth) {
      restrictions.push("restrict-max-width");
    }
    if (restrictMaxHeight) {
      restrictions.push("restrict-max-height");
    }
    if (card.animateContainer) {
      style = style.concat(animStyle);
      animStyle = [];
    }
    let footnote = null;
    if (card.footnote) {
      footnote = html`<div class="footnote">
            <div class="markdown-content" dangerouslySetInnerHTML=${{ __html: markdownify(card.footnote) }}></div>
        </div>`;
    }
    return html`<div style=${style} class="card ${cardType}-card any-card ${stackIndex ? "stacked" : ""} ${card.containerClass.join(" ")} ${restrictions.join(" ")}">
        <div style=${animStyle.join(" ")} class="animation-frame ${card.extraClass.join(" ")}">
        ${children}
        </div>
        ${footnote}
    </div>`;
  }
  function TitleCard({ card, stackIndex, primary, visible }) {
    return html`<${AnyCard} card=${card} cardType="title" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <h1>${card.title ?? card.id}</h1>
    </${AnyCard}>`;
  }
  function MarkdownCard({ card, stackIndex, primary, visible }) {
    return html`<${AnyCard} card=${card} cardType="markdown" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <div class="markdown-content" dangerouslySetInnerHTML=${{ __html: markdownify(card.content) }}></div>
    </${AnyCard}>`;
  }
  function HtmlCard({ card, stackIndex, primary, visible }) {
    return html`<${AnyCard} card=${card} cardType="html" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <div class="html-content" dangerouslySetInnerHTML=${{ __html: insane(card.content) }}></div>
    </${AnyCard}>`;
  }
  function ImageCard({ card, stackIndex, primary, visible }) {
    return html`<${AnyCard} card=${card} cardType="image" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <img src=${card.imageUrl} alt=${card.alt} title=${card.title}/>
    </${AnyCard}>`;
  }
  function AnimatedImageCard({ card, primary, visible, stackIndex }) {
    let [animatedImageInterval, setAnimatedImageInterval] = h2(null);
    let imagesToCycleThrough = card.pngs;
    let fps = card.pngsFps ?? 24;
    let isLoop = card.loop;
    p2(() => {
      if (primary) {
        clearInterval(animatedImageInterval);
        let images2 = this.base.querySelectorAll("img");
        let index = 0;
        setTimeout(() => {
          animatedImageInterval = setInterval(() => {
            images2.forEach((img, i3) => {
              if (i3 === index) {
                img.style.display = "block";
              } else {
                img.style.display = "none";
              }
            });
            index = (index + 1) % images2.length;
            if (!isLoop && index === 0) {
              clearInterval(animatedImageInterval);
            }
          }, 1e3 / fps);
          setAnimatedImageInterval(animatedImageInterval);
        }, card.delay ?? 0);
      } else {
        clearInterval(animatedImageInterval);
      }
    }, [primary]);
    let images = imagesToCycleThrough.map((imageUrl, index) => {
      return html`<img src=${imageUrl} alt=${card.alt} title=${card.title} style="display: ${index === 0 ? "block" : "none"};"/>`;
    });
    return html`<${AnyCard} card=${card} cardType="animated-image" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        ${images}
    </${AnyCard}>`;
  }
  function BasicTextAnimation({ text, next, fps, wave, bounce, jitter, fadeIn, rainbow, cursor, color, style, className, em, strong }) {
    let [animatedTextInterval, setAnimatedTextInterval] = h2(null);
    p2(() => {
      let characters = this.base.querySelectorAll("span");
      setTimeout(() => {
        clearInterval(animatedTextInterval);
        let index = 0;
        if (cursor) {
          let cursor2 = this.base.querySelector(".cursor");
          let a3 = anime_es_default({
            targets: cursor2,
            opacity: [1, 0],
            duration: 500,
            easing: "linear",
            loop: true
          });
          a3.play();
        }
        animatedTextInterval = setInterval(() => {
          characters.forEach((char, i3) => {
            if (i3 === index) {
              char.style.display = "inline";
              if (fadeIn) {
                let a3 = anime_es_default({
                  targets: char,
                  opacity: [0, 1],
                  duration: 1e3,
                  easing: "linear"
                });
                a3.play();
              } else {
                char.style.opacity = "1";
              }
              if (wave) {
                char.style.display = "inline-block";
                char.style.minWidth = "0.25em";
                let a3 = anime_es_default({
                  targets: char,
                  translateY: [-3, 3, -3, 3, -3, 3, 0],
                  duration: 5e3,
                  easing: "easeInOutQuad"
                });
                a3.play();
              }
              if (bounce) {
                char.style.display = "inline-block";
                char.style.minWidth = "0.25em";
                let a3 = anime_es_default({
                  targets: char,
                  translateY: [-3, 3, -3, 3, -3, 3, -3, 3, -3, 3, -3, 3, -3, 3, -3, 3, -3, 3, 0],
                  duration: 5e3,
                  easing: "easeInOutBounce"
                });
                a3.play();
              }
              if (jitter) {
                char.style.display = "inline-block";
                char.style.minWidth = "0.25em";
                let yTranslations = [];
                for (let i4 = 0; i4 < 20; i4++) {
                  yTranslations.push(Math.random() * 4 - 2);
                }
                yTranslations.push(0);
                let a3 = anime_es_default({
                  targets: char,
                  translateY: yTranslations,
                  duration: 1500,
                  easing: "easeInOutBack"
                });
                a3.play();
              }
              if (rainbow) {
                let a3 = anime_es_default({
                  targets: char,
                  color: ["#ff0000", "#ff7f00", "#ffff00", "#00ff00", "#0000ff", "#4b0082", "#8b00ff"],
                  duration: 1e3,
                  easing: "linear",
                  loop: true
                });
                a3.play();
              }
            }
          });
          index = (index + 1) % (characters.length + 1);
          if (index === 0) {
            clearInterval(animatedTextInterval);
            if (cursor) {
              let cursor2 = this.base.querySelector(".cursor");
              if (cursor2) {
                cursor2.style.display = "none";
              }
            }
            next();
          }
        }, 1e3 / fps);
        setAnimatedTextInterval(animatedTextInterval);
      }, 0);
    }, []);
    let styleExtras = "";
    if (color) {
      styleExtras += `color: ${color};`;
    }
    if (em) {
      styleExtras += `font-style: italic;`;
    }
    if (strong) {
      styleExtras += `font-weight: bold;`;
    }
    if (style) {
      styleExtras += `${style};`;
    }
    let textSeparated = text.split("").map((char, index) => {
      return html`<span class=${className} style="display: none; opacity: 0;${styleExtras}">${char}</span>`;
    });
    let cursy = "";
    if (cursor) {
      cursy = html`<span class="cursor" style="opacity: 1;${styleExtras}">_</span>`;
    }
    return html`
        <span class="basic-text-animation">
            ${textSeparated}
            ${cursy}
        </span>
    `;
  }
  function LineBreakAnimation({ next }) {
    let lineBreakMs = 750;
    p2(() => {
      setTimeout(() => {
        next();
      }, lineBreakMs);
    }, []);
    return html`<br />`;
  }
  function NbspAnimation({ next, fps }) {
    p2(() => {
      setTimeout(() => {
        next();
      }, 1e3 / fps);
    }, []);
    let nbsp = String.fromCharCode(160);
    return html`<span>${nbsp}</span>`;
  }
  function TabAnimation({ next, fps }) {
    p2(() => {
      setTimeout(() => {
        next();
      }, 1e3 / fps);
    }, []);
    let nbsp = String.fromCharCode(160);
    return html`<span>${nbsp}${nbsp}${nbsp}${nbsp}</span>`;
  }
  function DelayAnimation({ next, delay }) {
    p2(() => {
      setTimeout(() => {
        next();
      }, delay);
    }, []);
    return null;
  }
  function ComplexTextAnimation({ node, next, fps, primary, visible, delay = 0, wave, bounce, jitter, fadeIn, rainbow, cursor, color, strong, em, style, className }) {
    let [currentIndex, setCurrentIndex] = h2(1);
    let [active, setActive] = h2(false);
    let animations = [];
    p2(() => {
      if (primary) {
        setTimeout(() => {
          setCurrentIndex(1);
          setActive(true);
        }, delay);
      } else {
        setCurrentIndex(0);
        setActive(false);
      }
    }, [primary]);
    function newNext() {
      setCurrentIndex(currentIndex + 1);
      if (currentIndex === animations.length) {
        next();
      }
    }
    let counter = 0;
    for (let child of node.childNodes) {
      let key = `anim-${counter++}`;
      let _wave = wave;
      if (child.nodeName === "wave" || child.nodeName === "wavy" || child.getAttribute && child.getAttribute("wave")) {
        _wave = true;
      }
      let _bounce = bounce;
      if (child.nodeName === "bounce" || child.getAttribute && child.getAttribute("bounce")) {
        _bounce = true;
      }
      let _jitter = jitter;
      if (child.nodeName === "jitter" || child.getAttribute && child.getAttribute("jitter")) {
        _jitter = true;
      }
      let _fadeIn = fadeIn;
      if (child.nodeName === "fade" || child.getAttribute && child.getAttribute("fade")) {
        _fadeIn = true;
      }
      let _rainbow = rainbow;
      if (child.nodeName === "rainbow" || child.getAttribute && child.getAttribute("rainbow")) {
        _rainbow = true;
      }
      let _cursor = cursor;
      if (child.nodeName === "cursor" || child.getAttribute && child.getAttribute("cursor")) {
        _cursor = true;
      }
      let _color = color;
      if (child.nodeName === "color") {
        if (child.getAttribute) {
          _color = child.getAttribute("value") ?? "white";
        } else {
          _color = "white";
        }
      }
      if (child.getAttribute && child.getAttribute("color")) {
        _color = child.getAttribute("color");
      }
      let _style = style;
      if (child.nodeName === "style") {
        if (child.getAttribute) {
          _style = `${style};${child.getAttribute("value") ?? ""}`;
        }
      }
      if (child.getAttribute && child.getAttribute("style")) {
        _style = `${style};${child.getAttribute("style")}`;
      }
      let _class = className;
      if (child.getAttribute && child.getAttribute("class")) {
        _class = `${className} ${child.getAttribute("class") ?? ""}`;
      }
      let _em = em;
      if (child.nodeName === "em") {
        _em = true;
      }
      let _strong = strong;
      if (child.nodeName === "strong") {
        _strong = true;
      }
      let _fps = fps;
      if (child.nodeName === "slow") {
        _fps = fps / 2;
      } else if (child.nodeName === "slower") {
        _fps = fps / 4;
      } else if (child.nodeName === "slowest") {
        _fps = fps / 8;
      } else if (child.nodeName === "fast") {
        _fps = fps * 2;
      } else if (child.nodeName === "faster") {
        _fps = fps * 4;
      } else if (child.nodeName === "fastest") {
        _fps = fps * 8;
      }
      let complex = false;
      if (child.nodeName !== "#text") {
        for (let c3 of child.childNodes) {
          if (c3.nodeName !== "#text") {
            complex = true;
          }
        }
        if (child.nodeName === "div") {
          complex = true;
        }
      }
      if (child.nodeName === "br") {
        animations.push(html`<${LineBreakAnimation} next=${newNext} fps=${_fps} key=${key} />`);
        continue;
      }
      if (child.nodeName === "nbsp") {
        animations.push(html`<${NbspAnimation} next=${newNext} fps=${_fps} key=${key} />`);
        continue;
      }
      if (child.nodeName === "tab") {
        animations.push(html`<${TabAnimation} next=${newNext} fps=${_fps} key=${key} />`);
        continue;
      } else if (child.nodeName === "beat") {
        let delayAmount = child.getAttribute("ms") ?? 750;
        animations.push(html`<${DelayAnimation} next=${newNext} delay=${delayAmount} key=${key} />`);
        continue;
      } else if (child.nodeName === "delay") {
        let delayAmount = child.getAttribute("ms") ?? 1500;
        animations.push(html`<${DelayAnimation} next=${newNext} delay=${delayAmount} key=${key} />`);
        continue;
      } else if (child.nodeName === "pause") {
        let delayAmount = child.getAttribute("ms") ?? 3e3;
        animations.push(html`<${DelayAnimation} next=${newNext} delay=${delayAmount} key=${key} />`);
        continue;
      } else if (complex) {
        let tempStyle, tempClass, tempColor;
        if (child.nodeName === "div") {
          tempStyle = _style;
          _style = null;
          tempClass = _class;
          _class = null;
          tempColor = _color;
          _color = null;
        }
        let animation = html`<${ComplexTextAnimation}
                node=${child}
                next=${newNext}
                fps=${_fps}
                primary=${primary}
                visible=${visible}
                wave=${_wave}
                bounce=${_bounce}
                jitter=${_jitter}
                fadeIn=${_fadeIn}
                rainbow=${_rainbow}
                cursor=${_cursor}
                color=${_color}
                strong=${_strong}
                em=${_em}
                style=${_style}
                className=${_class}
                key=${key} />`;
        if (child.nodeName === "div") {
          if (tempColor) {
            tempStyle = `${tempStyle};color:${tempColor}`;
          }
          animations.push(html`<div class="complex-animation" style=${tempStyle} class=${tempClass}>${animation}</div>`);
        } else {
          animations.push(animation);
        }
      } else {
        animations.push(html`<${BasicTextAnimation}
                text=${child.textContent}
                next=${newNext}
                fps=${_fps}
                wave=${_wave}
                bounce=${_bounce}
                jitter=${_jitter}
                fadeIn=${_fadeIn}
                rainbow=${_rainbow}
                cursor=${_cursor}
                color=${_color}
                strong=${_strong}
                em=${_em}
                style=${_style}
                className=${_class}
                key=${key} />`);
      }
    }
    let visibleAnimations = animations.slice(0, currentIndex);
    if (!active) {
      return null;
    }
    return html`
        <span class="complex-animation">
            ${visibleAnimations}
        </span>
    `;
  }
  function AnimatedTextCard({ card, primary, visible, stackIndex }) {
    let fps = card.fps ?? 24;
    let parsedXml = new DOMParser().parseFromString(`<animation>${card.content}</animation>`, "text/xml");
    if (parsedXml.documentElement.nodeName === "parsererror") {
      console.error("Error parsing XML");
      return html`<${ErrorCard} message="Error parsing Animation XML" card=${card} stackIndex=${stackIndex} primary=${primary} visible=${visible} />`;
    }
    function done() {
      console.log("done");
    }
    return html`<${AnyCard} card=${card} cardType="animated-text" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <div class="animated-text-content">
            <${ComplexTextAnimation} node=${parsedXml.childNodes[0]} fps=${fps} next=${done} primary=${primary} visible=${visible} delay=${card.delay ?? 0} />
        </div>
    </${AnyCard}>`;
  }
  function VideoCard({ card, primary, visible, stackIndex }) {
    p2(() => {
      let video = this.base.querySelector("video");
      if (primary) {
        video.play();
      } else {
        video.currentTime = 0;
        video.pause();
      }
    }, [primary]);
    let loop = card.loop ? "loop" : "";
    let muted = card.videoHasSound ? "" : "muted";
    let controls = card.videoControls ? "controls" : "";
    console.log(`video: ${loop} ${muted} ${controls}`);
    let videoType = card.videoUrl.split(".").pop();
    return html`<${AnyCard} card=${card} cardType="video" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <video muted=${!card.videoHasSound} loop=${card.loop} controls=${card.videoControls} playsinline="true" preload="true">
            <source src=${card.videoUrl} type="video/${videoType}" />
        </video>
    </${AnyCard}>`;
  }
  function ErrorCard({ card, message, stackIndex, primary, visible }) {
    return html`<${AnyCard} card=${card} cardType="error" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <h4>Error</h4>
        <p>${message}</p>
        <div class="error-content">
            <pre>
            <code>
                ${JSON.stringify(card, null, 2)}
            </code>
            </pre>
        </div>
    </${AnyCard}>`;
  }
  function typeToCardClass(type) {
    let cardClass = ErrorCard;
    if (type === "markdown") {
      cardClass = MarkdownCard;
    }
    if (type === "html") {
      cardClass = HtmlCard;
    }
    if (type === "title") {
      cardClass = TitleCard;
    }
    if (type === "image") {
      cardClass = ImageCard;
    }
    if (type === "animated_text" || type === "animated-text") {
      cardClass = AnimatedTextCard;
    }
    if (type === "video") {
      cardClass = VideoCard;
    }
    if (type === "pngs") {
      cardClass = AnimatedImageCard;
    }
    if (type === "stack") {
      cardClass = StackedCard;
    }
    return cardClass;
  }
  function StackedCard({ card, primary, visible, stackIndex }) {
    return html`<${AnyCard} card=${card} cardType="stack" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        ${card.stack.map((c3, index) => {
      let cardClass = typeToCardClass(c3.type);
      let newStackIndex = (stackIndex ?? 0 * 100) + index + 1;
      return html`<${cardClass} card=${c3} primary=${primary} visible=${visible} stackIndex=${newStackIndex} />`;
    })}
    </${AnyCard}>`;
  }
  function RenderedContent({ content, primary, visible }) {
    let card = content;
    let cardClass = typeToCardClass(card.type);
    return html`<div class="rendered-content">
        <${cardClass} card=${card} primary=${primary} visible=${visible}/>
    </div>`;
  }

  // src/components/VisibilityTriggerFrame.js
  var html2 = htm_module_default.bind(y);
  var showHandlers = {};
  var hideHandlers = {};
  var visibilityObserver = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        let id = entry.target.id;
        if (entry.isIntersecting) {
          entry.target.classList.add("currently-visible");
          if (showHandlers[id]) {
            showHandlers[id]();
          }
        } else {
          entry.target.classList.remove("currently-visible");
          if (hideHandlers[id]) {
            hideHandlers[id]();
          }
        }
      });
    },
    {
      root: null,
      // use the viewport
      rootMargin: "0px",
      // anywhere in the viewport
      threshold: 0.01
      // a tiny fraction of the element must be visible
    }
  );
  var primaryShowHandlers = {};
  var primaryHideHandlers = {};
  var primaryObserver = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        let id = entry.target.id;
        if (entry.isIntersecting) {
          entry.target.classList.add("currently-visible");
          if (primaryShowHandlers[id]) {
            primaryShowHandlers[id]();
          }
        } else {
          entry.target.classList.remove("currently-visible");
          if (primaryHideHandlers[id]) {
            primaryHideHandlers[id]();
          }
        }
      });
    },
    {
      root: null,
      // use the viewport
      rootMargin: "0px",
      // anywhere in the viewport
      threshold: 0.9
      // 100% of the element must be visible
    }
  );
  function observe(id, element, showHandler, hideHandler, primaryShowHandler, primaryHideHandler) {
    showHandlers[id] = showHandler;
    hideHandlers[id] = hideHandler;
    primaryShowHandlers[id] = primaryShowHandler;
    primaryHideHandlers[id] = primaryHideHandler;
    visibilityObserver.observe(element);
    primaryObserver.observe(element);
  }
  function unobserve(id, element) {
    delete showHandlers[id];
    delete hideHandlers[id];
    delete primaryShowHandlers[id];
    delete primaryHideHandlers[id];
    visibilityObserver.unobserve(element);
    primaryObserver.unobserve(element);
  }
  var VisibilityTriggerFrame = class extends b {
    /*
        The purpose of the VisibilityTriggerFrame is to provide a way to
        trigger events when a frame is visible or invisible. This is
        useful for triggering animations and other effects.
    */
    constructor(props) {
      super(props);
      this.data = props.data;
      this.order = props.order;
      let noop = () => {
      };
      this.onPrimary = props.onPrimary ?? noop;
      this.onUnprimary = props.onUnprimary ?? noop;
      this.onVisible = props.onVisible ?? noop;
      this.onInvisible = props.onInvisible ?? noop;
      this.state = {
        visible: false,
        primary: false,
        id: this.props.id
      };
    }
    async visible() {
      this.setState({
        visible: true
      });
      let node = await this.data.getContent({ id: this.state.id });
      this.setState({ node });
      this.onVisible();
    }
    invisible() {
      this.setState({
        visible: false
      });
      this.onInvisible();
    }
    async primary() {
      this.data.setCurrentLocation(this.order);
      this.setState({
        primary: true
      });
      this.onPrimary();
    }
    unprimary() {
      this.setState({
        primary: false
      });
      this.onUnprimary();
    }
    componentDidMount() {
      let element = this.base;
      this.id = this.props.id ?? uuid();
      element.id = this.id;
      observe(
        this.id,
        element,
        this.visible.bind(this),
        this.invisible.bind(this),
        this.primary.bind(this),
        this.unprimary.bind(this)
      );
    }
    componentWillUnmount() {
      let element = this.base;
      unobserve(this.id, element);
    }
    render() {
      let frameClass = "";
      let node = this.state.node;
      if (this.state.primary) {
        frameClass = "frame-primary";
      } else if (this.state.visible) {
        frameClass = "frame-visible";
      } else {
        frameClass = "frame-invisible";
      }
      let maybeContent = "";
      if (node && (this.state.primary || this.state.visible)) {
        maybeContent = html2`<${RenderedContent} content=${node} primary=${this.state.primary} visible=${this.state.visible}/>`;
      }
      return html2`<div class="frame ${frameClass}">
            ${maybeContent}
        </div>`;
    }
  };

  // src/components/Icon.js
  var html3 = htm_module_default.bind(y);
  function Icon({ name }) {
    if (name == "chevron-down") {
      return html3`<svg viewBox="0 0 64 64" class="svg-icon icon-circle-chevron-down">
            <path class="opt" d="M32,1.8C15.3,1.8,1.8,15.3,1.8,32S15.3,62.3,32,62.3S62.3,48.7,62.3,32S48.7,1.8,32,1.8z M32,57.8
                C17.8,57.8,6.3,46.2,6.3,32C6.3,17.8,17.8,6.3,32,6.3c14.2,0,25.8,11.6,25.8,25.8C57.8,46.2,46.2,57.8,32,57.8z"/>
            <path d="M43.9,23.8L32,35.5L20.1,23.8c-0.9-0.9-2.3-0.9-3.2,0c-0.9,0.9-0.9,2.3,0,3.2l13.5,13.3c0.4,0.4,1,0.6,1.6,0.6
                c0.6,0,1.1-0.2,1.6-0.6L47.1,27c0.9-0.9,0.9-2.3,0-3.2C46.3,22.9,44.8,22.9,43.9,23.8z"/>
        </svg>`;
    }
    if (name == "double-down") {
      return html3`<svg viewBox="0 0 64 64" class="svg-icon icon-double-down">
            <path class="opt" d="M30.4,38.4c0.4,0.4,1,0.6,1.6,0.6c0.6,0,1.1-0.2,1.6-0.6l23-22.6c0.9-0.9,0.9-2.3,0-3.2c-0.9-0.9-2.3-0.9-3.2,0L32,33.6
                L10.6,12.6c-0.9-0.9-2.3-0.9-3.2,0c-0.9,0.9-0.9,2.3,0,3.2L30.4,38.4z"/>
            <path d="M53.4,25.6L32,46.6L10.6,25.6c-0.9-0.9-2.3-0.9-3.2,0s-0.9,2.3,0,3.2l23,22.6c0.4,0.4,1,0.6,1.6,0.6c0.6,0,1.1-0.2,1.6-0.6
                l23-22.6c0.9-0.9,0.9-2.3,0-3.2S54.3,24.7,53.4,25.6z"/>
        </svg>`;
    }
    if (name == "chevron-up") {
      return html3`<svg viewBox="0 0 64 64" class="svg-icon icon-circle-chevron-up">
            <path class="opt" d="M32,1.8C15.3,1.8,1.8,15.3,1.8,32S15.3,62.3,32,62.3S62.3,48.7,62.3,32S48.7,1.8,32,1.8z M32,57.8
                C17.8,57.8,6.3,46.2,6.3,32C6.3,17.8,17.8,6.3,32,6.3c14.2,0,25.8,11.6,25.8,25.8C57.8,46.2,46.2,57.8,32,57.8z"/>
            <path d="M33.6,23.8c-0.9-0.9-2.3-0.9-3.2,0L16.9,37c-0.9,0.9-0.9,2.3,0,3.2c0.4,0.4,1,0.7,1.6,0.7c0.6,0,1.1-0.2,1.6-0.6L32,28.5
                l11.9,11.7c0.9,0.9,2.3,0.9,3.2,0c0.9-0.9,0.9-2.3,0-3.2L33.6,23.8z"/>
        </svg>`;
    }
    if (name == "double-up") {
      return html3`<svg viewBox="0 0 64 64" class="svg-icon icon-double-up">
            <path class="opt" d="M33.6,25.6c-0.9-0.9-2.3-0.9-3.2,0l-23,22.6c-0.9,0.9-0.9,2.3,0,3.2c0.9,0.9,2.3,0.9,3.2,0L32,30.4l21.4,21.1
                c0.4,0.4,1,0.6,1.6,0.6c0.6,0,1.2-0.2,1.6-0.7c0.9-0.9,0.9-2.3,0-3.2L33.6,25.6z"/>
            <path d="M10.6,38.4L32,17.4l21.4,21.1c0.4,0.4,1,0.6,1.6,0.6c0.6,0,1.2-0.2,1.6-0.7c0.9-0.9,0.9-2.3,0-3.2l-23-22.6
                c-0.9-0.9-2.3-0.9-3.2,0l-23,22.6c-0.9,0.9-0.9,2.3,0,3.2C8.3,39.3,9.7,39.3,10.6,38.4z"/>
        </svg>`;
    }
    if (name == "question") {
      return html3`<svg viewBox="0 0 64 64" class="svg-icon icon-circle-question">
            <path class="opt" d="M32,1.8C15.3,1.8,1.8,15.3,1.8,32S15.3,62.3,32,62.3S62.3,48.7,62.3,32S48.7,1.8,32,1.8z M32,57.8
                C17.8,57.8,6.3,46.2,6.3,32C6.3,17.8,17.8,6.3,32,6.3c14.2,0,25.8,11.6,25.8,25.8C57.8,46.2,46.2,57.8,32,57.8z"/>
            <path d="M33.8,12.1c-2.9-0.5-5.9,0.3-8.1,2.2c-2.2,1.9-3.5,4.6-3.5,7.6c0,1.1,0.2,2.2,0.6,3.3c0.4,1.2,1.7,1.8,2.9,1.4
                c1.2-0.4,1.8-1.7,1.4-2.9c-0.2-0.6-0.3-1.2-0.3-1.8c0-1.6,0.7-3.1,1.9-4.1c1.2-1,2.8-1.5,4.5-1.2c2.1,0.4,3.9,2.2,4.3,4.3
                c0.4,2.5-0.9,5-3.2,6c-2.6,1.1-4.3,3.7-4.3,6.7v6.2c0,1.2,1,2.3,2.3,2.3c1.2,0,2.3-1,2.3-2.3v-6.2c0-1.1,0.6-2.1,1.5-2.5
                c4.3-1.8,6.8-6.3,6-10.9C41,16.1,37.8,12.8,33.8,12.1z"/>
            <path d="M32.1,45.8h-0.3c-1.2,0-2.3,1-2.3,2.3s1,2.3,2.3,2.3h0.3c1.2,0,2.2-1,2.2-2.3S33.4,45.8,32.1,45.8z"/>
        </svg>`;
    }
    if (name == "home") {
      return html3`<svg viewBox="0 0 64 64" class="svg-icon icon-home">
            <path d="M61.2,21.2L35.4,4.6c-2.1-1.3-4.8-1.3-6.8,0L2.8,21.2c-1,0.7-1.3,2.1-0.7,3.1c0.7,1,2.1,1.3,3.1,0.7l1.7-1.1v30.1
                c0,3.5,2.8,6.3,6.3,6.3h37.6c3.5,0,6.3-2.8,6.3-6.3V23.9l1.7,1.1c0.4,0.2,0.8,0.4,1.2,0.4c0.7,0,1.5-0.4,1.9-1
                C62.6,23.3,62.3,21.9,61.2,21.2z M52.6,54.1c0,1-0.8,1.8-1.8,1.8H13.2c-1,0-1.8-0.8-1.8-1.8V21L31,8.4c0.6-0.4,1.4-0.4,2,0L52.6,21
                V54.1z"/>
            <path class="opt" d="M27.2,24.6c-2.2,0-4.3,0.9-5.8,2.4c-3.2,3.2-3.2,8.4,0,11.6l0.6,0.6c0,0,0,0,0,0l8.4,8.5c0.4,0.4,1,0.7,1.6,0.7
                s1.2-0.2,1.6-0.7l8.4-8.5c0,0,0,0,0,0l0.6-0.6c1.5-1.6,2.4-3.6,2.4-5.8c0-2.2-0.8-4.2-2.4-5.8c-1.5-1.6-3.6-2.4-5.8-2.4
                c0,0,0,0,0,0c-1.7,0-3.4,0.5-4.8,1.6C30.6,25.2,29,24.6,27.2,24.6z M34.2,30.2c0.7-0.7,1.6-1.1,2.6-1.1c0,0,0,0,0,0
                c1,0,1.9,0.4,2.6,1.1c0.7,0.7,1.1,1.6,1.1,2.6c0,1-0.4,1.9-1.1,2.7L32,43l-6.8-6.8l-0.6-0.6c-1.4-1.4-1.4-3.8,0-5.3
                c1.4-1.4,3.8-1.4,5.2,0l0.6,0.6c0.4,0.4,1,0.7,1.6,0.7h0c0.6,0,1.2-0.2,1.6-0.7L34.2,30.2z"/>
        </svg>`;
    }
    if (name == "hamburger") {
      return html3`<svg viewBox="0 0 64 64" class="svg-icon icon-circle-hamburger">
            <path class="opt" d="M32.0008 1.80078C15.3008 1.80078 1.80078 15.3008 1.80078 32.0008C1.80078 48.7008 15.3008 62.3008 32.0008 62.3008C48.7008 62.3008 62.3008 48.7008 62.3008 32.0008C62.3008 15.3008 48.7008 1.80078 32.0008 1.80078ZM32.0008 57.8008C17.8008 57.8008 6.30078 46.2008 6.30078 32.0008C6.30078 17.8008 17.8008 6.30078 32.0008 6.30078C46.2008 6.30078 57.8008 17.9008 57.8008 32.1008C57.8008 46.2008 46.2008 57.8008 32.0008 57.8008Z"/>
            <path d="M42.1016 18.1016H21.9016C20.7016 18.1016 19.6016 19.1016 19.6016 20.4016C19.6016 21.7016 20.6016 22.7016 21.9016 22.7016H42.0016C43.2016 22.7016 44.3016 21.7016 44.3016 20.4016C44.3016 19.1016 43.3016 18.1016 42.1016 18.1016Z"/>
            <path d="M42.1016 29.8008H21.9016C20.7016 29.8008 19.6016 30.8008 19.6016 32.1008C19.6016 33.3008 20.6016 34.4008 21.9016 34.4008H42.0016C43.2016 34.4008 44.3016 33.4008 44.3016 32.1008C44.3016 30.8008 43.3016 29.8008 42.1016 29.8008Z"/>
            <path d="M42.1016 41.4004H21.9016C20.7016 41.4004 19.6016 42.4004 19.6016 43.7004C19.6016 45.0004 20.6016 46.0004 21.9016 46.0004H42.0016C43.2016 46.0004 44.3016 45.0004 44.3016 43.7004C44.3016 42.4004 43.3016 41.4004 42.1016 41.4004Z"/>
        </svg>`;
    } else {
      return html3`<svg viewBox="0 0 64 64" class="svg-icon icon-not-found">
            <path class="opt" d="M32,1.8C15.3,1.8,1.8,15.3,1.8,32S15.3,62.3,32,62.3S62.3,48.7,62.3,32S48.7,1.8,32,1.8z M32,57.8
                C17.8,57.8,6.3,46.2,6.3,32C6.3,17.8,17.8,6.3,32,6.3c14.2,0,25.8,11.6,25.8,25.8C57.8,46.2,46.2,57.8,32,57.8z"/>
            <path d="M33.8,12.1c-2.9-0.5-5.9,0.3-8.1,2.2c-2.2,1.9-3.5,4.6-3.5,7.6c0,1.1,0.2,2.2,0.6,3.3c0.4,1.2,1.7,1.8,2.9,1.4
                c1.2-0.4,1.8-1.7,1.4-2.9c-0.2-0.6-0.3-1.2-0.3-1.8c0-1.6,0.7-3.1,1.9-4.1c1.2-1,2.8-1.5,4.5-1.2c2.1,0.4,3.9,2.2,4.3,4.3
                c0.4,2.5-0.9,5-3.2,6c-2.6,1.1-4.3,3.7-4.3,6.7v6.2c0,1.2,1,2.3,2.3,2.3c1.2,0,2.3-1,2.3-2.3v-6.2c0-1.1,0.6-2.1,1.5-2.5
                c4.3-1.8,6.8-6.3,6-10.9C41,16.1,37.8,12.8,33.8,12.1z"/>
            <path d="M32.1,45.8h-0.3c-1.2,0-2.3,1-2.3,2.3s1,2.3,2.3,2.3h0.3c1.2,0,2.2-1,2.2-2.3S33.4,45.8,32.1,45.8z"/>
        </svg>`;
    }
  }

  // src/components/Nav.js
  var html4 = htm_module_default.bind(y);
  function Nav({ onTop, onBottom, onDown, onUp, onMenu }) {
    if (!onTop) {
      onTop = () => {
      };
    }
    if (!onBottom) {
      onBottom = () => {
      };
    }
    if (!onDown) {
      onDown = () => {
      };
    }
    if (!onUp) {
      onUp = () => {
      };
    }
    if (!onMenu) {
      onMenu = () => {
      };
    }
    let [currentAnimation, setCurrentAnimation] = h2(null);
    const pressAnimation = (thinger) => {
      if (currentAnimation && currentAnimation[thinger]) {
        currentAnimation[thinger].reset();
        currentAnimation[thinger].play();
      } else {
        let a3 = anime_es_default({
          targets: `.nav-${thinger} svg`,
          scale: [1.3],
          duration: 200,
          easing: "easeInOutQuad",
          direction: "alternate"
        });
        a3.play();
        setCurrentAnimation({ ...currentAnimation, [thinger]: a3 });
      }
    };
    const top = () => {
      pressAnimation("top");
      onTop();
    };
    const bottom = () => {
      pressAnimation("bottom");
      onBottom();
    };
    const down = () => {
      pressAnimation("down");
      onDown();
    };
    const up = () => {
      pressAnimation("up");
      onUp();
    };
    const menu = () => {
      pressAnimation("menu");
      onMenu();
    };
    return html4`<nav id="primary-nav">
            <ul class="navbar">
                <li>
                    <a onClick=${top} title="Navigate to the first card in the deck" class="nav-top" aria-label="First card" tabindex="-1">
                        <${Icon} name="double-up" />
                    </a>
                </li>
                <li>
                    <a onClick=${up} title="Navigate to the previous card in the deck" class="nav-up" aria-label="Previous card" tabindex="-1">
                        <${Icon} name="chevron-up" />
                    </a>
                </li>
                <li>
                    <a onClick=${menu} title="Open a menu with a description of the current page, table of contents, and sitemap" class="nav-menu" aria-label="Extended Nav Menu" tabindex="-1">
                        <${Icon} name="hamburger" />
                    </a>
                </li>
                <li>
                    <a onClick=${down} title="Navigate to the next card in the deck" class="nav-down" aria-label="Next Card" tabindex="-1">
                        <${Icon} name="chevron-down" />
                    </a>
                </li>
                <li>
                    <a onClick=${bottom} title="Navigate to the final card in the deck" class="nav-bottom" aria-label="Last Card" tabindex="-1">
                        <${Icon} name="double-down" />
                    </a>
                </li>
            </ul>
        </nav>`;
  }

  // src/thumbnailify.js
  function thumbnailify({ imageUrl, height, width }) {
    let url = new URL(imageUrl);
    url.searchParams.set("width", width);
    url.searchParams.set("height", height);
    return url.toString();
  }

  // src/components/TitleCard.js
  var html5 = htm_module_default.bind(y);
  function TitleCard2({ index }) {
    let updatedDate = index.updatedAt.toLocaleString();
    let thumbnailImage = null;
    if (index.thumbnailImageUrl) {
      thumbnailImage = html5`<img class="thumbnail"
                                    src="${thumbnailify({
        imageUrl: `${window.location.origin}${window.location.pathname}${index.thumbnailImageUrl}`,
        height: 250,
        width: 300
      })}"
                                    alt="${index.name}" />`;
    }
    let authorText = index.author;
    if (index.authorLink) {
      authorText = html5`<a href="${index.authorLink}">${index.author}</a>`;
    }
    return html5`
        <div class="title-card">
            <div class="title-row">
                <h2>${index.name}</h2>
            </div>

            <div class="title-row">
                ${thumbnailImage}
                <a class="qrlink" href="/qr_html?link=${window.location.origin}${window.location.pathname}">
                    <img src="/qr?link=${window.location.origin}${window.location.pathname}" alt="QR Code" />
                </a>
            </div>

            <div class="title-row">
                <h4> ${authorText} - <small>${updatedDate}</small></h4>
                <p class="description">${index.description}</p>
            </div>
        </div>
    `;
  }

  // src/components/NavDropdown.js
  var html6 = htm_module_default.bind(y);
  function NavDropdown({ onMenu, navigateTo, data }) {
    let index = data.getIndex();
    let sitemap = data.getSitemap();
    let entries = Object.entries(sitemap);
    entries.sort((a3, b3) => {
      let newestA = a3[1].reduce((acc, deck) => {
        return Math.max(acc, deck.last_update_time.secs_since_epoch);
      }, 0);
      let newestB = b3[1].reduce((acc, deck) => {
        return Math.max(acc, deck.last_update_time.secs_since_epoch);
      }, 0);
      return newestB - newestA;
    });
    return html6`<nav id="full-nav">
        <ul class="navbar">
            <li>
                <a onClick=${onMenu} title="Close the Hamburger Zone" autofocus>
                    <${Icon} name="hamburger" />
                </a>
            </li>
        </ul>
        <div class="nav-dropdown">

            <${TitleCard2} index=${index} />

            <div class="toc">
                <h3>Table of Contents</h3>
                <ul>
                    ${index.toc.map(({ title, id, depth }) => {
      if (depth < 0) {
        return null;
      }
      let depthstyle = `margin-left: ${depth}em;`;
      return html6`<li style=${depthstyle}><a onClick=${(evt) => {
        evt.preventDefault();
        navigateTo(id);
      }}
                                        href="${window.location.origin}${window.location.pathname}#${id}">${title ?? id}</a></li>`;
    })}
                </ul>
            </div>
            <hr/>
            ${entries.length > 0 ? html6`<h3>Sitemap</h3>` : ""}
            <div>
                ${entries.map(([authorSlug, listOfDecks]) => {
      let author = listOfDecks[0].author;
      let countOfVisibleDecks = listOfDecks.filter((deck) => {
        return !deck.hidden;
      }).length;
      listOfDecks.sort((a3, b3) => {
        return b3.last_update_time.secs_since_epoch - a3.last_update_time.secs_since_epoch;
      });
      if (countOfVisibleDecks == 0) {
        return null;
      }
      return html6`<div class='sitemap-author'>
                        <h4>${author}</h4>
                        <ul>
                            ${listOfDecks.map((deck) => {
        if (deck.hidden) {
          return null;
        }
        let image_url = thumbnailify({
          imageUrl: `${window.location.origin}/s/${deck.author_slug}/${deck.slug}/${deck.image_url}`,
          height: 100,
          width: 120
        });
        let updatedDate = new Date(deck.last_update_time.secs_since_epoch * 1e3).toLocaleString();
        return html6`<li>
                                    <a class="sitemap-entry" href="${window.location.origin}/s/${deck.author_slug}/${deck.slug}" title="${deck.title}">
                                        <div class="panel-left">
                                            <h4>${deck.title}</h4>
                                            <p><small>${updatedDate}</small></p>
                                            <p>${deck.description}</p>
                                        </div>
                                        <div class="panel-right">
                                            <img src="${image_url}" alt="${deck.title}" />
                                        </div>
                                    </a>
                                </li>`;
      })}
                        </ul>
                    </div>`;
    })}
            </div>
            <hr/>

            <div class="credits">
                <p>
                    <a href="https://github.com/cube-drone/ministry/">CardChapter</a> is a lightweight,
                        open-source, web-based card presentation system
                        by <a href="https://cube-drone.com">cube drone</a>.
                </p>

            </div>
        </div>
    </nav>`;
  }

  // src/components/AudioPlayer.js
  var html7 = htm_module_default.bind(y);
  function AudioPlayer({ mp3, onTimeUpdate }) {
    if (!mp3) {
      return null;
    }
    let onPlay = (_evt) => {
      console.log("Playing audio");
    };
    let onPause = (_evt) => {
      console.log("Pausing audio");
    };
    let _onTimeUpdate = (evt) => {
      onTimeUpdate(Math.floor(evt.target.currentTime * 1e3));
    };
    let onEnded = (_evt) => {
      console.log("Audio ended");
    };
    return html7`<div class="audio-footer">
        <audio controls preload onPlay=${onPlay} onPause=${onPause} onTimeUpdate=${_onTimeUpdate} onEnded=${onEnded}>
            <source src="${mp3}" type="audio/mpeg" />
        </audio>
    </div>`;
  }

  // app.js
  var html8 = htm_module_default.bind(y);
  function debounce(func, timeout = 300) {
    let timer;
    return (...args) => {
      clearTimeout(timer);
      timer = setTimeout(() => {
        func.apply(this, args);
      }, timeout);
    };
  }
  var App = class extends b {
    constructor(props) {
      super(props);
      this.data = props.data;
      this.index = this.data.getIndex();
      this.lastScrollTop = 0;
      this.lastNavInteraction = Date.now();
      this.state = {
        scrollDirection: "backward",
        expandedMenu: false,
        index: this.index,
        length: this.index.count,
        currentlySelected: null,
        currentlySelectedOrder: 0
      };
      this.initialElement = props.initialElement;
    }
    componentDidMount() {
      let element = this.base;
      let everything = element.querySelector(".everything-feed");
      everything.addEventListener("scroll", (e3) => {
        let scrollTop = everything.scrollTop;
        let changeDirection = debounce((direction) => {
          this.setState({
            scrollDirection: direction
          });
        });
        if (this.lastScrollTop > scrollTop && this.state.scrollDirection != "backward") {
          changeDirection("backward");
        } else if (this.lastScrollTop < scrollTop && this.state.scrollDirection != "forward") {
          changeDirection("forward");
          this.setState({
            scrollDirection: "forward"
          });
        } else {
        }
        this.lastScrollTop = scrollTop;
      });
      window.onkeyup = (e3) => {
        let key = e3.key;
        if (!this.state.expandedMenu) {
          if (key === "ArrowUp" || key.toLowerCase() === "w" || key === "PageUp") {
            e3.preventDefault();
            this.goUpOne();
          }
          if (key === "ArrowDown" || key.toLowerCase() === "s" || key === "PageDown" || key === " ") {
            e3.preventDefault();
            this.goDownOne();
          }
          if (key.toLowerCase() === "h" || key === "Home") {
            e3.preventDefault();
            this.goToTop();
          }
          if (key.toLowerCase() === "e" || key === "End") {
            e3.preventDefault();
            this.goToBottom();
          }
        }
        if (key.toLowerCase() === "m") {
          e3.preventDefault();
          this.setState({
            expandedMenu: !this.state.expandedMenu
          });
        }
      };
      if (this.initialElement) {
        console.warn("initial element is set: ", this.initialElement);
        this.moveTo({ id: this.initialElement.replace("#", "") });
        window.onload = () => {
          console.warn("initial element is set: ", this.initialElement);
          this.moveTo({ id: this.initialElement.replace("#", "") });
        };
      }
    }
    onTimeUpdate(time_ms) {
      if (this.state.index.audioGuide == false || this.state.index.mp3 == null) {
        return;
      }
      let time_counter = 0;
      for (let i3 = 0; i3 < this.state.index.toc.length; i3++) {
        let { id, timing } = this.state.index.toc[i3];
        let duration_ms = timing;
        if (time_ms < time_counter + duration_ms) {
          this.moveTo({ id });
          break;
        } else {
          time_counter += duration_ms;
        }
      }
      console.dir(this.state.index.toc);
    }
    goToTop() {
      this.lastNavInteraction = Date.now();
      let element = this.base;
      let everything = element.querySelector(".everything-feed");
      everything.scrollTop = 0;
    }
    moveTo({ id }) {
      if (this.state.currentlySelected == id) {
        return;
      }
      let element = document.getElementById(id);
      console.warn(`moving to ${id}`);
      console.warn(element);
      element.scrollIntoView({
        behavior: "smooth",
        block: "center"
      });
      this.setCurrentlySelected(id, this.data.getContentOrder(id));
    }
    goUpOne() {
      this.lastNavInteraction = Date.now();
      let upOneId = this.data.getPreviousContentId();
      if (upOneId) {
        this.moveTo({ id: upOneId });
      } else {
        console.warn("no previous content");
      }
    }
    goToBottom() {
      this.lastNavInteraction = Date.now();
      let element = this.base;
      let everything = element.querySelector(".everything-feed");
      everything.scrollTop = everything.scrollHeight;
    }
    goDownOne() {
      this.lastNavInteraction = Date.now();
      let downOneId = this.data.getNextContentId();
      if (downOneId) {
        this.moveTo({ id: downOneId });
      } else {
        console.warn("no next content");
      }
    }
    setCurrentlySelected(id, n3) {
      this.setState({
        currentlySelected: id,
        currentlySelectedOrder: n3
      });
      history.replaceState(null, null, `#${id}`);
    }
    render() {
      let headerVisible = "header-visible";
      let disableTransparentIcons = this.lastScrollTop > 60 ? "disable-transparent-icons" : "";
      let fullNavExpandedClass = this.state.expandedMenu ? "expanded" : "";
      let onMenu = () => {
        this.setState({
          expandedMenu: !this.state.expandedMenu
        });
      };
      let navigateTo = (id) => {
        this.moveTo({ id });
        this.setState({
          expandedMenu: false
        });
      };
      let items = this.state.index.contentIds.map((id, n3) => {
        let select = () => {
          this.setCurrentlySelected(id, n3);
        };
        return html8`<${VisibilityTriggerFrame} data=${this.data} order=${n3} id=${id} onPrimary=${select}/>`;
      });
      let mp3 = this.state.index.mp3;
      return html8`<div class="primary-card">
            <div class="content">
                <header id="primary-header" class="${headerVisible} ${disableTransparentIcons}">
                    <${Nav}
                        onTop=${this.goToTop.bind(this)}
                        onBottom=${this.goToBottom.bind(this)}
                        onDown=${this.goDownOne.bind(this)}
                        onUp=${this.goUpOne.bind(this)}
                        onMenu=${onMenu}
                    />
                </header>
                <header id="full-header" class="${fullNavExpandedClass} disable-transparent-icons">
                    <${NavDropdown}
                        onMenu=${onMenu.bind(this)}
                        navigateTo=${navigateTo.bind(this)}
                        data=${this.data}
                    />
                </header>
                <div class="everything-feed">
                    <div class="frames">
                    ${items}
                    </div>
                </div>
                <${AudioPlayer} mp3=${mp3} onTimeUpdate=${this.onTimeUpdate.bind(this)} />
            </div>
        </div>`;
    }
  };
  var serverUrl = window.location.origin;
  var Data2 = initialize({ serverUrl });
  if (!window.location.pathname.endsWith("/")) {
    window.location = `${window.location.origin}${window.location.pathname}/${window.location.hash}`;
  }
  async function main() {
    if (window.location.pathname == "/") {
      let hash = window.location.hash;
      await Data2.loadIndex({ userSlug: null, contentSlug: null, contentId: hash });
    } else {
      let parts = window.location.pathname.split("/");
      let userSlug = parts[2];
      let contentSlug = parts[3];
      let hash = window.location.hash;
      console.warn(`loading index for s/${userSlug}/${contentSlug}#${hash}`);
      await Data2.loadIndex({ userSlug, contentSlug, contentId: hash });
    }
    let app = html8`<${App} data=${Data2} initialElement=${window.location.hash} />`;
    B(app, document.getElementById("app"));
  }
  main();
})();
