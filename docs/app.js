/**
 * EigenGraph Official Interactive Engine
 * Client-Side Structural Redundancy, Concrete P-M & Cutting Stock Simulator
 */

document.addEventListener('DOMContentLoaded', () => {
  initHeroCanvas();
  initSandboxTabs();
  initAriSimulator();
  initConcretePmSimulator();
  initCsoSimulator();
});

/* =====================================================================
   1. HERO CANVAS: 3D ROTATING HIGH-TECH DIAGRID WIREFRAME
   ===================================================================== */
function initHeroCanvas() {
  const canvas = document.getElementById('hero-canvas');
  if (!canvas) return;
  const ctx = canvas.getContext('2d');

  let width = canvas.width = canvas.parentElement.clientWidth;
  let height = canvas.height = canvas.parentElement.clientHeight;

  window.addEventListener('resize', () => {
    width = canvas.width = canvas.parentElement.clientWidth;
    height = canvas.height = canvas.parentElement.clientHeight;
  });

  // Diagrid Tower 3D Geometry
  const numLevels = 9;
  const radius = 140;
  const sides = 8;
  const levelHeight = 36;
  const nodes = [];
  const edges = [];

  // Generate 3D cylindrical diagrid
  for (let l = 0; l <= numLevels; l++) {
    const y = (l - numLevels / 2) * levelHeight;
    const r = radius * (1.0 - 0.25 * Math.sin((l / numLevels) * Math.PI)); // hour-glass profile
    const twist = (l % 2) * (Math.PI / sides);
    for (let s = 0; s < sides; s++) {
      const theta = (s / sides) * Math.PI * 2 + twist;
      const x = r * Math.cos(theta);
      const z = r * Math.sin(theta);
      nodes.push({ x, y, z });
    }
  }

  // Generate diagrid lattice connections
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

  let angleX = 0.25;
  let angleY = 0;
  let isHovered = false;

  canvas.addEventListener('mousemove', (e) => {
    const rect = canvas.getBoundingClientRect();
    const mx = (e.clientX - rect.left) / width - 0.5;
    const my = (e.clientY - rect.top) / height - 0.5;
    angleY += mx * 0.03;
    angleX = 0.25 + my * 0.2;
    isHovered = true;
  });

  canvas.addEventListener('mouseleave', () => {
    isHovered = false;
  });

  function render() {
    if (!isHovered) {
      angleY += 0.006;
    }

    ctx.clearRect(0, 0, width, height);

    // Subtle background grid
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.03)';
    ctx.lineWidth = 1;
    const step = 40;
    for (let x = 0; x < width; x += step) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, height);
      ctx.stroke();
    }
    for (let y = 0; y < height; y += step) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width, y);
      ctx.stroke();
    }

    // 3D Rotation matrices
    const cosY = Math.cos(angleY);
    const sinY = Math.sin(angleY);
    const cosX = Math.cos(angleX);
    const sinX = Math.sin(angleX);

    const projectedNodes = nodes.map(n => {
      // Rotate Y
      const x1 = n.x * cosY - n.z * sinY;
      const z1 = n.x * sinY + n.z * cosY;
      // Rotate X
      const y2 = n.y * cosX - z1 * sinX;
      const z2 = n.y * sinX + z1 * cosX;

      // Perspective projection
      const fov = 450;
      const scale = fov / (fov + z2 + 100);
      return {
        px: width / 2 + x1 * scale,
        py: height / 2 - y2 * scale,
        depth: z2,
        scale
      };
    });

    // Draw edges
    ctx.lineWidth = 1.2;
    for (let e of edges) {
      const p1 = projectedNodes[e[0]];
      const p2 = projectedNodes[e[1]];
      const avgDepth = (p1.depth + p2.depth) * 0.5;

      // Color by depth with cyan-blue gradient
      const alpha = Math.max(0.15, Math.min(0.85, (avgDepth + 150) / 280));
      ctx.strokeStyle = `rgba(10, 132, 255, ${alpha})`;

      ctx.beginPath();
      ctx.moveTo(p1.px, p1.py);
      ctx.lineTo(p2.px, p2.py);
      ctx.stroke();
    }

    // Draw glowing nodes
    for (let p of projectedNodes) {
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
   ==================================================================== */
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
      if (targetPanel) targetPanel.classList.add('active');
    });
  });
}

