///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Register `INT_RAW` writer
pub type W = crate::W<INT_RAW_SPEC>;
///Field `SUPER_WDT` reader - need_des
pub type SUPER_WDT_R = crate::BitReader;
///Field `SUPER_WDT` writer - need_des
pub type SUPER_WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_WDT` reader - need_des
pub type LP_WDT_R = crate::BitReader;
///Field `LP_WDT` writer - need_des
pub type LP_WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("INT_RAW")
            .field("super_wdt", &self.super_wdt())
            .field("lp_wdt", &self.lp_wdt())
            .finish()
    }
}
impl W {
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn super_wdt(&mut self) -> SUPER_WDT_W<INT_RAW_SPEC> {
        SUPER_WDT_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_wdt(&mut self) -> LP_WDT_W<INT_RAW_SPEC> {
        LP_WDT_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw::R`](R) reader structure
impl crate::Readable for INT_RAW_SPEC {}
///`write(|w| ..)` method takes [`int_raw::W`](W) writer structure
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_RAW to value 0
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
