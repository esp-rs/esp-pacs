#[doc = "Register `DPI_H_CFG1` reader"]
pub type R = crate::R<DPI_H_CFG1_SPEC>;
#[doc = "Register `DPI_H_CFG1` writer"]
pub type W = crate::W<DPI_H_CFG1_SPEC>;
#[doc = "Field `HBANK` reader - this field configures the length between hsync and pixel data valid (by pixel num) for dpi output"]
pub type HBANK_R = crate::FieldReader<u16>;
#[doc = "Field `HBANK` writer - this field configures the length between hsync and pixel data valid (by pixel num) for dpi output"]
pub type HBANK_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HSYNC` reader - this field configures the length of hsync (by pixel num) for dpi output"]
pub type HSYNC_R = crate::FieldReader<u16>;
#[doc = "Field `HSYNC` writer - this field configures the length of hsync (by pixel num) for dpi output"]
pub type HSYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures the length between hsync and pixel data valid (by pixel num) for dpi output"]
    #[inline(always)]
    pub fn hbank(&self) -> HBANK_R {
        HBANK_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures the length of hsync (by pixel num) for dpi output"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_H_CFG1")
            .field("hbank", &self.hbank())
            .field("hsync", &self.hsync())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures the length between hsync and pixel data valid (by pixel num) for dpi output"]
    #[inline(always)]
    pub fn hbank(&mut self) -> HBANK_W<'_, DPI_H_CFG1_SPEC> {
        HBANK_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures the length of hsync (by pixel num) for dpi output"]
    #[inline(always)]
    pub fn hsync(&mut self) -> HSYNC_W<'_, DPI_H_CFG1_SPEC> {
        HSYNC_W::new(self, 16)
    }
}
#[doc = "dsi bridge dpi h config register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_h_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_h_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_H_CFG1_SPEC;
impl crate::RegisterSpec for DPI_H_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_h_cfg1::R`](R) reader structure"]
impl crate::Readable for DPI_H_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpi_h_cfg1::W`](W) writer structure"]
impl crate::Writable for DPI_H_CFG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_H_CFG1 to value 0x0060_0030"]
impl crate::Resettable for DPI_H_CFG1_SPEC {
    const RESET_VALUE: u32 = 0x0060_0030;
}
