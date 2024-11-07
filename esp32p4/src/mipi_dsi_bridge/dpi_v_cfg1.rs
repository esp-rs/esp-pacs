#[doc = "Register `DPI_V_CFG1` reader"]
pub type R = crate::R<DPI_V_CFG1_SPEC>;
#[doc = "Register `DPI_V_CFG1` writer"]
pub type W = crate::W<DPI_V_CFG1_SPEC>;
#[doc = "Field `VBANK` reader - this field configures the length between vsync and valid line (by line) for dpi output"]
pub type VBANK_R = crate::FieldReader<u16>;
#[doc = "Field `VBANK` writer - this field configures the length between vsync and valid line (by line) for dpi output"]
pub type VBANK_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `VSYNC` reader - this field configures the length of vsync (by line) for dpi output"]
pub type VSYNC_R = crate::FieldReader<u16>;
#[doc = "Field `VSYNC` writer - this field configures the length of vsync (by line) for dpi output"]
pub type VSYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures the length between vsync and valid line (by line) for dpi output"]
    #[inline(always)]
    pub fn vbank(&self) -> VBANK_R {
        VBANK_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures the length of vsync (by line) for dpi output"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_V_CFG1")
            .field("vbank", &self.vbank())
            .field("vsync", &self.vsync())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures the length between vsync and valid line (by line) for dpi output"]
    #[inline(always)]
    pub fn vbank(&mut self) -> VBANK_W<DPI_V_CFG1_SPEC> {
        VBANK_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures the length of vsync (by line) for dpi output"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VSYNC_W<DPI_V_CFG1_SPEC> {
        VSYNC_W::new(self, 16)
    }
}
#[doc = "dsi bridge dpi v config register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_v_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_v_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_V_CFG1_SPEC;
impl crate::RegisterSpec for DPI_V_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_v_cfg1::R`](R) reader structure"]
impl crate::Readable for DPI_V_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpi_v_cfg1::W`](W) writer structure"]
impl crate::Writable for DPI_V_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_V_CFG1 to value 0x0002_0021"]
impl crate::Resettable for DPI_V_CFG1_SPEC {
    const RESET_VALUE: u32 = 0x0002_0021;
}
