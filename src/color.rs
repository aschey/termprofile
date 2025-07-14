use crate::TermProfile;

pub struct ProfileColor<T> {
    default: T,
    ansi_256: Option<T>,
    ansi_16: Option<T>,
}

impl<T> ProfileColor<T> {
    pub fn new(default_color: T) -> Self {
        Self {
            default: default_color,
            ansi_256: None,
            ansi_16: None,
        }
    }
}

impl<T> ProfileColor<T>
where
    T: Clone + Into<anstyle::Color>,
{
    pub fn adapt(&self, profile: &TermProfile) -> Option<anstyle::Color> {
        if *profile == TermProfile::Ansi256
            && let Some(ansi_256) = &self.ansi_256
        {
            return Some(ansi_256.clone().into());
        }

        if *profile == TermProfile::Ansi16
            && let Some(ansi_16) = &self.ansi_16
        {
            return Some(ansi_16.clone().into());
        }

        profile.adapt(self.default.clone())
    }
}

impl<T> ProfileColor<T>
where
    T: Clone + Into<Option<anstyle::Color>>,
{
    pub fn try_adapt(&self, profile: &TermProfile) -> Option<anstyle::Color> {
        if *profile == TermProfile::Ansi256
            && let Some(ansi_256) = &self.ansi_256
        {
            return ansi_256.clone().into();
        }

        if *profile == TermProfile::Ansi16
            && let Some(ansi_16) = &self.ansi_16
        {
            return ansi_16.clone().into();
        }

        if let Some(default) = self.default.clone().into() {
            profile.adapt(default)
        } else {
            None
        }
    }
}
