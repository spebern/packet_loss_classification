use crate::PacketLoss;

/// Packet loss classifier based on the [`ZigZag`] scheme.
/// 
/// In wireless networks wireless errors are the most common errors. Congestion
/// errors usually come with a higher delay. Intuitively classification can be
/// done with a threshold. The ZigZag scheme uses thresholds that depend on the
/// number of packets that have been lost consecutively.
///
/// The more packets lost in a row the higher the threshold for classifying congestion
/// errors. This is because a higher number of lost packets comes with a higher expectation
/// for the ROTT.
///
/// The thresholds depend on the standard deviation and the mean value of the ROTT which
/// are calculated on the fly using an exponential average with the exponential decaying
/// factor 1.0 - `alpha`.
///
/// ```rust
/// use packet_loss_classification::{ZigZag, PacketLoss};
///
/// let mut zigzag = ZigZag::default();
/// assert_eq!(zigzag.classify(10.0, 1), PacketLoss::Congestion);
/// assert_eq!(zigzag.classify(10.0, 1), PacketLoss::Congestion);
/// assert_eq!(zigzag.classify(0.3, 4), PacketLoss::Wireless);
/// ````
///
/// [`ZigZag`]: struct.ZigZag.html
#[derive(Debug)]
pub struct ZigZag {
    rott_mean: f64,
    rott_dev: f64,
    alpha: f64,
}

impl Default for ZigZag {
    fn default() -> Self {
        Self {
            rott_mean: 0.0,
            rott_dev: 0.0,
            alpha: 1.0 / 32.0,
        }
    }
}

impl ZigZag {
    /// Creates a new packet loss classifier based on the ZigZag scheme.
    ///
    /// # Arguments
    ///
    /// - 1.0 - `alpha`: exponential decaying factor that weights the impact of the
    /// current `rott`.
    ///
    /// # Panics
    /// Panics if `alpha` is not in [0.0, 1.0].
    pub fn new(alpha: f64) -> Self {
        assert!(0.0 <= alpha && alpha <= 1.0);
        Self {
            rott_mean: 0.0,
            rott_dev: 0.0,
            alpha,
        }
    }

    /// Classifies the reason of packet loss based on the ROTT of the current packet.
    ///
    /// This function is called with the ROTT (relative one-way trip time) of the current
    /// packet if previous packets were lost
    ///
    /// # Arguments
    ///
    /// - `rott`: relative one-way trip time of the current packet.
    /// - `num_lost_packets`: the number of packets that have been lost.
    pub fn classify(&mut self, rott: f64, num_lost_packets: u32) -> PacketLoss {
        assert!(rott >= 0.0);
        assert!(num_lost_packets > 0);

        self.rott_mean = (1.0 - self.alpha) * self.rott_mean + self.alpha * rott;
        self.rott_dev = (1.0 - 2.0 * self.alpha) + 2.0 * self.alpha * (rott - self.rott_mean).abs();

        match num_lost_packets {
            1 => {
                if rott < self.rott_mean - self.rott_dev {
                    PacketLoss::Wireless
                } else {
                    PacketLoss::Congestion
                }
            }
            2 => {
                if rott < self.rott_mean - self.rott_dev / 2.0 {
                    PacketLoss::Wireless
                } else {
                    PacketLoss::Congestion
                }
            }
            3 => {
                if rott < self.rott_mean {
                    PacketLoss::Wireless
                } else {
                    PacketLoss::Congestion
                }
            }
            _ => {
                if rott < self.rott_mean + self.rott_dev / 2.0 {
                    PacketLoss::Wireless
                } else {
                    PacketLoss::Congestion
                }
            }
        }
    }
}
