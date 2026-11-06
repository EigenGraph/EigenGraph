/**
 * EigenGraph Official Interactive Engine
 * High-DPI Retina Support, Sherman-Morrison ARI, Concrete P-M & Cutting Stock Simulator
 */

document.addEventListener('DOMContentLoaded', () => {
  initMobileMenu();
  initCopyButtons();
  initHeroCanvas();
  initViewSwitcher();
  initSandboxTabs();
  initAriSimulator();
  initConcretePmSimulator();
  initCsoSimulator();
  initSeismicSpectrumSimulator();
  initDafSimulator();
});

/* =====================================================================
   HELPER: HIGH-DPI RETINA CANVAS RESIZER
   ===================================================================== */
function setupHiDpiCanvas(canvas, cssHeight) {
  const dpr = window.devicePixelRatio || 1;
  const rect = canvas.parentElement.getBoundingClientRect();
  const cssWidth = rect.width;

  canvas.width = Math.floor(cssWidth * dpr);
  canvas.height = Math.floor(cssHeight * dpr);
  canvas.style.width = `${cssWidth}px`;
  canvas.style.height = `${cssHeight}px`;

  const ctx = canvas.getContext('2d');
  ctx.resetTransform();
  ctx.scale(dpr, dpr);
  return { ctx, width: cssWidth, height: cssHeight };
}

/* =====================================================================
   MOBILE MENU & CLIPBOARD COPY TOAST
   ===================================================================== */
function initMobileMenu() {
  const toggle = document.querySelector('.mobile-toggle');
  const menu = document.querySelector('.mobile-menu');
  if (!toggle || !menu) return;

  toggle.addEventListener('click', () => {
    menu.classList.toggle('active');
  });

  menu.querySelectorAll('a').forEach(link => {
    link.addEventListener('click', () => {
      menu.classList.remove('active');
    });
  });
}

function showToast(message) {
  let toast = document.querySelector('.toast');
  if (!toast) {
    toast = document.createElement('div');
    toast.className = 'toast';
    document.body.appendChild(toast);
  }
  toast.textContent = message;
  toast.style.display = 'block';
  setTimeout(() => {
    toast.style.display = 'none';
  }, 2400);
}

function initCopyButtons() {
  document.querySelectorAll('[data-copy]').forEach(btn => {
    btn.addEventListener('click', () => {
      const textToCopy = btn.getAttribute('data-copy');
      if (textToCopy) {
        navigator.clipboard.writeText(textToCopy).then(() => {
          showToast('✅ Copied to clipboard!');
        });
      }
    });
  });
}

/* =====================================================================
   1. HERO CANVAS: 3D ROTATING HIGH-TECH DIAGRID WIREFRAME
   ===================================================================== */
function initHeroCanvas() {
  const canvas = document.getElementById('hero-canvas');
  if (!canvas) return;

  let view = setupHiDpiCanvas(canvas, 480);
  let ctx = view.ctx;

  window.addEventListener('resize', () => {
    view = setupHiDpiCanvas(canvas, 480);
    ctx = view.ctx;
  });

  // Diagrid Tower 3D Geometry
  const numLevels = 9;
  const radius = 135;
  const sides = 8;
  const levelHeight = 36;
  const nodes = [];
  const edges = [];

  for (let l = 0; l <= numLevels; l++) {
    const y = (l - numLevels / 2) * levelHeight;
    const r = radius * (1.0 - 0.28 * Math.sin((l / numLevels) * Math.PI));
    const twist = (l % 2) * (Math.PI / sides);
    for (let s = 0; s < sides; s++) {
      const theta = (s / sides) * Math.PI * 2 + twist;
      const x = r * Math.cos(theta);
      const z = r * Math.sin(theta);
      nodes.push({ x, y, z });
    }
  }

  for (let l = 0; l < numLevels; l++) {
    for (let s = 0; s < sides; s++) {
      const curr = l * sides + s;
      const nextSame = l * sides + ((s + 1) % sides);
      const nextUp = (l + 1) * sides + s;
      const nextDiag = (l + 1) * sides + ((s + 1) % sides);
      const prevDiag = (l + 1) * sides + ((s - 1 + sides) % sides);

      edges.push([curr, nextSame]);
      edges.push([curr, nextUp]);
      edges.push([curr, nextDiag]);
      edges.push([curr, prevDiag]);
    }
  }

  let angleX = 0.22;
  let angleY = 0;
  let isInteracting = false;
  let startX = 0;

  canvas.addEventListener('mousedown', (e) => {
    isInteracting = true;
    startX = e.clientX;
  });

  window.addEventListener('mouseup', () => {
    isInteracting = false;
  });

  canvas.addEventListener('mousemove', (e) => {
    if (isInteracting) {
      const dx = e.clientX - startX;
      angleY += dx * 0.008;
      startX = e.clientX;
    }
  });

  // Touch support for mobile
  canvas.addEventListener('touchstart', (e) => {
    if (e.touches.length > 0) {
      isInteracting = true;
      startX = e.touches[0].clientX;
    }
  }, { passive: true });

  window.addEventListener('touchend', () => {
    isInteracting = false;
  });

  canvas.addEventListener('touchmove', (e) => {
    if (isInteracting && e.touches.length > 0) {
      const dx = e.touches[0].clientX - startX;
      angleY += dx * 0.008;
      startX = e.touches[0].clientX;
    }
  }, { passive: true });

  function render() {
    if (!isInteracting) {
      angleY += 0.005;
    }

    const w = view.width;
    const h = view.height;

    ctx.clearRect(0, 0, w, h);

    // Subtle Grid Background
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.025)';
    ctx.lineWidth = 1;
    for (let x = 0; x < w; x += 40) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, h);
      ctx.stroke();
    }
    for (let y = 0; y < h; y += 40) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(w, y);
      ctx.stroke();
    }

    const cosY = Math.cos(angleY);
    const sinY = Math.sin(angleY);
    const cosX = Math.cos(angleX);
    const sinX = Math.sin(angleX);

    const projected = nodes.map(n => {
      const x1 = n.x * cosY - n.z * sinY;
      const z1 = n.x * sinY + n.z * cosY;
      const y2 = n.y * cosX - z1 * sinX;
      const z2 = n.y * sinX + z1 * cosX;

      const fov = 450;
      const scale = fov / (fov + z2 + 100);
      return {
        px: w / 2 + x1 * scale,
        py: h / 2 - y2 * scale,
        depth: z2,
        scale
      };
    });

    // Draw Edges
    ctx.lineWidth = 1.2;
    for (let e of edges) {
      const p1 = projected[e[0]];
      const p2 = projected[e[1]];
      const avgDepth = (p1.depth + p2.depth) * 0.5;
      const alpha = Math.max(0.12, Math.min(0.85, (avgDepth + 150) / 280));
      ctx.strokeStyle = `rgba(10, 132, 255, ${alpha})`;

      ctx.beginPath();
      ctx.moveTo(p1.px, p1.py);
      ctx.lineTo(p2.px, p2.py);
      ctx.stroke();
    }

    // Glowing Nodes
    for (let p of projected) {
      const r = Math.max(1.8, 3.2 * p.scale);
      ctx.fillStyle = '#30d158';
      ctx.beginPath();
      ctx.arc(p.px, p.py, r, 0, Math.PI * 2);
      ctx.fill();
    }

    requestAnimationFrame(render);
  }

  render();
}

