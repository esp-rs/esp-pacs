#[doc = "Register `POLL_PERIOD` reader"]
pub type R = crate::R<POLL_PERIOD_SPEC>;
#[doc = "Register `POLL_PERIOD` writer"]
pub type W = crate::W<POLL_PERIOD_SPEC>;
#[doc = "Field `COMP_POLL_PERIOD` reader - poll period time for each cahnnel,it must be greater than or equal to 1"]
pub type COMP_POLL_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `COMP_POLL_PERIOD` writer - poll period time for each cahnnel,it must be greater than or equal to 1"]
pub type COMP_POLL_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - poll period time for each cahnnel,it must be greater than or equal to 1"]
    #[inline(always)]
    pub fn comp_poll_period(&self) -> COMP_POLL_PERIOD_R {
        COMP_POLL_PERIOD_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POLL_PERIOD")
            .field("comp_poll_period", &self.comp_poll_period())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - poll period time for each cahnnel,it must be greater than or equal to 1"]
    #[inline(always)]
    pub fn comp_poll_period(&mut self) -> COMP_POLL_PERIOD_W<'_, POLL_PERIOD_SPEC> {
        COMP_POLL_PERIOD_W::new(self, 0)
    }
}
#[doc = "poll period time reg\n\nYou can [`read`](crate::Reg::read) this register and get [`poll_period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poll_period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLL_PERIOD_SPEC;
impl crate::RegisterSpec for POLL_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`poll_period::R`](R) reader structure"]
impl crate::Readable for POLL_PERIOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`poll_period::W`](W) writer structure"]
impl crate::Writable for POLL_PERIOD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POLL_PERIOD to value 0x0f"]
impl crate::Resettable for POLL_PERIOD_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
