#[doc = "Register `SP_PC` reader"]
pub type R = crate::R<SP_PC_SPEC>;
#[doc = "Field `CORE_0_SP_PC` reader - reg_core_0_sp_pc"]
pub type CORE_0_SP_PC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_sp_pc"]
    #[inline(always)]
    pub fn core_0_sp_pc(&self) -> CORE_0_SP_PC_R {
        CORE_0_SP_PC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SP_PC")
            .field("core_0_sp_pc", &self.core_0_sp_pc())
            .finish()
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_SP_PC_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_pc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SP_PC_SPEC;
impl crate::RegisterSpec for SP_PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_pc::R`](R) reader structure"]
impl crate::Readable for SP_PC_SPEC {}
#[doc = "`reset()` method sets SP_PC to value 0"]
impl crate::Resettable for SP_PC_SPEC {}
