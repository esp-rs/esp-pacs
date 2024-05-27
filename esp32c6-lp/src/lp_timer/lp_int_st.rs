///Register `LP_INT_ST` reader
pub type R = crate::R<LP_INT_ST_SPEC>;
///Field `MAIN_TIMER_OVERFLOW` reader - need_des
pub type MAIN_TIMER_OVERFLOW_R = crate::BitReader;
///Field `MAIN_TIMER` reader - need_des
pub type MAIN_TIMER_R = crate::BitReader;
impl R {
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn main_timer_overflow(&self) -> MAIN_TIMER_OVERFLOW_R {
        MAIN_TIMER_OVERFLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ST")
            .field("main_timer_overflow", &self.main_timer_overflow())
            .field("main_timer", &self.main_timer())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_INT_ST_SPEC;
impl crate::RegisterSpec for LP_INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_int_st::R`](R) reader structure
impl crate::Readable for LP_INT_ST_SPEC {}
///`reset()` method sets LP_INT_ST to value 0
impl crate::Resettable for LP_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
