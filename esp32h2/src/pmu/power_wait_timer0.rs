#[doc = "Register `POWER_WAIT_TIMER0` reader"]
pub struct R(crate::R<POWER_WAIT_TIMER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_WAIT_TIMER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_WAIT_TIMER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_WAIT_TIMER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_WAIT_TIMER0` writer"]
pub struct W(crate::W<POWER_WAIT_TIMER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_WAIT_TIMER0_SPEC>;
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
impl From<crate::W<POWER_WAIT_TIMER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_WAIT_TIMER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DG_HP_POWERDOWN_TIMER` reader - need_des"]
pub type DG_HP_POWERDOWN_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DG_HP_POWERDOWN_TIMER` writer - need_des"]
pub type DG_HP_POWERDOWN_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_WAIT_TIMER0_SPEC, 9, O, u16, u16>;
#[doc = "Field `DG_HP_POWERUP_TIMER` reader - need_des"]
pub type DG_HP_POWERUP_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DG_HP_POWERUP_TIMER` writer - need_des"]
pub type DG_HP_POWERUP_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_WAIT_TIMER0_SPEC, 9, O, u16, u16>;
#[doc = "Field `DG_HP_WAIT_TIMER` reader - need_des"]
pub type DG_HP_WAIT_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DG_HP_WAIT_TIMER` writer - need_des"]
pub type DG_HP_WAIT_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_WAIT_TIMER0_SPEC, 9, O, u16, u16>;
impl R {
    #[doc = "Bits 5:13 - need_des"]
    #[inline(always)]
    pub fn dg_hp_powerdown_timer(&self) -> DG_HP_POWERDOWN_TIMER_R {
        DG_HP_POWERDOWN_TIMER_R::new(((self.bits >> 5) & 0x01ff) as u16)
    }
    #[doc = "Bits 14:22 - need_des"]
    #[inline(always)]
    pub fn dg_hp_powerup_timer(&self) -> DG_HP_POWERUP_TIMER_R {
        DG_HP_POWERUP_TIMER_R::new(((self.bits >> 14) & 0x01ff) as u16)
    }
    #[doc = "Bits 23:31 - need_des"]
    #[inline(always)]
    pub fn dg_hp_wait_timer(&self) -> DG_HP_WAIT_TIMER_R {
        DG_HP_WAIT_TIMER_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_WAIT_TIMER0")
            .field(
                "dg_hp_powerdown_timer",
                &format_args!("{}", self.dg_hp_powerdown_timer().bits()),
            )
            .field(
                "dg_hp_powerup_timer",
                &format_args!("{}", self.dg_hp_powerup_timer().bits()),
            )
            .field(
                "dg_hp_wait_timer",
                &format_args!("{}", self.dg_hp_wait_timer().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_WAIT_TIMER0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 5:13 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dg_hp_powerdown_timer(&mut self) -> DG_HP_POWERDOWN_TIMER_W<5> {
        DG_HP_POWERDOWN_TIMER_W::new(self)
    }
    #[doc = "Bits 14:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dg_hp_powerup_timer(&mut self) -> DG_HP_POWERUP_TIMER_W<14> {
        DG_HP_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Bits 23:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dg_hp_wait_timer(&mut self) -> DG_HP_WAIT_TIMER_W<23> {
        DG_HP_WAIT_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_wait_timer0](index.html) module"]
pub struct POWER_WAIT_TIMER0_SPEC;
impl crate::RegisterSpec for POWER_WAIT_TIMER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_wait_timer0::R](R) reader structure"]
impl crate::Readable for POWER_WAIT_TIMER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_wait_timer0::W](W) writer structure"]
impl crate::Writable for POWER_WAIT_TIMER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_WAIT_TIMER0 to value 0x7fbf_dfe0"]
impl crate::Resettable for POWER_WAIT_TIMER0_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fbf_dfe0;
}
