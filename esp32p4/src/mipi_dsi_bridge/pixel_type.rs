#[doc = "Register `PIXEL_TYPE` reader"]
pub type R = crate::R<PIXEL_TYPE_SPEC>;
#[doc = "Register `PIXEL_TYPE` writer"]
pub type W = crate::W<PIXEL_TYPE_SPEC>;
#[doc = "Field `RAW_TYPE` reader - this field configures the pixel type. 0: rgb888, 1:rgb666, 2:rgb565"]
pub type RAW_TYPE_R = crate::FieldReader;
#[doc = "Field `RAW_TYPE` writer - this field configures the pixel type. 0: rgb888, 1:rgb666, 2:rgb565"]
pub type RAW_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPI_CONFIG` reader - this field configures the pixel arrange type of dpi interface"]
pub type DPI_CONFIG_R = crate::FieldReader;
#[doc = "Field `DPI_CONFIG` writer - this field configures the pixel arrange type of dpi interface"]
pub type DPI_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATA_IN_TYPE` reader - input data type, 0: rgb, 1: yuv"]
pub type DATA_IN_TYPE_R = crate::BitReader;
#[doc = "Field `DATA_IN_TYPE` writer - input data type, 0: rgb, 1: yuv"]
pub type DATA_IN_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - this field configures the pixel type. 0: rgb888, 1:rgb666, 2:rgb565"]
    #[inline(always)]
    pub fn raw_type(&self) -> RAW_TYPE_R {
        RAW_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - this field configures the pixel arrange type of dpi interface"]
    #[inline(always)]
    pub fn dpi_config(&self) -> DPI_CONFIG_R {
        DPI_CONFIG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - input data type, 0: rgb, 1: yuv"]
    #[inline(always)]
    pub fn data_in_type(&self) -> DATA_IN_TYPE_R {
        DATA_IN_TYPE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIXEL_TYPE")
            .field("raw_type", &self.raw_type())
            .field("dpi_config", &self.dpi_config())
            .field("data_in_type", &self.data_in_type())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - this field configures the pixel type. 0: rgb888, 1:rgb666, 2:rgb565"]
    #[inline(always)]
    pub fn raw_type(&mut self) -> RAW_TYPE_W<PIXEL_TYPE_SPEC> {
        RAW_TYPE_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - this field configures the pixel arrange type of dpi interface"]
    #[inline(always)]
    pub fn dpi_config(&mut self) -> DPI_CONFIG_W<PIXEL_TYPE_SPEC> {
        DPI_CONFIG_W::new(self, 4)
    }
    #[doc = "Bit 6 - input data type, 0: rgb, 1: yuv"]
    #[inline(always)]
    pub fn data_in_type(&mut self) -> DATA_IN_TYPE_W<PIXEL_TYPE_SPEC> {
        DATA_IN_TYPE_W::new(self, 6)
    }
}
#[doc = "dsi bridge dpi type control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pixel_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pixel_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIXEL_TYPE_SPEC;
impl crate::RegisterSpec for PIXEL_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pixel_type::R`](R) reader structure"]
impl crate::Readable for PIXEL_TYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pixel_type::W`](W) writer structure"]
impl crate::Writable for PIXEL_TYPE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIXEL_TYPE to value 0"]
impl crate::Resettable for PIXEL_TYPE_SPEC {
    const RESET_VALUE: u32 = 0;
}
