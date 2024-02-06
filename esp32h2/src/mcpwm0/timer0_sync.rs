#[doc = "Register `TIMER0_SYNC` reader"]
pub type R = crate::R<TIMER0_SYNC_SPEC>;
#[doc = "Register `TIMER0_SYNC` writer"]
pub type W = crate::W<TIMER0_SYNC_SPEC>;
#[doc = "Field `TIMER0_SYNCI_EN` reader - When set, timer reloading with phase on sync input event is enabled."]
pub type TIMER0_SYNCI_EN_R = crate::BitReader;
#[doc = "Field `TIMER0_SYNCI_EN` writer - When set, timer reloading with phase on sync input event is enabled."]
pub type TIMER0_SYNCI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW` reader - Toggling this bit will trigger a software sync."]
pub type SW_R = crate::BitReader;
#[doc = "Field `SW` writer - Toggling this bit will trigger a software sync."]
pub type SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_SYNCO_SEL` reader - PWM timer0 sync_out selection, 0: sync_in, 1: TEZ, 2: TEP, and sync out will always generate when toggling the reg_timer0_sync_sw bit"]
pub type TIMER0_SYNCO_SEL_R = crate::FieldReader;
#[doc = "Field `TIMER0_SYNCO_SEL` writer - PWM timer0 sync_out selection, 0: sync_in, 1: TEZ, 2: TEP, and sync out will always generate when toggling the reg_timer0_sync_sw bit"]
pub type TIMER0_SYNCO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMER0_PHASE` reader - phase for timer reload on sync event"]
pub type TIMER0_PHASE_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER0_PHASE` writer - phase for timer reload on sync event"]
pub type TIMER0_PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMER0_PHASE_DIRECTION` reader - Configure the PWM timer0's direction when timer0 mode is up-down mode: 0-increase,1-decrease"]
pub type TIMER0_PHASE_DIRECTION_R = crate::BitReader;
#[doc = "Field `TIMER0_PHASE_DIRECTION` writer - Configure the PWM timer0's direction when timer0 mode is up-down mode: 0-increase,1-decrease"]
pub type TIMER0_PHASE_DIRECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set, timer reloading with phase on sync input event is enabled."]
    #[inline(always)]
    pub fn timer0_synci_en(&self) -> TIMER0_SYNCI_EN_R {
        TIMER0_SYNCI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Toggling this bit will trigger a software sync."]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PWM timer0 sync_out selection, 0: sync_in, 1: TEZ, 2: TEP, and sync out will always generate when toggling the reg_timer0_sync_sw bit"]
    #[inline(always)]
    pub fn timer0_synco_sel(&self) -> TIMER0_SYNCO_SEL_R {
        TIMER0_SYNCO_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:19 - phase for timer reload on sync event"]
    #[inline(always)]
    pub fn timer0_phase(&self) -> TIMER0_PHASE_R {
        TIMER0_PHASE_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bit 20 - Configure the PWM timer0's direction when timer0 mode is up-down mode: 0-increase,1-decrease"]
    #[inline(always)]
    pub fn timer0_phase_direction(&self) -> TIMER0_PHASE_DIRECTION_R {
        TIMER0_PHASE_DIRECTION_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0_SYNC")
            .field(
                "timer0_synci_en",
                &format_args!("{}", self.timer0_synci_en().bit()),
            )
            .field("sw", &format_args!("{}", self.sw().bit()))
            .field(
                "timer0_synco_sel",
                &format_args!("{}", self.timer0_synco_sel().bits()),
            )
            .field(
                "timer0_phase",
                &format_args!("{}", self.timer0_phase().bits()),
            )
            .field(
                "timer0_phase_direction",
                &format_args!("{}", self.timer0_phase_direction().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER0_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - When set, timer reloading with phase on sync input event is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_synci_en(&mut self) -> TIMER0_SYNCI_EN_W<TIMER0_SYNC_SPEC> {
        TIMER0_SYNCI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Toggling this bit will trigger a software sync."]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<TIMER0_SYNC_SPEC> {
        SW_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - PWM timer0 sync_out selection, 0: sync_in, 1: TEZ, 2: TEP, and sync out will always generate when toggling the reg_timer0_sync_sw bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_synco_sel(&mut self) -> TIMER0_SYNCO_SEL_W<TIMER0_SYNC_SPEC> {
        TIMER0_SYNCO_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:19 - phase for timer reload on sync event"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_phase(&mut self) -> TIMER0_PHASE_W<TIMER0_SYNC_SPEC> {
        TIMER0_PHASE_W::new(self, 4)
    }
    #[doc = "Bit 20 - Configure the PWM timer0's direction when timer0 mode is up-down mode: 0-increase,1-decrease"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_phase_direction(&mut self) -> TIMER0_PHASE_DIRECTION_W<TIMER0_SYNC_SPEC> {
        TIMER0_PHASE_DIRECTION_W::new(self, 20)
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
#[doc = "PWM timer0 sync function configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER0_SYNC_SPEC;
impl crate::RegisterSpec for TIMER0_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0_sync::R`](R) reader structure"]
impl crate::Readable for TIMER0_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer0_sync::W`](W) writer structure"]
impl crate::Writable for TIMER0_SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER0_SYNC to value 0"]
impl crate::Resettable for TIMER0_SYNC_SPEC {
    const RESET_VALUE: u32 = 0;
}
