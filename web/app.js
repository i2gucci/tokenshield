// Token Shield App - VRF Anonymous Enrollment

// State Management
const appState = {
    activeTab: 'create',
    tokenInfo: null,
    currentPolicy: null,
    poolStats: null
};

// Initialize app
document.addEventListener('DOMContentLoaded', () => {
    initializeTabs();
    initializeCreatePolicyForm();
    initializeLookupForm();
    initializePoolStats();
});

// Tab Management
function initializeTabs() {
    const tabs = document.querySelectorAll('.app-tab');
    const tabContents = document.querySelectorAll('.app-tab-content');

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            const tabName = tab.getAttribute('data-tab');
            
            // Update active tab
            tabs.forEach(t => {
                t.classList.remove('active');
                t.setAttribute('aria-selected', 'false');
            });
            tab.classList.add('active');
            tab.setAttribute('aria-selected', 'true');
            
            // Update active content
            tabContents.forEach(content => {
                content.classList.remove('active');
                if (content.id === `tab-${tabName}`) {
                    content.classList.add('active');
                }
            });
            
            appState.activeTab = tabName;
            
            // Load data for the active tab
            if (tabName === 'pool') {
                loadPoolStats();
            }
        });
    });
}

// Create Policy Form
function initializeCreatePolicyForm() {
    const walletAddressInput = document.getElementById('walletAddress');
    const tokenMintInput = document.getElementById('tokenMint');
    const positionSizeInput = document.getElementById('positionSize');
    const coverageInputs = document.querySelectorAll('input[name="coverage"]');
    const durationSlider = document.getElementById('duration');
    const durationValue = document.getElementById('durationValue');
    const createButton = document.getElementById('createPolicy');

    // Wallet address validation
    walletAddressInput.addEventListener('input', () => {
        validateForm();
    });

    // Token mint lookup
    let tokenLookupTimeout;
    tokenMintInput.addEventListener('input', (e) => {
        clearTimeout(tokenLookupTimeout);
        tokenLookupTimeout = setTimeout(() => {
            lookupToken(e.target.value);
        }, 500);
    });

    // Position size updates
    positionSizeInput.addEventListener('input', () => {
        updatePremiumCalculation();
        validateForm();
    });

    // Coverage level updates
    coverageInputs.forEach(input => {
        input.addEventListener('change', () => {
            updatePremiumCalculation();
        });
    });

    // Duration slider
    durationSlider.addEventListener('input', (e) => {
        durationValue.textContent = e.target.value;
        updatePremiumCalculation();
    });

    // Create policy button
    createButton.addEventListener('click', async () => {
        await createPolicy();
    });

    // Policy success modal handlers
    const backupConfirm = document.getElementById('backupConfirm');
    const closeModalBtn = document.getElementById('closeSuccessModal');
    const downloadBtn = document.getElementById('downloadBackup');
    const printBtn = document.getElementById('printBackup');
    const copyBtn = document.getElementById('copyCredentials');

    if (backupConfirm) {
        backupConfirm.addEventListener('change', (e) => {
            closeModalBtn.disabled = !e.target.checked;
        });
    }

    if (closeModalBtn) {
        closeModalBtn.addEventListener('click', () => {
            document.getElementById('policyCreatedModal').style.display = 'none';
            resetCreateForm();
        });
    }

    if (downloadBtn) {
        downloadBtn.addEventListener('click', downloadPolicyBackup);
    }

    if (printBtn) {
        printBtn.addEventListener('click', printPolicyBackup);
    }

    if (copyBtn) {
        copyBtn.addEventListener('click', copyCredentialsToClipboard);
    }
}

function validateForm() {
    const walletAddress = document.getElementById('walletAddress').value;
    const positionSize = parseFloat(document.getElementById('positionSize').value) || 0;
    const createButton = document.getElementById('createPolicy');

    // Basic Solana address validation (32-44 characters, base58)
    const isValidAddress = walletAddress.length >= 32 && walletAddress.length <= 44;
    const hasValidPosition = positionSize > 0 && appState.tokenInfo;

    createButton.disabled = !(isValidAddress && hasValidPosition);
}

