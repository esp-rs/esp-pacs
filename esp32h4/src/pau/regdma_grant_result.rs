#[doc = "Register `REGDMA_GRANT_RESULT` reader"]
pub type R = crate::R<REGDMA_GRANT_RESULT_SPEC>;
#[doc = "Register `REGDMA_GRANT_RESULT` writer"]
pub type W = crate::W<REGDMA_GRANT_RESULT_SPEC>;
#[doc = "Field `GRANT_START_RESULT` reader - Grant start result"]
pub type GRANT_START_RESULT_R = crate::FieldReader;
#[doc = "Field `GRANT_DONE_RESULT` reader - Grant done result"]
pub type GRANT_DONE_RESULT_R = crate::FieldReader;
#[doc = "Field `GRANT_RESULT_CLR` writer - Grant result clear"]
pub type GRANT_RESULT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Grant start result"]
    #[inline(always)]
    pub fn grant_start_result(&self) -> GRANT_START_RESULT_R {
        GRANT_START_RESULT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Grant done result"]
    #[inline(always)]
    pub fn grant_done_result(&self) -> GRANT_DONE_RESULT_R {
        GRANT_DONE_RESULT_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_GRANT_RESULT")
            .field("grant_start_result", &self.grant_start_result())
            .field("grant_done_result", &self.grant_done_result())
            .finish()
    }
}
impl W {
    #[doc = "Bit 14 - Grant result clear"]
    #[inline(always)]
    pub fn grant_result_clr(&mut self) -> GRANT_RESULT_CLR_W<'_, REGDMA_GRANT_RESULT_SPEC> {
        GRANT_RESULT_CLR_W::new(self, 14)
    }
}
#[doc = "Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_grant_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_grant_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_GRANT_RESULT_SPEC;
impl crate::RegisterSpec for REGDMA_GRANT_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_grant_result::R`](R) reader structure"]
impl crate::Readable for REGDMA_GRANT_RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regdma_grant_result::W`](W) writer structure"]
impl crate::Writable for REGDMA_GRANT_RESULT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGDMA_GRANT_RESULT to value 0"]
impl crate::Resettable for REGDMA_GRANT_RESULT_SPEC {}
