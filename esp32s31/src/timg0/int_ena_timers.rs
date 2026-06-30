#[doc = "Register `INT_ENA_TIMERS` reader"]
pub type R = crate::R<INT_ENA_TIMERS_SPEC>;
#[doc = "Register `INT_ENA_TIMERS` writer"]
pub type W = crate::W<INT_ENA_TIMERS_SPEC>;
#[doc = "Field `T0_INT_ENA` reader - Write 1 to enable the TIMG_T0_INT interrupt."]
pub type T0_INT_ENA_R = crate::BitReader;
#[doc = "Field `T0_INT_ENA` writer - Write 1 to enable the TIMG_T0_INT interrupt."]
pub type T0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T1_INT_ENA` reader - Write 1 to enable the TIMG_T1_INT interrupt."]
pub type T1_INT_ENA_R = crate::BitReader;
#[doc = "Field `T1_INT_ENA` writer - Write 1 to enable the TIMG_T1_INT interrupt."]
pub type T1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_ENA` reader - Write 1 to enable the TIMG_WDT_INT interrupt."]
pub type WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `WDT_INT_ENA` writer - Write 1 to enable the TIMG_WDT_INT interrupt."]
pub type WDT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable the TIMG_T0_INT interrupt."]
    #[inline(always)]
    pub fn t0_int_ena(&self) -> T0_INT_ENA_R {
        T0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable the TIMG_T1_INT interrupt."]
    #[inline(always)]
    pub fn t1_int_ena(&self) -> T1_INT_ENA_R {
        T1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_TIMERS")
            .field("t0_int_ena", &self.t0_int_ena())
            .field("t1_int_ena", &self.t1_int_ena())
            .field("wdt_int_ena", &self.wdt_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable the TIMG_T0_INT interrupt."]
    #[inline(always)]
    pub fn t0_int_ena(&mut self) -> T0_INT_ENA_W<'_, INT_ENA_TIMERS_SPEC> {
        T0_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable the TIMG_T1_INT interrupt."]
    #[inline(always)]
    pub fn t1_int_ena(&mut self) -> T1_INT_ENA_W<'_, INT_ENA_TIMERS_SPEC> {
        T1_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W<'_, INT_ENA_TIMERS_SPEC> {
        WDT_INT_ENA_W::new(self, 2)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena_timers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_timers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ENA_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena_timers::R`](R) reader structure"]
impl crate::Readable for INT_ENA_TIMERS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena_timers::W`](W) writer structure"]
impl crate::Writable for INT_ENA_TIMERS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA_TIMERS to value 0"]
impl crate::Resettable for INT_ENA_TIMERS_SPEC {}
