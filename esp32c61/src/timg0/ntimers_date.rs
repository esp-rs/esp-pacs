#[doc = "Register `NTIMERS_DATE` reader"]
pub type R = crate::R<NTIMERS_DATE_SPEC>;
#[doc = "Register `NTIMERS_DATE` writer"]
pub type W = crate::W<NTIMERS_DATE_SPEC>;
#[doc = "Field `NTIMGS_DATE` reader - Version control register"]
pub type NTIMGS_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `NTIMGS_DATE` writer - Version control register"]
pub type NTIMGS_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Version control register"]
    #[inline(always)]
    pub fn ntimgs_date(&self) -> NTIMGS_DATE_R {
        NTIMGS_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NTIMERS_DATE")
            .field("ntimgs_date", &self.ntimgs_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Version control register"]
    #[inline(always)]
    pub fn ntimgs_date(&mut self) -> NTIMGS_DATE_W<NTIMERS_DATE_SPEC> {
        NTIMGS_DATE_W::new(self, 0)
    }
}
#[doc = "Timer version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ntimers_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ntimers_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NTIMERS_DATE_SPEC;
impl crate::RegisterSpec for NTIMERS_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ntimers_date::R`](R) reader structure"]
impl crate::Readable for NTIMERS_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ntimers_date::W`](W) writer structure"]
impl crate::Writable for NTIMERS_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NTIMERS_DATE to value 0x0220_9142"]
impl crate::Resettable for NTIMERS_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0220_9142;
}