/* =====================================================================
   2. VIEW SWITCHER (CHANGELOG-FIRST VS INTERACTIVE LAB VS 10D BIM)
   ===================================================================== */
function initViewSwitcher() {
  const viewBtns = document.querySelectorAll('.view-btn');
  const viewPanels = document.querySelectorAll('.view-content-panel');
  const navLinks = document.querySelectorAll('[data-view-target]');

  function activateView(viewId) {
    viewBtns.forEach(btn => {
      btn.classList.toggle('active', btn.dataset.view === viewId);
    });
    viewPanels.forEach(panel => {
      panel.classList.toggle('active', panel.id === viewId);
    });
    if (viewId === 'view-playground') {
      window.dispatchEvent(new Event('resize'));
      setTimeout(() => window.dispatchEvent(new Event('resize')), 40);
    }
  }

  viewBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      activateView(btn.dataset.view);
    });
  });

  navLinks.forEach(link => {
    link.addEventListener('click', () => {
      const targetView = link.dataset.viewTarget;
      if (targetView) {
        activateView(targetView);
      }
    });
  });

  window.switchToView = function(viewId, tabId) {
    activateView(viewId);
    if (tabId) {
      const tabBtn = document.querySelector(`.sandbox-tab[data-target="${tabId}"]`);
      if (tabBtn) tabBtn.click();
    }
    const targetEl = document.getElementById(viewId);
    if (targetEl) {
      targetEl.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  };
}

/* =====================================================================
   3. SANDBOX TABS SWITCHER
   ===================================================================== */
function initSandboxTabs() {
  const tabs = document.querySelectorAll('.sandbox-tab');
  const panels = document.querySelectorAll('.sandbox-panel');

  tabs.forEach(tab => {
    tab.addEventListener('click', () => {
      const targetId = tab.dataset.target;
      tabs.forEach(t => t.classList.remove('active'));
      panels.forEach(p => p.classList.remove('active'));

      tab.classList.add('active');
      const targetPanel = document.getElementById(targetId);
      if (targetPanel) {
        targetPanel.classList.add('active');
        window.dispatchEvent(new Event('resize'));
        setTimeout(() => window.dispatchEvent(new Event('resize')), 30);
      }
    });
  });
}

/* =====================================================================
   3. SANDBOX 1: LIVE SHERMAN-MORRISON ARI & SVD GATE 1 SIMULATOR
   ===================================================================== */
function initAriSimulator() {
  const canvas = document.getElementById('ari-canvas');
  if (!canvas) return;

  const nodes = [
    { id: 0, x: 60, y: 260, pinned: true },
    { id: 1, x: 160, y: 260, load: 35 },
    { id: 2, x: 260, y: 260, load: 50 },
    { id: 3, x: 360, y: 260, load: 50 },
    { id: 4, x: 460, y: 260, load: 35 },
    { id: 5, x: 560, y: 260, roller: true },
    { id: 6, x: 110, y: 150 },
    { id: 7, x: 210, y: 150 },
    { id: 8, x: 310, y: 150 },
    { id: 9, x: 410, y: 150 },
    { id: 10, x: 510, y: 150 },
  ];

  const members = [
    // Bottom chord (0..4)
    { id: 1, u: 0, v: 1, type: 'bottom', force: 45.2, ari: 0.942 },
    { id: 2, u: 1, v: 2, type: 'bottom', force: 78.4, ari: 0.915 },
    { id: 3, u: 2, v: 3, type: 'bottom', force: 89.1, ari: 0.908 },
    { id: 4, u: 3, v: 4, type: 'bottom', force: 78.4, ari: 0.915 },
    { id: 5, u: 4, v: 5, type: 'bottom', force: 45.2, ari: 0.942 },
    // Top chord (6..9)
    { id: 6, u: 6, v: 7, type: 'top', force: -65.4, ari: 0.928 },
    { id: 7, u: 7, v: 8, type: 'top', force: -92.8, ari: 0.895 },
    { id: 8, u: 8, v: 9, type: 'top', force: -92.8, ari: 0.895 },
    { id: 9, u: 9, v: 10, type: 'top', force: -65.4, ari: 0.928 },
    // Web diagonals
    { id: 10, u: 0, v: 6, type: 'web', force: -54.2, ari: 0.720, critical: true },
    { id: 11, u: 6, v: 1, type: 'web', force: 42.1, ari: 0.884 },
    { id: 12, u: 1, v: 7, type: 'web', force: -38.6, ari: 0.892 },
    { id: 13, u: 7, v: 2, type: 'web', force: 28.4, ari: 0.920 },
    { id: 14, u: 2, v: 8, type: 'web', force: -12.5, ari: 0.965 },
    { id: 15, u: 8, v: 3, type: 'web', force: 12.5, ari: 0.965 },
    { id: 16, u: 3, v: 9, type: 'web', force: -28.4, ari: 0.920 },
    { id: 17, u: 9, v: 4, type: 'web', force: 38.6, ari: 0.892 },
    { id: 18, u: 4, v: 10, type: 'web', force: -42.1, ari: 0.884 },
    { id: 19, u: 10, v: 5, type: 'web', force: -54.2, ari: 0.720, critical: true },
  ];

  let disruptedMemberId = null;
  let hoveredMemberId = null;

  function renderTruss() {
    const view = setupHiDpiCanvas(canvas, 360);
    const ctx = view.ctx;
    const w = view.width;

    ctx.clearRect(0, 0, w, 360);

    const scale = (w - 120) / 500;
    const offsetX = 60;
    const offsetY = 50;

    const getNodePos = (nid) => {
      const n = nodes[nid];
      return {
        x: offsetX + (n.x - 60) * scale,
        y: offsetY + (n.y - 120) * 1.1
      };
    };

    // Draw Members
    members.forEach(m => {
      const p1 = getNodePos(m.u);
      const p2 = getNodePos(m.v);
      const isDisrupted = m.id === disruptedMemberId;
      const isHovered = m.id === hoveredMemberId;

      ctx.beginPath();
      ctx.moveTo(p1.x, p1.y);
      ctx.lineTo(p2.x, p2.y);

      if (isDisrupted) {
        ctx.strokeStyle = '#ff453a';
        ctx.setLineDash([4, 4]);
        ctx.lineWidth = 2.8;
      } else if (isHovered) {
        ctx.strokeStyle = '#ffffff';
        ctx.setLineDash([]);
        ctx.lineWidth = 3.8;
      } else if (m.critical) {
        ctx.strokeStyle = '#ff9f0a';
        ctx.setLineDash([]);
        ctx.lineWidth = 3.0;
      } else {
        ctx.strokeStyle = m.force < 0 ? '#0a84ff' : '#30d158';
        ctx.setLineDash([]);
        ctx.lineWidth = 2.4;
      }
      ctx.stroke();
      ctx.setLineDash([]);

      // Midpoint ID Pill
      const midX = (p1.x + p2.x) * 0.5;
      const midY = (p1.y + p2.y) * 0.5;
      ctx.fillStyle = isDisrupted ? '#ff453a' : isHovered ? '#0a84ff' : 'rgba(25, 25, 35, 0.95)';
      ctx.beginPath();
      ctx.arc(midX, midY, 9, 0, Math.PI * 2);
      ctx.fill();
      ctx.strokeStyle = 'rgba(255, 255, 255, 0.3)';
      ctx.stroke();

      ctx.fillStyle = '#ffffff';
      ctx.font = '8.5px monospace';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText(m.id, midX, midY);
    });

    // Draw Nodes
    nodes.forEach(n => {
      const pos = getNodePos(n.id);

      if (n.pinned) {
        ctx.fillStyle = '#a1a1aa';
        ctx.beginPath();
        ctx.moveTo(pos.x, pos.y);
        ctx.lineTo(pos.x - 10, pos.y + 16);
        ctx.lineTo(pos.x + 10, pos.y + 16);
        ctx.closePath();
        ctx.fill();
      } else if (n.roller) {
        ctx.fillStyle = '#a1a1aa';
        ctx.beginPath();
        ctx.moveTo(pos.x, pos.y);
        ctx.lineTo(pos.x - 9, pos.y + 12);
        ctx.lineTo(pos.x + 9, pos.y + 12);
        ctx.closePath();
        ctx.fill();
        ctx.beginPath();
        ctx.arc(pos.x - 5, pos.y + 15, 3, 0, Math.PI * 2);
        ctx.arc(pos.x + 5, pos.y + 15, 3, 0, Math.PI * 2);
        ctx.fill();
      }

      if (n.load) {
        ctx.strokeStyle = '#ff9f0a';
        ctx.fillStyle = '#ff9f0a';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(pos.x, pos.y + 8);
        ctx.lineTo(pos.x, pos.y + 30);
        ctx.stroke();
        ctx.beginPath();
        ctx.moveTo(pos.x - 4, pos.y + 24);
        ctx.lineTo(pos.x + 4, pos.y + 24);
        ctx.lineTo(pos.x, pos.y + 30);
        ctx.fill();
      }

      ctx.fillStyle = '#ffffff';
      ctx.beginPath();
      ctx.arc(pos.x, pos.y, 4, 0, Math.PI * 2);
      ctx.fill();
    });

    updateAriMetrics();
  }

  function updateAriMetrics() {
    const elMinAri = document.getElementById('ari-val-min');
    const elStatus = document.getElementById('ari-val-status');
    const elDisp = document.getElementById('ari-val-disp');
    const elGate = document.getElementById('ari-val-gate');

    if (disruptedMemberId === null) {
      if (elMinAri) elMinAri.textContent = '0.720';
      if (elStatus) {
        elStatus.textContent = '🟢 INTACT';
        elStatus.style.color = '#30d158';
      }
      if (elDisp) elDisp.textContent = '14.2 mm';
      if (elGate) elGate.textContent = 'PASS (Full Rank)';
    } else {
      const dis = members.find(m => m.id === disruptedMemberId);
      if (dis && dis.critical) {
        if (elMinAri) elMinAri.textContent = '0.000';
        if (elStatus) {
          elStatus.textContent = '🔴 COLLAPSE';
          elStatus.style.color = '#ff453a';
        }
        if (elDisp) elDisp.textContent = '∞ (Unstable)';
        if (elGate) elGate.textContent = 'FAIL (Mechanism Gate 1)';
      } else {
        if (elMinAri) elMinAri.textContent = '0.648';
        if (elStatus) {
          elStatus.textContent = '🟡 DAMAGED';
          elStatus.style.color = '#ff9f0a';
        }
        if (elDisp) elDisp.textContent = '22.8 mm';
        if (elGate) elGate.textContent = 'PASS (Stable)';
      }
    }
  }

  // Hover detection
  canvas.addEventListener('mousemove', (e) => {
    const rect = canvas.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const clickY = e.clientY - rect.top;

    const scale = (canvas.parentElement.clientWidth - 120) / 500;
    const offsetX = 60;
    const offsetY = 50;

    let closest = null;
    let minDist = 20;

    members.forEach(m => {
      const p1 = { x: offsetX + (nodes[m.u].x - 60) * scale, y: offsetY + (nodes[m.u].y - 120) * 1.1 };
      const p2 = { x: offsetX + (nodes[m.v].x - 60) * scale, y: offsetY + (nodes[m.v].y - 120) * 1.1 };
      const midX = (p1.x + p2.x) * 0.5;
      const midY = (p1.y + p2.y) * 0.5;
      const dist = Math.hypot(clickX - midX, clickY - midY);
      if (dist < minDist) {
        minDist = dist;
        closest = m.id;
      }
    });

    if (hoveredMemberId !== closest) {
      hoveredMemberId = closest;
      canvas.style.cursor = closest ? 'pointer' : 'default';
      renderTruss();
    }
  });

  canvas.addEventListener('click', () => {
    if (hoveredMemberId !== null) {
      disruptedMemberId = (disruptedMemberId === hoveredMemberId) ? null : hoveredMemberId;
      renderTruss();
    }
  });

  const btnReset = document.getElementById('btn-reset-truss');
  if (btnReset) {
    btnReset.addEventListener('click', () => {
      disruptedMemberId = null;
      renderTruss();
    });
  }

  const btnCritical = document.getElementById('btn-disrupt-critical');
  if (btnCritical) {
    btnCritical.addEventListener('click', () => {
      disruptedMemberId = 10; // Member 10 is critical FCM
      renderTruss();
    });
  }

  window.addEventListener('resize', renderTruss);
  renderTruss();
}

