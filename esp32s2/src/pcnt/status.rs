#[doc = "Register `STATUS%s` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT_THR_ZERO_MODE` reader - The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive."]
pub type CNT_THR_ZERO_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT_THR_THRES1_LAT` reader - The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others."]
pub type CNT_THR_THRES1_LAT_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_THRES0_LAT` reader - The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others."]
pub type CNT_THR_THRES0_LAT_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_L_LIM_LAT` reader - The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others."]
pub type CNT_THR_L_LIM_LAT_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_H_LIM_LAT` reader - The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others."]
pub type CNT_THR_H_LIM_LAT_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_ZERO_LAT` reader - The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others."]
pub type CNT_THR_ZERO_LAT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive."]
    #[inline(always)]
    pub fn cnt_thr_zero_mode(&self) -> CNT_THR_ZERO_MODE_R {
        CNT_THR_ZERO_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others."]
    #[inline(always)]
    pub fn cnt_thr_thres1_lat(&self) -> CNT_THR_THRES1_LAT_R {
        CNT_THR_THRES1_LAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others."]
    #[inline(always)]
    pub fn cnt_thr_thres0_lat(&self) -> CNT_THR_THRES0_LAT_R {
        CNT_THR_THRES0_LAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others."]
    #[inline(always)]
    pub fn cnt_thr_l_lim_lat(&self) -> CNT_THR_L_LIM_LAT_R {
        CNT_THR_L_LIM_LAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others."]
    #[inline(always)]
    pub fn cnt_thr_h_lim_lat(&self) -> CNT_THR_H_LIM_LAT_R {
        CNT_THR_H_LIM_LAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others."]
    #[inline(always)]
    pub fn cnt_thr_zero_lat(&self) -> CNT_THR_ZERO_LAT_R {
        CNT_THR_ZERO_LAT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "PNCT UNIT%s status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS%s to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
