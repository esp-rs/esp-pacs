#[doc = "Register `AHBINF_RESP_ERR_STATUS1` reader"]
pub type R = crate::R<AHBINF_RESP_ERR_STATUS1_SPEC>;
#[doc = "Field `AHBINF_RESP_ERR_WR` reader - Represents the AHB response error is write request."]
pub type AHBINF_RESP_ERR_WR_R = crate::BitReader;
#[doc = "Field `AHBINF_RESP_ERR_ID` reader - Represents the AHB response error request id."]
pub type AHBINF_RESP_ERR_ID_R = crate::FieldReader;
#[doc = "Field `AHBINF_RESP_ERR_CH_ID` reader - Represents the AHB response error request channel id."]
pub type AHBINF_RESP_ERR_CH_ID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Represents the AHB response error is write request."]
    #[inline(always)]
    pub fn ahbinf_resp_err_wr(&self) -> AHBINF_RESP_ERR_WR_R {
        AHBINF_RESP_ERR_WR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Represents the AHB response error request id."]
    #[inline(always)]
    pub fn ahbinf_resp_err_id(&self) -> AHBINF_RESP_ERR_ID_R {
        AHBINF_RESP_ERR_ID_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - Represents the AHB response error request channel id."]
    #[inline(always)]
    pub fn ahbinf_resp_err_ch_id(&self) -> AHBINF_RESP_ERR_CH_ID_R {
        AHBINF_RESP_ERR_CH_ID_R::new(((self.bits >> 5) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBINF_RESP_ERR_STATUS1")
            .field("ahbinf_resp_err_wr", &self.ahbinf_resp_err_wr())
            .field("ahbinf_resp_err_id", &self.ahbinf_resp_err_id())
            .field("ahbinf_resp_err_ch_id", &self.ahbinf_resp_err_ch_id())
            .finish()
    }
}
#[doc = "Represents the AHB response error is write request.\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbinf_resp_err_status1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBINF_RESP_ERR_STATUS1_SPEC;
impl crate::RegisterSpec for AHBINF_RESP_ERR_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbinf_resp_err_status1::R`](R) reader structure"]
impl crate::Readable for AHBINF_RESP_ERR_STATUS1_SPEC {}
#[doc = "`reset()` method sets AHBINF_RESP_ERR_STATUS1 to value 0"]
impl crate::Resettable for AHBINF_RESP_ERR_STATUS1_SPEC {}
