#[doc = "Register `VERID_FILEDS` reader"]
pub type R = crate::R<VERID_FILEDS_SPEC>;
#[doc = "Field `ICM_REG_VERID` reader - NA"]
pub type ICM_REG_VERID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn icm_reg_verid(&self) -> ICM_REG_VERID_R {
        ICM_REG_VERID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERID_FILEDS")
            .field(
                "icm_reg_verid",
                &format_args!("{}", self.icm_reg_verid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VERID_FILEDS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verid_fileds::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERID_FILEDS_SPEC;
impl crate::RegisterSpec for VERID_FILEDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verid_fileds::R`](R) reader structure"]
impl crate::Readable for VERID_FILEDS_SPEC {}
#[doc = "`reset()` method sets VERID_FILEDS to value 0x3430_342a"]
impl crate::Resettable for VERID_FILEDS_SPEC {
    const RESET_VALUE: Self::Ux = 0x3430_342a;
}