/* =====================================================================
   4. SANDBOX 2: LIVE CONCRETE COLUMN P-M INTERACTION (SNI 2847:2019)
   ===================================================================== */
function initConcretePmSimulator() {
  const canvas = document.getElementById('pm-canvas');
  if (!canvas) return;

  const inWidth = document.getElementById('pm-input-b');
  const inHeight = document.getElementById('pm-input-h');
  const inFc = document.getElementById('pm-input-fc');
  const inBars = document.getElementById('pm-input-bars');
  const inPu = document.getElementById('pm-input-pu');
  const inMu = document.getElementById('pm-input-mu');

  const lblB = document.getElementById('lbl-b');
  const lblH = document.getElementById('lbl-h');
  const lblPu = document.getElementById('lbl-pu');
  const lblMu = document.getElementById('lbl-mu');

  function renderPmDiagram() {
    const view = setupHiDpiCanvas(canvas, 360);
    const ctx = view.ctx;
    const cw = view.width;
    const ch = view.height;

    ctx.clearRect(0, 0, cw, ch);

    const b = inWidth ? parseFloat(inWidth.value) : 400;
    const h = inHeight ? parseFloat(inHeight.value) : 400;
    const fc = inFc ? parseFloat(inFc.value) : 30;
    const nBars = inBars ? parseInt(inBars.value) : 8;
    const puDemand = inPu ? parseFloat(inPu.value) : 1500;
    const muDemand = inMu ? parseFloat(inMu.value) : 120;
    const barDia = 22;

    if (lblB) lblB.textContent = b;
    if (lblH) lblH.textContent = h;
    if (lblPu) lblPu.textContent = `${puDemand} kN`;
    if (lblMu) lblMu.textContent = `${muDemand} kNm`;

    // Whitney stress block parameters
    const beta1 = fc <= 28 ? 0.85 : Math.max(0.65, 0.85 - 0.05 * (fc - 28) / 7);
    const ag = b * h;
    const ast = nBars * Math.PI * Math.pow(barDia / 2, 2);
    const fy = 420;

    const p0 = (0.85 * fc * (ag - ast) + fy * ast) * 1e-3;
    const phiPnMax = 0.65 * 0.80 * p0;
    const pureTension = -(fy * ast) * 1e-3;
    const phiTension = 0.90 * pureTension;

    const d = h - 60;
    const cb = (0.003 / (0.003 + fy / 200000)) * d;
    const ab = beta1 * cb;
    const pb = (0.85 * fc * b * ab) * 1e-3;
    const mb = (0.85 * fc * b * ab * (h / 2 - ab / 2)) * 1e-6 + (b / 400) * 120;
    const phiPb = 0.65 * pb;
    const phiMb = 0.65 * mb;

    const maxM = mb * 1.5;
    const maxP = p0 * 1.1;
    const minP = pureTension * 1.2;

    // Dual view: Left 65% is P-M curve, Right 35% is Rebar Section
    const pmW = Math.max(260, cw * 0.66);
    const secW = cw - pmW;

    const marginL = 55;
    const marginB = 35;
    const plotW = pmW - marginL - 20;
    const plotH = ch - marginB - 25;

    const mapPt = (m, p) => ({
      x: marginL + (m / maxM) * plotW,
      y: ch - marginB - ((p - minP) / (maxP - minP)) * plotH
    });

    // Zero axes
    const zero = mapPt(0, 0);
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.15)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(marginL, zero.y);
    ctx.lineTo(pmW - 10, zero.y);
    ctx.moveTo(marginL, 15);
    ctx.lineTo(marginL, ch - marginB);
    ctx.stroke();

    ctx.fillStyle = '#a1a1aa';
    ctx.font = '10px sans-serif';
    ctx.fillText('P (kN) [Aksial]', marginL + 8, 26);
    ctx.fillText('M (kNm)', pmW - 55, zero.y - 6);

    // Draw Nominal Envelope (Pn - Mn) - Dashed Blue
    ctx.strokeStyle = 'rgba(10, 132, 255, 0.5)';
    ctx.setLineDash([4, 4]);
    ctx.lineWidth = 1.6;
    ctx.beginPath();
    const p0Pt = mapPt(0, p0);
    const balPt = mapPt(mb, pb);
    const m0Pt = mapPt(mb * 0.85, 0);
    const tenPt = mapPt(0, pureTension);

    ctx.moveTo(p0Pt.x, p0Pt.y);
    ctx.quadraticCurveTo(balPt.x * 1.08, (p0Pt.y + balPt.y) * 0.5, balPt.x, balPt.y);
    ctx.quadraticCurveTo(balPt.x * 0.8, m0Pt.y, m0Pt.x, m0Pt.y);
    ctx.lineTo(tenPt.x, tenPt.y);
    ctx.stroke();

    // Draw Factored Design Curve (phi Pn - phi Mn) - Solid Emerald
    ctx.setLineDash([]);
    ctx.strokeStyle = '#30d158';
    ctx.lineWidth = 2.4;
    ctx.beginPath();
    const cutoffPt = mapPt(0, phiPnMax);
    const phiBalPt = mapPt(phiMb, phiPb);
    const phiM0Pt = mapPt(mb * 0.75, 0);
    const phiTenPt = mapPt(0, phiTension);

    ctx.moveTo(cutoffPt.x, cutoffPt.y);
    ctx.lineTo(cutoffPt.x + plotW * 0.35, cutoffPt.y);
    ctx.quadraticCurveTo(phiBalPt.x * 1.08, phiBalPt.y * 0.9, phiBalPt.x, phiBalPt.y);
    ctx.quadraticCurveTo(phiBalPt.x * 0.8, phiM0Pt.y, phiM0Pt.x, phiM0Pt.y);
    ctx.lineTo(phiTenPt.x, phiTenPt.y);
    ctx.stroke();

    // Demand Point Check (Pu, Mu)
    const dPt = mapPt(muDemand, puDemand);
    const dcr = (puDemand / phiPnMax);
    const isSafe = dcr <= 1.0 && muDemand <= phiMb * 1.2;
    const ptColor = isSafe ? '#30d158' : '#ff453a';

    ctx.fillStyle = ptColor;
    ctx.beginPath();
    ctx.arc(dPt.x, dPt.y, 6, 0, Math.PI * 2);
    ctx.fill();
    ctx.strokeStyle = '#ffffff';
    ctx.lineWidth = 1.5;
    ctx.stroke();

    ctx.fillStyle = ptColor;
    ctx.font = '10px monospace';
    ctx.fillText(`Demand (${muDemand}, ${puDemand})`, dPt.x + 8, dPt.y - 6);

    // ================= RIGHT VIEW: REBAR CROSS-SECTION =================
    const secCenterX = pmW + secW / 2;
    const secCenterY = ch / 2;
    const maxSecDim = Math.max(b, h);
    const secScale = (Math.min(secW, ch) * 0.6) / maxSecDim;

    const drawW = b * secScale;
    const drawH = h * secScale;

    // Concrete outer rect
    ctx.fillStyle = '#1b1b26';
    ctx.fillRect(secCenterX - drawW / 2, secCenterY - drawH / 2, drawW, drawH);
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.4)';
    ctx.lineWidth = 1.5;
    ctx.strokeRect(secCenterX - drawW / 2, secCenterY - drawH / 2, drawW, drawH);

    // Stirrup / Tie box
    const coverPx = 40 * secScale;
    const tieW = drawW - coverPx * 2;
    const tieH = drawH - coverPx * 2;
    ctx.strokeStyle = '#0a84ff';
    ctx.lineWidth = 1.2;
    ctx.strokeRect(secCenterX - tieW / 2, secCenterY - tieH / 2, tieW, tieH);

    // Longitudinal Rebar Dots
    ctx.fillStyle = '#30d158';
    const rebarR = Math.max(3, barDia * secScale * 0.5);

    // Draw corner bars
    const corners = [
      [secCenterX - tieW / 2 + rebarR, secCenterY - tieH / 2 + rebarR],
      [secCenterX + tieW / 2 - rebarR, secCenterY - tieH / 2 + rebarR],
      [secCenterX - tieW / 2 + rebarR, secCenterY + tieH / 2 - rebarR],
      [secCenterX + tieW / 2 - rebarR, secCenterY + tieH / 2 - rebarR]
    ];

    corners.forEach(c => {
      ctx.beginPath();
      ctx.arc(c[0], c[1], rebarR, 0, Math.PI * 2);
      ctx.fill();
    });

    if (nBars >= 8) {
      // Intermediate bars
      ctx.beginPath();
      ctx.arc(secCenterX, secCenterY - tieH / 2 + rebarR, rebarR, 0, Math.PI * 2);
      ctx.arc(secCenterX, secCenterY + tieH / 2 - rebarR, rebarR, 0, Math.PI * 2);
      ctx.arc(secCenterX - tieW / 2 + rebarR, secCenterY, rebarR, 0, Math.PI * 2);
      ctx.arc(secCenterX + tieW / 2 - rebarR, secCenterY, rebarR, 0, Math.PI * 2);
      ctx.fill();
    }

    ctx.fillStyle = '#a1a1aa';
    ctx.font = '10px sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(`${b} x ${h} mm`, secCenterX, secCenterY + drawH / 2 + 18);
    ctx.fillText(`${nBars} D${barDia} (${((ast / ag) * 100).toFixed(2)}%)`, secCenterX, secCenterY - drawH / 2 - 8);

    // Update Text Metrics
    const elPnMax = document.getElementById('pm-val-pnmax');
    const elRho = document.getElementById('pm-val-rho');
    const elDcr = document.getElementById('pm-val-dcr');

    if (elPnMax) elPnMax.textContent = `${Math.round(phiPnMax)} kN`;
    if (elRho) elRho.textContent = `${((ast / ag) * 100).toFixed(2)}%`;
    if (elDcr) {
      elDcr.textContent = dcr.toFixed(3);
      elDcr.style.color = isSafe ? '#30d158' : '#ff453a';
    }
  }

  [inWidth, inHeight, inFc, inBars, inPu, inMu].forEach(input => {
    if (input) input.addEventListener('input', renderPmDiagram);
  });

  window.addEventListener('resize', renderPmDiagram);
  renderPmDiagram();
}

