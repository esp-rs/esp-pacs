#[doc = "Register `SEL_AG3_INS_BANDW_DATA3` reader"]
pub type R = crate::R<SEL_AG3_INS_BANDW_DATA3_SPEC>;
#[doc = "Field `SEL_AG3_INS_BANDW_DATA3` reader - The latest x bandwidth date num in config time record for sel agent, \\[15:0\\] for Read and \\[31:16\\] for write"]
pub type SEL_AG3_INS_BANDW_DATA3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The latest x bandwidth date num in config time record for sel agent, \\[15:0\\] for Read and \\[31:16\\] for write"]
    #[inline(always)]
    pub fn sel_ag3_ins_bandw_data3(&self) -> SEL_AG3_INS_BANDW_DATA3_R {
        SEL_AG3_INS_BANDW_DATA3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG3_INS_BANDW_DATA3")
            .field("sel_ag3_ins_bandw_data3", &self.sel_ag3_ins_bandw_data3())
            .finish()
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_ins_bandw_data3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG3_INS_BANDW_DATA3_SPEC;
impl crate::RegisterSpec for SEL_AG3_INS_BANDW_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag3_ins_bandw_data3::R`](R) reader structure"]
impl crate::Readable for SEL_AG3_INS_BANDW_DATA3_SPEC {}
#[doc = "`reset()` method sets SEL_AG3_INS_BANDW_DATA3 to value 0"]
impl crate::Resettable for SEL_AG3_INS_BANDW_DATA3_SPEC {}
