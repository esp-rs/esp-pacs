///Register `CORE_0_AREA_SP` reader
pub type R = crate::R<CORE_0_AREA_SP_SPEC>;
///Field `CORE_0_AREA_SP` reader - reg_core_0_area_sp
pub type CORE_0_AREA_SP_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - reg_core_0_area_sp
    #[inline(always)]
    pub fn core_0_area_sp(&self) -> CORE_0_AREA_SP_R {
        CORE_0_AREA_SP_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_AREA_SP")
            .field("core_0_area_sp", &self.core_0_area_sp())
            .finish()
    }
}
/**ASSIST_DEBUG_CORE_0_AREA_SP_REG

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_sp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_AREA_SP_SPEC;
impl crate::RegisterSpec for CORE_0_AREA_SP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_0_area_sp::R`](R) reader structure
impl crate::Readable for CORE_0_AREA_SP_SPEC {}
///`reset()` method sets CORE_0_AREA_SP to value 0
impl crate::Resettable for CORE_0_AREA_SP_SPEC {
    const RESET_VALUE: u32 = 0;
}
