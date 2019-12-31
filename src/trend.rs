use crate::PacketLoss;

/// Packet loss classifier based on the [`Trend`] scheme.
///
/// Classification of packet loss depends on the observation of values
/// such as packet timestamps and packet serial numbers.
/// Algorithms such as [`ZigZag`] and [`Spike`] rely on the ROTT, but perform
/// unreliably if ROTT is around the threshold.
///
/// [`Trend`] takes the delay trend of packets into account which is summarized
/// in the cited paper as follows:
///
/// "When a packet loss is observed at time t, it
/// should be considered as a congestion loss if T_d is in an
/// ascending phase; otherwise it is categorized as wireless
/// loss."
///
/// ```rust
/// use packet_loss_classification::{Trend, PacketLoss};
///
/// let mut trend = Trend::default();
/// for i in 0..16 {
///     assert_eq!(trend.classify((i as f64).powf(1.5)), PacketLoss::Wireless);
/// }
/// assert_eq!(trend.classify(150.0), PacketLoss::Congestion);
/// ````
///
/// [`MBiaz`]: struct.MBiaz.html
/// [`Spike`]: struct.Spike.html
/// [`Trend`]: struct.Trend.html
/// [`ZBS`]: struct.ZBS.html
/// [`ZigZag`]: struct.ZigZag.html
#[derive(Debug)]
pub struct Trend {
    previous_rott: f64,
    gamma: f64,
    s_threshold: f64,
    s_f: f64,
}

impl Default for Trend {
    fn default() -> Self {
        Self {
            previous_rott: 0.0,
            gamma: 1.0 / 30.0,
            s_threshold: 0.4,
            s_f: 0.0,
        }
    }
}

impl Trend {
    /// Creates a new packet loss classifier based on the `Trend` scheme.
    ///
    /// # Arguments
    ///
    /// - 1.0 - `gamma`: exponential decaying factor that weights the impact of the
    /// current `rott`.
    ///
    /// # Panics
    ///
    /// Panics if `gamma` or `s_threshold` are not in [0.0, 1.0].
    pub fn new(gamma: f64, s_threshold: f64) -> Self {
        assert!(0.0 <= gamma && gamma <= 1.0);
        assert!(0.0 <= s_threshold && s_threshold <= 1.0);
        Self {
            previous_rott: 0.0,
            gamma,
            s_threshold,
            s_f: 0.0,
        }
    }

    /// Classifies the reason of packet loss based on the ROTT of the current packet.
    ///
    /// This function is called with the ROTT (relative one-way trip time) of the current
    /// packet if previous packets were lost.
    ///
    /// # Arguments
    ///
    /// - `rott`: relative one-way trip time of the current packet.
    pub fn classify(&mut self, rott: f64) -> PacketLoss {
        assert!(rott >= 0.0);

        self.s_f = (1.0 - self.gamma) * self.s_f;
        if rott > self.previous_rott {
            self.s_f += self.gamma;
        }

        self.previous_rott = rott;

        if self.s_threshold < self.s_f {
            PacketLoss::Congestion
        } else {
            PacketLoss::Wireless
        }
    }
}
