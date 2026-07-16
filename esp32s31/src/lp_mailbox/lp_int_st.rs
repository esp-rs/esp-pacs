#[doc = "Register `LP_INT_ST` reader"]
pub type R = crate::R<LP_INT_ST_SPEC>;
#[doc = "Field `LP_0_INT_ST` reader - "]
pub type LP_0_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_1_INT_ST` reader - "]
pub type LP_1_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_2_INT_ST` reader - "]
pub type LP_2_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_3_INT_ST` reader - "]
pub type LP_3_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_4_INT_ST` reader - "]
pub type LP_4_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_5_INT_ST` reader - "]
pub type LP_5_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_6_INT_ST` reader - "]
pub type LP_6_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_7_INT_ST` reader - "]
pub type LP_7_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_8_INT_ST` reader - "]
pub type LP_8_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_9_INT_ST` reader - "]
pub type LP_9_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_10_INT_ST` reader - "]
pub type LP_10_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_11_INT_ST` reader - "]
pub type LP_11_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_12_INT_ST` reader - "]
pub type LP_12_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_13_INT_ST` reader - "]
pub type LP_13_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_14_INT_ST` reader - "]
pub type LP_14_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_15_INT_ST` reader - "]
pub type LP_15_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lp_0_int_st(&self) -> LP_0_INT_ST_R {
        LP_0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lp_1_int_st(&self) -> LP_1_INT_ST_R {
        LP_1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lp_2_int_st(&self) -> LP_2_INT_ST_R {
        LP_2_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lp_3_int_st(&self) -> LP_3_INT_ST_R {
        LP_3_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lp_4_int_st(&self) -> LP_4_INT_ST_R {
        LP_4_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lp_5_int_st(&self) -> LP_5_INT_ST_R {
        LP_5_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lp_6_int_st(&self) -> LP_6_INT_ST_R {
        LP_6_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lp_7_int_st(&self) -> LP_7_INT_ST_R {
        LP_7_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lp_8_int_st(&self) -> LP_8_INT_ST_R {
        LP_8_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lp_9_int_st(&self) -> LP_9_INT_ST_R {
        LP_9_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lp_10_int_st(&self) -> LP_10_INT_ST_R {
        LP_10_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lp_11_int_st(&self) -> LP_11_INT_ST_R {
        LP_11_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lp_12_int_st(&self) -> LP_12_INT_ST_R {
        LP_12_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn lp_13_int_st(&self) -> LP_13_INT_ST_R {
        LP_13_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lp_14_int_st(&self) -> LP_14_INT_ST_R {
        LP_14_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lp_15_int_st(&self) -> LP_15_INT_ST_R {
        LP_15_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ST")
            .field("lp_0_int_st", &self.lp_0_int_st())
            .field("lp_1_int_st", &self.lp_1_int_st())
            .field("lp_2_int_st", &self.lp_2_int_st())
            .field("lp_3_int_st", &self.lp_3_int_st())
            .field("lp_4_int_st", &self.lp_4_int_st())
            .field("lp_5_int_st", &self.lp_5_int_st())
            .field("lp_6_int_st", &self.lp_6_int_st())
            .field("lp_7_int_st", &self.lp_7_int_st())
            .field("lp_8_int_st", &self.lp_8_int_st())
            .field("lp_9_int_st", &self.lp_9_int_st())
            .field("lp_10_int_st", &self.lp_10_int_st())
            .field("lp_11_int_st", &self.lp_11_int_st())
            .field("lp_12_int_st", &self.lp_12_int_st())
            .field("lp_13_int_st", &self.lp_13_int_st())
            .field("lp_14_int_st", &self.lp_14_int_st())
            .field("lp_15_int_st", &self.lp_15_int_st())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_ST_SPEC;
impl crate::RegisterSpec for LP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_st::R`](R) reader structure"]
impl crate::Readable for LP_INT_ST_SPEC {}
#[doc = "`reset()` method sets LP_INT_ST to value 0"]
impl crate::Resettable for LP_INT_ST_SPEC {}
