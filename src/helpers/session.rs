use governor::{Quota, RateLimiter};
use reqwest::Client;

const LIMITER: RateLimiter = RateLimiter::direct(Quota::per_second(5));
