#[doc = "Register `CORE_1_AREA_PC` reader"]
pub type R = crate::R<CORE_1_AREA_PC_SPEC>;
#[doc = "Field `CORE_1_AREA_PC` reader - Represents the PC value when an interrupt is triggered during region monitoring."]
pub type CORE_1_AREA_PC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the PC value when an interrupt is triggered during region monitoring."]
    #[inline(always)]
    pub fn core_1_area_pc(&self) -> CORE_1_AREA_PC_R {
        CORE_1_AREA_PC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_AREA_PC")
            .field("core_1_area_pc", &self.core_1_area_pc())
            .finish()
    }
}
#[doc = "Region monitoring HP CPU PC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_area_pc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_AREA_PC_SPEC;
impl crate::RegisterSpec for CORE_1_AREA_PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_area_pc::R`](R) reader structure"]
impl crate::Readable for CORE_1_AREA_PC_SPEC {}
#[doc = "`reset()` method sets CORE_1_AREA_PC to value 0"]
impl crate::Resettable for CORE_1_AREA_PC_SPEC {}
