use crate::{MBiaz, PacketLoss, Spike, ZigZag};

/// Packet loss classifier based on the [`ZBS`] hybrid scheme.
///
/// Each congestion control algorithm performs well on topologies that it was designed for.
/// [`ZBS`] is a hybrid algorithm that selects the congestion control algorithm depending on
/// the network characteristics.
///
/// In WLH (Wireless Last Hop) topologies [`ZigZag`] and [`MBiaz`] perform well, while in WB
/// (Wireless Backbone) [`Spike`] performs best and [`ZigZag`] works reasonably well.
///
/// [`ZigZag`]:
///   - ambiguous network topology estimation (e.g. at startup)
/// [`Spike`]:
///   - slowest link underutilized (ROTT is close to its minimum)
///   - multiple competing flows
/// [`MBiaz`]
///   - wireless link is bottleneck and not shared
///
/// ```rust
/// use packet_loss_classification::{ZBS, PacketLoss};
///
/// let mut zbs = ZBS::default();
/// assert_eq!(zbs.classify(5.0, 10.0, 1), PacketLoss::Wireless);
/// assert_eq!(zbs.classify(6.0, 10.0, 1), PacketLoss::Congestion);
/// ````
///
/// [`MBiaz`]: struct.MBiaz.html
/// [`Spike`]: struct.Spike.html
/// [`ZBS`]: struct.ZBS.html
/// [`ZigZag`]: struct.ZigZag.html
#[derive(Debug)]
pub struct ZBS {
    mbiaz: MBiaz,
    spike: Spike,
    zigzag: ZigZag,
    t_avg: f64,
    t_min: f64,
    rott_min: f64,
}

impl Default for ZBS {
    fn default() -> Self {
        Self {
            mbiaz: MBiaz::default(),
            spike: Spike::default(),
            zigzag: ZigZag::default(),
            t_avg: 0.0,
            t_min: std::f64::MAX,
            rott_min: std::f64::MAX,
        }
    }
}

impl ZBS {
    /// Creates a new packet loss classifier based on the `ZBS` hybrid scheme.
    ///
    /// # Arguments
    ///
    /// - `mbiaz`: classifier based on the [`MBiaz`] scheme.
    /// - `spike`: classifier based on the [`Spike`] scheme.
    /// - `zigzag`: classifier based on the [`ZigZag`] scheme.
    ///
    /// [`MBiaz`]: struct.MBiaz.html
    /// [`Spike`]: struct.Spike.html
    /// [`Trend`]: struct.Trend.html
    /// [`ZBS`]: struct.ZBS.html
    /// [`ZigZag`]: struct.ZigZag.html
    pub fn new(mbiaz: MBiaz, spike: Spike, zigzag: ZigZag) -> Self {
        Self {
            mbiaz,
            spike,
            zigzag,
            t_avg: 0.0,
            t_min: std::f64::MAX,
            rott_min: std::f64::MAX,
        }
    }

    /// Classifies the reason of packet loss based on the ROTT of the current packet.
    ///
    /// # Arguments
    ///
    /// - `rott`: relative one-way trip time of the current packet.
    /// - `interarrival_time`: interarrival time between packet P(i) and packet P(i + n + 1).
    /// - `num_lost_packets`: number of packets that have been lost in a row.
    pub fn classify(
        &mut self,
        rott: f64,
        interarrival_time: f64,
        num_lost_packets: u32,
    ) -> PacketLoss {
        assert!(rott >= 0.0);
        assert!(interarrival_time >= 0.0);
        assert!(num_lost_packets > 0);

        if interarrival_time < self.t_min {
            self.t_min = interarrival_time;
        }
        if rott < self.rott_min {
            self.rott_min = rott;
        }

        self.t_avg = 0.875 * self.t_avg
            + 0.125 * interarrival_time * interarrival_time / num_lost_packets as f64;

        // In WLH topology t_narr ~ 1, while in WB topology t_narr ~ N, where N is the number of flows
        // sharing the link with the lowest bandwidth.
        let t_narr = self.t_avg / self.t_min;

        if rott < self.rott_min + 0.05 * self.t_min {
            self.spike.classify(rott)
        } else {
            if t_narr < 0.875 {
                self.zigzag.classify(rott, num_lost_packets)
            } else if t_narr < 1.5 {
                self.mbiaz.classify(interarrival_time, num_lost_packets)
            } else if t_narr < 2.0 {
                self.zigzag.classify(rott, num_lost_packets)
            } else {
                self.spike.classify(rott)
            }
        }
    }
}
