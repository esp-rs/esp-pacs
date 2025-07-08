#[doc = "Register `LP_INT_ST` reader"]
pub type R = crate::R<LP_INT_ST_SPEC>;
#[doc = "Field `BOD_MODE0_LP_INT_ST` reader - brownout mode0 lp interrupt status register"]
pub type BOD_MODE0_LP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - brownout mode0 lp interrupt status register"]
    #[inline(always)]
    pub fn bod_mode0_lp_int_st(&self) -> BOD_MODE0_LP_INT_ST_R {
        BOD_MODE0_LP_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ST")
            .field("bod_mode0_lp_int_st", &self.bod_mode0_lp_int_st())
            .finish()
    }
}
#[doc = "lp interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_ST_SPEC;
impl crate::RegisterSpec for LP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_st::R`](R) reader structure"]
impl crate::Readable for LP_INT_ST_SPEC {}
#[doc = "`reset()` method sets LP_INT_ST to value 0"]
impl crate::Resettable for LP_INT_ST_SPEC {}
