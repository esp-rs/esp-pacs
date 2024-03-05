#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `LSTIMER0_OVF_INT_CLR` writer - reg_lstimer0_ovf_int_clr."]
pub type LSTIMER0_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER1_OVF_INT_CLR` writer - reg_lstimer1_ovf_int_clr."]
pub type LSTIMER1_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER2_OVF_INT_CLR` writer - reg_lstimer2_ovf_int_clr."]
pub type LSTIMER2_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER3_OVF_INT_CLR` writer - reg_lstimer3_ovf_int_clr."]
pub type LSTIMER3_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_CLR` writer - reg_duty_chng_end_lsch0_int_clr."]
pub type DUTY_CHNG_END_LSCH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_CLR` writer - reg_duty_chng_end_lsch1_int_clr."]
pub type DUTY_CHNG_END_LSCH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_CLR` writer - reg_duty_chng_end_lsch2_int_clr."]
pub type DUTY_CHNG_END_LSCH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_CLR` writer - reg_duty_chng_end_lsch3_int_clr."]
pub type DUTY_CHNG_END_LSCH3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_CLR` writer - reg_duty_chng_end_lsch4_int_clr."]
pub type DUTY_CHNG_END_LSCH4_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_CLR` writer - reg_duty_chng_end_lsch5_int_clr."]
pub type DUTY_CHNG_END_LSCH5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_LSCH0_INT_CLR` writer - reg_ovf_cnt_lsch0_int_clr."]
pub type OVF_CNT_LSCH0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_LSCH1_INT_CLR` writer - reg_ovf_cnt_lsch1_int_clr."]
pub type OVF_CNT_LSCH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_LSCH2_INT_CLR` writer - reg_ovf_cnt_lsch2_int_clr."]
pub type OVF_CNT_LSCH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_LSCH3_INT_CLR` writer - reg_ovf_cnt_lsch3_int_clr."]
pub type OVF_CNT_LSCH3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_LSCH4_INT_CLR` writer - reg_ovf_cnt_lsch4_int_clr."]
pub type OVF_CNT_LSCH4_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_LSCH5_INT_CLR` writer - reg_ovf_cnt_lsch5_int_clr."]
pub type OVF_CNT_LSCH5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn lstimer0_ovf_int_clr(&mut self) -> LSTIMER0_OVF_INT_CLR_W<INT_CLR_SPEC> {
        LSTIMER0_OVF_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_lstimer1_ovf_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer1_ovf_int_clr(&mut self) -> LSTIMER1_OVF_INT_CLR_W<INT_CLR_SPEC> {
        LSTIMER1_OVF_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - reg_lstimer2_ovf_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer2_ovf_int_clr(&mut self) -> LSTIMER2_OVF_INT_CLR_W<INT_CLR_SPEC> {
        LSTIMER2_OVF_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - reg_lstimer3_ovf_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer3_ovf_int_clr(&mut self) -> LSTIMER3_OVF_INT_CLR_W<INT_CLR_SPEC> {
        LSTIMER3_OVF_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - reg_duty_chng_end_lsch0_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch0_int_clr(&mut self) -> DUTY_CHNG_END_LSCH0_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH0_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - reg_duty_chng_end_lsch1_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch1_int_clr(&mut self) -> DUTY_CHNG_END_LSCH1_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH1_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - reg_duty_chng_end_lsch2_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch2_int_clr(&mut self) -> DUTY_CHNG_END_LSCH2_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH2_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - reg_duty_chng_end_lsch3_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch3_int_clr(&mut self) -> DUTY_CHNG_END_LSCH3_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH3_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - reg_duty_chng_end_lsch4_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch4_int_clr(&mut self) -> DUTY_CHNG_END_LSCH4_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH4_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - reg_duty_chng_end_lsch5_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch5_int_clr(&mut self) -> DUTY_CHNG_END_LSCH5_INT_CLR_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH5_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - reg_ovf_cnt_lsch0_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch0_int_clr(&mut self) -> OVF_CNT_LSCH0_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_LSCH0_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - reg_ovf_cnt_lsch1_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch1_int_clr(&mut self) -> OVF_CNT_LSCH1_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_LSCH1_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - reg_ovf_cnt_lsch2_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch2_int_clr(&mut self) -> OVF_CNT_LSCH2_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_LSCH2_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - reg_ovf_cnt_lsch3_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch3_int_clr(&mut self) -> OVF_CNT_LSCH3_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_LSCH3_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - reg_ovf_cnt_lsch4_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch4_int_clr(&mut self) -> OVF_CNT_LSCH4_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_LSCH4_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_lsch5_int_clr."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_lsch5_int_clr(&mut self) -> OVF_CNT_LSCH5_INT_CLR_W<INT_CLR_SPEC> {
        OVF_CNT_LSCH5_INT_CLR_W::new(self, 15)
    }
}
#[doc = "LEDC_INT_CLR.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