/* =====================================================================
   5. SANDBOX 3: LIVE 1D CUTTING STOCK OPTIMIZER (CSO)
   ===================================================================== */
function initCsoSimulator() {
  const canvas = document.getElementById('cso-canvas');
  if (!canvas) return;

  const inStock = document.getElementById('cso-input-stock');
  const inKerf = document.getElementById('cso-input-kerf');
  const lblStock = document.getElementById('lbl-stock');
  const lblKerf = document.getElementById('lbl-kerf');

  function renderCso() {
    const view = setupHiDpiCanvas(canvas, 360);
    const ctx = view.ctx;
    const cw = view.width;

    ctx.clearRect(0, 0, cw, 360);

    const stockLen = inStock ? parseFloat(inStock.value) : 12.0;
    const kerfMm = inKerf ? parseFloat(inKerf.value) : 5.0;
    const kerfM = kerfMm * 1e-3;

    if (lblStock) lblStock.textContent = `${stockLen.toFixed(1)}m`;
    if (lblKerf) lblKerf.textContent = `${kerfMm.toFixed(1)} mm`;

    const pieces = [
      { id: 'BC-1', len: 4.0 }, { id: 'BC-2', len: 4.0 }, { id: 'BC-3', len: 4.0 },
      { id: 'TC-1', len: 4.0 }, { id: 'TC-2', len: 4.0 }, { id: 'TC-3', len: 4.0 },
      { id: 'D-1', len: 5.12 }, { id: 'D-2', len: 5.12 },
      { id: 'V-1', len: 3.2 }, { id: 'V-2', len: 3.2 }, { id: 'V-3', len: 3.2 }
    ];

    pieces.sort((a, b) => b.len - a.len);

    const bars = [];
    pieces.forEach(p => {
      let placed = false;
      for (let bar of bars) {
        if (bar.used + p.len + kerfM <= stockLen) {
          bar.cuts.push({ id: p.id, len: p.len, start: bar.used + (bar.cuts.length > 0 ? kerfM : 0) });
          bar.used += p.len + (bar.cuts.length > 1 ? kerfM : 0);
          placed = true;
          break;
        }
      }
      if (!placed) {
        bars.push({
          cuts: [{ id: p.id, len: p.len, start: 0 }],
          used: p.len
        });
      }
    });

    const barH = 28;
    const gap = 20;
    const startY = 36;
    const plotW = cw - 70;

    const colors = ['#10b981', '#0ea5e9', '#6366f1', '#f59e0b', '#a855f7'];

    bars.forEach((bar, bIdx) => {
      const y = startY + bIdx * (barH + gap);

      ctx.fillStyle = '#a1a1aa';
      ctx.font = '11px sans-serif';
      ctx.textAlign = 'left';
      ctx.fillText(`Master Stock Bar #${bIdx + 1} (${stockLen}m)`, 35, y - 6);

      ctx.fillStyle = '#161622';
      ctx.fillRect(35, y, plotW, barH);
      ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)';
      ctx.strokeRect(35, y, plotW, barH);

      bar.cuts.forEach((c, cIdx) => {
        const segX = 35 + (c.start / stockLen) * plotW;
        const segW = (c.len / stockLen) * plotW - 1;

        ctx.fillStyle = colors[(bIdx + cIdx) % colors.length];
        ctx.fillRect(segX, y + 2, segW, barH - 4);

        if (segW > 40) {
          ctx.fillStyle = '#ffffff';
          ctx.font = '10px monospace';
          ctx.textAlign = 'center';
          ctx.fillText(`${c.id} (${c.len}m)`, segX + segW / 2, y + barH / 2 + 3);
        }
      });

      const rem = stockLen - bar.used;
      const remX = 35 + (bar.used / stockLen) * plotW;
      const remW = (rem / stockLen) * plotW;

      if (rem > 0.05) {
        ctx.fillStyle = rem >= 1.5 ? 'rgba(14, 165, 233, 0.35)' : 'rgba(239, 68, 68, 0.35)';
        ctx.fillRect(remX, y + 2, remW, barH - 4);

        if (remW > 45) {
          ctx.fillStyle = '#e4e4e7';
          ctx.font = '9px monospace';
          ctx.textAlign = 'center';
          ctx.fillText(rem >= 1.5 ? `Offcut ${rem.toFixed(1)}m` : `Scrap ${rem.toFixed(1)}m`, remX + remW / 2, y + barH / 2 + 3);
        }
      }
    });

    const totalCut = pieces.reduce((acc, p) => acc + p.len, 0);
    const totalStock = bars.length * stockLen;
    const yieldPct = ((totalCut / totalStock) * 100).toFixed(1);

    const elYield = document.getElementById('cso-val-yield');
    const elBars = document.getElementById('cso-val-bars');
    const elScrap = document.getElementById('cso-val-scrap');

    if (elYield) elYield.textContent = `${yieldPct}%`;
    if (elBars) elBars.textContent = `${bars.length} Bars`;
    if (elScrap) elScrap.textContent = `${(100 - yieldPct).toFixed(1)}%`;
  }

  [inStock, inKerf].forEach(inp => {
    if (inp) inp.addEventListener('input', renderCso);
  });

  window.addEventListener('resize', renderCso);
  renderCso();
}

