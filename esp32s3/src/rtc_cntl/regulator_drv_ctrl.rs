#[doc = "Register `REGULATOR_DRV_CTRL` reader"]
pub type R = crate::R<REGULATOR_DRV_CTRL_SPEC>;
#[doc = "Register `REGULATOR_DRV_CTRL` writer"]
pub type W = crate::W<REGULATOR_DRV_CTRL_SPEC>;
#[doc = "Field `REGULATOR_DRV_B_MONITOR` reader - No public"]
pub type REGULATOR_DRV_B_MONITOR_R = crate::FieldReader;
#[doc = "Field `REGULATOR_DRV_B_MONITOR` writer - No public"]
pub type REGULATOR_DRV_B_MONITOR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REGULATOR_DRV_B_SLP` reader - No public"]
pub type REGULATOR_DRV_B_SLP_R = crate::FieldReader;
#[doc = "Field `REGULATOR_DRV_B_SLP` writer - No public"]
pub type REGULATOR_DRV_B_SLP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DG_VDD_DRV_B_SLP` reader - No public"]
pub type DG_VDD_DRV_B_SLP_R = crate::FieldReader;
#[doc = "Field `DG_VDD_DRV_B_SLP` writer - No public"]
pub type DG_VDD_DRV_B_SLP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DG_VDD_DRV_B_MONITOR` reader - No public"]
pub type DG_VDD_DRV_B_MONITOR_R = crate::FieldReader;
#[doc = "Field `DG_VDD_DRV_B_MONITOR` writer - No public"]
pub type DG_VDD_DRV_B_MONITOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - No public"]
    #[inline(always)]
    pub fn regulator_drv_b_monitor(&self) -> REGULATOR_DRV_B_MONITOR_R {
        REGULATOR_DRV_B_MONITOR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - No public"]
    #[inline(always)]
    pub fn regulator_drv_b_slp(&self) -> REGULATOR_DRV_B_SLP_R {
        REGULATOR_DRV_B_SLP_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - No public"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp(&self) -> DG_VDD_DRV_B_SLP_R {
        DG_VDD_DRV_B_SLP_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:27 - No public"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_monitor(&self) -> DG_VDD_DRV_B_MONITOR_R {
        DG_VDD_DRV_B_MONITOR_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGULATOR_DRV_CTRL")
            .field(
                "regulator_drv_b_monitor",
                &self.regulator_drv_b_monitor().bits(),
            )
            .field("regulator_drv_b_slp", &self.regulator_drv_b_slp().bits())
            .field("dg_vdd_drv_b_slp", &self.dg_vdd_drv_b_slp().bits())
            .field("dg_vdd_drv_b_monitor", &self.dg_vdd_drv_b_monitor().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGULATOR_DRV_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn regulator_drv_b_monitor(
        &mut self,
    ) -> REGULATOR_DRV_B_MONITOR_W<REGULATOR_DRV_CTRL_SPEC> {
        REGULATOR_DRV_B_MONITOR_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn regulator_drv_b_slp(&mut self) -> REGULATOR_DRV_B_SLP_W<REGULATOR_DRV_CTRL_SPEC> {
        REGULATOR_DRV_B_SLP_W::new(self, 6)
    }
    #[doc = "Bits 12:19 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn dg_vdd_drv_b_slp(&mut self) -> DG_VDD_DRV_B_SLP_W<REGULATOR_DRV_CTRL_SPEC> {
        DG_VDD_DRV_B_SLP_W::new(self, 12)
    }
    #[doc = "Bits 20:27 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn dg_vdd_drv_b_monitor(&mut self) -> DG_VDD_DRV_B_MONITOR_W<REGULATOR_DRV_CTRL_SPEC> {
        DG_VDD_DRV_B_MONITOR_W::new(self, 20)
    }
}
#[doc = "No public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regulator_drv_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regulator_drv_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGULATOR_DRV_CTRL_SPEC;
impl crate::RegisterSpec for REGULATOR_DRV_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regulator_drv_ctrl::R`](R) reader structure"]
impl crate::Readable for REGULATOR_DRV_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regulator_drv_ctrl::W`](W) writer structure"]
impl crate::Writable for REGULATOR_DRV_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGULATOR_DRV_CTRL to value 0"]
impl crate::Resettable for REGULATOR_DRV_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