async function lookupToken(mintAddress) {
    if (!mintAddress || mintAddress.length < 32) {
        document.getElementById('tokenInfo').style.display = 'none';
        return;
    }

    try {
        // TODO: Replace with actual Solana RPC call
        // For now, use mock data
        const mockTokenData = {
            name: 'Example Token',
            symbol: 'EXAMPLE',
            price: 0.123456,
            change24h: -12.5,
            liquidity: 2500000
        };

        appState.tokenInfo = mockTokenData;
        validateForm();
        
        // Display token info
        const tokenInfoDiv = document.getElementById('tokenInfo');
        tokenInfoDiv.innerHTML = `
            <div class="token-info-display">
                <h3>${mockTokenData.name} (${mockTokenData.symbol})</h3>
                <p>Price: $${mockTokenData.price.toFixed(6)}</p>
                <p>24h Change: ${mockTokenData.change24h}%</p>
                <p>Liquidity: $${mockTokenData.liquidity.toLocaleString()}</p>
            </div>
        `;
        tokenInfoDiv.style.display = 'block';
        
    } catch (error) {
        console.error('Failed to lookup token:', error);
        document.getElementById('tokenInfo').style.display = 'none';
    }
}

async function createPolicy() {
    const button = document.getElementById('createPolicy');
    button.disabled = true;
    button.textContent = '$ processing...';

    try {
        const walletAddress = document.getElementById('walletAddress').value;
        const tokenMint = document.getElementById('tokenMint').value;
        const positionSize = parseFloat(document.getElementById('positionSize').value);
        const coverageLevel = parseInt(document.querySelector('input[name="coverage"]:checked').value);
        const duration = parseInt(document.getElementById('duration').value);

        // Generate VRF-based policy ID and secret (simulated - in production, use Chainlink VRF)
        const policyCredentials = generatePolicyCredentials();

        // TODO: Implement actual Anchor transaction with VRF proof
        // For now, simulate the transaction
        await new Promise(resolve => setTimeout(resolve, 2000));

        // Store credentials for display
        appState.currentPolicy = {
            policyId: policyCredentials.policyId,
            secret: policyCredentials.secret,
            walletAddress,
            tokenMint,
            positionSize,
            coverageLevel,
            duration,
            createdAt: new Date().toISOString()
        };

        // Show success modal with credentials
        showPolicySuccessModal(policyCredentials);
        
    } catch (error) {
        console.error('Failed to create policy:', error);
        alert('Failed to create policy. Please try again.');
    } finally {
        button.disabled = false;
        button.textContent = '$ ./create_policy_vrf.sh';
    }
}

// Generate policy credentials (VRF simulation)
function generatePolicyCredentials() {
    // In production, this would be derived from Chainlink VRF
    const policyId = generateRandomHex(64); // 256-bit policy ID
    const secret = generateRandomHex(32); // 128-bit secret
    
    return { policyId, secret };
}

function generateRandomHex(length) {
    const chars = '0123456789abcdef';
    let result = '';
    for (let i = 0; i < length; i++) {
        result += chars[Math.floor(Math.random() * chars.length)];
    }
    return result;
}

function showPolicySuccessModal(credentials) {
    const modal = document.getElementById('policyCreatedModal');
    document.getElementById('generatedPolicyId').textContent = credentials.policyId;
    document.getElementById('generatedSecret').textContent = credentials.secret;
    document.getElementById('backupConfirm').checked = false;
    document.getElementById('closeSuccessModal').disabled = true;
    modal.style.display = 'block';
}

