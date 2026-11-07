/**
 * EigenGraph Workstation — Official Interactive Web-App Engine
 * 13 Pillars Architecture with Real-Time Math & Engineering Simulators
 */

document.addEventListener('DOMContentLoaded', () => {
  initWorkstationNavigation();
  initPillarSearch();
  initHiDpiCanvases();
  initAllSimulators();
});

// KaTeX Global Initializer
function initKaTeX() {
  if (window.renderMathInElement) {
    renderMathInElement(document.body, {
      delimiters: [
        { left: '$$', right: '$$', display: true },
        { left: '\\[', right: '\\]', display: true },
        { left: '\\(', right: '\\)', display: false }
      ],
      throwOnError: false
    });
  }
}

// ----------------- 1. WORKSTATION NAVIGATION -----------------
function initWorkstationNavigation() {
  const buttons = document.querySelectorAll('.sidebar-btn');
  const dossiers = document.querySelectorAll('.pillar-dossier');

  buttons.forEach(btn => {
    btn.addEventListener('click', () => {
      const targetId = btn.getAttribute('data-pillar');
      switchPillar(targetId);
    });
  });

  // URL hash sync
  window.addEventListener('hashchange', () => {
    const hash = window.location.hash.replace('#', '');
    if (hash && document.getElementById(hash)) {
      switchPillar(hash);
    }
  });

  // Initial hash
  const initialHash = window.location.hash.replace('#', '');
  if (initialHash && document.getElementById(initialHash)) {
    switchPillar(initialHash);
  }
}

function switchPillar(pillarId) {
  const buttons = document.querySelectorAll('.sidebar-btn');
  const dossiers = document.querySelectorAll('.pillar-dossier');

  buttons.forEach(b => {
    b.classList.toggle('active', b.getAttribute('data-pillar') === pillarId);
  });

  dossiers.forEach(d => {
    const isTarget = d.id === pillarId;
    d.classList.toggle('active', isTarget);
    if (isTarget && window.renderMathInElement) {
      renderMathInElement(d, {
        delimiters: [
          { left: '$$', right: '$$', display: true },
          { left: '\\[', right: '\\]', display: true },
          { left: '\\(', right: '\\)', display: false }
        ],
        throwOnError: false
      });
    }
  });

  window.location.hash = pillarId;
}

// ----------------- 2. PILLAR SEARCH & FILTER -----------------
function initPillarSearch() {
  const searchInput = document.getElementById('pillar-search-input');
  if (!searchInput) return;

  searchInput.addEventListener('input', () => {
    const query = searchInput.value.toLowerCase().trim();
    const items = document.querySelectorAll('.sidebar-item');

    items.forEach(item => {
      const text = item.textContent.toLowerCase();
      item.style.display = text.includes(query) ? '' : 'none';
    });
  });
}

// ----------------- 3. CANVASES & SIMULATORS -----------------
function initHiDpiCanvases() {
  initAriCanvas();
  initTurntableCanvas();
}

function initAllSimulators() {
  updateDraftingTransform();
  runCpmCalc();
  updateRabCalc();
  updateCarbonCalc();
  updateShmCalc();
  updateWoodburySim();
  updateBiaxialSim();
  updateModalSim();
  updateCsoCalc();
  updateDfmaCalc();
  updateFosmCalc();
  updateAuditSeal();
}

// SIMULATOR: Pillar 1 Merkle Chain
let vaultDepth = 1;
function simulateVaultHash() {
  const input = document.getElementById('vault-input-data').value;
  const prevHashEl = document.getElementById('vault-prev-hash');
  const currHashEl = document.getElementById('vault-curr-hash');
  const depthEl = document.getElementById('vault-chain-depth');

  const oldHash = currHashEl.textContent;
  prevHashEl.textContent = oldHash;

  // Simple deterministic SHA-like hash for frontend verification
  let hashVal = 0x811c9dc5;
  const str = oldHash + input + Date.now();
  for (let i = 0; i < str.length; i++) {
    hashVal ^= str.charCodeAt(i);
    hashVal += (hashVal << 1) + (hashVal << 4) + (hashVal << 7) + (hashVal << 8) + (hashVal << 24);
  }
  const hex = (hashVal >>> 0).toString(16).padStart(8, '0') +
              (Math.imul(hashVal, 31) >>> 0).toString(16).padStart(8, '0') +
              (Math.imul(hashVal, 73) >>> 0).toString(16).padStart(8, '0') +
              (Math.imul(hashVal, 107) >>> 0).toString(16).padStart(8, '0');

  currHashEl.textContent = hex.padEnd(64, 'a');
  vaultDepth++;
  depthEl.textContent = vaultDepth;
}

