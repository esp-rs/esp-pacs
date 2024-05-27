#[doc = "Register `TIMER4` reader"]
pub type R = crate::R<TIMER4_SPEC>;
#[doc = "Register `TIMER4` writer"]
pub type W = crate::W<TIMER4_SPEC>;
#[doc = "Field `DG_WRAP_WAIT_TIMER` reader - Need add desc"]
pub type DG_WRAP_WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `DG_WRAP_WAIT_TIMER` writer - Need add desc"]
pub type DG_WRAP_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DG_WRAP_POWERUP_TIMER` reader - Need add desc"]
pub type DG_WRAP_POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_WRAP_POWERUP_TIMER` writer - Need add desc"]
pub type DG_WRAP_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 16:24 - Need add desc"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&self) -> DG_WRAP_WAIT_TIMER_R {
        DG_WRAP_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Need add desc"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&self) -> DG_WRAP_POWERUP_TIMER_R {
        DG_WRAP_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER4")
            .field("dg_wrap_wait_timer", &self.dg_wrap_wait_timer())
            .field("dg_wrap_powerup_timer", &self.dg_wrap_powerup_timer())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:24 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_wait_timer(&mut self) -> DG_WRAP_WAIT_TIMER_W<TIMER4_SPEC> {
        DG_WRAP_WAIT_TIMER_W::new(self, 16)
    }
    #[doc = "Bits 25:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_powerup_timer(&mut self) -> DG_WRAP_POWERUP_TIMER_W<TIMER4_SPEC> {
        DG_WRAP_POWERUP_TIMER_W::new(self, 25)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets TIMER4 to value 0x1020_0000"]
impl crate::Resettable for TIMER4_SPEC {
    const RESET_VALUE: u32 = 0x1020_0000;
}
