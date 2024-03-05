#[doc = "Register `REGION_PMS_CONSTRAIN_6` reader"]
pub type R = crate::R<REGION_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Register `REGION_PMS_CONSTRAIN_6` writer"]
pub type W = crate::W<REGION_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Field `REGION_PMS_CONSTRAIN_ADDR_3` reader - region_pms_constrain_addr_3"]
pub type REGION_PMS_CONSTRAIN_ADDR_3_R = crate::FieldReader<u32>;
#[doc = "Field `REGION_PMS_CONSTRAIN_ADDR_3` writer - region_pms_constrain_addr_3"]
pub type REGION_PMS_CONSTRAIN_ADDR_3_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - region_pms_constrain_addr_3"]
    #[inline(always)]
    pub fn region_pms_constrain_addr_3(&self) -> REGION_PMS_CONSTRAIN_ADDR_3_R {
        REGION_PMS_CONSTRAIN_ADDR_3_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_PMS_CONSTRAIN_6")
            .field(
                "region_pms_constrain_addr_3",
                &format_args!("{}", self.region_pms_constrain_addr_3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION_PMS_CONSTRAIN_6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:29 - region_pms_constrain_addr_3"]
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_addr_3(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_ADDR_3_W<REGION_PMS_CONSTRAIN_6_SPEC> {
        REGION_PMS_CONSTRAIN_ADDR_3_W::new(self, 0)
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_6_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region_pms_constrain_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region_pms_constrain_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION_PMS_CONSTRAIN_6_SPEC;
impl crate::RegisterSpec for REGION_PMS_CONSTRAIN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_pms_constrain_6::R`](R) reader structure"]
impl crate::Readable for REGION_PMS_CONSTRAIN_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region_pms_constrain_6::W`](W) writer structure"]
impl crate::Writable for REGION_PMS_CONSTRAIN_6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGION_PMS_CONSTRAIN_6 to value 0"]
impl crate::Resettable for REGION_PMS_CONSTRAIN_6_SPEC {
    const RESET_VALUE: u32 = 0;
}