function downloadPolicyBackup() {
    const data = {
        policyId: appState.currentPolicy.policyId,
        secret: appState.currentPolicy.secret,
        walletAddress: appState.currentPolicy.walletAddress,
        createdAt: appState.currentPolicy.createdAt,
        warning: 'Keep this file secure. Without these credentials, you cannot claim payouts.'
    };

    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `token-shield-policy-${appState.currentPolicy.policyId.slice(0, 8)}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}

function printPolicyBackup() {
    const printWindow = window.open('', '', 'width=600,height=600');
    printWindow.document.write(`
        <html>
        <head>
            <title>Token Shield Policy Backup</title>
            <style>
                body { font-family: monospace; padding: 20px; }
                h1 { border-bottom: 2px solid #000; padding-bottom: 10px; }
                .info { margin: 10px 0; }
            </style>
        </head>
        <body>
            <h1>Token Shield Policy Backup</h1>
            <div class="info">
                <p><strong>Policy ID:</strong> ${appState.currentPolicy.policyId}</p>
                <p><strong>Secret:</strong> ${appState.currentPolicy.secret}</p>
                <p><strong>Created:</strong> ${appState.currentPolicy.createdAt}</p>
                <p><strong>WARNING:</strong> Keep this file secure!</p>
            </div>
        </body>
        </html>
    `);
    printWindow.document.close();
    printWindow.print();
}

// Policy Lookup
function initializeLookupForm() {
    const lookupBtn = document.getElementById('lookupPolicy');
    
    if (lookupBtn) {
        lookupBtn.addEventListener('click', async () => {
            await lookupPolicy();
        });
    }
}

async function lookupPolicy() {
    const policyId = document.getElementById('lookupPolicyId').value;
    const secret = document.getElementById('lookupSecret').value;
    const container = document.getElementById('policiesContainer');

    if (!policyId || !secret) {
        alert('Please enter both policy ID and secret');
        return;
    }

    container.innerHTML = '<div class="loading-state"><p class="mono-text">$ verifying credentials...</p></div>';

    try {
        // TODO: Fetch actual policy from Solana using policyId
        // Verify secret matches stored hash
        // For now, use mock data
        await new Promise(resolve => setTimeout(resolve, 1000));

        const mockPolicy = {
            id: policyId.slice(0, 12) + '...',
            tokenName: 'BONK',
            positionSize: '1,000,000',
            coverage: '50%',
            entryPrice: '$0.000023',
            currentPrice: '$0.000019',
            maxPayout: '575 USDC',
            expiry: '2026-03-15 14:23:01',
            status: 'Active'
        };

        displayPolicy(mockPolicy);
        
    } catch (error) {
        console.error('Failed to lookup policy:', error);
        container.innerHTML = '<div class="error-state"><p class="mono-text error">$ Policy not found or invalid credentials</p></div>';
    }
}

function displayPolicy(policy) {
    const container = document.getElementById('policiesContainer');
    const template = document.getElementById('policyCardTemplate');
    
    container.innerHTML = '';
    
    const card = template.content.cloneNode(true);
    
    card.querySelector('.policy-id-value').textContent = policy.id;
    card.querySelector('.status-badge').textContent = policy.status;
    card.querySelector('.token-name').textContent = policy.tokenName;
    card.querySelector('.position-size').textContent = policy.positionSize;
    card.querySelector('.coverage-level').textContent = policy.coverage;
    card.querySelector('.entry-price').textContent = policy.entryPrice;
    card.querySelector('.current-price').textContent = policy.currentPrice;
    card.querySelector('.max-payout').textContent = policy.maxPayout;
    card.querySelector('.expiry').textContent = policy.expiry;
    
    // Add status class
    const statusBadge = card.querySelector('.status-badge');
    statusBadge.classList.add(policy.status.toLowerCase());
    
    container.appendChild(card);
}

function updatePremiumCalculation() {
    const positionSize = parseFloat(document.getElementById('positionSize').value) || 0;
    const coverageLevel = parseInt(document.querySelector('input[name="coverage"]:checked').value) || 50;
    const duration = parseInt(document.getElementById('duration').value) || 12;

    // Calculate premium based on position size and coverage
    const basePremium = positionSize * 0.01;
    const riskMultiplier = coverageLevel / 50;
    const durationFactor = duration / 12;
    const totalPremium = basePremium * riskMultiplier * durationFactor;
    const maxPayout = totalPremium * 5.75;

    // Update UI
    document.getElementById('basePremium').textContent = `${basePremium.toFixed(2)} USDC`;
    document.getElementById('riskMultiplier').textContent = `${riskMultiplier.toFixed(2)}x`;
    document.getElementById('durationFactor').textContent = `${durationFactor.toFixed(2)}x`;
    document.getElementById('totalPremium').textContent = `${totalPremium.toFixed(2)} USDC`;
    document.getElementById('maxPayout').textContent = `${maxPayout.toFixed(2)} USDC`;
}

function resetCreateForm() {
    document.getElementById('walletAddress').value = '';
    document.getElementById('tokenMint').value = '';
    document.getElementById('positionSize').value = '';
    document.getElementById('tokenInfo').style.display = 'none';
    document.getElementById('duration').value = 12;
    document.getElementById('durationValue').textContent = '12';
    validateForm();
}

function copyCredentialsToClipboard() {
    const credentials = `Policy ID: ${appState.currentPolicy.policyId}\nSecret: ${appState.currentPolicy.secret}`;
    navigator.clipboard.writeText(credentials).then(() => {
        alert('Credentials copied to clipboard!');
    });
}

// Policies View
function initializePoliciesView() {
    // Initial load handled by wallet connection
}

async function loadUserPolicies() {
    if (!appState.connected) {
        clearPoliciesView();
        return;
    }

    const container = document.getElementById('policiesContainer');
    container.innerHTML = '<div class="loading-state"><p class="mono-text">$ loading policies...</p></div>';

    try {
        // TODO: Fetch actual policies from Solana
        // For now, use mock data
        const mockPolicies = [
            {
                id: '0x7a3f2b1c',
                tokenName: 'BONK',
                positionSize: '1,000,000',
                coverage: '50%',
                entryPrice: '$0.000023',
                currentPrice: '$0.000019',
                maxPayout: '575 USDC',
                expiry: '2026-03-15 14:23:01',
                status: 'Active'
            }
        ];

        if (mockPolicies.length === 0) {
            clearPoliciesView();
            return;
        }

        appState.policies = mockPolicies;
        displayPolicies(mockPolicies);
        
    } catch (error) {
        console.error('Failed to load policies:', error);
        container.innerHTML = '<div class="error-state"><p class="mono-text error">$ Error loading policies</p></div>';
    }
}

function displayPolicies(policies) {
    const container = document.getElementById('policiesContainer');
    const template = document.getElementById('policyCardTemplate');
    
    container.innerHTML = '';
    
    policies.forEach(policy => {
        const card = template.content.cloneNode(true);
        
        card.querySelector('.policy-id-value').textContent = policy.id;
        card.querySelector('.status-badge').textContent = policy.status;
        card.querySelector('.token-name').textContent = policy.tokenName;
        card.querySelector('.position-size').textContent = policy.positionSize;
        card.querySelector('.coverage-level').textContent = policy.coverage;
        card.querySelector('.entry-price').textContent = policy.entryPrice;
        card.querySelector('.current-price').textContent = policy.currentPrice;
        card.querySelector('.max-payout').textContent = policy.maxPayout;
        card.querySelector('.expiry').textContent = policy.expiry;
        
        // Add status class
        const statusBadge = card.querySelector('.status-badge');
        statusBadge.classList.add(policy.status.toLowerCase());
        
        container.appendChild(card);
    });
}

function clearPoliciesView() {
    const container = document.getElementById('policiesContainer');
    container.innerHTML = `
        <div class="empty-state">
            <p class="mono-text">
                No active policies found.
                <br><br>
                $ connect wallet to view policies
                <br>$ or create a new policy to get started
            </p>
        </div>
    `;
}

// Pool Stats
function initializePoolStats() {
    // Will be loaded when tab is clicked
}

async function loadPoolStats() {
    try {
        // TODO: Fetch actual pool stats from Solana
        // For now, use mock data
        const mockStats = {
            tvl: 12500000,
            activePolicies: 342,
            totalCoverage: 8750000,
            totalPremiums: 185000,
            totalPayouts: 42500,
            collateralRatio: 157
        };

        appState.poolStats = mockStats;
        displayPoolStats(mockStats);
        
    } catch (error) {
        console.error('Failed to load pool stats:', error);
    }
}

function displayPoolStats(stats) {
    document.getElementById('poolTVL').textContent = `$${(stats.tvl / 1000000).toFixed(2)}M`;
    document.getElementById('activePolicies').textContent = stats.activePolicies.toLocaleString();
    document.getElementById('totalCoverage').textContent = `$${(stats.totalCoverage / 1000000).toFixed(2)}M`;
    document.getElementById('totalPremiums').textContent = `$${stats.totalPremiums.toLocaleString()}`;
    document.getElementById('totalPayouts').textContent = `$${stats.totalPayouts.toLocaleString()}`;
    document.getElementById('collateralRatio').textContent = `${stats.collateralRatio}%`;
}

// Utility Functions
function formatAddress(address) {
    return `${address.slice(0, 4)}...${address.slice(-4)}`;
}

function formatDate(timestamp) {
    const date = new Date(timestamp);
    return date.toLocaleString();
}

function formatUSD(amount) {
    return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD'
    }).format(amount);
}