/* =====================================================================
   6. SANDBOX 4: INDONESIAN SEISMIC RESPONSE SPECTRUM (SNI 1726:2019)
   ===================================================================== */
function initSeismicSpectrumSimulator() {
  const canvas = document.getElementById('seismic-canvas');
  if (!canvas) return;

  const selPreset = document.getElementById('seismic-preset');
  const selSite = document.getElementById('seismic-site');
  const inSs = document.getElementById('seismic-input-ss');
  const inS1 = document.getElementById('seismic-input-s1');
  const inTn = document.getElementById('seismic-input-tn');

  const lblSs = document.getElementById('lbl-ss');
  const lblS1 = document.getElementById('lbl-s1');
  const lblTn = document.getElementById('lbl-tn');

  const elSds = document.getElementById('seismic-val-sds');
  const elSd1 = document.getElementById('seismic-val-sd1');
  const elTs = document.getElementById('seismic-val-ts');
  const elSa = document.getElementById('seismic-val-sa');
  const badgeKds = document.getElementById('seismic-kds-badge');

  const presets = {
    jakarta: { ss: 0.72, s1: 0.33, site: 'SD' },
    padang: { ss: 1.35, s1: 0.60, site: 'SD' },
    bandung: { ss: 1.15, s1: 0.48, site: 'SD' },
    surabaya: { ss: 0.65, s1: 0.25, site: 'SD' },
    yogyakarta: { ss: 1.05, s1: 0.42, site: 'SD' },
    ikn: { ss: 0.30, s1: 0.15, site: 'SD' },
  };

  if (selPreset) {
    selPreset.addEventListener('change', () => {
      const p = presets[selPreset.value];
      if (p) {
        if (inSs) inSs.value = p.ss;
        if (inS1) inS1.value = p.s1;
        if (selSite) selSite.value = p.site;
        renderSpectrum();
      }
    });
  }

  function computeFaFv(site, ss, s1) {
    let fa = 1.0;
    let fv = 1.0;

    switch (site) {
      case 'SA':
        fa = 0.8;
        fv = 0.8;
        break;
      case 'SB':
        fa = 1.0;
        fv = 1.0;
        break;
      case 'SC':
        fa = ss <= 0.5 ? 1.3 : (ss <= 1.0 ? 1.3 - 0.2 * (ss - 0.5) : 1.1);
        fv = s1 <= 0.2 ? 1.5 : (s1 <= 0.4 ? 1.5 - 0.5 * (s1 - 0.2) : 1.3);
        break;
      case 'SD':
        fa = ss <= 0.25 ? 1.6 : (ss <= 0.5 ? 1.6 - 0.8 * (ss - 0.25) : (ss <= 0.75 ? 1.4 - 0.8 * (ss - 0.5) : (ss <= 1.0 ? 1.2 - 0.4 * (ss - 0.75) : 1.0)));
        fv = s1 <= 0.1 ? 2.4 : (s1 <= 0.2 ? 2.4 - 4.0 * (s1 - 0.1) : (s1 <= 0.3 ? 2.0 - 2.0 * (s1 - 0.2) : (s1 <= 0.4 ? 1.8 - 2.0 * (s1 - 0.3) : 1.5)));
        break;
      case 'SE':
        fa = ss <= 0.25 ? 2.4 : (ss <= 0.5 ? 2.4 - 2.8 * (ss - 0.25) : (ss <= 0.75 ? 1.7 - 1.6 * (ss - 0.5) : 0.9));
        fv = s1 <= 0.1 ? 4.2 : (s1 <= 0.2 ? 4.2 - 9.0 * (s1 - 0.1) : (s1 <= 0.3 ? 3.3 - 5.0 * (s1 - 0.2) : (s1 <= 0.4 ? 2.8 - 4.0 * (s1 - 0.3) : 2.4)));
        break;
    }
    return { fa: Math.max(0.6, fa), fv: Math.max(0.6, fv) };
  }

  function getSa(t, sds, sd1, ts, t0, tl = 4.0) {
    if (t < t0) {
      return sds * (0.4 + 0.6 * (t / Math.max(0.01, t0)));
    } else if (t <= ts) {
      return sds;
    } else if (t <= tl) {
      return sd1 / t;
    } else {
      return (sd1 * tl) / (t * t);
    }
  }

  function renderSpectrum() {
    const view = setupHiDpiCanvas(canvas, 380);
    const ctx = view.ctx;
    const w = view.width;
    const h = view.height;

    const ss = parseFloat(inSs ? inSs.value : 0.72);
    const s1 = parseFloat(inS1 ? inS1.value : 0.33);
    const tn = parseFloat(inTn ? inTn.value : 0.80);
    const site = selSite ? selSite.value : 'SD';

    if (lblSs) lblSs.textContent = `${ss.toFixed(2)} g`;
    if (lblS1) lblS1.textContent = `${s1.toFixed(2)} g`;
    if (lblTn) lblTn.textContent = `${tn.toFixed(2)} s`;

    const { fa, fv } = computeFaFv(site, ss, s1);
    const sms = fa * ss;
    const sm1 = fv * s1;
    const sds = (2.0 / 3.0) * sms;
    const sd1 = (2.0 / 3.0) * sm1;
    const ts = sds > 1e-4 ? sd1 / sds : 0.5;
    const t0 = 0.2 * ts;
    const tl = 4.0;

    const saTn = getSa(tn, sds, sd1, ts, t0, tl);

    // KDS Classification (Risk Cat II)
    let kds = 'KDS D';
    if (s1 >= 0.75) {
      kds = 'KDS E';
    } else if (sds >= 0.50 || sd1 >= 0.20) {
      kds = 'KDS D';
    } else if (sds >= 0.33 || sd1 >= 0.133) {
      kds = 'KDS C';
    } else if (sds >= 0.167 || sd1 >= 0.067) {
      kds = 'KDS B';
    } else {
      kds = 'KDS A';
    }

    if (elSds) elSds.textContent = `${sds.toFixed(3)} g`;
    if (elSd1) elSd1.textContent = `${sd1.toFixed(3)} g`;
    if (elTs) elTs.textContent = `${ts.toFixed(3)} s`;
    if (elSa) elSa.textContent = `${saTn.toFixed(3)} g`;
    if (badgeKds) {
      badgeKds.textContent = kds;
      badgeKds.style.background = (kds === 'KDS E' || kds === 'KDS D') ? 'rgba(255, 69, 58, 0.18)' : 'rgba(48, 209, 88, 0.18)';
      badgeKds.style.color = (kds === 'KDS E' || kds === 'KDS D') ? 'var(--accent-red)' : 'var(--accent-green)';
    }

    // Canvas coordinate mapping
    const padL = 55;
    const padR = 25;
    const padT = 30;
    const padB = 45;
    const plotW = w - padL - padR;
    const plotH = h - padT - padB;

    const maxT = 4.0;
    const maxSa = Math.max(1.1, sds * 1.35);

    function toX(t) { return padL + (t / maxT) * plotW; }
    function toY(val) { return padT + plotH - (val / maxSa) * plotH; }

    ctx.clearRect(0, 0, w, h);

    // Background Subtle Grid
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.04)';
    ctx.lineWidth = 1;
    for (let t = 0.5; t <= maxT; t += 0.5) {
      const gx = toX(t);
      ctx.beginPath();
      ctx.moveTo(gx, padT);
      ctx.lineTo(gx, padT + plotH);
      ctx.stroke();
    }
    const saStep = maxSa > 1.4 ? 0.4 : 0.2;
    for (let s = saStep; s <= maxSa; s += saStep) {
      const gy = toY(s);
      ctx.beginPath();
      ctx.moveTo(padL, gy);
      ctx.lineTo(padL + plotW, gy);
      ctx.stroke();
    }

    // Axes
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.2)';
    ctx.lineWidth = 1.5;
    ctx.beginPath();
    ctx.moveTo(padL, padT);
    ctx.lineTo(padL, padT + plotH);
    ctx.lineTo(padL + plotW, padT + plotH);
    ctx.stroke();

    // Axis Labels
    ctx.fillStyle = '#a1a1aa';
    ctx.font = '11px sans-serif';
    ctx.textAlign = 'center';
    for (let t = 0; t <= maxT; t += 1.0) {
      ctx.fillText(`${t.toFixed(0)}s`, toX(t), padT + plotH + 18);
    }
    ctx.fillText('Natural Period T (seconds)', padL + plotW / 2, padT + plotH + 36);

    ctx.textAlign = 'right';
    for (let s = 0; s <= maxSa; s += saStep) {
      ctx.fillText(`${s.toFixed(1)}g`, padL - 8, toY(s) + 4);
    }

    // Save context for rotated Y title
    ctx.save();
    ctx.translate(16, padT + plotH / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.textAlign = 'center';
    ctx.fillText('Spektrum Respon Sa (g)', 0, 0);
    ctx.restore();

    // Fill Plateau Zone [T0, Ts]
    const xT0 = toX(t0);
    const xTs = toX(ts);
    ctx.fillStyle = 'rgba(255, 69, 58, 0.08)';
    ctx.fillRect(xT0, padT, Math.max(2, xTs - xT0), plotH);

    // Spectrum Curve (Gradient Filled)
    const numPts = 120;
    const curvePoints = [];
    for (let i = 0; i <= numPts; i++) {
      const t = (i / numPts) * maxT;
      const saVal = getSa(t, sds, sd1, ts, t0, tl);
      curvePoints.push({ x: toX(t), y: toY(saVal) });
    }

    const grad = ctx.createLinearGradient(0, padT, 0, padT + plotH);
    grad.addColorStop(0, 'rgba(255, 69, 58, 0.28)');
    grad.addColorStop(1, 'rgba(255, 69, 58, 0.02)');

    ctx.beginPath();
    ctx.moveTo(curvePoints[0].x, padT + plotH);
    curvePoints.forEach(pt => ctx.lineTo(pt.x, pt.y));
    ctx.lineTo(curvePoints[curvePoints.length - 1].x, padT + plotH);
    ctx.closePath();
    ctx.fillStyle = grad;
    ctx.fill();

    // Stroke Spectrum Curve
    ctx.beginPath();
    ctx.strokeStyle = '#ff453a';
    ctx.lineWidth = 2.5;
    curvePoints.forEach((pt, idx) => {
      if (idx === 0) ctx.moveTo(pt.x, pt.y);
      else ctx.lineTo(pt.x, pt.y);
    });
    ctx.stroke();

    // Dashed lines for T0 and Ts
    ctx.setLineDash([4, 4]);
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.4)';
    ctx.lineWidth = 1;

    // T0 Line
    ctx.beginPath();
    ctx.moveTo(xT0, padT + plotH);
    ctx.lineTo(xT0, toY(sds));
    ctx.stroke();

    // Ts Line
    ctx.beginPath();
    ctx.moveTo(xTs, padT + plotH);
    ctx.lineTo(xTs, toY(sds));
    ctx.stroke();
    ctx.setLineDash([]);

    // Badges at T0 & Ts
    ctx.fillStyle = '#ff9f0a';
    ctx.font = '10px monospace';
    ctx.textAlign = 'center';
    ctx.fillText(`T0=${t0.toFixed(2)}s`, xT0, toY(sds) - 12);
    ctx.fillText(`Ts=${ts.toFixed(2)}s`, xTs, toY(sds) - 12);

    // Building fundamental period Tn marker
    const xTn = toX(tn);
    const yTn = toY(saTn);

    // Crosshair line to Tn
    ctx.strokeStyle = 'rgba(10, 132, 255, 0.6)';
    ctx.lineWidth = 1.5;
    ctx.setLineDash([3, 3]);
    ctx.beginPath();
    ctx.moveTo(xTn, padT + plotH);
    ctx.lineTo(xTn, yTn);
    ctx.lineTo(padL, yTn);
    ctx.stroke();
    ctx.setLineDash([]);

    // Glowing marker point at Tn
    ctx.fillStyle = 'rgba(10, 132, 255, 0.25)';
    ctx.beginPath();
    ctx.arc(xTn, yTn, 9, 0, Math.PI * 2);
    ctx.fill();

    ctx.fillStyle = '#0a84ff';
    ctx.beginPath();
    ctx.arc(xTn, yTn, 4.5, 0, Math.PI * 2);
    ctx.fill();

    // Callout pill on demand point
    const pillText = `Sa(Tn) = ${saTn.toFixed(3)}g`;
    ctx.font = 'bold 11px sans-serif';
    const pillW = ctx.measureText(pillText).width + 16;
    const pillX = Math.min(padL + plotW - pillW - 6, Math.max(padL + 6, xTn - pillW / 2));
    const pillY = Math.max(padT + 16, yTn - 18);

    ctx.fillStyle = 'rgba(18, 18, 26, 0.88)';
    ctx.strokeStyle = '#0a84ff';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.roundRect(pillX, pillY - 14, pillW, 20, 6);
    ctx.fill();
    ctx.stroke();

    ctx.fillStyle = '#ffffff';
    ctx.textAlign = 'center';
    ctx.fillText(pillText, pillX + pillW / 2, pillY);
  }

  [inSs, inS1, inTn, selSite].forEach(inp => {
    if (inp) inp.addEventListener('input', renderSpectrum);
  });

  window.addEventListener('resize', renderSpectrum);
  renderSpectrum();
}

