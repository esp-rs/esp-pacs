#[doc = "Register `LACTALARMLO` reader"]
pub type R = crate::R<LACTALARMLO_SPEC>;
#[doc = "Register `LACTALARMLO` writer"]
pub type W = crate::W<LACTALARMLO_SPEC>;
#[doc = "Field `ALARM_LO` reader - "]
pub type ALARM_LO_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_LO` writer - "]
pub type ALARM_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn alarm_lo(&self) -> ALARM_LO_R {
        ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTALARMLO")
            .field("alarm_lo", &self.alarm_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn alarm_lo(&mut self) -> ALARM_LO_W<'_, LACTALARMLO_SPEC> {
        ALARM_LO_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lactalarmlo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactalarmlo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTALARMLO_SPEC;
impl crate::RegisterSpec for LACTALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactalarmlo::R`](R) reader structure"]
impl crate::Readable for LACTALARMLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactalarmlo::W`](W) writer structure"]
impl crate::Writable for LACTALARMLO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LACTALARMLO to value 0"]
impl crate::Resettable for LACTALARMLO_SPEC {}
