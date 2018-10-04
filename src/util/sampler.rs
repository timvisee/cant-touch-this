//! A streaming/lazy iterator that resamples points in real-time.

use std::iter::{Iterator, Peekable};

use nalgebra::geometry;

use config::sample::DISTANCE as SAMPLE_DISTANCE;

/// The point type used in the sampler iterator.
type Point3 = geometry::Point3<f64>;

/// A point iterator that resamples incomming points in real-time.
///
/// Right now, the sampler uses a fixed sample distance defined in `config::sample::DISTANCE`.
pub(crate) struct Sampler<I>
where
    I: Iterator<Item = Point3>,
{
    /// The iterator the non-sampled points are pulled from.
    iter: Peekable<I>,

    /// The last sampled point, used to determine where the next point can be sampled.
    last: Option<Point3>,
}

impl<I> SamplerIter<I>
where
    I: Iterator<Item = Point3>,
{
    /// Construct a new sampler iterator, mounted on the given point iterator.
    pub fn new(iter: I) -> Sampler<I> {
        Sampler {
            iter: iter.peekable(),
            last: None,
        }
    }
}

impl<I> Iterator for Sampler<I>
where
    I: Iterator<Item = Point3>,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Peek the next point
        while let Some(p) = self.iter.peek().cloned() {
            match self.last {
                Some(last) => {
                    // Advance the points until we find one further than the sample distance
                    if (last - p).magnitude() < SAMPLE_DISTANCE {
                        self.iter.next();
                        continue;
                    }

                    // Get the point vector, normalize it to the preferred sample distance
                    let vector = (p - last).normalize() * SAMPLE_DISTANCE;

                    // Define the new sample point with this vector
                    let sampled = last + vector;
                    self.last.replace(sampled);
                    return Some(sampled);
                }

                None => {
                    // Sample the origin as first point
                    self.last.replace(p);
                    return self.iter.next();
                }
            }
        }

        None
    }
}

pub(crate) trait SamplerIter<I>
where
    I: Iterator<Item = Point3>,
{
    fn sample_points(self) -> Sampler<I>;
}

impl<I> SamplerIter<I> for I
where
    I: Iterator<Item = Point3>,
{
    fn sample_points(self) -> Sampler<I> {
        SamplerIter::new(self)
    }
}
