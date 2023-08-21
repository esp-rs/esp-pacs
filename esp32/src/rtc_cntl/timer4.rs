#[doc = "Register `TIMER4` reader"]
pub type R = crate::R<TIMER4_SPEC>;
#[doc = "Register `TIMER4` writer"]
pub type W = crate::W<TIMER4_SPEC>;
#[doc = "Field `WAIT_TIMER` reader - "]
pub type WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `WAIT_TIMER` writer - "]
pub type WAIT_TIMER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `POWERUP_TIMER` reader - "]
pub type POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `POWERUP_TIMER` writer - "]
pub type POWERUP_TIMER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DG_WRAP_WAIT_TIMER` reader - "]
pub type DG_WRAP_WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `DG_WRAP_WAIT_TIMER` writer - "]
pub type DG_WRAP_WAIT_TIMER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `DG_WRAP_POWERUP_TIMER` reader - "]
pub type DG_WRAP_POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_WRAP_POWERUP_TIMER` writer - "]
pub type DG_WRAP_POWERUP_TIMER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn wait_timer(&self) -> WAIT_TIMER_R {
        WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn powerup_timer(&self) -> POWERUP_TIMER_R {
        POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&self) -> DG_WRAP_WAIT_TIMER_R {
        DG_WRAP_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&self) -> DG_WRAP_POWERUP_TIMER_R {
        DG_WRAP_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER4")
            .field("wait_timer", &format_args!("{}", self.wait_timer().bits()))
            .field(
                "powerup_timer",
                &format_args!("{}", self.powerup_timer().bits()),
            )
            .field(
                "dg_wrap_wait_timer",
                &format_args!("{}", self.dg_wrap_wait_timer().bits()),
            )
            .field(
                "dg_wrap_powerup_timer",
                &format_args!("{}", self.dg_wrap_powerup_timer().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn wait_timer(&mut self) -> WAIT_TIMER_W<TIMER4_SPEC, 0> {
        WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    #[must_use]
    pub fn powerup_timer(&mut self) -> POWERUP_TIMER_W<TIMER4_SPEC, 9> {
        POWERUP_TIMER_W::new(self)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_wait_timer(&mut self) -> DG_WRAP_WAIT_TIMER_W<TIMER4_SPEC, 16> {
        DG_WRAP_WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_powerup_timer(&mut self) -> DG_WRAP_POWERUP_TIMER_W<TIMER4_SPEC, 25> {
        DG_WRAP_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER4_SPEC;
impl crate::RegisterSpec for TIMER4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer4::R`](R) reader structure"]
impl crate::Readable for TIMER4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer4::W`](W) writer structure"]
impl crate::Writable for TIMER4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER4 to value 0x1020_0a08"]
impl crate::Resettable for TIMER4_SPEC {
    const RESET_VALUE: Self::Ux = 0x1020_0a08;
}
