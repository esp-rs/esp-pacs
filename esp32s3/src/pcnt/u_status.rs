#[doc = "Register `U%s_STATUS` reader"]
pub struct R(crate::R<U_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT_THR_ZERO_MODE_U` reader - The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive."]
pub type CNT_THR_ZERO_MODE_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT_THR_THRES1_LAT_U` reader - The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others"]
pub type CNT_THR_THRES1_LAT_U_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_THRES0_LAT_U` reader - The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others"]
pub type CNT_THR_THRES0_LAT_U_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_L_LIM_LAT_U` reader - The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others"]
pub type CNT_THR_L_LIM_LAT_U_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_H_LIM_LAT_U` reader - The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others"]
pub type CNT_THR_H_LIM_LAT_U_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_ZERO_LAT_U` reader - The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others"]
pub type CNT_THR_ZERO_LAT_U_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive."]
    #[inline(always)]
    pub fn cnt_thr_zero_mode_u(&self) -> CNT_THR_ZERO_MODE_U_R {
        CNT_THR_ZERO_MODE_U_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others"]
    #[inline(always)]
    pub fn cnt_thr_thres1_lat_u(&self) -> CNT_THR_THRES1_LAT_U_R {
        CNT_THR_THRES1_LAT_U_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others"]
    #[inline(always)]
    pub fn cnt_thr_thres0_lat_u(&self) -> CNT_THR_THRES0_LAT_U_R {
        CNT_THR_THRES0_LAT_U_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others"]
    #[inline(always)]
    pub fn cnt_thr_l_lim_lat_u(&self) -> CNT_THR_L_LIM_LAT_U_R {
        CNT_THR_L_LIM_LAT_U_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others"]
    #[inline(always)]
    pub fn cnt_thr_h_lim_lat_u(&self) -> CNT_THR_H_LIM_LAT_U_R {
        CNT_THR_H_LIM_LAT_U_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others"]
    #[inline(always)]
    pub fn cnt_thr_zero_lat_u(&self) -> CNT_THR_ZERO_LAT_U_R {
        CNT_THR_ZERO_LAT_U_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "PNCT UNIT%s status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u_status](index.html) module"]
pub struct U_STATUS_SPEC;
impl crate::RegisterSpec for U_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u_status::R](R) reader structure"]
impl crate::Readable for U_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets U%s_STATUS to value 0"]
impl crate::Resettable for U_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
