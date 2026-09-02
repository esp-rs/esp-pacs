#[doc = "Register `PMT_RWUFFR` reader"]
pub type R = crate::R<PMT_RWUFFR_SPEC>;
#[doc = "Field `WKUPFRM_FILTER` reader - This is the address through which the application writes or reads the remote wake-up frame filter registers.The reg_wkupfrm_filter register is a pointer to eight reg_wkupfrm_filter registers.The reg_wkupfrm_filter register is loaded by sequentially loading the eight register values.Eight sequential writes to this address(0x0028)write all reg_wkupfrm_filter registers.Similarly, eight sequential reads from this address(0x0028) read all reg_wkupfrm_filter registers. This register is present only when you select the PMT module Remote Wake-Up feature in coreConsultant."]
pub type WKUPFRM_FILTER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This is the address through which the application writes or reads the remote wake-up frame filter registers.The reg_wkupfrm_filter register is a pointer to eight reg_wkupfrm_filter registers.The reg_wkupfrm_filter register is loaded by sequentially loading the eight register values.Eight sequential writes to this address(0x0028)write all reg_wkupfrm_filter registers.Similarly, eight sequential reads from this address(0x0028) read all reg_wkupfrm_filter registers. This register is present only when you select the PMT module Remote Wake-Up feature in coreConsultant."]
    #[inline(always)]
    pub fn wkupfrm_filter(&self) -> WKUPFRM_FILTER_R {
        WKUPFRM_FILTER_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMT_RWUFFR")
            .field("wkupfrm_filter", &self.wkupfrm_filter())
            .finish()
    }
}
#[doc = "Remote Wake-Up Frame Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmt_rwuffr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMT_RWUFFR_SPEC;
impl crate::RegisterSpec for PMT_RWUFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmt_rwuffr::R`](R) reader structure"]
impl crate::Readable for PMT_RWUFFR_SPEC {}
#[doc = "`reset()` method sets PMT_RWUFFR to value 0"]
impl crate::Resettable for PMT_RWUFFR_SPEC {}
