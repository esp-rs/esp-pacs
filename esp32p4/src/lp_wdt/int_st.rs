///Register `INT_ST` reader
pub type R = crate::R<INT_ST_SPEC>;
///Field `SUPER_WDT` reader - need_des
pub type SUPER_WDT_R = crate::BitReader;
///Field `LP_WDT` reader - need_des
pub type LP_WDT_R = crate::BitReader;
impl R {
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn super_wdt(&self) -> SUPER_WDT_R {
        SUPER_WDT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn lp_wdt(&self) -> LP_WDT_R {
        LP_WDT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("super_wdt", &self.super_wdt())
            .field("lp_wdt", &self.lp_wdt())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_st::R`](R) reader structure
impl crate::Readable for INT_ST_SPEC {}
///`reset()` method sets INT_ST to value 0
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