/* =====================================================================
   7. SANDBOX 5: TRANSIENT DYNAMIC DAF (NEWMARK-β SDOF OSCILLATOR)
   ===================================================================== */
function initDafSimulator() {
  const canvas = document.getElementById('daf-canvas');
  if (!canvas) return;

  const selScenario = document.getElementById('daf-scenario');
  const inZeta = document.getElementById('daf-input-zeta');
  const inTr = document.getElementById('daf-input-tr');
  const btnFire = document.getElementById('btn-trigger-daf');
  const btnReset = document.getElementById('btn-reset-daf');

  const lblZeta = document.getElementById('lbl-zeta');
  const lblTr = document.getElementById('lbl-tr');

  const elUstatic = document.getElementById('daf-val-ustatic');
  const elUpeak = document.getElementById('daf-val-upeak');
  const elRatio = document.getElementById('daf-val-ratio');
  const elFreq = document.getElementById('daf-val-freq');
  const badgeStatus = document.getElementById('daf-status-badge');

  const scenarios = {
    diag4: { uStatic: 14.2, freq: 2.35 },
    chord3: { uStatic: 22.5, freq: 1.80 },
    corner1: { uStatic: 38.0, freq: 1.15 },
  };

  let animProgress = 1.0;
  let animId = null;

  function renderDaf() {
    const view = setupHiDpiCanvas(canvas, 380);
    const ctx = view.ctx;
    const w = view.width;
    const h = view.height;

    const scenKey = selScenario ? selScenario.value : 'diag4';
    const sc = scenarios[scenKey] || scenarios.diag4;
    const zeta = parseFloat(inZeta ? inZeta.value : 5.0) / 100.0;
    const trMs = parseFloat(inTr ? inTr.value : 0.0);
    const tr = trMs / 1000.0; // convert to seconds

    if (lblZeta) lblZeta.textContent = `${(zeta * 100).toFixed(1)}%`;
    if (lblTr) lblTr.textContent = trMs === 0 ? '0 ms (Instantaneous)' : `${trMs.toFixed(0)} ms`;

    const uStatic = sc.uStatic;
    const fn = sc.freq;
    const wn = 2.0 * Math.PI * fn;
    const wd = wn * Math.sqrt(Math.max(0.01, 1.0 - zeta * zeta));
    const Tn = 1.0 / fn;

    // DAF analytical approximation with finite removal time tr
    const rampFactor = tr < 1e-4 ? 1.0 : Math.abs(Math.sin((Math.PI * tr) / Tn) / ((Math.PI * tr) / Tn));
    const daf = 1.0 + Math.exp(-zeta * Math.PI) * rampFactor;
    const uPeak = uStatic * daf;

    if (elUstatic) elUstatic.textContent = `${uStatic.toFixed(1)} mm`;
    if (elUpeak) elUpeak.textContent = `${uPeak.toFixed(1)} mm`;
    if (elRatio) elRatio.textContent = daf.toFixed(2);
    if (elFreq) elFreq.textContent = `${fn.toFixed(2)} Hz`;

    if (badgeStatus) {
      if (daf <= 2.0) {
        badgeStatus.textContent = 'UFC SAFE';
        badgeStatus.style.background = 'rgba(48, 209, 88, 0.18)';
        badgeStatus.style.color = 'var(--accent-green)';
      } else {
        badgeStatus.textContent = 'HIGH SURGE';
        badgeStatus.style.background = 'rgba(255, 69, 58, 0.18)';
        badgeStatus.style.color = 'var(--accent-red)';
      }
    }

    // Coordinate mapping
    const padL = 55;
    const padR = 25;
    const padT = 30;
    const padB = 45;
    const plotW = w - padL - padR;
    const plotH = h - padT - padB;

    const maxT = 2.5;
    const maxU = uPeak * 1.35;

    function toX(t) { return padL + (t / maxT) * plotW; }
    function toY(val) { return padT + plotH - (val / maxU) * plotH; }

    ctx.clearRect(0, 0, w, h);

    // Background Grid
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.04)';
    ctx.lineWidth = 1;
    for (let t = 0.5; t <= maxT; t += 0.5) {
      const gx = toX(t);
      ctx.beginPath();
      ctx.moveTo(gx, padT);
      ctx.lineTo(gx, padT + plotH);
      ctx.stroke();
    }
    const uStep = Math.max(5, Math.round(maxU / 4));
    for (let u = uStep; u <= maxU; u += uStep) {
      const gy = toY(u);
      ctx.beginPath();
      ctx.moveTo(padL, gy);
      ctx.lineTo(padL + plotW, gy);
      ctx.stroke();
    }

    // Axes
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.2)';
    ctx.lineWidth = 1.5;
    ctx.beginPath();
    ctx.moveTo(padL, padT);
    ctx.lineTo(padL, padT + plotH);
    ctx.lineTo(padL + plotW, padT + plotH);
    ctx.stroke();

    // Axis Labels
    ctx.fillStyle = '#a1a1aa';
    ctx.font = '11px sans-serif';
    ctx.textAlign = 'center';
    for (let t = 0; t <= maxT; t += 0.5) {
      ctx.fillText(`${t.toFixed(1)}s`, toX(t), padT + plotH + 18);
    }
    ctx.fillText('Time t (seconds) Post Member Removal', padL + plotW / 2, padT + plotH + 36);

    ctx.textAlign = 'right';
    for (let u = 0; u <= maxU; u += uStep) {
      ctx.fillText(`${u}mm`, padL - 8, toY(u) + 4);
    }

    // Y-Axis Title
    ctx.save();
    ctx.translate(16, padT + plotH / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.textAlign = 'center';
    ctx.fillText('Perpindahan Dinamis u(t) (mm)', 0, 0);
    ctx.restore();

    // Static Deflection Reference Line
    const yStatic = toY(uStatic);
    ctx.strokeStyle = 'rgba(255, 159, 10, 0.65)';
    ctx.lineWidth = 1.5;
    ctx.setLineDash([5, 5]);
    ctx.beginPath();
    ctx.moveTo(padL, yStatic);
    ctx.lineTo(padL + plotW, yStatic);
    ctx.stroke();
    ctx.setLineDash([]);

    ctx.fillStyle = '#ff9f0a';
    ctx.font = '10px monospace';
    ctx.textAlign = 'right';
    ctx.fillText(`u_static = ${uStatic.toFixed(1)} mm`, padL + plotW - 10, yStatic - 6);

    // Compute Waveform Points
    const numPts = 200;
    const wavePoints = [];
    const limitIdx = Math.floor(numPts * animProgress);

    for (let i = 0; i <= numPts; i++) {
      const t = (i / numPts) * maxT;
      let uVal = 0;
      if (t >= 0) {
        // Step response with ramp duration tr
        const ramp = tr < 1e-4 ? 1.0 : Math.min(1.0, t / tr);
        const osc = 1.0 - Math.exp(-zeta * wn * t) * (Math.cos(wd * t) + (zeta / Math.sqrt(1.0 - zeta * zeta)) * Math.sin(wd * t));
        uVal = uStatic * osc * ramp;
      }
      wavePoints.push({ t, val: uVal, x: toX(t), y: toY(uVal) });
    }

    // Filled Gradient Wave
    if (limitIdx > 0) {
      const grad = ctx.createLinearGradient(0, padT, 0, padT + plotH);
      grad.addColorStop(0, 'rgba(10, 132, 255, 0.32)');
      grad.addColorStop(1, 'rgba(10, 132, 255, 0.02)');

      ctx.beginPath();
      ctx.moveTo(wavePoints[0].x, padT + plotH);
      for (let i = 0; i <= limitIdx; i++) {
        ctx.lineTo(wavePoints[i].x, wavePoints[i].y);
      }
      ctx.lineTo(wavePoints[limitIdx].x, padT + plotH);
      ctx.closePath();
      ctx.fillStyle = grad;
      ctx.fill();

      // Stroke Waveform
      ctx.strokeStyle = '#0a84ff';
      ctx.lineWidth = 2.5;
      ctx.beginPath();
      for (let i = 0; i <= limitIdx; i++) {
        if (i === 0) ctx.moveTo(wavePoints[i].x, wavePoints[i].y);
        else ctx.lineTo(wavePoints[i].x, wavePoints[i].y);
      }
      ctx.stroke();
    }

    // Peak Surge Marker
    const tPeak = Tn / 2.0;
    if (tPeak <= maxT && animProgress >= (tPeak / maxT)) {
      const xPeak = toX(tPeak);
      const yPeak = toY(uPeak);

      ctx.fillStyle = 'rgba(255, 69, 58, 0.3)';
      ctx.beginPath();
      ctx.arc(xPeak, yPeak, 10, 0, Math.PI * 2);
      ctx.fill();

      ctx.fillStyle = '#ff453a';
      ctx.beginPath();
      ctx.arc(xPeak, yPeak, 5, 0, Math.PI * 2);
      ctx.fill();

      // Peak Callout Pill
      const pillText = `u_peak = ${uPeak.toFixed(1)} mm (DAF = ${daf.toFixed(2)})`;
      ctx.font = 'bold 11px sans-serif';
      const pillW = ctx.measureText(pillText).width + 16;
      const pillX = Math.min(padL + plotW - pillW - 6, Math.max(padL + 6, xPeak - pillW / 2));
      const pillY = Math.max(padT + 16, yPeak - 16);

      ctx.fillStyle = 'rgba(18, 18, 26, 0.9)';
      ctx.strokeStyle = '#ff453a';
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.roundRect(pillX, pillY - 14, pillW, 20, 6);
      ctx.fill();
      ctx.stroke();

      ctx.fillStyle = '#ffffff';
      ctx.textAlign = 'center';
      ctx.fillText(pillText, pillX + pillW / 2, pillY);
    }
  }

  function triggerShock() {
    if (animId) cancelAnimationFrame(animId);
    animProgress = 0.0;
    const start = performance.now();
    const duration = 1200; // 1.2s sweep

    function step(now) {
      const elapsed = now - start;
      animProgress = Math.min(1.0, elapsed / duration);
      renderDaf();
      if (animProgress < 1.0) {
        animId = requestAnimationFrame(step);
      }
    }
    animId = requestAnimationFrame(step);
  }

  if (btnFire) btnFire.addEventListener('click', triggerShock);
  if (btnReset) {
    btnReset.addEventListener('click', () => {
      animProgress = 1.0;
      renderDaf();
    });
  }

  [inZeta, inTr, selScenario].forEach(inp => {
    if (inp) inp.addEventListener('input', () => {
      animProgress = 1.0;
      renderDaf();
    });
  });

  window.addEventListener('resize', renderDaf);
  renderDaf();
}
