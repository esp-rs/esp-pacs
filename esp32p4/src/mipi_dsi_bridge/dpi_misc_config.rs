#[doc = "Register `DPI_MISC_CONFIG` reader"]
pub type R = crate::R<DPI_MISC_CONFIG_SPEC>;
#[doc = "Register `DPI_MISC_CONFIG` writer"]
pub type W = crate::W<DPI_MISC_CONFIG_SPEC>;
#[doc = "Field `DPI_EN` reader - this bit configures enable of dpi output, 0: disable, 1: enable"]
pub type DPI_EN_R = crate::BitReader;
#[doc = "Field `DPI_EN` writer - this bit configures enable of dpi output, 0: disable, 1: enable"]
pub type DPI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_UNDERRUN_DISCARD_VCNT` reader - this field configures the underrun interrupt musk, when underrun occurs and line cnt is less then this field"]
pub type FIFO_UNDERRUN_DISCARD_VCNT_R = crate::FieldReader<u16>;
#[doc = "Field `FIFO_UNDERRUN_DISCARD_VCNT` writer - this field configures the underrun interrupt musk, when underrun occurs and line cnt is less then this field"]
pub type FIFO_UNDERRUN_DISCARD_VCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - this bit configures enable of dpi output, 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dpi_en(&self) -> DPI_EN_R {
        DPI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - this field configures the underrun interrupt musk, when underrun occurs and line cnt is less then this field"]
    #[inline(always)]
    pub fn fifo_underrun_discard_vcnt(&self) -> FIFO_UNDERRUN_DISCARD_VCNT_R {
        FIFO_UNDERRUN_DISCARD_VCNT_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_MISC_CONFIG")
            .field("dpi_en", &self.dpi_en())
            .field(
                "fifo_underrun_discard_vcnt",
                &self.fifo_underrun_discard_vcnt(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures enable of dpi output, 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpi_en(&mut self) -> DPI_EN_W<DPI_MISC_CONFIG_SPEC> {
        DPI_EN_W::new(self, 0)
    }
    #[doc = "Bits 4:15 - this field configures the underrun interrupt musk, when underrun occurs and line cnt is less then this field"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_underrun_discard_vcnt(
        &mut self,
    ) -> FIFO_UNDERRUN_DISCARD_VCNT_W<DPI_MISC_CONFIG_SPEC> {
        FIFO_UNDERRUN_DISCARD_VCNT_W::new(self, 4)
    }
}
#[doc = "dsi_bridge dpi misc config register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_misc_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_misc_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_MISC_CONFIG_SPEC;
impl crate::RegisterSpec for DPI_MISC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_misc_config::R`](R) reader structure"]
impl crate::Readable for DPI_MISC_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpi_misc_config::W`](W) writer structure"]
impl crate::Writable for DPI_MISC_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_MISC_CONFIG to value 0x19d0"]
impl crate::Resettable for DPI_MISC_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x19d0;
}
