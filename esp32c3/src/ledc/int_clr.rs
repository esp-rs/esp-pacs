#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSTIMER0_OVF_INT_CLR` writer - reg_lstimer0_ovf_int_clr."]
pub type LSTIMER0_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `LSTIMER1_OVF_INT_CLR` writer - reg_lstimer1_ovf_int_clr."]
pub type LSTIMER1_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `LSTIMER2_OVF_INT_CLR` writer - reg_lstimer2_ovf_int_clr."]
pub type LSTIMER2_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `LSTIMER3_OVF_INT_CLR` writer - reg_lstimer3_ovf_int_clr."]
pub type LSTIMER3_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_CLR` writer - reg_duty_chng_end_lsch0_int_clr."]
pub type DUTY_CHNG_END_LSCH0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_CLR` writer - reg_duty_chng_end_lsch1_int_clr."]
pub type DUTY_CHNG_END_LSCH1_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_CLR` writer - reg_duty_chng_end_lsch2_int_clr."]
pub type DUTY_CHNG_END_LSCH2_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_CLR` writer - reg_duty_chng_end_lsch3_int_clr."]
pub type DUTY_CHNG_END_LSCH3_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_CLR` writer - reg_duty_chng_end_lsch4_int_clr."]
pub type DUTY_CHNG_END_LSCH4_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_CLR` writer - reg_duty_chng_end_lsch5_int_clr."]
pub type DUTY_CHNG_END_LSCH5_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OVF_CNT_LSCH0_INT_CLR` writer - reg_ovf_cnt_lsch0_int_clr."]
pub type OVF_CNT_LSCH0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OVF_CNT_LSCH1_INT_CLR` writer - reg_ovf_cnt_lsch1_int_clr."]
pub type OVF_CNT_LSCH1_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OVF_CNT_LSCH2_INT_CLR` writer - reg_ovf_cnt_lsch2_int_clr."]
pub type OVF_CNT_LSCH2_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OVF_CNT_LSCH3_INT_CLR` writer - reg_ovf_cnt_lsch3_int_clr."]
pub type OVF_CNT_LSCH3_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OVF_CNT_LSCH4_INT_CLR` writer - reg_ovf_cnt_lsch4_int_clr."]
pub type OVF_CNT_LSCH4_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `OVF_CNT_LSCH5_INT_CLR` writer - reg_ovf_cnt_lsch5_int_clr."]
pub type OVF_CNT_LSCH5_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - reg_lstimer0_ovf_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer0_ovf_int_clr(&mut self) -> LSTIMER0_OVF_INT_CLR_W<0> {
        LSTIMER0_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - reg_lstimer1_ovf_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer1_ovf_int_clr(&mut self) -> LSTIMER1_OVF_INT_CLR_W<1> {
        LSTIMER1_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - reg_lstimer2_ovf_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer2_ovf_int_clr(&mut self) -> LSTIMER2_OVF_INT_CLR_W<2> {
        LSTIMER2_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - reg_lstimer3_ovf_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer3_ovf_int_clr(&mut self) -> LSTIMER3_OVF_INT_CLR_W<3> {
        LSTIMER3_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - reg_duty_chng_end_lsch0_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch0_int_clr(&mut self) -> DUTY_CHNG_END_LSCH0_INT_CLR_W<4> {
        DUTY_CHNG_END_LSCH0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - reg_duty_chng_end_lsch1_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch1_int_clr(&mut self) -> DUTY_CHNG_END_LSCH1_INT_CLR_W<5> {
        DUTY_CHNG_END_LSCH1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - reg_duty_chng_end_lsch2_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch2_int_clr(&mut self) -> DUTY_CHNG_END_LSCH2_INT_CLR_W<6> {
        DUTY_CHNG_END_LSCH2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - reg_duty_chng_end_lsch3_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch3_int_clr(&mut self) -> DUTY_CHNG_END_LSCH3_INT_CLR_W<7> {
        DUTY_CHNG_END_LSCH3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - reg_duty_chng_end_lsch4_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch4_int_clr(&mut self) -> DUTY_CHNG_END_LSCH4_INT_CLR_W<8> {
        DUTY_CHNG_END_LSCH4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - reg_duty_chng_end_lsch5_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch5_int_clr(&mut self) -> DUTY_CHNG_END_LSCH5_INT_CLR_W<9> {
        DUTY_CHNG_END_LSCH5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - reg_ovf_cnt_lsch0_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch0_int_clr(&mut self) -> OVF_CNT_LSCH0_INT_CLR_W<10> {
        OVF_CNT_LSCH0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - reg_ovf_cnt_lsch1_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch1_int_clr(&mut self) -> OVF_CNT_LSCH1_INT_CLR_W<11> {
        OVF_CNT_LSCH1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - reg_ovf_cnt_lsch2_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch2_int_clr(&mut self) -> OVF_CNT_LSCH2_INT_CLR_W<12> {
        OVF_CNT_LSCH2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - reg_ovf_cnt_lsch3_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch3_int_clr(&mut self) -> OVF_CNT_LSCH3_INT_CLR_W<13> {
        OVF_CNT_LSCH3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - reg_ovf_cnt_lsch4_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch4_int_clr(&mut self) -> OVF_CNT_LSCH4_INT_CLR_W<14> {
        OVF_CNT_LSCH4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_lsch5_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch5_int_clr(&mut self) -> OVF_CNT_LSCH5_INT_CLR_W<15> {
        OVF_CNT_LSCH5_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_INT_CLR.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
