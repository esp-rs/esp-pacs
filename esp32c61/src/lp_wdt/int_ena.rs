#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `SUPER_WDT_INT_ENA` reader - Configure whether or not to enable the SWD to send timeout interrupt.\\\\0:Disable \\\\1:Enable"]
pub type SUPER_WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SUPER_WDT_INT_ENA` writer - Configure whether or not to enable the SWD to send timeout interrupt.\\\\0:Disable \\\\1:Enable"]
pub type SUPER_WDT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_WDT_INT_ENA` reader - Configure whether or not to enable the RWDT to send timeout interrupt.\\\\0:Disable \\\\1:Enable"]
pub type LP_WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_WDT_INT_ENA` writer - Configure whether or not to enable the RWDT to send timeout interrupt.\\\\0:Disable \\\\1:Enable"]
pub type LP_WDT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Configure whether or not to enable the SWD to send timeout interrupt.\\\\0:Disable \\\\1:Enable"]
    #[inline(always)]
    pub fn super_wdt_int_ena(&self) -> SUPER_WDT_INT_ENA_R {
        SUPER_WDT_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configure whether or not to enable the RWDT to send timeout interrupt.\\\\0:Disable \\\\1:Enable"]
    #[inline(always)]
    pub fn lp_wdt_int_ena(&self) -> LP_WDT_INT_ENA_R {
        LP_WDT_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("super_wdt_int_ena", &self.super_wdt_int_ena())
            .field("lp_wdt_int_ena", &self.lp_wdt_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Configure whether or not to enable the SWD to send timeout interrupt.\\\\0:Disable \\\\1:Enable"]
    #[inline(always)]
    pub fn super_wdt_int_ena(&mut self) -> SUPER_WDT_INT_ENA_W<INT_ENA_SPEC> {
        SUPER_WDT_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configure whether or not to enable the RWDT to send timeout interrupt.\\\\0:Disable \\\\1:Enable"]
    #[inline(always)]
    pub fn lp_wdt_int_ena(&mut self) -> LP_WDT_INT_ENA_W<INT_ENA_SPEC> {
        LP_WDT_INT_ENA_W::new(self, 31)
    }
}
#[doc = "The interrupt enable register of WDT\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
