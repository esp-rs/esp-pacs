#[doc = "Register `INT_RAW_TIMERS` reader"]
pub type R = crate::R<INT_RAW_TIMERS_SPEC>;
#[doc = "Register `INT_RAW_TIMERS` writer"]
pub type W = crate::W<INT_RAW_TIMERS_SPEC>;
#[doc = "Field `T(0-1)` reader - The raw interrupt status bit for the TIMG_T%s_INT interrupt."]
pub type T_R = crate::BitReader;
#[doc = "Field `T(0-1)` writer - The raw interrupt status bit for the TIMG_T%s_INT interrupt."]
pub type T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT` reader - The raw interrupt status bit for the TIMG_WDT_INT interrupt."]
pub type WDT_R = crate::BitReader;
#[doc = "Field `WDT` writer - The raw interrupt status bit for the TIMG_WDT_INT interrupt."]
pub type WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "The raw interrupt status bit for the TIMG_T(0-1)_INT interrupt."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `T0` field"]
    #[inline(always)]
    pub fn t(&self, n: u8) -> T_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        T_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The raw interrupt status bit for the TIMG_T(0-1)_INT interrupt."]
    #[inline(always)]
    pub fn t_iter(&self) -> impl Iterator<Item = T_R> + '_ {
        (0..2).map(move |n| T_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The raw interrupt status bit for the TIMG_T0_INT interrupt."]
    #[inline(always)]
    pub fn t0(&self) -> T_R {
        T_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the TIMG_T1_INT interrupt."]
    #[inline(always)]
    pub fn t1(&self) -> T_R {
        T_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW_TIMERS")
            .field("t0", &self.t0())
            .field("t1", &self.t1())
            .field("wdt", &self.wdt())
            .finish()
    }
}
impl W {
    #[doc = "The raw interrupt status bit for the TIMG_T(0-1)_INT interrupt."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `T0` field"]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self, n: u8) -> T_W<INT_RAW_TIMERS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        T_W::new(self, n)
    }
    #[doc = "Bit 0 - The raw interrupt status bit for the TIMG_T0_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn t0(&mut self) -> T_W<INT_RAW_TIMERS_SPEC> {
        T_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the TIMG_T1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn t1(&mut self) -> T_W<INT_RAW_TIMERS_SPEC> {
        T_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<INT_RAW_TIMERS_SPEC> {
        WDT_W::new(self, 2)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw_timers::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_TIMERS_SPEC;
impl crate::RegisterSpec for INT_RAW_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw_timers::R`](R) reader structure"]
impl crate::Readable for INT_RAW_TIMERS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw_timers::W`](W) writer structure"]
impl crate::Writable for INT_RAW_TIMERS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW_TIMERS to value 0"]
impl crate::Resettable for INT_RAW_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
