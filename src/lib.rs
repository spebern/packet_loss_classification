//! This crate provides some popular packet loss classifiers.
//!
//! Packet loss in networks happens due to congestion and wireless errors.
//! Depending on the reason behind such errors an application might have to
//! take different measures to enhance performance.
//!
//! This crate provides five classifiers ([`MBiaz`], [`Spike`], [`ZigZag`], [`ZBS`] and [`Trend`])
//! for packet loss classification. Each performs well under certain circumstances and it is
//! up to the user to decide on the best fit. [`ZBS`] being a hybrid version of the first four
//! can lead to good results across a number of network topologies based on topology estimation.
//!
//! For the theory behind all algorithms the following two papers (where theory and algorithms
//! are taken from) are highly recommended:
//!
//! - Cen, Song, Pamela C. Cosman, and Geoffrey M. Voelker. "End-to-end differentiation of congestion and wireless losses." IEEE/ACM Transactions on Networking (TON) 11.5 (2003): 703-717
//! - Hsiao, Hsu-Feng, et al. "A new multimedia packet loss classification algorithm for congestion control over wired/wireless channels." Proceedings.(ICASSP'05). IEEE International Conference on Acoustics, Speech, and Signal Processing, 2005.. Vol. 2. IEEE, 2005.
//!
//! [`MBiaz`]: struct.MBiaz.html
//! [`Spike`]: struct.Spike.html
//! [`Trend`]: struct.Trend.html
//! [`ZBS`]: struct.ZBS.html
//! [`ZigZag`]: struct.ZigZag.html

/// The classified reason of packet loss.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PacketLoss {
    /// Packet loss due to congestion.
    Congestion,
    /// Packet loss due to wireless error.
    Wireless,
}

mod mbiaz;
mod spike;
mod trend;
mod zbs;
mod zigzag;

pub use mbiaz::MBiaz;
pub use spike::Spike;
pub use trend::Trend;
pub use zbs::ZBS;
pub use zigzag::ZigZag;
