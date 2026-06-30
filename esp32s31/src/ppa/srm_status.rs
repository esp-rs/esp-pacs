#[doc = "Register `SRM_STATUS` reader"]
pub type R = crate::R<SRM_STATUS_SPEC>;
#[doc = "Field `SRM_RX_DSCR_SAMPLE_STATE` reader - Reserved."]
pub type SRM_RX_DSCR_SAMPLE_STATE_R = crate::FieldReader;
#[doc = "Field `SRM_RX_SCAN_STATE` reader - Reserved."]
pub type SRM_RX_SCAN_STATE_R = crate::FieldReader;
#[doc = "Field `SRM_TX_DSCR_SAMPLE_STATE` reader - Reserved."]
pub type SRM_TX_DSCR_SAMPLE_STATE_R = crate::FieldReader;
#[doc = "Field `SRM_TX_SCAN_STATE` reader - Reserved."]
pub type SRM_TX_SCAN_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Reserved."]
    #[inline(always)]
    pub fn srm_rx_dscr_sample_state(&self) -> SRM_RX_DSCR_SAMPLE_STATE_R {
        SRM_RX_DSCR_SAMPLE_STATE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Reserved."]
    #[inline(always)]
    pub fn srm_rx_scan_state(&self) -> SRM_RX_SCAN_STATE_R {
        SRM_RX_SCAN_STATE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Reserved."]
    #[inline(always)]
    pub fn srm_tx_dscr_sample_state(&self) -> SRM_TX_DSCR_SAMPLE_STATE_R {
        SRM_TX_DSCR_SAMPLE_STATE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:8 - Reserved."]
    #[inline(always)]
    pub fn srm_tx_scan_state(&self) -> SRM_TX_SCAN_STATE_R {
        SRM_TX_SCAN_STATE_R::new(((self.bits >> 6) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRM_STATUS")
            .field("srm_rx_dscr_sample_state", &self.srm_rx_dscr_sample_state())
            .field("srm_rx_scan_state", &self.srm_rx_scan_state())
            .field("srm_tx_dscr_sample_state", &self.srm_tx_dscr_sample_state())
            .field("srm_tx_scan_state", &self.srm_tx_scan_state())
            .finish()
    }
}
#[doc = "SR FSM register\n\nYou can [`read`](crate::Reg::read) this register and get [`srm_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRM_STATUS_SPEC;
impl crate::RegisterSpec for SRM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srm_status::R`](R) reader structure"]
impl crate::Readable for SRM_STATUS_SPEC {}
#[doc = "`reset()` method sets SRM_STATUS to value 0"]
impl crate::Resettable for SRM_STATUS_SPEC {}
