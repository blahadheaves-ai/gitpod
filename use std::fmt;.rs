use std::fmt;

/// Risk Tiers defined by simulated annual drift loss bounds ($L_ann)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskTier {
    Tier1Standard,
    Tier2MidMarket,
    Tier3Enterprise,
    Tier4Sovereign,
}

impl RiskTier {
    /// Determines risk tier from simulated annual loss
    pub fn from_simulated_loss(loss: f64) -> Self {
        match loss {
            l if l < 1_000_000.0 => RiskTier::Tier1Standard,
            l if l < 10_000_000.0 => RiskTier::Tier2MidMarket,
            l if l < 50_000_000.0 => RiskTier::Tier3Enterprise,
            _ => RiskTier::Tier4Sovereign,
        }
    }

    /// Fixed base engine allocation fee ($F_base)
    pub fn base_fee(&self) -> f64 {
        match self {
            RiskTier::Tier1Standard => 25_000.0,
            RiskTier::Tier2MidMarket => 75_000.0,
            RiskTier::Tier3Enterprise => 200_000.0,
            RiskTier::Tier4Sovereign => 500_000.0,
        }
    }

    /// Variable risk coefficient (alpha)
    pub fn alpha_coefficient(&self) -> f64 {
        match self {
            RiskTier::Tier1Standard => 0.15,
            RiskTier::Tier2MidMarket => 0.10,
            RiskTier::Tier3Enterprise => 0.065,
            RiskTier::Tier4Sovereign => 0.035,
        }
    }
}

/// Simulated drift metrics ingested from the "What-If" engine
#[derive(Debug, Clone)]
pub struct DriftSimulationOutput {
    pub temporal_clock_skew_loss: f64,
    pub bit_reconciliation_loss: f64,
    pub latency_bleed_loss: f64,
}

impl DriftSimulationOutput {
    pub fn new(clock_skew: f64, bit_loss: f64, latency_bleed: f64) -> Self {
        Self {
            temporal_clock_skew_loss: clock_skew,
            bit_reconciliation_loss: bit_loss,
            latency_bleed_loss: latency_bleed,
        }
    }

    /// Aggregates vectors into total simulated annual drift loss ($L_ann)
    pub fn total_annual_loss(&self) -> f64 {
        self.temporal_clock_skew_loss + self.bit_reconciliation_loss + self.latency_bleed_loss
    }
}

/// Consolidated Audit Fee Quote
#[derive(Debug, Clone)]
pub struct PricingQuote {
    pub tier: RiskTier,
    pub simulated_annual_loss: f64,
    pub base_fee: f64,
    pub variable_risk_fee: f64,
    pub total_fee: f64,
    pub roi_ratio: f64,
}

impl PricingQuote {
    /// Calculates pricing quote using the VCLA model:
    /// F_total = max(F_base, F_base + (alpha * L_ann))
    pub fn calculate(simulation: &DriftSimulationOutput, recovery_bounty: Option<f64>) -> Self {
        let annual_loss = simulation.total_annual_loss();
        let tier = RiskTier::from_simulated_loss(annual_loss);

        let base_fee = tier.base_fee();
        let alpha = tier.alpha_coefficient();
        let variable_fee = alpha * annual_loss;
        let bounty = recovery_bounty.unwrap_or(0.0);

        // Calculate total fee with floor guarantee (minimum fee is F_base)
        let computed_total = base_fee + variable_fee + bounty;
        let total_fee = computed_total.max(base_fee);

        let roi_ratio = if total_fee > 0.0 {
            annual_loss / total_fee
        } else {
            0.0
        };

        Self {
            tier,
            simulated_annual_loss: annual_loss,
            base_fee,
            variable_risk_fee: variable_fee,
            total_fee,
            roi_ratio,
        }
    }
}

impl fmt::Display for PricingQuote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "--- AUDIT PRICING QUOTE ---\n\
             Risk Tier:            {:?}\n\
             Simulated Loss (L_ann): ${:,.2}\n\
             Base Fee (F_base):    ${:,.2}\n\
             Variable Fee (alpha): ${:,.2} ({:.1}%)\n\
             Total Audit Investment: ${:,.2}\n\
             Projected Year 1 ROI: {:.1}x\n\
             ---------------------------",
            self.tier,
            self.simulated_annual_loss,
            self.base_fee,
            self.variable_risk_fee,
            self.tier.alpha_coefficient() * 100.0,
            self.total_fee,
            self.roi_ratio
        )
    }
}

fn main() {
    // Example: Simulation yields $12.5M in compounding drift losses
    let sim_output = DriftSimulationOutput::new(
        4_500_000.0, // Temporal Clock Skew
        5_000_000.0, // Bit Reconciliation Loss
        3_000_000.0, // Latency Bleed
    );

    let quote = PricingQuote::calculate(&sim_output, None);
    println!("{}", quote);
}