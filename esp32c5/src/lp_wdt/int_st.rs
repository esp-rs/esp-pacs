#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SUPER_WDT_INT_ST` reader - Represents the SWD whether or not has generated and sent timeout interrupt to CPU.\\\\ 0:No \\\\1: Yes"]
pub type SUPER_WDT_INT_ST_R = crate::BitReader;
#[doc = "Field `WDT_INT_ST` reader - Represents the RWDT whether or not has generated and sent timeout interrupt to CPU.\\\\ 0:No \\\\1: Yes"]
pub type WDT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 30 - Represents the SWD whether or not has generated and sent timeout interrupt to CPU.\\\\ 0:No \\\\1: Yes"]
    #[inline(always)]
    pub fn super_wdt_int_st(&self) -> SUPER_WDT_INT_ST_R {
        SUPER_WDT_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents the RWDT whether or not has generated and sent timeout interrupt to CPU.\\\\ 0:No \\\\1: Yes"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("super_wdt_int_st", &self.super_wdt_int_st())
            .field("wdt_int_st", &self.wdt_int_st())
            .finish()
    }
}
#[doc = "The interrupt status register of WDT\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
