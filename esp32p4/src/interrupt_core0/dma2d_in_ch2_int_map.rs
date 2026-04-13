#[doc = "Register `DMA2D_IN_CH2_INT_MAP` reader"]
pub type R = crate::R<DMA2D_IN_CH2_INT_MAP_SPEC>;
#[doc = "Register `DMA2D_IN_CH2_INT_MAP` writer"]
pub type W = crate::W<DMA2D_IN_CH2_INT_MAP_SPEC>;
#[doc = "Field `CORE0_DMA2D_IN_CH2_INT_MAP` reader - "]
pub type CORE0_DMA2D_IN_CH2_INT_MAP_R = crate::FieldReader;
#[doc = "Field `CORE0_DMA2D_IN_CH2_INT_MAP` writer - "]
pub type CORE0_DMA2D_IN_CH2_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CORE0_DMA2D_IN_CH2_INT_SRC_PASS_IN_SEC` reader - "]
pub type CORE0_DMA2D_IN_CH2_INT_SRC_PASS_IN_SEC_R = crate::BitReader;
#[doc = "Field `CORE0_DMA2D_IN_CH2_INT_SRC_PASS_IN_SEC` writer - "]
pub type CORE0_DMA2D_IN_CH2_INT_SRC_PASS_IN_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_DMA2D_IN_CH2_INT_SRC_IN_SEC_FLAG` reader - "]
pub type CORE0_DMA2D_IN_CH2_INT_SRC_IN_SEC_FLAG_R = crate::BitReader;
#[doc = "Field `CORE0_DMA2D_IN_CH2_INT_SRC_IN_SEC_FLAG` writer - "]
pub type CORE0_DMA2D_IN_CH2_INT_SRC_IN_SEC_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn core0_dma2d_in_ch2_int_map(&self) -> CORE0_DMA2D_IN_CH2_INT_MAP_R {
        CORE0_DMA2D_IN_CH2_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn core0_dma2d_in_ch2_int_src_pass_in_sec(
        &self,
    ) -> CORE0_DMA2D_IN_CH2_INT_SRC_PASS_IN_SEC_R {
        CORE0_DMA2D_IN_CH2_INT_SRC_PASS_IN_SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn core0_dma2d_in_ch2_int_src_in_sec_flag(
        &self,
    ) -> CORE0_DMA2D_IN_CH2_INT_SRC_IN_SEC_FLAG_R {
        CORE0_DMA2D_IN_CH2_INT_SRC_IN_SEC_FLAG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA2D_IN_CH2_INT_MAP")
            .field(
                "core0_dma2d_in_ch2_int_map",
                &self.core0_dma2d_in_ch2_int_map(),
            )
            .field(
                "core0_dma2d_in_ch2_int_src_pass_in_sec",
                &self.core0_dma2d_in_ch2_int_src_pass_in_sec(),
            )
            .field(
                "core0_dma2d_in_ch2_int_src_in_sec_flag",
                &self.core0_dma2d_in_ch2_int_src_in_sec_flag(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn core0_dma2d_in_ch2_int_map(
        &mut self,
    ) -> CORE0_DMA2D_IN_CH2_INT_MAP_W<'_, DMA2D_IN_CH2_INT_MAP_SPEC> {
        CORE0_DMA2D_IN_CH2_INT_MAP_W::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn core0_dma2d_in_ch2_int_src_pass_in_sec(
        &mut self,
    ) -> CORE0_DMA2D_IN_CH2_INT_SRC_PASS_IN_SEC_W<'_, DMA2D_IN_CH2_INT_MAP_SPEC> {
        CORE0_DMA2D_IN_CH2_INT_SRC_PASS_IN_SEC_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn core0_dma2d_in_ch2_int_src_in_sec_flag(
        &mut self,
    ) -> CORE0_DMA2D_IN_CH2_INT_SRC_IN_SEC_FLAG_W<'_, DMA2D_IN_CH2_INT_MAP_SPEC> {
        CORE0_DMA2D_IN_CH2_INT_SRC_IN_SEC_FLAG_W::new(self, 7)
    }
}
#[doc = "DMA2D IN channel 2 interrupt map\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2d_in_ch2_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2d_in_ch2_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA2D_IN_CH2_INT_MAP_SPEC;
impl crate::RegisterSpec for DMA2D_IN_CH2_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma2d_in_ch2_int_map::R`](R) reader structure"]
impl crate::Readable for DMA2D_IN_CH2_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma2d_in_ch2_int_map::W`](W) writer structure"]
impl crate::Writable for DMA2D_IN_CH2_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA2D_IN_CH2_INT_MAP to value 0"]
impl crate::Resettable for DMA2D_IN_CH2_INT_MAP_SPEC {}
