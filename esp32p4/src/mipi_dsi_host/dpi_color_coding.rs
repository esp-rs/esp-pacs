#[doc = "Register `DPI_COLOR_CODING` reader"]
pub type R = crate::R<DPI_COLOR_CODING_SPEC>;
#[doc = "Register `DPI_COLOR_CODING` writer"]
pub type W = crate::W<DPI_COLOR_CODING_SPEC>;
#[doc = "Field `DPI_COLOR_CODING` reader - NA"]
pub type DPI_COLOR_CODING_R = crate::FieldReader;
#[doc = "Field `DPI_COLOR_CODING` writer - NA"]
pub type DPI_COLOR_CODING_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOOSELY18_EN` reader - NA"]
pub type LOOSELY18_EN_R = crate::BitReader;
#[doc = "Field `LOOSELY18_EN` writer - NA"]
pub type LOOSELY18_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn dpi_color_coding(&self) -> DPI_COLOR_CODING_R {
        DPI_COLOR_CODING_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn loosely18_en(&self) -> LOOSELY18_EN_R {
        LOOSELY18_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_COLOR_CODING")
            .field("dpi_color_coding", &self.dpi_color_coding())
            .field("loosely18_en", &self.loosely18_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn dpi_color_coding(&mut self) -> DPI_COLOR_CODING_W<DPI_COLOR_CODING_SPEC> {
        DPI_COLOR_CODING_W::new(self, 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn loosely18_en(&mut self) -> LOOSELY18_EN_W<DPI_COLOR_CODING_SPEC> {
        LOOSELY18_EN_W::new(self, 8)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_color_coding::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_color_coding::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_COLOR_CODING_SPEC;
impl crate::RegisterSpec for DPI_COLOR_CODING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_color_coding::R`](R) reader structure"]
impl crate::Readable for DPI_COLOR_CODING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpi_color_coding::W`](W) writer structure"]
impl crate::Writable for DPI_COLOR_CODING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_COLOR_CODING to value 0"]
impl crate::Resettable for DPI_COLOR_CODING_SPEC {
    const RESET_VALUE: u32 = 0;
}
