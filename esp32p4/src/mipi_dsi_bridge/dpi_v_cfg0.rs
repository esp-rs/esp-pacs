#[doc = "Register `DPI_V_CFG0` reader"]
pub type R = crate::R<DPI_V_CFG0_SPEC>;
#[doc = "Register `DPI_V_CFG0` writer"]
pub type W = crate::W<DPI_V_CFG0_SPEC>;
#[doc = "Field `VTOTAL` reader - this field configures the total length of one frame (by line) for dpi output, must meet: reg_vtotal > reg_vdisp+reg_vsync+reg_vbank"]
pub type VTOTAL_R = crate::FieldReader<u16>;
#[doc = "Field `VTOTAL` writer - this field configures the total length of one frame (by line) for dpi output, must meet: reg_vtotal > reg_vdisp+reg_vsync+reg_vbank"]
pub type VTOTAL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `VDISP` reader - this field configures the length of valid line (by line) for dpi output"]
pub type VDISP_R = crate::FieldReader<u16>;
#[doc = "Field `VDISP` writer - this field configures the length of valid line (by line) for dpi output"]
pub type VDISP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures the total length of one frame (by line) for dpi output, must meet: reg_vtotal > reg_vdisp+reg_vsync+reg_vbank"]
    #[inline(always)]
    pub fn vtotal(&self) -> VTOTAL_R {
        VTOTAL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures the length of valid line (by line) for dpi output"]
    #[inline(always)]
    pub fn vdisp(&self) -> VDISP_R {
        VDISP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_V_CFG0")
            .field("vtotal", &self.vtotal())
            .field("vdisp", &self.vdisp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures the total length of one frame (by line) for dpi output, must meet: reg_vtotal > reg_vdisp+reg_vsync+reg_vbank"]
    #[inline(always)]
    #[must_use]
    pub fn vtotal(&mut self) -> VTOTAL_W<DPI_V_CFG0_SPEC> {
        VTOTAL_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures the length of valid line (by line) for dpi output"]
    #[inline(always)]
    #[must_use]
    pub fn vdisp(&mut self) -> VDISP_W<DPI_V_CFG0_SPEC> {
        VDISP_W::new(self, 16)
    }
}
#[doc = "dsi bridge dpi v config register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_v_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_v_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_V_CFG0_SPEC;
impl crate::RegisterSpec for DPI_V_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_v_cfg0::R`](R) reader structure"]
impl crate::Readable for DPI_V_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpi_v_cfg0::W`](W) writer structure"]
impl crate::Writable for DPI_V_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_V_CFG0 to value 0x01e0_020d"]
impl crate::Resettable for DPI_V_CFG0_SPEC {
    const RESET_VALUE: u32 = 0x01e0_020d;
}
