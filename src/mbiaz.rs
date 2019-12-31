use crate::PacketLoss;

/// Packet loss classifier based on the [`MBiaz`] scheme.
///
/// The [`MBiaz`] scheme assumes that wireless errors have less influence on
/// delays than congestion errors. In case of `n` lost packets between
/// two consecutively received packets the classifier checks if the second
/// packet arrived "on time". "On time" means that it received in an expected
/// timeframe defined and bounded by:
///   - the so far minimal interarrival time
///   - the number of lost packets `n`
///   - the lower window limit
///   - the upper window limit
///
/// ```rust
/// use packet_loss_classification::{MBiaz, PacketLoss};
///
/// let mut mbiaz = MBiaz::default();
/// assert_eq!(mbiaz.classify(1.0, 1), PacketLoss::Congestion);
/// assert_eq!(mbiaz.classify(2.1, 1), PacketLoss::Wireless);
/// ````
///
/// [`MBiaz`]: struct.MBiaz.html
#[derive(Debug)]
pub struct MBiaz {
    lower_window_limit: f64,
    upper_window_limit: f64,
    interarrival_time_min: f64,
}

impl Default for MBiaz {
    fn default() -> Self {
        Self {
            lower_window_limit: 1.0,
            upper_window_limit: 1.25,
            interarrival_time_min: std::f64::MAX,
        }
    }
}

impl MBiaz {
    /// Creates a new packet loss classifier based on the MBiaz scheme.
    ///
    /// # Arguments
    ///
    /// - `lower_window_limit`: bias used to adjust the lower boundary for wireless
    /// packet loss classification.
    /// - `upper_window_limit`: bias used to adjust the upper boundary of wireless
    /// packet loss detection.
    ///
    /// # Panics
    /// Panics if `lower_window_limit` is bigger than `upper_window_limit`.
    pub fn new(lower_window_limit: f64, upper_window_limit: f64) -> Self {
        assert!(lower_window_limit <= upper_window_limit);
        Self {
            lower_window_limit,
            upper_window_limit,
            interarrival_time_min: std::f64::MAX,
        }
    }

    /// Classifies if the `n` lost packets are lost due to congestion or wireless errors.
    ///
    /// This function classifies the reason of packet loss between two consecutive packets
    /// P(i) and P(i + n + 1) where the numbers inside the brackets correspond to some sequence
    /// number.
    /// Therefor this function should only be called if there has been more than one lost packet.
    ///
    /// # Arguments
    ///
    /// - `interarrival_time`: interarrival time between packet P(i) and packet P(i + n + 1).
    /// - `num_lost_packets`: number of packets that have been lost.
    pub fn classify(&mut self, interarrival_time: f64, num_lost_packets: u32) -> PacketLoss {
        assert!(num_lost_packets > 0);

        if interarrival_time < self.interarrival_time_min {
            self.interarrival_time_min = interarrival_time;
        }

        let a = (num_lost_packets as f64 + self.lower_window_limit) * self.interarrival_time_min;
        let b = (num_lost_packets as f64 + self.upper_window_limit) * self.interarrival_time_min;

        if a <= interarrival_time && interarrival_time < b {
            PacketLoss::Wireless
        } else {
            PacketLoss::Congestion
        }
    }
}
