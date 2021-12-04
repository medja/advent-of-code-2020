use anyhow::Context;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    solve::<2>(input)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    solve::<3>(input)
}

fn solve<const N: usize>(input: &[&str]) -> anyhow::Result<u32> {
    let numbers = input
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()?;

    Pairs::<_, N>::new(&numbers)
        .find(|values| values.iter().sum::<u32>() == 2020)
        .map(|values| values.iter().product::<u32>())
        .with_context(|| format!("Could not find {} values that sum up to 2020", N))
}

struct Pairs<'a, T, const N: usize>(&'a [T], [usize; N]);

impl<'a, T, const N: usize> Pairs<'a, T, N> {
    fn new(values: &'a [T]) -> Self {
        let mut indices = [0usize; N];

        for (i, index) in indices.iter_mut().enumerate() {
            *index = i;
        }

        Pairs(values, indices)
    }

    fn advance(&mut self, index: usize) {
        let next = self.1[index] + 1;
        let max = self.0.len() + index + 1 - N;

        if next >= max && index > 0 {
            self.advance(index - 1);
            self.1[index] = self.1[index - 1] + 1;
        } else {
            self.1[index] = next;
        }
    }
}

impl<T: Default + Copy, const N: usize> Iterator for Pairs<'_, T, N> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.1[N - 1] >= self.0.len() {
            None
        } else {
            let mut values = [T::default(); N];

            for (i, value) in values.iter_mut().enumerate() {
                *value = self.0[self.1[i]];
            }

            if N > 0 {
                self.advance(N - 1);
            }

            Some(values)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.0.len();
        let size = (0..N).map(|i| length - i).product();
        (size, Some(size))
    }
}
