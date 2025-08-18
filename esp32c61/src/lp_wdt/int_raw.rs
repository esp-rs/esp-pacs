#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `SUPER_WDT_INT_RAW` reader - Represents the SWD whether or not generates timeout interrupt.\\\\ 0:No \\\\1: Yes"]
pub type SUPER_WDT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SUPER_WDT_INT_RAW` writer - Represents the SWD whether or not generates timeout interrupt.\\\\ 0:No \\\\1: Yes"]
pub type SUPER_WDT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_WDT_INT_RAW` reader - Represents the RWDT whether or not generates timeout interrupt.\\\\ 0:No \\\\1: Yes"]
pub type LP_WDT_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_WDT_INT_RAW` writer - Represents the RWDT whether or not generates timeout interrupt.\\\\ 0:No \\\\1: Yes"]
pub type LP_WDT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Represents the SWD whether or not generates timeout interrupt.\\\\ 0:No \\\\1: Yes"]
    #[inline(always)]
    pub fn super_wdt_int_raw(&self) -> SUPER_WDT_INT_RAW_R {
        SUPER_WDT_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents the RWDT whether or not generates timeout interrupt.\\\\ 0:No \\\\1: Yes"]
    #[inline(always)]
    pub fn lp_wdt_int_raw(&self) -> LP_WDT_INT_RAW_R {
        LP_WDT_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("super_wdt_int_raw", &self.super_wdt_int_raw())
            .field("lp_wdt_int_raw", &self.lp_wdt_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Represents the SWD whether or not generates timeout interrupt.\\\\ 0:No \\\\1: Yes"]
    #[inline(always)]
    pub fn super_wdt_int_raw(&mut self) -> SUPER_WDT_INT_RAW_W<'_, INT_RAW_SPEC> {
        SUPER_WDT_INT_RAW_W::new(self, 30)
    }
    #[doc = "Bit 31 - Represents the RWDT whether or not generates timeout interrupt.\\\\ 0:No \\\\1: Yes"]
    #[inline(always)]
    pub fn lp_wdt_int_raw(&mut self) -> LP_WDT_INT_RAW_W<'_, INT_RAW_SPEC> {
        LP_WDT_INT_RAW_W::new(self, 31)
    }
}
#[doc = "Configure whether to generate timeout interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
