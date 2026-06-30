#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `FIFO_OVERFLOW_INT_ST` reader - need_des"]
pub type FIFO_OVERFLOW_INT_ST_R = crate::BitReader;
#[doc = "Field `THRES1_LOW_INT_ST` reader - need_des"]
pub type THRES1_LOW_INT_ST_R = crate::BitReader;
#[doc = "Field `THRES0_LOW_INT_ST` reader - need_des"]
pub type THRES0_LOW_INT_ST_R = crate::BitReader;
#[doc = "Field `THRES1_HIGH_INT_ST` reader - need_des"]
pub type THRES1_HIGH_INT_ST_R = crate::BitReader;
#[doc = "Field `THRES0_HIGH_INT_ST` reader - need_des"]
pub type THRES0_HIGH_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR2_DONE_INT_ST` reader - need_des"]
pub type SAR2_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR1_DONE_INT_ST` reader - need_des"]
pub type SAR1_DONE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn fifo_overflow_int_st(&self) -> FIFO_OVERFLOW_INT_ST_R {
        FIFO_OVERFLOW_INT_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn thres1_low_int_st(&self) -> THRES1_LOW_INT_ST_R {
        THRES1_LOW_INT_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn thres0_low_int_st(&self) -> THRES0_LOW_INT_ST_R {
        THRES0_LOW_INT_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn thres1_high_int_st(&self) -> THRES1_HIGH_INT_ST_R {
        THRES1_HIGH_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn thres0_high_int_st(&self) -> THRES0_HIGH_INT_ST_R {
        THRES0_HIGH_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn sar2_done_int_st(&self) -> SAR2_DONE_INT_ST_R {
        SAR2_DONE_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sar1_done_int_st(&self) -> SAR1_DONE_INT_ST_R {
        SAR1_DONE_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("fifo_overflow_int_st", &self.fifo_overflow_int_st())
            .field("thres1_low_int_st", &self.thres1_low_int_st())
            .field("thres0_low_int_st", &self.thres0_low_int_st())
            .field("thres1_high_int_st", &self.thres1_high_int_st())
            .field("thres0_high_int_st", &self.thres0_high_int_st())
            .field("sar2_done_int_st", &self.sar2_done_int_st())
            .field("sar1_done_int_st", &self.sar1_done_int_st())
            .finish()
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
