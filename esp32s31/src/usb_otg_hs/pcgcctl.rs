#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PCGCCTL_SPEC>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PCGCCTL_SPEC>;
#[doc = "Field `STOPPCLK` reader - Stop Pclk (StopPclk) - The application sets this bit to stop the PHY clock (phy_clk) when the USB is suspended, the session is not valid, or the device is disconnected. - The application clears this bit when the USB is resumed or a new session starts."]
pub type STOPPCLK_R = crate::BitReader;
#[doc = "Field `STOPPCLK` writer - Stop Pclk (StopPclk) - The application sets this bit to stop the PHY clock (phy_clk) when the USB is suspended, the session is not valid, or the device is disconnected. - The application clears this bit when the USB is resumed or a new session starts."]
pub type STOPPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATEHCLK` reader - Gate Hclk (GateHclk) - The application sets this bit to gate hclk to modules other than the AHB Slave and Master and wakeup logic when the USB is suspended or the session is not valid. - The application clears this bit when the USB is resumed or a new session starts."]
pub type GATEHCLK_R = crate::BitReader;
#[doc = "Field `GATEHCLK` writer - Gate Hclk (GateHclk) - The application sets this bit to gate hclk to modules other than the AHB Slave and Master and wakeup logic when the USB is suspended or the session is not valid. - The application clears this bit when the USB is resumed or a new session starts."]
pub type GATEHCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRCLMP` reader - Power Clamp (PwrClmp) This bit is valid only in Partial Power-Down mode - The application sets this bit before the power is turned off to clamp the signals between the power-on modules and the power-off modules. - The application clears the bit to disable the clamping before the power is turned on."]
pub type PWRCLMP_R = crate::BitReader;
#[doc = "Field `PWRCLMP` writer - Power Clamp (PwrClmp) This bit is valid only in Partial Power-Down mode - The application sets this bit before the power is turned off to clamp the signals between the power-on modules and the power-off modules. - The application clears the bit to disable the clamping before the power is turned on."]
pub type PWRCLMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTPDWNMODULE` reader - Reset Power-Down Modules (RstPdwnModule) This bit is valid only in Partial Power-Down mode. - The application sets this bit when the power is turned off. - The application clears this bit after the power is turned on and the PHY clock is up. Note: The R/W of all core registers are possible only when this bit is set to 1b0."]
pub type RSTPDWNMODULE_R = crate::BitReader;
#[doc = "Field `RSTPDWNMODULE` writer - Reset Power-Down Modules (RstPdwnModule) This bit is valid only in Partial Power-Down mode. - The application sets this bit when the power is turned off. - The application clears this bit after the power is turned on and the PHY clock is up. Note: The R/W of all core registers are possible only when this bit is set to 1b0."]
pub type RSTPDWNMODULE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSLEEP` reader - PHY In Sleep Indicates that the PHY is in Sleep State."]
pub type PHYSLEEP_R = crate::BitReader;
#[doc = "Field `L1SUSPENDED` reader - L1 Deep Sleep Indicates that the PHY is in deep sleep when in L1 state."]
pub type L1SUSPENDED_R = crate::BitReader;
#[doc = "Field `RESETAFTERSUSP` reader - Reset after suspend Applicable in Partial power-down mode In partial power-down mode of operation, this bit needs to be set in host mode before clamp is removed if the host needs to issue reset after suspend. If this bit is not set, then the host issues resume after suspend. This bit is not applicable in device mode and non-partial power-down mode. In Hibernation mode, this bit needs to be set at RESTORE_POINT before PCGCCTL.EssRegRestored is set. In this case, PCGCCTL.restore_mode needs to be set to wait_restore."]
pub type RESETAFTERSUSP_R = crate::BitReader;
#[doc = "Field `RESETAFTERSUSP` writer - Reset after suspend Applicable in Partial power-down mode In partial power-down mode of operation, this bit needs to be set in host mode before clamp is removed if the host needs to issue reset after suspend. If this bit is not set, then the host issues resume after suspend. This bit is not applicable in device mode and non-partial power-down mode. In Hibernation mode, this bit needs to be set at RESTORE_POINT before PCGCCTL.EssRegRestored is set. In this case, PCGCCTL.restore_mode needs to be set to wait_restore."]
pub type RESETAFTERSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop Pclk (StopPclk) - The application sets this bit to stop the PHY clock (phy_clk) when the USB is suspended, the session is not valid, or the device is disconnected. - The application clears this bit when the USB is resumed or a new session starts."]
    #[inline(always)]
    pub fn stoppclk(&self) -> STOPPCLK_R {
        STOPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate Hclk (GateHclk) - The application sets this bit to gate hclk to modules other than the AHB Slave and Master and wakeup logic when the USB is suspended or the session is not valid. - The application clears this bit when the USB is resumed or a new session starts."]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Clamp (PwrClmp) This bit is valid only in Partial Power-Down mode - The application sets this bit before the power is turned off to clamp the signals between the power-on modules and the power-off modules. - The application clears the bit to disable the clamping before the power is turned on."]
    #[inline(always)]
    pub fn pwrclmp(&self) -> PWRCLMP_R {
        PWRCLMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Power-Down Modules (RstPdwnModule) This bit is valid only in Partial Power-Down mode. - The application sets this bit when the power is turned off. - The application clears this bit after the power is turned on and the PHY clock is up. Note: The R/W of all core registers are possible only when this bit is set to 1b0."]
    #[inline(always)]
    pub fn rstpdwnmodule(&self) -> RSTPDWNMODULE_R {
        RSTPDWNMODULE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - PHY In Sleep Indicates that the PHY is in Sleep State."]
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - L1 Deep Sleep Indicates that the PHY is in deep sleep when in L1 state."]
    #[inline(always)]
    pub fn l1suspended(&self) -> L1SUSPENDED_R {
        L1SUSPENDED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset after suspend Applicable in Partial power-down mode In partial power-down mode of operation, this bit needs to be set in host mode before clamp is removed if the host needs to issue reset after suspend. If this bit is not set, then the host issues resume after suspend. This bit is not applicable in device mode and non-partial power-down mode. In Hibernation mode, this bit needs to be set at RESTORE_POINT before PCGCCTL.EssRegRestored is set. In this case, PCGCCTL.restore_mode needs to be set to wait_restore."]
    #[inline(always)]
    pub fn resetaftersusp(&self) -> RESETAFTERSUSP_R {
        RESETAFTERSUSP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCGCCTL")
            .field("stoppclk", &self.stoppclk())
            .field("gatehclk", &self.gatehclk())
            .field("pwrclmp", &self.pwrclmp())
            .field("rstpdwnmodule", &self.rstpdwnmodule())
            .field("physleep", &self.physleep())
            .field("l1suspended", &self.l1suspended())
            .field("resetaftersusp", &self.resetaftersusp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stop Pclk (StopPclk) - The application sets this bit to stop the PHY clock (phy_clk) when the USB is suspended, the session is not valid, or the device is disconnected. - The application clears this bit when the USB is resumed or a new session starts."]
    #[inline(always)]
    pub fn stoppclk(&mut self) -> STOPPCLK_W<'_, PCGCCTL_SPEC> {
        STOPPCLK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gate Hclk (GateHclk) - The application sets this bit to gate hclk to modules other than the AHB Slave and Master and wakeup logic when the USB is suspended or the session is not valid. - The application clears this bit when the USB is resumed or a new session starts."]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<'_, PCGCCTL_SPEC> {
        GATEHCLK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Power Clamp (PwrClmp) This bit is valid only in Partial Power-Down mode - The application sets this bit before the power is turned off to clamp the signals between the power-on modules and the power-off modules. - The application clears the bit to disable the clamping before the power is turned on."]
    #[inline(always)]
    pub fn pwrclmp(&mut self) -> PWRCLMP_W<'_, PCGCCTL_SPEC> {
        PWRCLMP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Power-Down Modules (RstPdwnModule) This bit is valid only in Partial Power-Down mode. - The application sets this bit when the power is turned off. - The application clears this bit after the power is turned on and the PHY clock is up. Note: The R/W of all core registers are possible only when this bit is set to 1b0."]
    #[inline(always)]
    pub fn rstpdwnmodule(&mut self) -> RSTPDWNMODULE_W<'_, PCGCCTL_SPEC> {
        RSTPDWNMODULE_W::new(self, 3)
    }
    #[doc = "Bit 8 - Reset after suspend Applicable in Partial power-down mode In partial power-down mode of operation, this bit needs to be set in host mode before clamp is removed if the host needs to issue reset after suspend. If this bit is not set, then the host issues resume after suspend. This bit is not applicable in device mode and non-partial power-down mode. In Hibernation mode, this bit needs to be set at RESTORE_POINT before PCGCCTL.EssRegRestored is set. In this case, PCGCCTL.restore_mode needs to be set to wait_restore."]
    #[inline(always)]
    pub fn resetaftersusp(&mut self) -> RESETAFTERSUSP_W<'_, PCGCCTL_SPEC> {
        RESETAFTERSUSP_W::new(self, 8)
    }
}
#[doc = "This register is used to control the Power and Clock Gating characteristics of the controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCGCCTL_SPEC;
impl crate::RegisterSpec for PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PCGCCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PCGCCTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCGCCTL to value 0"]
impl crate::Resettable for PCGCCTL_SPEC {}
