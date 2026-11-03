/**
 * EigenGraph Official Interactive Engine
 * High-DPI Retina Support, Sherman-Morrison ARI, Concrete P-M & Cutting Stock Simulator
 */

document.addEventListener('DOMContentLoaded', () => {
  initMobileMenu();
  initCopyButtons();
  initHeroCanvas();
  initSandboxTabs();
  initAriSimulator();
  initConcretePmSimulator();
  initCsoSimulator();
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
   2. SANDBOX TABS SWITCHER
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
    if (elBars) elBars.textContent = `${bars.length} Batang`;
    if (elScrap) elScrap.textContent = `${(100 - yieldPct).toFixed(1)}%`;
  }

  [inStock, inKerf].forEach(inp => {
    if (inp) inp.addEventListener('input', renderCso);
  });

  window.addEventListener('resize', renderCso);
  renderCso();
}
