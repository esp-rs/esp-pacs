#[doc = "Register `SEL_AG2_RD_AXI_INFO_RECORD1` reader"]
pub type R = crate::R<SEL_AG2_RD_AXI_INFO_RECORD1_SPEC>;
#[doc = "Field `SEL_AG2_RD_AXI_INFO_RECORD1` reader - The latest x axi transaction information record for sel agent, \\[7:0\\] for ARLEN, \\[23:8\\] for ARID"]
pub type SEL_AG2_RD_AXI_INFO_RECORD1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The latest x axi transaction information record for sel agent, \\[7:0\\] for ARLEN, \\[23:8\\] for ARID"]
    #[inline(always)]
    pub fn sel_ag2_rd_axi_info_record1(&self) -> SEL_AG2_RD_AXI_INFO_RECORD1_R {
        SEL_AG2_RD_AXI_INFO_RECORD1_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG2_RD_AXI_INFO_RECORD1")
            .field(
                "sel_ag2_rd_axi_info_record1",
                &self.sel_ag2_rd_axi_info_record1(),
            )
            .finish()
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_axi_info_record1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG2_RD_AXI_INFO_RECORD1_SPEC;
impl crate::RegisterSpec for SEL_AG2_RD_AXI_INFO_RECORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag2_rd_axi_info_record1::R`](R) reader structure"]
impl crate::Readable for SEL_AG2_RD_AXI_INFO_RECORD1_SPEC {}
#[doc = "`reset()` method sets SEL_AG2_RD_AXI_INFO_RECORD1 to value 0"]
impl crate::Resettable for SEL_AG2_RD_AXI_INFO_RECORD1_SPEC {}
