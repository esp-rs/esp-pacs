#[doc = "Register `EVT_TASK_EN2` reader"]
pub struct R(crate::R<EVT_TASK_EN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVT_TASK_EN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVT_TASK_EN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVT_TASK_EN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVT_TASK_EN2` writer"]
pub struct W(crate::W<EVT_TASK_EN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVT_TASK_EN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EVT_TASK_EN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVT_TASK_EN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TASK_GAMMA_RESTART_CH0_EN` reader - Ledc ch0 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH0_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESTART_CH0_EN` writer - Ledc ch0 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH0_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESTART_CH1_EN` reader - Ledc ch1 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH1_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESTART_CH1_EN` writer - Ledc ch1 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH1_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESTART_CH2_EN` reader - Ledc ch2 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH2_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESTART_CH2_EN` writer - Ledc ch2 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH2_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESTART_CH3_EN` reader - Ledc ch3 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH3_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESTART_CH3_EN` writer - Ledc ch3 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH3_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESTART_CH4_EN` reader - Ledc ch4 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH4_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESTART_CH4_EN` writer - Ledc ch4 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH4_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESTART_CH5_EN` reader - Ledc ch5 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH5_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESTART_CH5_EN` writer - Ledc ch5 gamma restart task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESTART_CH5_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_PAUSE_CH0_EN` reader - Ledc ch0 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH0_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_PAUSE_CH0_EN` writer - Ledc ch0 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH0_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_PAUSE_CH1_EN` reader - Ledc ch1 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH1_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_PAUSE_CH1_EN` writer - Ledc ch1 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH1_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_PAUSE_CH2_EN` reader - Ledc ch2 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH2_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_PAUSE_CH2_EN` writer - Ledc ch2 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH2_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_PAUSE_CH3_EN` reader - Ledc ch3 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH3_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_PAUSE_CH3_EN` writer - Ledc ch3 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH3_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_PAUSE_CH4_EN` reader - Ledc ch4 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH4_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_PAUSE_CH4_EN` writer - Ledc ch4 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH4_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_PAUSE_CH5_EN` reader - Ledc ch5 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH5_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_PAUSE_CH5_EN` writer - Ledc ch5 gamma pause task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_PAUSE_CH5_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESUME_CH0_EN` reader - Ledc ch0 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH0_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESUME_CH0_EN` writer - Ledc ch0 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH0_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESUME_CH1_EN` reader - Ledc ch1 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH1_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESUME_CH1_EN` writer - Ledc ch1 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH1_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESUME_CH2_EN` reader - Ledc ch2 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH2_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESUME_CH2_EN` writer - Ledc ch2 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH2_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESUME_CH3_EN` reader - Ledc ch3 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH3_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESUME_CH3_EN` writer - Ledc ch3 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH3_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESUME_CH4_EN` reader - Ledc ch4 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH4_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESUME_CH4_EN` writer - Ledc ch4 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH4_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
#[doc = "Field `TASK_GAMMA_RESUME_CH5_EN` reader - Ledc ch5 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH5_EN_R = crate::BitReader;
#[doc = "Field `TASK_GAMMA_RESUME_CH5_EN` writer - Ledc ch5 gamma resume task enable register, write 1 to enable this task."]
pub type TASK_GAMMA_RESUME_CH5_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Ledc ch0 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_restart_ch0_en(&self) -> TASK_GAMMA_RESTART_CH0_EN_R {
        TASK_GAMMA_RESTART_CH0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ledc ch1 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_restart_ch1_en(&self) -> TASK_GAMMA_RESTART_CH1_EN_R {
        TASK_GAMMA_RESTART_CH1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ledc ch2 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_restart_ch2_en(&self) -> TASK_GAMMA_RESTART_CH2_EN_R {
        TASK_GAMMA_RESTART_CH2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ledc ch3 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_restart_ch3_en(&self) -> TASK_GAMMA_RESTART_CH3_EN_R {
        TASK_GAMMA_RESTART_CH3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ledc ch4 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_restart_ch4_en(&self) -> TASK_GAMMA_RESTART_CH4_EN_R {
        TASK_GAMMA_RESTART_CH4_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ledc ch5 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_restart_ch5_en(&self) -> TASK_GAMMA_RESTART_CH5_EN_R {
        TASK_GAMMA_RESTART_CH5_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Ledc ch0 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_pause_ch0_en(&self) -> TASK_GAMMA_PAUSE_CH0_EN_R {
        TASK_GAMMA_PAUSE_CH0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ledc ch1 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_pause_ch1_en(&self) -> TASK_GAMMA_PAUSE_CH1_EN_R {
        TASK_GAMMA_PAUSE_CH1_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Ledc ch2 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_pause_ch2_en(&self) -> TASK_GAMMA_PAUSE_CH2_EN_R {
        TASK_GAMMA_PAUSE_CH2_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Ledc ch3 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_pause_ch3_en(&self) -> TASK_GAMMA_PAUSE_CH3_EN_R {
        TASK_GAMMA_PAUSE_CH3_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Ledc ch4 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_pause_ch4_en(&self) -> TASK_GAMMA_PAUSE_CH4_EN_R {
        TASK_GAMMA_PAUSE_CH4_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Ledc ch5 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_pause_ch5_en(&self) -> TASK_GAMMA_PAUSE_CH5_EN_R {
        TASK_GAMMA_PAUSE_CH5_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Ledc ch0 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_resume_ch0_en(&self) -> TASK_GAMMA_RESUME_CH0_EN_R {
        TASK_GAMMA_RESUME_CH0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ledc ch1 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_resume_ch1_en(&self) -> TASK_GAMMA_RESUME_CH1_EN_R {
        TASK_GAMMA_RESUME_CH1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ledc ch2 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_resume_ch2_en(&self) -> TASK_GAMMA_RESUME_CH2_EN_R {
        TASK_GAMMA_RESUME_CH2_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Ledc ch3 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_resume_ch3_en(&self) -> TASK_GAMMA_RESUME_CH3_EN_R {
        TASK_GAMMA_RESUME_CH3_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Ledc ch4 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_resume_ch4_en(&self) -> TASK_GAMMA_RESUME_CH4_EN_R {
        TASK_GAMMA_RESUME_CH4_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Ledc ch5 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_gamma_resume_ch5_en(&self) -> TASK_GAMMA_RESUME_CH5_EN_R {
        TASK_GAMMA_RESUME_CH5_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_TASK_EN2")
            .field(
                "task_gamma_restart_ch0_en",
                &format_args!("{}", self.task_gamma_restart_ch0_en().bit()),
            )
            .field(
                "task_gamma_restart_ch1_en",
                &format_args!("{}", self.task_gamma_restart_ch1_en().bit()),
            )
            .field(
                "task_gamma_restart_ch2_en",
                &format_args!("{}", self.task_gamma_restart_ch2_en().bit()),
            )
            .field(
                "task_gamma_restart_ch3_en",
                &format_args!("{}", self.task_gamma_restart_ch3_en().bit()),
            )
            .field(
                "task_gamma_restart_ch4_en",
                &format_args!("{}", self.task_gamma_restart_ch4_en().bit()),
            )
            .field(
                "task_gamma_restart_ch5_en",
                &format_args!("{}", self.task_gamma_restart_ch5_en().bit()),
            )
            .field(
                "task_gamma_pause_ch0_en",
                &format_args!("{}", self.task_gamma_pause_ch0_en().bit()),
            )
            .field(
                "task_gamma_pause_ch1_en",
                &format_args!("{}", self.task_gamma_pause_ch1_en().bit()),
            )
            .field(
                "task_gamma_pause_ch2_en",
                &format_args!("{}", self.task_gamma_pause_ch2_en().bit()),
            )
            .field(
                "task_gamma_pause_ch3_en",
                &format_args!("{}", self.task_gamma_pause_ch3_en().bit()),
            )
            .field(
                "task_gamma_pause_ch4_en",
                &format_args!("{}", self.task_gamma_pause_ch4_en().bit()),
            )
            .field(
                "task_gamma_pause_ch5_en",
                &format_args!("{}", self.task_gamma_pause_ch5_en().bit()),
            )
            .field(
                "task_gamma_resume_ch0_en",
                &format_args!("{}", self.task_gamma_resume_ch0_en().bit()),
            )
            .field(
                "task_gamma_resume_ch1_en",
                &format_args!("{}", self.task_gamma_resume_ch1_en().bit()),
            )
            .field(
                "task_gamma_resume_ch2_en",
                &format_args!("{}", self.task_gamma_resume_ch2_en().bit()),
            )
            .field(
                "task_gamma_resume_ch3_en",
                &format_args!("{}", self.task_gamma_resume_ch3_en().bit()),
            )
            .field(
                "task_gamma_resume_ch4_en",
                &format_args!("{}", self.task_gamma_resume_ch4_en().bit()),
            )
            .field(
                "task_gamma_resume_ch5_en",
                &format_args!("{}", self.task_gamma_resume_ch5_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVT_TASK_EN2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Ledc ch0 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_restart_ch0_en(&mut self) -> TASK_GAMMA_RESTART_CH0_EN_W<0> {
        TASK_GAMMA_RESTART_CH0_EN_W::new(self)
    }
    #[doc = "Bit 1 - Ledc ch1 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_restart_ch1_en(&mut self) -> TASK_GAMMA_RESTART_CH1_EN_W<1> {
        TASK_GAMMA_RESTART_CH1_EN_W::new(self)
    }
    #[doc = "Bit 2 - Ledc ch2 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_restart_ch2_en(&mut self) -> TASK_GAMMA_RESTART_CH2_EN_W<2> {
        TASK_GAMMA_RESTART_CH2_EN_W::new(self)
    }
    #[doc = "Bit 3 - Ledc ch3 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_restart_ch3_en(&mut self) -> TASK_GAMMA_RESTART_CH3_EN_W<3> {
        TASK_GAMMA_RESTART_CH3_EN_W::new(self)
    }
    #[doc = "Bit 4 - Ledc ch4 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_restart_ch4_en(&mut self) -> TASK_GAMMA_RESTART_CH4_EN_W<4> {
        TASK_GAMMA_RESTART_CH4_EN_W::new(self)
    }
    #[doc = "Bit 5 - Ledc ch5 gamma restart task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_restart_ch5_en(&mut self) -> TASK_GAMMA_RESTART_CH5_EN_W<5> {
        TASK_GAMMA_RESTART_CH5_EN_W::new(self)
    }
    #[doc = "Bit 8 - Ledc ch0 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_pause_ch0_en(&mut self) -> TASK_GAMMA_PAUSE_CH0_EN_W<8> {
        TASK_GAMMA_PAUSE_CH0_EN_W::new(self)
    }
    #[doc = "Bit 9 - Ledc ch1 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_pause_ch1_en(&mut self) -> TASK_GAMMA_PAUSE_CH1_EN_W<9> {
        TASK_GAMMA_PAUSE_CH1_EN_W::new(self)
    }
    #[doc = "Bit 10 - Ledc ch2 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_pause_ch2_en(&mut self) -> TASK_GAMMA_PAUSE_CH2_EN_W<10> {
        TASK_GAMMA_PAUSE_CH2_EN_W::new(self)
    }
    #[doc = "Bit 11 - Ledc ch3 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_pause_ch3_en(&mut self) -> TASK_GAMMA_PAUSE_CH3_EN_W<11> {
        TASK_GAMMA_PAUSE_CH3_EN_W::new(self)
    }
    #[doc = "Bit 12 - Ledc ch4 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_pause_ch4_en(&mut self) -> TASK_GAMMA_PAUSE_CH4_EN_W<12> {
        TASK_GAMMA_PAUSE_CH4_EN_W::new(self)
    }
    #[doc = "Bit 13 - Ledc ch5 gamma pause task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_pause_ch5_en(&mut self) -> TASK_GAMMA_PAUSE_CH5_EN_W<13> {
        TASK_GAMMA_PAUSE_CH5_EN_W::new(self)
    }
    #[doc = "Bit 16 - Ledc ch0 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_resume_ch0_en(&mut self) -> TASK_GAMMA_RESUME_CH0_EN_W<16> {
        TASK_GAMMA_RESUME_CH0_EN_W::new(self)
    }
    #[doc = "Bit 17 - Ledc ch1 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_resume_ch1_en(&mut self) -> TASK_GAMMA_RESUME_CH1_EN_W<17> {
        TASK_GAMMA_RESUME_CH1_EN_W::new(self)
    }
    #[doc = "Bit 18 - Ledc ch2 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_resume_ch2_en(&mut self) -> TASK_GAMMA_RESUME_CH2_EN_W<18> {
        TASK_GAMMA_RESUME_CH2_EN_W::new(self)
    }
    #[doc = "Bit 19 - Ledc ch3 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_resume_ch3_en(&mut self) -> TASK_GAMMA_RESUME_CH3_EN_W<19> {
        TASK_GAMMA_RESUME_CH3_EN_W::new(self)
    }
    #[doc = "Bit 20 - Ledc ch4 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_resume_ch4_en(&mut self) -> TASK_GAMMA_RESUME_CH4_EN_W<20> {
        TASK_GAMMA_RESUME_CH4_EN_W::new(self)
    }
    #[doc = "Bit 21 - Ledc ch5 gamma resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_gamma_resume_ch5_en(&mut self) -> TASK_GAMMA_RESUME_CH5_EN_W<21> {
        TASK_GAMMA_RESUME_CH5_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ledc event task enable bit register2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evt_task_en2](index.html) module"]
pub struct EVT_TASK_EN2_SPEC;
impl crate::RegisterSpec for EVT_TASK_EN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evt_task_en2::R](R) reader structure"]
impl crate::Readable for EVT_TASK_EN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evt_task_en2::W](W) writer structure"]
impl crate::Writable for EVT_TASK_EN2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVT_TASK_EN2 to value 0"]
impl crate::Resettable for EVT_TASK_EN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
