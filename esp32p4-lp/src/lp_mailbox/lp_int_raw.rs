#[doc = "Register `LP_INT_RAW` reader"]
pub type R = crate::R<LP_INT_RAW_SPEC>;
#[doc = "Field `LP_0_INT_RAW` reader - MB_LP_0_INT_RAW"]
pub type LP_0_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_1_INT_RAW` reader - MB_LP_1_INT_RAW"]
pub type LP_1_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_2_INT_RAW` reader - MB_LP_2_INT_RAW"]
pub type LP_2_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_3_INT_RAW` reader - MB_LP_3_INT_RAW"]
pub type LP_3_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_4_INT_RAW` reader - MB_LP_4_INT_RAW"]
pub type LP_4_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_5_INT_RAW` reader - MB_LP_5_INT_RAW"]
pub type LP_5_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_6_INT_RAW` reader - MB_LP_6_INT_RAW"]
pub type LP_6_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_7_INT_RAW` reader - MB_LP_7_INT_RAW"]
pub type LP_7_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_8_INT_RAW` reader - MB_LP_8_INT_RAW"]
pub type LP_8_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_9_INT_RAW` reader - MB_LP_9_INT_RAW"]
pub type LP_9_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_10_INT_RAW` reader - MB_LP_10_INT_RAW"]
pub type LP_10_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_11_INT_RAW` reader - MB_LP_11_INT_RAW"]
pub type LP_11_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_12_INT_RAW` reader - MB_LP_12_INT_RAW"]
pub type LP_12_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_13_INT_RAW` reader - MB_LP_13_INT_RAW"]
pub type LP_13_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_14_INT_RAW` reader - MB_LP_14_INT_RAW"]
pub type LP_14_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_15_INT_RAW` reader - MB_LP_15_INT_RAW"]
pub type LP_15_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MB_LP_0_INT_RAW"]
    #[inline(always)]
    pub fn lp_0_int_raw(&self) -> LP_0_INT_RAW_R {
        LP_0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MB_LP_1_INT_RAW"]
    #[inline(always)]
    pub fn lp_1_int_raw(&self) -> LP_1_INT_RAW_R {
        LP_1_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MB_LP_2_INT_RAW"]
    #[inline(always)]
    pub fn lp_2_int_raw(&self) -> LP_2_INT_RAW_R {
        LP_2_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MB_LP_3_INT_RAW"]
    #[inline(always)]
    pub fn lp_3_int_raw(&self) -> LP_3_INT_RAW_R {
        LP_3_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MB_LP_4_INT_RAW"]
    #[inline(always)]
    pub fn lp_4_int_raw(&self) -> LP_4_INT_RAW_R {
        LP_4_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MB_LP_5_INT_RAW"]
    #[inline(always)]
    pub fn lp_5_int_raw(&self) -> LP_5_INT_RAW_R {
        LP_5_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MB_LP_6_INT_RAW"]
    #[inline(always)]
    pub fn lp_6_int_raw(&self) -> LP_6_INT_RAW_R {
        LP_6_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MB_LP_7_INT_RAW"]
    #[inline(always)]
    pub fn lp_7_int_raw(&self) -> LP_7_INT_RAW_R {
        LP_7_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MB_LP_8_INT_RAW"]
    #[inline(always)]
    pub fn lp_8_int_raw(&self) -> LP_8_INT_RAW_R {
        LP_8_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MB_LP_9_INT_RAW"]
    #[inline(always)]
    pub fn lp_9_int_raw(&self) -> LP_9_INT_RAW_R {
        LP_9_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MB_LP_10_INT_RAW"]
    #[inline(always)]
    pub fn lp_10_int_raw(&self) -> LP_10_INT_RAW_R {
        LP_10_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MB_LP_11_INT_RAW"]
    #[inline(always)]
    pub fn lp_11_int_raw(&self) -> LP_11_INT_RAW_R {
        LP_11_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MB_LP_12_INT_RAW"]
    #[inline(always)]
    pub fn lp_12_int_raw(&self) -> LP_12_INT_RAW_R {
        LP_12_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MB_LP_13_INT_RAW"]
    #[inline(always)]
    pub fn lp_13_int_raw(&self) -> LP_13_INT_RAW_R {
        LP_13_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MB_LP_14_INT_RAW"]
    #[inline(always)]
    pub fn lp_14_int_raw(&self) -> LP_14_INT_RAW_R {
        LP_14_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MB_LP_15_INT_RAW"]
    #[inline(always)]
    pub fn lp_15_int_raw(&self) -> LP_15_INT_RAW_R {
        LP_15_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_RAW")
            .field("lp_0_int_raw", &self.lp_0_int_raw())
            .field("lp_1_int_raw", &self.lp_1_int_raw())
            .field("lp_2_int_raw", &self.lp_2_int_raw())
            .field("lp_3_int_raw", &self.lp_3_int_raw())
            .field("lp_4_int_raw", &self.lp_4_int_raw())
            .field("lp_5_int_raw", &self.lp_5_int_raw())
            .field("lp_6_int_raw", &self.lp_6_int_raw())
            .field("lp_7_int_raw", &self.lp_7_int_raw())
            .field("lp_8_int_raw", &self.lp_8_int_raw())
            .field("lp_9_int_raw", &self.lp_9_int_raw())
            .field("lp_10_int_raw", &self.lp_10_int_raw())
            .field("lp_11_int_raw", &self.lp_11_int_raw())
            .field("lp_12_int_raw", &self.lp_12_int_raw())
            .field("lp_13_int_raw", &self.lp_13_int_raw())
            .field("lp_14_int_raw", &self.lp_14_int_raw())
            .field("lp_15_int_raw", &self.lp_15_int_raw())
            .finish()
    }
}
#[doc = "MB_LP_INT_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_RAW_SPEC;
impl crate::RegisterSpec for LP_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_raw::R`](R) reader structure"]
impl crate::Readable for LP_INT_RAW_SPEC {}
#[doc = "`reset()` method sets LP_INT_RAW to value 0"]
impl crate::Resettable for LP_INT_RAW_SPEC {}
