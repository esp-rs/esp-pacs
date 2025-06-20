#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `BOD_MODE0_INT_ST` reader - brownout mode0 interrupt status register"]
pub type BOD_MODE0_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - brownout mode0 interrupt status register"]
    #[inline(always)]
    pub fn bod_mode0_int_st(&self) -> BOD_MODE0_INT_ST_R {
        BOD_MODE0_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("bod_mode0_int_st", &self.bod_mode0_int_st())
            .finish()
    }
}
#[doc = "interrpt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
