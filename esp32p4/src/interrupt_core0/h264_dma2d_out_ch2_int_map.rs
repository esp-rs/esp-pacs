#[doc = "Register `H264_DMA2D_OUT_CH2_INT_MAP` reader"]
pub type R = crate::R<H264_DMA2D_OUT_CH2_INT_MAP_SPEC>;
#[doc = "Register `H264_DMA2D_OUT_CH2_INT_MAP` writer"]
pub type W = crate::W<H264_DMA2D_OUT_CH2_INT_MAP_SPEC>;
#[doc = "Field `H264_DMA2D_OUT_CH2_INT_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type H264_DMA2D_OUT_CH2_INT_MAP_R = crate::FieldReader;
#[doc = "Field `H264_DMA2D_OUT_CH2_INT_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type H264_DMA2D_OUT_CH2_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `H264_DMA2D_OUT_CH2_INT_SRC_PASS_IN_SEC` reader - NA"]
pub type H264_DMA2D_OUT_CH2_INT_SRC_PASS_IN_SEC_R = crate::BitReader;
#[doc = "Field `H264_DMA2D_OUT_CH2_INT_SRC_PASS_IN_SEC` writer - NA"]
pub type H264_DMA2D_OUT_CH2_INT_SRC_PASS_IN_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_DMA2D_OUT_CH2_INT_SRC_IN_SEC_FLAG` reader - NA"]
pub type H264_DMA2D_OUT_CH2_INT_SRC_IN_SEC_FLAG_R = crate::BitReader;
#[doc = "Field `H264_DMA2D_OUT_CH2_INT_SRC_IN_SEC_FLAG` writer - NA"]
pub type H264_DMA2D_OUT_CH2_INT_SRC_IN_SEC_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn h264_dma2d_out_ch2_int_map(&self) -> H264_DMA2D_OUT_CH2_INT_MAP_R {
        H264_DMA2D_OUT_CH2_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn h264_dma2d_out_ch2_int_src_pass_in_sec(
        &self,
    ) -> H264_DMA2D_OUT_CH2_INT_SRC_PASS_IN_SEC_R {
        H264_DMA2D_OUT_CH2_INT_SRC_PASS_IN_SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn h264_dma2d_out_ch2_int_src_in_sec_flag(
        &self,
    ) -> H264_DMA2D_OUT_CH2_INT_SRC_IN_SEC_FLAG_R {
        H264_DMA2D_OUT_CH2_INT_SRC_IN_SEC_FLAG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("H264_DMA2D_OUT_CH2_INT_MAP")
            .field(
                "h264_dma2d_out_ch2_int_map",
                &self.h264_dma2d_out_ch2_int_map(),
            )
            .field(
                "h264_dma2d_out_ch2_int_src_pass_in_sec",
                &self.h264_dma2d_out_ch2_int_src_pass_in_sec(),
            )
            .field(
                "h264_dma2d_out_ch2_int_src_in_sec_flag",
                &self.h264_dma2d_out_ch2_int_src_in_sec_flag(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn h264_dma2d_out_ch2_int_map(
        &mut self,
    ) -> H264_DMA2D_OUT_CH2_INT_MAP_W<'_, H264_DMA2D_OUT_CH2_INT_MAP_SPEC> {
        H264_DMA2D_OUT_CH2_INT_MAP_W::new(self, 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn h264_dma2d_out_ch2_int_src_pass_in_sec(
        &mut self,
    ) -> H264_DMA2D_OUT_CH2_INT_SRC_PASS_IN_SEC_W<'_, H264_DMA2D_OUT_CH2_INT_MAP_SPEC> {
        H264_DMA2D_OUT_CH2_INT_SRC_PASS_IN_SEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn h264_dma2d_out_ch2_int_src_in_sec_flag(
        &mut self,
    ) -> H264_DMA2D_OUT_CH2_INT_SRC_IN_SEC_FLAG_W<'_, H264_DMA2D_OUT_CH2_INT_MAP_SPEC> {
        H264_DMA2D_OUT_CH2_INT_SRC_IN_SEC_FLAG_W::new(self, 7)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`h264_dma2d_out_ch2_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`h264_dma2d_out_ch2_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct H264_DMA2D_OUT_CH2_INT_MAP_SPEC;
impl crate::RegisterSpec for H264_DMA2D_OUT_CH2_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h264_dma2d_out_ch2_int_map::R`](R) reader structure"]
impl crate::Readable for H264_DMA2D_OUT_CH2_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`h264_dma2d_out_ch2_int_map::W`](W) writer structure"]
impl crate::Writable for H264_DMA2D_OUT_CH2_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets H264_DMA2D_OUT_CH2_INT_MAP to value 0"]
impl crate::Resettable for H264_DMA2D_OUT_CH2_INT_MAP_SPEC {}
