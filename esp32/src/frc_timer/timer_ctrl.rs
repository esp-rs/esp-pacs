#[doc = "Register `TIMER_CTRL` reader"]
pub type R = crate::R<TIMER_CTRL_SPEC>;
#[doc = "Register `TIMER_CTRL` writer"]
pub type W = crate::W<TIMER_CTRL_SPEC>;
#[doc = "Field `TIMER_PRESCALER` reader - "]
pub type TIMER_PRESCALER_R = crate::FieldReader;
#[doc = "Field `TIMER_PRESCALER` writer - "]
pub type TIMER_PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 1:8"]
    #[inline(always)]
    pub fn timer_prescaler(&self) -> TIMER_PRESCALER_R {
        TIMER_PRESCALER_R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CTRL")
            .field("timer_prescaler", &self.timer_prescaler())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:8"]
    #[inline(always)]
    #[must_use]
    pub fn timer_prescaler(&mut self) -> TIMER_PRESCALER_W<TIMER_CTRL_SPEC> {
        TIMER_PRESCALER_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_CTRL_SPEC;
impl crate::RegisterSpec for TIMER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_ctrl::R`](R) reader structure"]
impl crate::Readable for TIMER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_ctrl::W`](W) writer structure"]
impl crate::Writable for TIMER_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_CTRL to value 0"]
impl crate::Resettable for TIMER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