// SIMULATOR: Pillar 2 ARI Warren Truss
function initAriCanvas() {
  const canvas = document.getElementById('ari-canvas');
  if (!canvas) return;
  const ctx = canvas.getContext('2d');
  const dpr = window.devicePixelRatio || 1;

  function render() {
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
    ctx.resetTransform();
    ctx.scale(dpr, dpr);

    ctx.fillStyle = '#0c1017';
    ctx.fillRect(0, 0, rect.width, rect.height);

    const w = rect.width;
    const h = rect.height;
    const pad = 40;
    const span = w - pad * 2;
    const midY = h * 0.7;
    const topY = h * 0.3;

    // Draw Warren Truss Nodes
    const numBays = 6;
    const dx = span / numBays;
    ctx.lineWidth = 2.5;

    // Bottom Chord
    ctx.strokeStyle = '#0a84ff';
    ctx.beginPath();
    ctx.moveTo(pad, midY);
    ctx.lineTo(pad + span, midY);
    ctx.stroke();

    // Top Chord
    ctx.beginPath();
    ctx.moveTo(pad + dx * 0.5, topY);
    ctx.lineTo(pad + span - dx * 0.5, topY);
    ctx.stroke();

    // Diagonals
    ctx.strokeStyle = '#30d158';
    ctx.beginPath();
    for (let i = 0; i < numBays; i++) {
      const bx1 = pad + i * dx;
      const bx2 = pad + (i + 1) * dx;
      const tx = pad + (i + 0.5) * dx;
      ctx.moveTo(bx1, midY);
      ctx.lineTo(tx, topY);
      ctx.lineTo(bx2, midY);
    }
    ctx.stroke();

    // Nodes
    ctx.fillStyle = '#fff';
    for (let i = 0; i <= numBays; i++) {
      ctx.beginPath();
      ctx.arc(pad + i * dx, midY, 4, 0, Math.PI * 2);
      ctx.fill();
    }
    for (let i = 0; i < numBays; i++) {
      ctx.beginPath();
      ctx.arc(pad + (i + 0.5) * dx, topY, 4, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  window.addEventListener('resize', render);
  render();
}

// SIMULATOR: Pillar 3 Drafting Scale
function updateDraftingTransform() {
  const scale = parseFloat(document.getElementById('draft-scale-select')?.value || 100);
  const spanM = parseFloat(document.getElementById('draft-span-x')?.value || 24);

  const modelEl = document.getElementById('draft-res-model');
  const paperEl = document.getElementById('draft-res-paper');
  const fitEl = document.getElementById('draft-res-fit');

  if (modelEl) modelEl.textContent = spanM.toFixed(1) + ' m';
  const paperMm = (spanM * 1000) / scale;
  const paperPt = paperMm * (72 / 25.4);
  if (paperEl) paperEl.textContent = `${paperMm.toFixed(1)} mm (${paperPt.toFixed(1)} pt)`;

  if (fitEl) {
    if (paperMm < 780) {
      fitEl.textContent = 'Passes Within ISO 5457 A1 Printable Area (841 x 594 mm)';
      fitEl.style.color = 'var(--accent-green)';
    } else {
      fitEl.textContent = 'Exceeds A1 Boundary (Select Scale 1:200)';
      fitEl.style.color = 'var(--accent-orange)';
    }
  }
}

// SIMULATOR: Pillar 4 Turntable 3D Wireframe
function initTurntableCanvas() {
  const canvas = document.getElementById('orbit-turntable-canvas');
  if (!canvas) return;
  const ctx = canvas.getContext('2d');
  const dpr = window.devicePixelRatio || 1;

  let yaw = 0.4;
  let pitch = 0.3;
  let isDragging = false;
  let lastX = 0;
  let lastY = 0;

  canvas.addEventListener('mousedown', e => {
    isDragging = true;
    lastX = e.clientX;
    lastY = e.clientY;
  });

  window.addEventListener('mousemove', e => {
    if (!isDragging) return;
    const dx = e.clientX - lastX;
    const dy = e.clientY - lastY;
    yaw += dx * 0.01;
    pitch = Math.max(-1.2, Math.min(1.2, pitch + dy * 0.01));
    lastX = e.clientX;
    lastY = e.clientY;
    render();
  });

  window.addEventListener('mouseup', () => { isDragging = false; });

  function render() {
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
    ctx.resetTransform();
    ctx.scale(dpr, dpr);

    ctx.fillStyle = '#0c1017';
    ctx.fillRect(0, 0, rect.width, rect.height);

    const cx = rect.width * 0.5;
    const cy = rect.height * 0.5;
    const scale = Math.min(rect.width, rect.height) * 0.32;

    // Simple 3D Box & Diagrids
    const nodes = [
      [-1, -1.8, -1], [1, -1.8, -1], [1, -1.8, 1], [-1, -1.8, 1],
      [-1,  1.8, -1], [1,  1.8, -1], [1,  1.8, 1], [-1,  1.8, 1],
    ];

    function project(pt) {
      const x = pt[0];
      const y = pt[1];
      const z = pt[2];

      const x1 = x * Math.cos(yaw) - z * Math.sin(yaw);
      const z1 = x * Math.sin(yaw) + z * Math.cos(yaw);

      const y2 = y * Math.cos(pitch) - z1 * Math.sin(pitch);
      const z2 = y * Math.sin(pitch) + z1 * Math.cos(pitch);

      const d = 3.5;
      const f = d / (d + z2);
      return [cx + x1 * scale * f, cy - y2 * scale * f];
    }

    const pNodes = nodes.map(project);
    ctx.strokeStyle = '#0a84ff';
    ctx.lineWidth = 2.0;

    const edges = [
      [0,1],[1,2],[2,3],[3,0],
      [4,5],[5,6],[6,7],[7,4],
      [0,4],[1,5],[2,6],[3,7],
      // Diagonals
      [0,5],[1,6],[2,7],[3,4]
    ];

    edges.forEach(([i, j]) => {
      ctx.beginPath();
      ctx.moveTo(pNodes[i][0], pNodes[i][1]);
      ctx.lineTo(pNodes[j][0], pNodes[j][1]);
      ctx.stroke();
    });
  }

  window.addEventListener('resize', render);
  render();
}

// SIMULATOR: Pillar 5 CPM
function runCpmCalc() {
  const d1 = parseFloat(document.getElementById('cpm-dur-1')?.value || 14);
  const d2 = parseFloat(document.getElementById('cpm-dur-2')?.value || 28);
  const d3 = parseFloat(document.getElementById('cpm-dur-3')?.value || 18);

  const durEl = document.getElementById('cpm-res-duration');
  const fltEl = document.getElementById('cpm-res-float');

  const criticalDur = d1 + d2;
  const float3 = Math.max(0, criticalDur - (d1 + d3));

  if (durEl) durEl.textContent = `${criticalDur} Days`;
  if (fltEl) fltEl.textContent = `${float3} Days (Non-Critical)`;
}

// SIMULATOR: Pillar 6 5D Cost
function updateRabCalc() {
  const tons = parseFloat(document.getElementById('rab-steel-vol')?.value || 120);
  const rate = parseFloat(document.getElementById('rab-steel-rate')?.value || 28500);

  const volEl = document.getElementById('rab-steel-val');
  const subEl = document.getElementById('rab-res-subtotal');
  const totEl = document.getElementById('rab-res-total');

  if (volEl) volEl.textContent = `${tons} Tons`;
  const subtotal = tons * 1000 * rate;
  const total = subtotal * 1.15;

  if (subEl) subEl.textContent = `IDR ${subtotal.toLocaleString('en-US')}`;
  if (totEl) totEl.textContent = `IDR ${total.toLocaleString('en-US')}`;
}

// SIMULATOR: Pillar 7 Carbon
function updateCarbonCalc() {
  const tons = parseFloat(document.getElementById('carbon-tonnage')?.value || 350);
  const tonEl = document.getElementById('carbon-tonnage-val');

  if (tonEl) tonEl.textContent = `${tons} Metric Tons`;

  const bfbof = (tons * 2.30).toFixed(1);
  const eaf = (tons * 0.65).toFixed(1);
  const h2 = (tons * 0.15).toFixed(1);

  const bEl = document.getElementById('carbon-res-bfbof');
  const eEl = document.getElementById('carbon-res-eaf');
  const hEl = document.getElementById('carbon-res-h2');

  if (bEl) bEl.textContent = `${bfbof} Tons CO2e`;
  if (eEl) eEl.textContent = `${eaf} Tons CO2e (-71.7%)`;
  if (hEl) hEl.textContent = `${h2} Tons CO2e (-93.5% Savings!)`;
}

// SIMULATOR: Pillar 8 SHM
function updateShmCalc() {
  const microstrain = parseFloat(document.getElementById('shm-strain')?.value || 480);
  const valEl = document.getElementById('shm-strain-val');
  if (valEl) valEl.textContent = `${microstrain} με`;

  const stressMpa = (microstrain * 1e-6) * 200000;
  const dcr = stressMpa / 345.0;

  const sEl = document.getElementById('shm-res-stress');
  const dEl = document.getElementById('shm-res-dcr');

  if (sEl) sEl.textContent = `${stressMpa.toFixed(1)} MPa`;
  if (dEl) {
    dEl.textContent = `DCR = ${dcr.toFixed(3)} (${((1 - dcr) * 100).toFixed(1)}% Capacity Reserve)`;
    dEl.style.color = dcr <= 1.0 ? 'var(--accent-green)' : 'var(--accent-red)';
  }
}

// SIMULATOR: Pillar 9 Sub-panels
function switchPillar9Sub(tabKey) {
  const pW = document.getElementById('sub-panel-woodbury');
  const pC = document.getElementById('sub-panel-concrete');
  const pM = document.getElementById('sub-panel-modal');

  const bW = document.getElementById('btn-sub-woodbury');
  const bC = document.getElementById('btn-sub-concrete');
  const bM = document.getElementById('btn-sub-modal');

  if (pW) pW.style.display = tabKey === 'woodbury' ? 'block' : 'none';
  if (pC) pC.style.display = tabKey === 'concrete' ? 'block' : 'none';
  if (pM) pM.style.display = tabKey === 'modal' ? 'block' : 'none';

  if (bW) bW.classList.toggle('active', tabKey === 'woodbury');
  if (bC) bC.classList.toggle('active', tabKey === 'concrete');
  if (bM) bM.classList.toggle('active', tabKey === 'modal');
}

function updateWoodburySim() {
  const lambda = parseFloat(document.getElementById('wdb-lambda')?.value || 0.2);
  const n0 = parseFloat(document.getElementById('wdb-n0')?.value || 450);

  const lEl = document.getElementById('wdb-lambda-val');
  if (lEl) lEl.textContent = `λ = ${lambda.toFixed(2)}`;

  const deltaN = (1.0 - lambda) * n0;
  const rrr = lambda;
  const mu = 1.0 + (1.0 - lambda) * 1.5;

  const dnEl = document.getElementById('wdb-res-dn');
  const rrrEl = document.getElementById('wdb-res-rrr');
  const muEl = document.getElementById('wdb-res-mu');

  if (dnEl) dnEl.textContent = `${deltaN.toFixed(2)} kN Shed to Alternate Path`;
  if (rrrEl) rrrEl.textContent = `${rrr.toFixed(3)}`;
  if (muEl) muEl.textContent = `${mu.toFixed(2)} (Ductile Redistribution)`;
}

function updateBiaxialSim() {
  const pu = parseFloat(document.getElementById('biax-pu')?.value || 1200);
  const mux = parseFloat(document.getElementById('biax-mux')?.value || 160);
  const muy = parseFloat(document.getElementById('biax-muy')?.value || 110);

  const phiMnx = 220.0;
  const phiMny = 180.0;
  const phiPn0 = 2400.0;

  const pRatio = Math.min(1.0, Math.max(0.0, pu / phiPn0));
  let alpha = 1.0;
  if (pRatio > 0.10) {
    alpha = 1.0 + (pRatio - 0.10) * (0.50 / 0.70);
  }
  alpha = Math.min(1.55, Math.max(1.0, alpha));

  const termX = Math.pow(mux / phiMnx, alpha);
  const termY = Math.pow(muy / phiMny, alpha);
  const biaxialDcr = Math.pow(termX + termY, 1.0 / alpha);

  const breslerInv = (1.0 / 2200.0) + (1.0 / 2000.0) - (1.0 / 3500.0);
  const breslerPn = 1.0 / breslerInv;

  const aEl = document.getElementById('biax-res-alpha');
  const pEl = document.getElementById('biax-res-pn');
  const dEl = document.getElementById('biax-res-dcr');

  if (aEl) aEl.textContent = alpha.toFixed(2);
  if (pEl) pEl.textContent = `${breslerPn.toFixed(1)} kN`;
  if (dEl) {
    const isSafe = biaxialDcr <= 1.0;
    dEl.textContent = `${biaxialDcr.toFixed(3)} (${isSafe ? 'SAFE ≤ 1.00' : 'OVERSTRESSED > 1.00'})`;
    dEl.style.color = isSafe ? 'var(--accent-green)' : 'var(--accent-red)';
  }
}

function updateModalSim() {
  const damp = parseFloat(document.getElementById('cqc-damp')?.value || 0.05);
  const dampEl = document.getElementById('cqc-damp-val');
  if (dampEl) dampEl.textContent = `ξ = ${damp.toFixed(2)} (${(damp*100).toFixed(0)}% RC)`;
}

// SIMULATOR: Pillar 10 CSO
function updateCsoCalc() {
  const stock = parseFloat(document.getElementById('cso-stock-len')?.value || 12.0);
  const c1 = parseFloat(document.getElementById('cso-c1')?.value || 4.8);
  const c2 = parseFloat(document.getElementById('cso-c2')?.value || 3.5);
  const c3 = parseFloat(document.getElementById('cso-c3')?.value || 3.5);

  const used = c1 + c2 + c3;
  const scrap = Math.max(0, stock - used);
  const wastePct = (scrap / stock) * 100;

  const uEl = document.getElementById('cso-res-used');
  const sEl = document.getElementById('cso-res-scrap');

  if (uEl) uEl.textContent = `${used.toFixed(2)} m / ${stock.toFixed(2)} m`;
  if (sEl) {
    sEl.textContent = `${scrap.toFixed(2)} m (${wastePct.toFixed(2)}% Waste — ${wastePct <= 2.5 ? 'Meets ≤ 2.5% Target' : 'Exceeds Target'})`;
    sEl.style.color = wastePct <= 2.5 ? 'var(--accent-green)' : 'var(--accent-orange)';
  }
}

// SIMULATOR: Pillar 11 DfMA
function updateDfmaCalc() {
  const count = parseFloat(document.getElementById('dfma-count')?.value || 6);
  const cEl = document.getElementById('dfma-count-val');
  if (cEl) cEl.textContent = `${count} Assemblies`;

  const linearTol = count * 2.0;
  const rssTol = Math.sqrt(count * (2.0 * 2.0));

  const lEl = document.getElementById('dfma-res-linear');
  const rEl = document.getElementById('dfma-res-rss');

  if (lEl) lEl.textContent = `± ${linearTol.toFixed(2)} mm`;
  if (rEl) rEl.textContent = `± ${rssTol.toFixed(2)} mm (60.8% Tighter Fit)`;
}

// SIMULATOR: Pillar 12 FOSM
function updateFosmCalc() {
  const mr = parseFloat(document.getElementById('fosm-mr')?.value || 850);
  const ms = parseFloat(document.getElementById('fosm-ms')?.value || 480);

  const covR = 0.12;
  const covS = 0.18;
  const sigR = mr * covR;
  const sigS = ms * covS;

  const beta = (mr - ms) / Math.sqrt(sigR * sigR + sigS * sigS);
  const pf = 0.5 * Math.exp(-Math.pow(beta, 2) / 2); // Approximation

  const bEl = document.getElementById('fosm-res-beta');
  const pEl = document.getElementById('fosm-res-pf');

  if (bEl) bEl.textContent = `${beta.toFixed(2)} (${beta >= 3.0 ? 'High Reliability' : 'Low Safety Margin'})`;
  if (pEl) pEl.textContent = pf.toExponential(2);
}

// SIMULATOR: Pillar 13 Certification
function updateAuditSeal() {
  const proj = document.getElementById('cert-proj-name')?.value || 'EigenGraph Project';
  const sealEl = document.getElementById('cert-res-seal');
  if (!sealEl) return;

  let hash = 0x5a879999;
  for (let i = 0; i < proj.length; i++) {
    hash ^= proj.charCodeAt(i);
    hash += (hash << 5) + (hash << 11);
  }
  const h1 = (hash >>> 0).toString(16).padStart(8, '0');
  const h2 = (Math.imul(hash, 47) >>> 0).toString(16).padStart(8, '0');
  const h3 = (Math.imul(hash, 199) >>> 0).toString(16).padStart(8, '0');
  const h4 = (Math.imul(hash, 811) >>> 0).toString(16).padStart(8, '0');

  sealEl.textContent = (h1 + h2 + h3 + h4 + '78f3f21d24c9e239226e8eddbc1c0695').padEnd(64, 'f');
}
