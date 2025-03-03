// Reference rust implementation of AluVM (arithmetic logic unit virtual machine).
// To find more on AluVM please check <https://github.com/internet2-org/aluvm-spec>
//
// Designed & written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
// This work is donated to LNP/BP Standards Association by Pandora Core AG
//
// This software is licensed under the terms of MIT License.
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

//! Business logic and data structures for working with AluVM code libraries

pub mod constants;
mod cursor;
mod model;
mod rw;
mod segs;

pub use cursor::Cursor;
pub use model::{AssemblerError, Lib, LibId, LibIdError, LibIdTag, LibSite};
pub use rw::{CodeEofError, Read, Write, WriteError};
pub use segs::{IsaSeg, IsaSegError, LibSeg, LibSegOverflow, SegmentError};
