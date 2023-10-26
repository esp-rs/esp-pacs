#[doc = "Register `TIMER1_SYNC` reader"]
pub type R = crate::R<TIMER1_SYNC_SPEC>;
#[doc = "Register `TIMER1_SYNC` writer"]
pub type W = crate::W<TIMER1_SYNC_SPEC>;
#[doc = "Field `TIMER1_SYNCI_EN` reader - When set, timer reloading with phase on sync input event is enabled."]
pub type TIMER1_SYNCI_EN_R = crate::BitReader;
#[doc = "Field `TIMER1_SYNCI_EN` writer - When set, timer reloading with phase on sync input event is enabled."]
pub type TIMER1_SYNCI_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SW` reader - Toggling this bit will trigger a software sync."]
pub type SW_R = crate::BitReader;
#[doc = "Field `SW` writer - Toggling this bit will trigger a software sync."]
pub type SW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER1_SYNCO_SEL` reader - PWM timer1 sync_out selection, 0: sync_in, 1: TEZ, 2: TEP, and sync out will always generate when toggling the reg_timer1_sync_sw bit"]
pub type TIMER1_SYNCO_SEL_R = crate::FieldReader;
#[doc = "Field `TIMER1_SYNCO_SEL` writer - PWM timer1 sync_out selection, 0: sync_in, 1: TEZ, 2: TEP, and sync out will always generate when toggling the reg_timer1_sync_sw bit"]
pub type TIMER1_SYNCO_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TIMER1_PHASE` reader - phase for timer reload on sync event"]
pub type TIMER1_PHASE_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER1_PHASE` writer - phase for timer reload on sync event"]
pub type TIMER1_PHASE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `TIMER1_PHASE_DIRECTION` reader - Configure the PWM timer1's direction when timer1 mode is up-down mode: 0-increase,1-decrease"]
pub type TIMER1_PHASE_DIRECTION_R = crate::BitReader;
#[doc = "Field `TIMER1_PHASE_DIRECTION` writer - Configure the PWM timer1's direction when timer1 mode is up-down mode: 0-increase,1-decrease"]
pub type TIMER1_PHASE_DIRECTION_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - When set, timer reloading with phase on sync input event is enabled."]
    #[inline(always)]
    pub fn timer1_synci_en(&self) -> TIMER1_SYNCI_EN_R {
        TIMER1_SYNCI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Toggling this bit will trigger a software sync."]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PWM timer1 sync_out selection, 0: sync_in, 1: TEZ, 2: TEP, and sync out will always generate when toggling the reg_timer1_sync_sw bit"]
    #[inline(always)]
    pub fn timer1_synco_sel(&self) -> TIMER1_SYNCO_SEL_R {
        TIMER1_SYNCO_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:19 - phase for timer reload on sync event"]
    #[inline(always)]
    pub fn timer1_phase(&self) -> TIMER1_PHASE_R {
        TIMER1_PHASE_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bit 20 - Configure the PWM timer1's direction when timer1 mode is up-down mode: 0-increase,1-decrease"]
    #[inline(always)]
    pub fn timer1_phase_direction(&self) -> TIMER1_PHASE_DIRECTION_R {
        TIMER1_PHASE_DIRECTION_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1_SYNC")
            .field(
                "timer1_synci_en",
                &format_args!("{}", self.timer1_synci_en().bit()),
            )
            .field("sw", &format_args!("{}", self.sw().bit()))
            .field(
                "timer1_synco_sel",
                &format_args!("{}", self.timer1_synco_sel().bits()),
            )
            .field(
                "timer1_phase",
                &format_args!("{}", self.timer1_phase().bits()),
            )
            .field(
                "timer1_phase_direction",
                &format_args!("{}", self.timer1_phase_direction().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER1_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When set, timer reloading with phase on sync input event is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_synci_en(&mut self) -> TIMER1_SYNCI_EN_W<TIMER1_SYNC_SPEC, 0> {
        TIMER1_SYNCI_EN_W::new(self)
    }
    #[doc = "Bit 1 - Toggling this bit will trigger a software sync."]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<TIMER1_SYNC_SPEC, 1> {
        SW_W::new(self)
    }
    #[doc = "Bits 2:3 - PWM timer1 sync_out selection, 0: sync_in, 1: TEZ, 2: TEP, and sync out will always generate when toggling the reg_timer1_sync_sw bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_synco_sel(&mut self) -> TIMER1_SYNCO_SEL_W<TIMER1_SYNC_SPEC, 2> {
        TIMER1_SYNCO_SEL_W::new(self)
    }
    #[doc = "Bits 4:19 - phase for timer reload on sync event"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_phase(&mut self) -> TIMER1_PHASE_W<TIMER1_SYNC_SPEC, 4> {
        TIMER1_PHASE_W::new(self)
    }
    #[doc = "Bit 20 - Configure the PWM timer1's direction when timer1 mode is up-down mode: 0-increase,1-decrease"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_phase_direction(&mut self) -> TIMER1_PHASE_DIRECTION_W<TIMER1_SYNC_SPEC, 20> {
        TIMER1_PHASE_DIRECTION_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM timer1 sync function configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER1_SYNC_SPEC;
impl crate::RegisterSpec for TIMER1_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1_sync::R`](R) reader structure"]
impl crate::Readable for TIMER1_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer1_sync::W`](W) writer structure"]
impl crate::Writable for TIMER1_SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER1_SYNC to value 0"]
impl crate::Resettable for TIMER1_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
