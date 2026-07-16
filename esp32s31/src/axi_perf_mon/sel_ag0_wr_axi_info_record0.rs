#[doc = "Register `SEL_AG0_WR_AXI_INFO_RECORD0` reader"]
pub type R = crate::R<SEL_AG0_WR_AXI_INFO_RECORD0_SPEC>;
#[doc = "Field `SEL_AG0_WR_AXI_INFO_RECORD0` reader - The latest x axi transaction information record for sel agent, \\[7:0\\] for AWLEN, \\[23:8\\] for AWID"]
pub type SEL_AG0_WR_AXI_INFO_RECORD0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The latest x axi transaction information record for sel agent, \\[7:0\\] for AWLEN, \\[23:8\\] for AWID"]
    #[inline(always)]
    pub fn sel_ag0_wr_axi_info_record0(&self) -> SEL_AG0_WR_AXI_INFO_RECORD0_R {
        SEL_AG0_WR_AXI_INFO_RECORD0_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG0_WR_AXI_INFO_RECORD0")
            .field(
                "sel_ag0_wr_axi_info_record0",
                &self.sel_ag0_wr_axi_info_record0(),
            )
            .finish()
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_axi_info_record0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG0_WR_AXI_INFO_RECORD0_SPEC;
impl crate::RegisterSpec for SEL_AG0_WR_AXI_INFO_RECORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag0_wr_axi_info_record0::R`](R) reader structure"]
impl crate::Readable for SEL_AG0_WR_AXI_INFO_RECORD0_SPEC {}
#[doc = "`reset()` method sets SEL_AG0_WR_AXI_INFO_RECORD0 to value 0"]
impl crate::Resettable for SEL_AG0_WR_AXI_INFO_RECORD0_SPEC {}
