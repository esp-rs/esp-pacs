#[doc = "Register `TIMER4` reader"]
pub type R = crate::R<TIMER4_SPEC>;
#[doc = "Register `TIMER4` writer"]
pub type W = crate::W<TIMER4_SPEC>;
#[doc = "Field `WAIT_TIMER` reader - "]
pub type WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `WAIT_TIMER` writer - "]
pub type WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `POWERUP_TIMER` reader - "]
pub type POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `POWERUP_TIMER` writer - "]
pub type POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DG_WRAP_WAIT_TIMER` reader - "]
pub type DG_WRAP_WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `DG_WRAP_WAIT_TIMER` writer - "]
pub type DG_WRAP_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DG_WRAP_POWERUP_TIMER` reader - "]
pub type DG_WRAP_POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_WRAP_POWERUP_TIMER` writer - "]
pub type DG_WRAP_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
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
            .field("wait_timer", &self.wait_timer())
            .field("powerup_timer", &self.powerup_timer())
            .field("dg_wrap_wait_timer", &self.dg_wrap_wait_timer())
            .field("dg_wrap_powerup_timer", &self.dg_wrap_powerup_timer())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn wait_timer(&mut self) -> WAIT_TIMER_W<TIMER4_SPEC> {
        WAIT_TIMER_W::new(self, 0)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    #[must_use]
    pub fn powerup_timer(&mut self) -> POWERUP_TIMER_W<TIMER4_SPEC> {
        POWERUP_TIMER_W::new(self, 9)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_wait_timer(&mut self) -> DG_WRAP_WAIT_TIMER_W<TIMER4_SPEC> {
        DG_WRAP_WAIT_TIMER_W::new(self, 16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_powerup_timer(&mut self) -> DG_WRAP_POWERUP_TIMER_W<TIMER4_SPEC> {
        DG_WRAP_POWERUP_TIMER_W::new(self, 25)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER4 to value 0x1020_0a08"]
impl crate::Resettable for TIMER4_SPEC {
    const RESET_VALUE: u32 = 0x1020_0a08;
}
