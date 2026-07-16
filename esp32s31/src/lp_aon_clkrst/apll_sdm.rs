#[doc = "Register `APLL_SDM` reader"]
pub type R = crate::R<APLL_SDM_SPEC>;
#[doc = "Register `APLL_SDM` writer"]
pub type W = crate::W<APLL_SDM_SPEC>;
#[doc = "Field `APLL_SDM` reader - apll sdm value"]
pub type APLL_SDM_R = crate::FieldReader<u32>;
#[doc = "Field `APLL_SDM` writer - apll sdm value"]
pub type APLL_SDM_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - apll sdm value"]
    #[inline(always)]
    pub fn apll_sdm(&self) -> APLL_SDM_R {
        APLL_SDM_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APLL_SDM")
            .field("apll_sdm", &self.apll_sdm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - apll sdm value"]
    #[inline(always)]
    pub fn apll_sdm(&mut self) -> APLL_SDM_W<'_, APLL_SDM_SPEC> {
        APLL_SDM_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`apll_sdm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apll_sdm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APLL_SDM_SPEC;
impl crate::RegisterSpec for APLL_SDM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apll_sdm::R`](R) reader structure"]
impl crate::Readable for APLL_SDM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apll_sdm::W`](W) writer structure"]
impl crate::Writable for APLL_SDM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APLL_SDM to value 0"]
impl crate::Resettable for APLL_SDM_SPEC {}
