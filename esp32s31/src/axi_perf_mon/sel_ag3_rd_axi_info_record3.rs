#[doc = "Register `SEL_AG3_RD_AXI_INFO_RECORD3` reader"]
pub type R = crate::R<SEL_AG3_RD_AXI_INFO_RECORD3_SPEC>;
#[doc = "Field `SEL_AG3_RD_AXI_INFO_RECORD3` reader - The latest x axi transaction information record for sel agent, \\[7:0\\] for ARLEN, \\[23:8\\] for ARID"]
pub type SEL_AG3_RD_AXI_INFO_RECORD3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The latest x axi transaction information record for sel agent, \\[7:0\\] for ARLEN, \\[23:8\\] for ARID"]
    #[inline(always)]
    pub fn sel_ag3_rd_axi_info_record3(&self) -> SEL_AG3_RD_AXI_INFO_RECORD3_R {
        SEL_AG3_RD_AXI_INFO_RECORD3_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG3_RD_AXI_INFO_RECORD3")
            .field(
                "sel_ag3_rd_axi_info_record3",
                &self.sel_ag3_rd_axi_info_record3(),
            )
            .finish()
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_axi_info_record3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG3_RD_AXI_INFO_RECORD3_SPEC;
impl crate::RegisterSpec for SEL_AG3_RD_AXI_INFO_RECORD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag3_rd_axi_info_record3::R`](R) reader structure"]
impl crate::Readable for SEL_AG3_RD_AXI_INFO_RECORD3_SPEC {}
#[doc = "`reset()` method sets SEL_AG3_RD_AXI_INFO_RECORD3 to value 0"]
impl crate::Resettable for SEL_AG3_RD_AXI_INFO_RECORD3_SPEC {}
