#[doc = "Register `INT_ENA_TIMERS` reader"]
pub type R = crate::R<INT_ENA_TIMERS_SPEC>;
#[doc = "Register `INT_ENA_TIMERS` writer"]
pub type W = crate::W<INT_ENA_TIMERS_SPEC>;
#[doc = "Field `T(0-1)` reader - interrupt when timer%s alarm"]
pub type T_R = crate::BitReader;
#[doc = "Field `T(0-1)` writer - interrupt when timer%s alarm"]
pub type T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT` reader - Interrupt when an interrupt stage timeout"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `WDT` writer - Interrupt when an interrupt stage timeout"]
pub type WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT` reader - "]
pub type LACT_R = crate::BitReader;
#[doc = "Field `LACT` writer - "]
pub type LACT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "interrupt when timer(0-1) alarm"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `T0` field.</div>"]
    #[inline(always)]
    pub fn t(&self, n: u8) -> T_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        T_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "interrupt when timer(0-1) alarm"]
    #[inline(always)]
    pub fn t_iter(&self) -> impl Iterator<Item = T_R> + '_ {
        (0..2).map(move |n| T_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn t0(&self) -> T_R {
        T_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn t1(&self) -> T_R {
        T_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lact(&self) -> LACT_R {
        LACT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_TIMERS")
            .field("t0", &self.t0())
            .field("t1", &self.t1())
            .field("wdt", &self.wdt())
            .field("lact", &self.lact())
            .finish()
    }
}
impl W {
    #[doc = "interrupt when timer(0-1) alarm"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `T0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self, n: u8) -> T_W<INT_ENA_TIMERS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        T_W::new(self, n)
    }
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    #[must_use]
    pub fn t0(&mut self) -> T_W<INT_ENA_TIMERS_SPEC> {
        T_W::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    #[must_use]
    pub fn t1(&mut self) -> T_W<INT_ENA_TIMERS_SPEC> {
        T_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<INT_ENA_TIMERS_SPEC> {
        WDT_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn lact(&mut self) -> LACT_W<INT_ENA_TIMERS_SPEC> {
        LACT_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena_timers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_timers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ENA_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena_timers::R`](R) reader structure"]
impl crate::Readable for INT_ENA_TIMERS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena_timers::W`](W) writer structure"]
impl crate::Writable for INT_ENA_TIMERS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA_TIMERS to value 0"]
impl crate::Resettable for INT_ENA_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
