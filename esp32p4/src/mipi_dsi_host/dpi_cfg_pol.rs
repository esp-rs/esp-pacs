#[doc = "Register `DPI_CFG_POL` reader"]
pub type R = crate::R<DPI_CFG_POL_SPEC>;
#[doc = "Register `DPI_CFG_POL` writer"]
pub type W = crate::W<DPI_CFG_POL_SPEC>;
#[doc = "Field `DATAEN_ACTIVE_LOW` reader - NA"]
pub type DATAEN_ACTIVE_LOW_R = crate::BitReader;
#[doc = "Field `DATAEN_ACTIVE_LOW` writer - NA"]
pub type DATAEN_ACTIVE_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_ACTIVE_LOW` reader - NA"]
pub type VSYNC_ACTIVE_LOW_R = crate::BitReader;
#[doc = "Field `VSYNC_ACTIVE_LOW` writer - NA"]
pub type VSYNC_ACTIVE_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNC_ACTIVE_LOW` reader - NA"]
pub type HSYNC_ACTIVE_LOW_R = crate::BitReader;
#[doc = "Field `HSYNC_ACTIVE_LOW` writer - NA"]
pub type HSYNC_ACTIVE_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHUTD_ACTIVE_LOW` reader - NA"]
pub type SHUTD_ACTIVE_LOW_R = crate::BitReader;
#[doc = "Field `SHUTD_ACTIVE_LOW` writer - NA"]
pub type SHUTD_ACTIVE_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLORM_ACTIVE_LOW` reader - NA"]
pub type COLORM_ACTIVE_LOW_R = crate::BitReader;
#[doc = "Field `COLORM_ACTIVE_LOW` writer - NA"]
pub type COLORM_ACTIVE_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dataen_active_low(&self) -> DATAEN_ACTIVE_LOW_R {
        DATAEN_ACTIVE_LOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn vsync_active_low(&self) -> VSYNC_ACTIVE_LOW_R {
        VSYNC_ACTIVE_LOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn hsync_active_low(&self) -> HSYNC_ACTIVE_LOW_R {
        HSYNC_ACTIVE_LOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn shutd_active_low(&self) -> SHUTD_ACTIVE_LOW_R {
        SHUTD_ACTIVE_LOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn colorm_active_low(&self) -> COLORM_ACTIVE_LOW_R {
        COLORM_ACTIVE_LOW_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_CFG_POL")
            .field("dataen_active_low", &self.dataen_active_low().bit())
            .field("vsync_active_low", &self.vsync_active_low().bit())
            .field("hsync_active_low", &self.hsync_active_low().bit())
            .field("shutd_active_low", &self.shutd_active_low().bit())
            .field("colorm_active_low", &self.colorm_active_low().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DPI_CFG_POL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn dataen_active_low(&mut self) -> DATAEN_ACTIVE_LOW_W<DPI_CFG_POL_SPEC> {
        DATAEN_ACTIVE_LOW_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_active_low(&mut self) -> VSYNC_ACTIVE_LOW_W<DPI_CFG_POL_SPEC> {
        VSYNC_ACTIVE_LOW_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hsync_active_low(&mut self) -> HSYNC_ACTIVE_LOW_W<DPI_CFG_POL_SPEC> {
        HSYNC_ACTIVE_LOW_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn shutd_active_low(&mut self) -> SHUTD_ACTIVE_LOW_W<DPI_CFG_POL_SPEC> {
        SHUTD_ACTIVE_LOW_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn colorm_active_low(&mut self) -> COLORM_ACTIVE_LOW_W<DPI_CFG_POL_SPEC> {
        COLORM_ACTIVE_LOW_W::new(self, 4)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_cfg_pol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_cfg_pol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_CFG_POL_SPEC;
impl crate::RegisterSpec for DPI_CFG_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_cfg_pol::R`](R) reader structure"]
impl crate::Readable for DPI_CFG_POL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpi_cfg_pol::W`](W) writer structure"]
impl crate::Writable for DPI_CFG_POL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_CFG_POL to value 0"]
impl crate::Resettable for DPI_CFG_POL_SPEC {
    const RESET_VALUE: u32 = 0;
}