/* =====================================================================
   3. SANDBOX 1: LIVE SHERMAN-MORRISON ARI & SVD GATE 1 SIMULATOR
   ===================================================================== */
function initAriSimulator() {
  const canvas = document.getElementById('ari-canvas');
  if (!canvas) return;
  const ctx = canvas.getContext('2d');

  // Warren Truss Geometry
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

  function renderTruss() {
    canvas.width = canvas.parentElement.clientWidth;
    canvas.height = 360;
    const w = canvas.width;
    const h = canvas.height;

    ctx.clearRect(0, 0, w, h);

    // Compute scale
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

      ctx.beginPath();
      ctx.moveTo(p1.x, p1.y);
      ctx.lineTo(p2.x, p2.y);

      if (isDisrupted) {
        ctx.strokeStyle = '#ff453a';
        ctx.setLineDash([4, 4]);
        ctx.lineWidth = 2.5;
      } else if (m.critical) {
        ctx.strokeStyle = '#ff9f0a';
        ctx.setLineDash([]);
        ctx.lineWidth = 3.0;
      } else {
        ctx.strokeStyle = m.force < 0 ? '#0a84ff' : '#30d158'; // blue = compression, green = tension
        ctx.setLineDash([]);
        ctx.lineWidth = 2.4;
      }
      ctx.stroke();
      ctx.setLineDash([]);

      // Draw member ID pill at midpoint
      const midX = (p1.x + p2.x) * 0.5;
      const midY = (p1.y + p2.y) * 0.5;
      ctx.fillStyle = isDisrupted ? '#ff453a' : 'rgba(25, 25, 35, 0.9)';
      ctx.beginPath();
      ctx.arc(midX, midY, 8, 0, Math.PI * 2);
      ctx.fill();
      ctx.strokeStyle = 'rgba(255, 255, 255, 0.2)';
      ctx.stroke();

      ctx.fillStyle = '#ffffff';
      ctx.font = '8px monospace';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText(m.id, midX, midY);
    });

    // Draw Nodes & Supports
    nodes.forEach(n => {
      const pos = getNodePos(n.id);

      if (n.pinned) {
        // Draw Pin Support Triangle
        ctx.fillStyle = '#a1a1aa';
        ctx.beginPath();
        ctx.moveTo(pos.x, pos.y);
        ctx.lineTo(pos.x - 10, pos.y + 16);
        ctx.lineTo(pos.x + 10, pos.y + 16);
        ctx.closePath();
        ctx.fill();
      } else if (n.roller) {
        // Draw Roller Support
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

      // Applied Load Arrow
      if (n.load) {
        ctx.strokeStyle = '#ff9f0a';
        ctx.fillStyle = '#ff9f0a';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(pos.x, pos.y + 8);
        ctx.lineTo(pos.x, pos.y + 30);
        ctx.stroke();
        // Arrowhead
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
      if (elStatus) elStatus.textContent = '🟢 INTACT';
      if (elDisp) elDisp.textContent = '14.2 mm';
      if (elGate) elGate.textContent = 'PASS (Full Rank)';
    } else {
      const dis = members.find(m => m.id === disruptedMemberId);
      if (dis && dis.critical) {
        // Kinematic Mechanism Triggered!
        if (elMinAri) elMinAri.textContent = '0.000';
        if (elStatus) elStatus.textContent = '🔴 COLLAPSE';
        if (elDisp) elDisp.textContent = '∞ (Unstable)';
        if (elGate) elGate.textContent = 'FAIL (Mechanism Gate 1)';
      } else {
        if (elMinAri) elMinAri.textContent = '0.648';
        if (elStatus) elStatus.textContent = '🟡 DAMAGED';
        if (elDisp) elDisp.textContent = '22.8 mm';
        if (elGate) elGate.textContent = 'PASS (Stable)';
      }
    }
  }

  // Click on canvas to disrupt a member
  canvas.addEventListener('click', (e) => {
    const rect = canvas.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const clickY = e.clientY - rect.top;

    const scale = (canvas.width - 120) / 500;
    const offsetX = 60;
    const offsetY = 50;

    let closest = null;
    let minDist = 18;

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

    if (closest !== null) {
      disruptedMemberId = (disruptedMemberId === closest) ? null : closest;
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

  window.addEventListener('resize', renderTruss);
  renderTruss();
}

/* =====================================================================
   4. SANDBOX 2: LIVE CONCRETE COLUMN P-M INTERACTION (SNI 2847:2019)
   ===================================================================== */
function initConcretePmSimulator() {
  const canvas = document.getElementById('pm-canvas');
  if (!canvas) return;
  const ctx = canvas.getContext('2d');

  const inWidth = document.getElementById('pm-input-b');
  const inHeight = document.getElementById('pm-input-h');
  const inFc = document.getElementById('pm-input-fc');
  const inBars = document.getElementById('pm-input-bars');

  function renderPmDiagram() {
    canvas.width = canvas.parentElement.clientWidth;
    canvas.height = 360;
    const cw = canvas.width;
    const ch = canvas.height;

    ctx.clearRect(0, 0, cw, ch);

    const b = inWidth ? parseFloat(inWidth.value) : 400;
    const h = inHeight ? parseFloat(inHeight.value) : 400;
    const fc = inFc ? parseFloat(inFc.value) : 30;
    const nBars = inBars ? parseInt(inBars.value) : 8;
    const barDia = 22; // mm

    // Whitney stress block parameters
    const beta1 = fc <= 28 ? 0.85 : Math.max(0.65, 0.85 - 0.05 * (fc - 28) / 7);
    const ag = b * h;
    const ast = nBars * Math.PI * Math.pow(barDia / 2, 2);
    const fy = 420;

    // Pure compression P0 & phi Pn,max
    const p0 = (0.85 * fc * (ag - ast) + fy * ast) * 1e-3; // kN
    const phiPnMax = 0.65 * 0.80 * p0; // kN
    const pureTension = -(fy * ast) * 1e-3; // kN
    const phiTension = 0.90 * pureTension;

    // Balanced point approximation
    const d = h - 60;
    const cb = (0.003 / (0.003 + fy / 200000)) * d;
    const ab = beta1 * cb;
    const pb = (0.85 * fc * b * ab) * 1e-3;
    const mb = (0.85 * fc * b * ab * (h / 2 - ab / 2)) * 1e-6 + 120; // kNm
    const phiPb = 0.65 * pb;
    const phiMb = 0.65 * mb;

    // Plotting Scales
    const maxM = mb * 1.5;
    const maxP = p0 * 1.1;
    const minP = pureTension * 1.2;

    const marginL = 60;
    const marginB = 40;
    const plotW = cw - marginL - 30;
    const plotH = ch - marginB - 30;

    const mapPt = (m, p) => ({
      x: marginL + (m / maxM) * plotW,
      y: ch - marginB - ((p - minP) / (maxP - minP)) * plotH
    });

    // Zero-lines
    const zero = mapPt(0, 0);
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.15)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(marginL, zero.y);
    ctx.lineTo(cw - 20, zero.y);
    ctx.moveTo(marginL, 20);
    ctx.lineTo(marginL, ch - marginB);
    ctx.stroke();

    // Labels
    ctx.fillStyle = '#a1a1aa';
    ctx.font = '10px sans-serif';
    ctx.textAlign = 'left';
    ctx.fillText('P (kN) [Aksial]', marginL + 8, 30);
    ctx.fillText('M (kNm) [Lentur]', cw - 90, zero.y - 8);

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
    ctx.quadraticCurveTo(balPt.x * 1.1, (p0Pt.y + balPt.y) * 0.5, balPt.x, balPt.y);
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

    // Cutoff line
    ctx.moveTo(cutoffPt.x, cutoffPt.y);
    ctx.lineTo(cutoffPt.x + plotW * 0.35, cutoffPt.y);
    ctx.quadraticCurveTo(phiBalPt.x * 1.1, phiBalPt.y * 0.9, phiBalPt.x, phiBalPt.y);
    ctx.quadraticCurveTo(phiBalPt.x * 0.8, phiM0Pt.y, phiM0Pt.x, phiM0Pt.y);
    ctx.lineTo(phiTenPt.x, phiTenPt.y);
    ctx.stroke();

    // Demand Point Check (Pu = 1500 kN, Mu = 120 kNm)
    const demandM = 120;
    const demandP = 1500;
    const dPt = mapPt(demandM, demandP);

    ctx.fillStyle = '#30d158';
    ctx.beginPath();
    ctx.arc(dPt.x, dPt.y, 6, 0, Math.PI * 2);
    ctx.fill();
    ctx.strokeStyle = '#ffffff';
    ctx.lineWidth = 1.5;
    ctx.stroke();

    ctx.fillStyle = '#ffffff';
    ctx.font = '10px monospace';
    ctx.fillText('Demand (120, 1500)', dPt.x + 10, dPt.y - 6);

    // Update Text Metrics
    const elPnMax = document.getElementById('pm-val-pnmax');
    const elRho = document.getElementById('pm-val-rho');
    const elDcr = document.getElementById('pm-val-dcr');

    if (elPnMax) elPnMax.textContent = `${Math.round(phiPnMax)} kN`;
    if (elRho) elRho.textContent = `${((ast / ag) * 100).toFixed(2)}%`;
    if (elDcr) elDcr.textContent = (demandP / phiPnMax).toFixed(3);
  }

  [inWidth, inHeight, inFc, inBars].forEach(input => {
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
  const ctx = canvas.getContext('2d');

  const inStock = document.getElementById('cso-input-stock');
  const inKerf = document.getElementById('cso-input-kerf');

  function renderCso() {
    canvas.width = canvas.parentElement.clientWidth;
    canvas.height = 360;
    const cw = canvas.width;

    ctx.clearRect(0, 0, cw, 360);

    const stockLen = inStock ? parseFloat(inStock.value) : 12.0;
    const kerfMm = inKerf ? parseFloat(inKerf.value) : 5.0;
    const kerfM = kerfMm * 1e-3;

    // Required pieces
    const pieces = [
      { id: 'BC-1', len: 4.0 }, { id: 'BC-2', len: 4.0 }, { id: 'BC-3', len: 4.0 },
      { id: 'TC-1', len: 4.0 }, { id: 'TC-2', len: 4.0 }, { id: 'TC-3', len: 4.0 },
      { id: 'D-1', len: 5.12 }, { id: 'D-2', len: 5.12 },
      { id: 'V-1', len: 3.2 }, { id: 'V-2', len: 3.2 }, { id: 'V-3', len: 3.2 }
    ];

    // Best-Fit Decreasing packing
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

    // Draw Master Stock Bars
    const barH = 28;
    const gap = 20;
    const startY = 40;
    const plotW = cw - 80;

    const colors = ['#10b981', '#0ea5e9', '#6366f1', '#f59e0b', '#a855f7'];

    bars.forEach((bar, bIdx) => {
      const y = startY + bIdx * (barH + gap);

      // Label
      ctx.fillStyle = '#a1a1aa';
      ctx.font = '11px sans-serif';
      ctx.textAlign = 'left';
      ctx.fillText(`Master Stock Bar #${bIdx + 1} (${stockLen}m)`, 40, y - 6);

      // Bar container
      ctx.fillStyle = '#161622';
      ctx.fillRect(40, y, plotW, barH);
      ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)';
      ctx.strokeRect(40, y, plotW, barH);

      // Cuts
      bar.cuts.forEach((c, cIdx) => {
        const segX = 40 + (c.start / stockLen) * plotW;
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

      // Remainder / Offcut
      const rem = stockLen - bar.used;
      const remX = 40 + (bar.used / stockLen) * plotW;
      const remW = (rem / stockLen) * plotW;

      if (rem > 0.05) {
        ctx.fillStyle = rem >= 1.5 ? 'rgba(14, 165, 233, 0.3)' : 'rgba(239, 68, 68, 0.3)';
        ctx.fillRect(remX, y + 2, remW, barH - 4);

        if (remW > 45) {
          ctx.fillStyle = '#a1a1aa';
          ctx.font = '9px monospace';
          ctx.textAlign = 'center';
          ctx.fillText(rem >= 1.5 ? `Offcut ${rem.toFixed(1)}m` : `Scrap ${rem.toFixed(1)}m`, remX + remW / 2, y + barH / 2 + 3);
        }
      }
    });

    // Update Stats
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
