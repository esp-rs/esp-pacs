#[doc = "Register `SEL_AG5_INS_BANDW_DATA0` reader"]
pub type R = crate::R<SEL_AG5_INS_BANDW_DATA0_SPEC>;
#[doc = "Field `SEL_AG5_INS_BANDW_DATA0` reader - The latest x bandwidth date num in config time record for sel agent, \\[15:0\\] for Read and \\[31:16\\] for write"]
pub type SEL_AG5_INS_BANDW_DATA0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The latest x bandwidth date num in config time record for sel agent, \\[15:0\\] for Read and \\[31:16\\] for write"]
    #[inline(always)]
    pub fn sel_ag5_ins_bandw_data0(&self) -> SEL_AG5_INS_BANDW_DATA0_R {
        SEL_AG5_INS_BANDW_DATA0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG5_INS_BANDW_DATA0")
            .field("sel_ag5_ins_bandw_data0", &self.sel_ag5_ins_bandw_data0())
            .finish()
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_ins_bandw_data0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG5_INS_BANDW_DATA0_SPEC;
impl crate::RegisterSpec for SEL_AG5_INS_BANDW_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag5_ins_bandw_data0::R`](R) reader structure"]
impl crate::Readable for SEL_AG5_INS_BANDW_DATA0_SPEC {}
#[doc = "`reset()` method sets SEL_AG5_INS_BANDW_DATA0 to value 0"]
impl crate::Resettable for SEL_AG5_INS_BANDW_DATA0_SPEC {}
