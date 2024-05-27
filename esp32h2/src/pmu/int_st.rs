///Register `INT_ST` reader
pub type R = crate::R<INT_ST_SPEC>;
///Field `LP_CPU_EXC` reader - need_des
pub type LP_CPU_EXC_R = crate::BitReader;
///Field `SDIO_IDLE` reader - need_des
pub type SDIO_IDLE_R = crate::BitReader;
///Field `SW` reader - need_des
pub type SW_R = crate::BitReader;
///Field `SOC_SLEEP_REJECT` reader - need_des
pub type SOC_SLEEP_REJECT_R = crate::BitReader;
///Field `SOC_WAKEUP` reader - need_des
pub type SOC_WAKEUP_R = crate::BitReader;
impl R {
    ///Bit 27 - need_des
    #[inline(always)]
    pub fn lp_cpu_exc(&self) -> LP_CPU_EXC_R {
        LP_CPU_EXC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    pub fn sdio_idle(&self) -> SDIO_IDLE_R {
        SDIO_IDLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn soc_sleep_reject(&self) -> SOC_SLEEP_REJECT_R {
        SOC_SLEEP_REJECT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn soc_wakeup(&self) -> SOC_WAKEUP_R {
        SOC_WAKEUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("lp_cpu_exc", &self.lp_cpu_exc())
            .field("sdio_idle", &self.sdio_idle())
            .field("sw", &self.sw())
            .field("soc_sleep_reject", &self.soc_sleep_reject())
            .field("soc_wakeup", &self.soc_wakeup())
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
