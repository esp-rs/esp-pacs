#[doc = "Register `SYS_CTRL` reader"]
pub type R = crate::R<SYS_CTRL_SPEC>;
#[doc = "Register `SYS_CTRL` writer"]
pub type W = crate::W<SYS_CTRL_SPEC>;
#[doc = "Field `LP_CORE_DISABLE` reader - lp cpu disable"]
pub type LP_CORE_DISABLE_R = crate::BitReader;
#[doc = "Field `LP_CORE_DISABLE` writer - lp cpu disable"]
pub type LP_CORE_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_SW_RST` writer - digital system software reset bit"]
pub type SYS_SW_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` reader - need_des"]
pub type FORCE_DOWNLOAD_BOOT_R = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` writer - need_des"]
pub type FORCE_DOWNLOAD_BOOT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIG_FIB` reader - need_des"]
pub type DIG_FIB_R = crate::FieldReader;
#[doc = "Field `DIG_FIB` writer - need_des"]
pub type DIG_FIB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IO_MUX_RESET_DISABLE` reader - reset disable bit for LP IOMUX"]
pub type IO_MUX_RESET_DISABLE_R = crate::BitReader;
#[doc = "Field `IO_MUX_RESET_DISABLE` writer - reset disable bit for LP IOMUX"]
pub type IO_MUX_RESET_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_FIB` reader - need_des"]
pub type ANA_FIB_R = crate::FieldReader;
#[doc = "Field `LP_FIB_SEL` reader - need_des"]
pub type LP_FIB_SEL_R = crate::FieldReader;
#[doc = "Field `LP_FIB_SEL` writer - need_des"]
pub type LP_FIB_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_CORE_ETM_WAKEUP_FLAG_CLR` writer - need_des"]
pub type LP_CORE_ETM_WAKEUP_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CORE_ETM_WAKEUP_FLAG` reader - need_des"]
pub type LP_CORE_ETM_WAKEUP_FLAG_R = crate::BitReader;
#[doc = "Field `LP_CORE_ETM_WAKEUP_FLAG` writer - need_des"]
pub type LP_CORE_ETM_WAKEUP_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_STALL_SEL` reader - 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1"]
pub type SYSTIMER_STALL_SEL_R = crate::BitReader;
#[doc = "Field `SYSTIMER_STALL_SEL` writer - 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1"]
pub type SYSTIMER_STALL_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lp cpu disable"]
    #[inline(always)]
    pub fn lp_core_disable(&self) -> LP_CORE_DISABLE_R {
        LP_CORE_DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_download_boot(&self) -> FORCE_DOWNLOAD_BOOT_R {
        FORCE_DOWNLOAD_BOOT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn dig_fib(&self) -> DIG_FIB_R {
        DIG_FIB_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - reset disable bit for LP IOMUX"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IO_MUX_RESET_DISABLE_R {
        IO_MUX_RESET_DISABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:20 - need_des"]
    #[inline(always)]
    pub fn ana_fib(&self) -> ANA_FIB_R {
        ANA_FIB_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    pub fn lp_fib_sel(&self) -> LP_FIB_SEL_R {
        LP_FIB_SEL_R::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_core_etm_wakeup_flag(&self) -> LP_CORE_ETM_WAKEUP_FLAG_R {
        LP_CORE_ETM_WAKEUP_FLAG_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1"]
    #[inline(always)]
    pub fn systimer_stall_sel(&self) -> SYSTIMER_STALL_SEL_R {
        SYSTIMER_STALL_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_CTRL")
            .field("lp_core_disable", &self.lp_core_disable())
            .field("force_download_boot", &self.force_download_boot())
            .field("dig_fib", &self.dig_fib())
            .field("io_mux_reset_disable", &self.io_mux_reset_disable())
            .field("ana_fib", &self.ana_fib())
            .field("lp_fib_sel", &self.lp_fib_sel())
            .field("lp_core_etm_wakeup_flag", &self.lp_core_etm_wakeup_flag())
            .field("systimer_stall_sel", &self.systimer_stall_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - lp cpu disable"]
    #[inline(always)]
    pub fn lp_core_disable(&mut self) -> LP_CORE_DISABLE_W<'_, SYS_CTRL_SPEC> {
        LP_CORE_DISABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - digital system software reset bit"]
    #[inline(always)]
    pub fn sys_sw_rst(&mut self) -> SYS_SW_RST_W<'_, SYS_CTRL_SPEC> {
        SYS_SW_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_download_boot(&mut self) -> FORCE_DOWNLOAD_BOOT_W<'_, SYS_CTRL_SPEC> {
        FORCE_DOWNLOAD_BOOT_W::new(self, 2)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn dig_fib(&mut self) -> DIG_FIB_W<'_, SYS_CTRL_SPEC> {
        DIG_FIB_W::new(self, 3)
    }
    #[doc = "Bit 11 - reset disable bit for LP IOMUX"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&mut self) -> IO_MUX_RESET_DISABLE_W<'_, SYS_CTRL_SPEC> {
        IO_MUX_RESET_DISABLE_W::new(self, 11)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    pub fn lp_fib_sel(&mut self) -> LP_FIB_SEL_W<'_, SYS_CTRL_SPEC> {
        LP_FIB_SEL_W::new(self, 21)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_core_etm_wakeup_flag_clr(
        &mut self,
    ) -> LP_CORE_ETM_WAKEUP_FLAG_CLR_W<'_, SYS_CTRL_SPEC> {
        LP_CORE_ETM_WAKEUP_FLAG_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_core_etm_wakeup_flag(&mut self) -> LP_CORE_ETM_WAKEUP_FLAG_W<'_, SYS_CTRL_SPEC> {
        LP_CORE_ETM_WAKEUP_FLAG_W::new(self, 30)
    }
    #[doc = "Bit 31 - 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1"]
    #[inline(always)]
    pub fn systimer_stall_sel(&mut self) -> SYSTIMER_STALL_SEL_W<'_, SYS_CTRL_SPEC> {
        SYSTIMER_STALL_SEL_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_CTRL_SPEC;
impl crate::RegisterSpec for SYS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctrl::R`](R) reader structure"]
impl crate::Readable for SYS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_ctrl::W`](W) writer structure"]
impl crate::Writable for SYS_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_CTRL to value 0x1fff_c7f8"]
impl crate::Resettable for SYS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1fff_c7f8;
}
