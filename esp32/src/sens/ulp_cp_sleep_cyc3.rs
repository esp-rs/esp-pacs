#[doc = "Register `ULP_CP_SLEEP_CYC3` reader"]
pub type R = crate::R<ULP_CP_SLEEP_CYC3_SPEC>;
#[doc = "Register `ULP_CP_SLEEP_CYC3` writer"]
pub type W = crate::W<ULP_CP_SLEEP_CYC3_SPEC>;
#[doc = "Field `SLEEP_CYCLES_S3` reader - "]
pub type SLEEP_CYCLES_S3_R = crate::FieldReader<u32>;
#[doc = "Field `SLEEP_CYCLES_S3` writer - "]
pub type SLEEP_CYCLES_S3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sleep_cycles_s3(&self) -> SLEEP_CYCLES_S3_R {
        SLEEP_CYCLES_S3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_SLEEP_CYC3")
            .field("sleep_cycles_s3", &self.sleep_cycles_s3().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ULP_CP_SLEEP_CYC3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_cycles_s3(&mut self) -> SLEEP_CYCLES_S3_W<ULP_CP_SLEEP_CYC3_SPEC> {
        SLEEP_CYCLES_S3_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_sleep_cyc3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_sleep_cyc3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ULP_CP_SLEEP_CYC3_SPEC;
impl crate::RegisterSpec for ULP_CP_SLEEP_CYC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ulp_cp_sleep_cyc3::R`](R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ulp_cp_sleep_cyc3::W`](W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ULP_CP_SLEEP_CYC3 to value 0x28"]
impl crate::Resettable for ULP_CP_SLEEP_CYC3_SPEC {
    const RESET_VALUE: u32 = 0x28;
}
