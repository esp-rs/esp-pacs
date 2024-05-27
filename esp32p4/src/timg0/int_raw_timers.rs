///Register `INT_RAW_TIMERS` reader
pub type R = crate::R<INT_RAW_TIMERS_SPEC>;
///Field `T(0-1)` reader - The raw interrupt status bit for the TIMG_T%s_INT interrupt.
pub type T_R = crate::BitReader;
///Field `WDT` reader - The raw interrupt status bit for the TIMG_WDT_INT interrupt.
pub type WDT_R = crate::BitReader;
impl R {
    ///The raw interrupt status bit for the TIMG_T(0-1)_INT interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `T0` field
    #[inline(always)]
    pub fn t(&self, n: u8) -> T_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        T_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///The raw interrupt status bit for the TIMG_T(0-1)_INT interrupt.
    #[inline(always)]
    pub fn t_iter(&self) -> impl Iterator<Item = T_R> + '_ {
        (0..2).map(move |n| T_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - The raw interrupt status bit for the TIMG_T0_INT interrupt.
    #[inline(always)]
    pub fn t0(&self) -> T_R {
        T_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The raw interrupt status bit for the TIMG_T1_INT interrupt.
    #[inline(always)]
    pub fn t1(&self) -> T_R {
        T_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The raw interrupt status bit for the TIMG_WDT_INT interrupt.
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
/**Raw interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_TIMERS_SPEC;
impl crate::RegisterSpec for INT_RAW_TIMERS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw_timers::R`](R) reader structure
impl crate::Readable for INT_RAW_TIMERS_SPEC {}
///`reset()` method sets INT_RAW_TIMERS to value 0
impl crate::Resettable for INT_RAW_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
