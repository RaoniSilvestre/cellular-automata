/// Estado de queima de uma célula.
///
/// sfb.py:178-180 (máscaras em `check`): 0 = suscetível, 1 = inflamada, 2 = queimada.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FireState {
    #[default]
    Susceptible,
    Burning,
    Burnt,
}
