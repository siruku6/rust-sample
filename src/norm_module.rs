pub mod norm {
    use rand_distr::Distribution;
    use rand_distr::Normal;

    pub fn sampling_norm(mu: f64, sigma: f64, count: i32) -> Vec<f64> {
        /*
        正規分布からの乱数生成器を使ってベクトルを生成する
        */

        // デフォルトの乱数生成器を初期化します
        let mut rng = rand::rng();
        // 平均mu, 標準偏差sigmaの正規分布を作成します
        let dist = Normal::<f64>::new(mu, sigma).unwrap();

        let mut vec_rand: Vec<f64> = vec![];
        for _i in 0..count {
            let sample = dist.sample(&mut rng);
            vec_rand.push(sample);
        }
        vec_rand
    }
}
