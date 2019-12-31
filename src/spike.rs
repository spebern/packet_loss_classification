use crate::PacketLoss;

#[derive(Debug, Clone, Copy)]
enum State {
    Spike,
    Normal,
}

/// Packet loss classifier based on the [`Spike`] scheme.
///
/// The [`Spike`] scheme assumes that congestion errors lead to spikes in delays.
/// ROTT (relative one-way trip time, time from sender to receiver) is used
/// as a measure for the delay. It assumes synchronized shared between sender
/// and receiver.
///
/// The classifier keeps track of the state which is either `Spike` or `Normal`.
/// If packet loss happens in `Spike` state it is classified as being due to
/// congestion otherwise due to wireless errors.
///
/// The state depends on the ROTT of the current packet. If it leaps over a certain
/// threshold the classifier moves to spike state. `Spike` state can be left, once
/// the ROTT goes under another threshold. Both thresholds depend on the so far
/// maximum and minimum observed ROTTs.
///
/// ```rust
/// use packet_loss_classification::{Spike, PacketLoss};
///
/// let mut spike = Spike::default();
/// assert_eq!(spike.classify(20.0), PacketLoss::Wireless);
/// assert_eq!(spike.classify(10.0), PacketLoss::Wireless);
/// assert_eq!(spike.classify(30.0), PacketLoss::Congestion);
/// ````
///
/// [`Spike`]: struct.Spike.html
#[derive(Debug)]
pub struct Spike {
    rott_min: f64,
    rott_max: f64,
    alpha: f64,
    beta: f64,
    state: State,
}

impl Default for Spike {
    fn default() -> Self {
        Self {
            rott_min: std::f64::MAX,
            rott_max: 0.0,
            alpha: 1.0 / 2.0,
            beta: 1.0 / 3.0,
            state: State::Normal,
        }
    }
}

impl Spike {
    /// Creates a new packet loss classifier based on the Spike scheme.
    ///
    /// # Arguments
    ///
    /// - `alpha`: used to adjust the spike start threshold
    /// - `beta`: used to adjust the spike end threshold
    ///
    /// # Panics
    /// Panics if `alpha` is smaller than `beta`.
    pub fn new(alpha: f64, beta: f64) -> Self {
        assert!(alpha >= beta);
        Self {
            alpha,
            beta,
            rott_min: std::f64::MAX,
            rott_max: 0.0,
            state: State::Normal,
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
    pub fn classify(&mut self, rott: f64) -> PacketLoss {
        if rott < self.rott_min {
            self.rott_min = rott;
        }
        if rott > self.rott_max {
            self.rott_max = rott;
        }

        let b_spike_start = self.rott_min + self.alpha * (self.rott_max - self.rott_min);
        let b_spike_end = self.rott_min + self.beta * (self.rott_max - self.rott_min);

        match self.state {
            State::Spike => {
                if rott < b_spike_end {
                    self.state = State::Normal;
                    return PacketLoss::Wireless;
                }
                return PacketLoss::Congestion;
            }
            State::Normal => {
                if rott > b_spike_start {
                    self.state = State::Spike;
                    return PacketLoss::Congestion;
                }
                return PacketLoss::Wireless;
            }
        }
    }
}
