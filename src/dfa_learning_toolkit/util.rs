/// StatsTracker struct is used to keep track of
/// minimum, maximum, average, variance and
/// standard deviation values given a sequence
/// of values. Mean and variance calculation
/// is done using Welford's online algorithm.
pub struct StatsTracker {
    min: f64,  // Minimum of values.
    max: f64,  // Maximum of values.
    count: i64,// Number of values.
    mean: f64, // Running average of values.
    m2: f64,   // Running value used to calculate variance/standard dev.
}

/// new_stats_tracker returns an empty StatsTracker struct.
pub fn new_stats_tracker() -> StatsTracker {
    return StatsTracker{
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
        count: 0,
        mean: 0.0,
        m2: 0.0
    }
}

impl StatsTracker {
    /// add adds a float value to the StatsTracker struct.
    pub fn add(&mut self, value: f64) {
        // Skip +Inf and -Inf values.
        if value == f64::INFINITY || value == f64::NEG_INFINITY {
            return
        }

        // If value is smaller than the minimum value,
        // set minimum value within struct to value.
        if value < self.min {
            self.min = value;
        }

        // If value is larger than the maximum value,
        // set maximum value within struct to value.
        if value > self.max {
            self.max = value;
        }

        // Increment counter.
        self.count += 1;

        let mu_new = self.mean + ((value - self.mean) / self.count as f64);

        self.m2 += (value - self.mean) * (value - mu_new);

        self.mean = mu_new;
    }

    /// add_int adds an integer value to the StatsTracker struct.
    pub fn add_int(&mut self, int_value: i64) {
        // Cast to f64 and call add function.
        self.add(int_value as f64)
    }

    /// min returns the minimum value within the StatsTracker struct.
    pub fn min(&self) -> f64 {
        return self.min
    }

    /// max returns the maximum value within the StatsTracker struct.
    pub fn max(&self) -> f64 {
        return self.max
    }

    /// mean returns the average value within the StatsTracker struct.
    pub fn mean(&self) -> f64 {
        return self.mean
    }

    /// population_variance returns the population variance value within the StatsTracker struct.
    pub fn population_variance(&self) -> f64 {
        if self.count > 1 {
            return self.m2 / self.count as f64
        }

        return 0.0
    }

    /// sample_variance returns the sample variance value within the StatsTracker struct.
    pub fn sample_variance(&self) -> f64 {
        if self.count > 1 {
            return self.m2 / (self.count - 1) as f64
        }

        return 0.0
    }

    /// population_standard_dev returns the population standard deviation value within the StatsTracker struct.
    pub fn population_standard_dev(&self) -> f64 {
        return f64::sqrt(self.population_variance())
    }

    /// sample_standard_dev returns the sample standard deviation value within the StatsTracker struct.
    pub fn sample_standard_dev(&self) -> f64 {
        return f64::sqrt(self.sample_variance())
    }
}