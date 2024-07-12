#[doc = "Register `BLEND1_CLUT_DATA` reader"]
pub type R = crate::R<BLEND1_CLUT_DATA_SPEC>;
#[doc = "Register `BLEND1_CLUT_DATA` writer"]
pub type W = crate::W<BLEND1_CLUT_DATA_SPEC>;
#[doc = "Field `RDWR_WORD_BLEND1_CLUT` reader - Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode."]
pub type RDWR_WORD_BLEND1_CLUT_R = crate::FieldReader<u32>;
#[doc = "Field `RDWR_WORD_BLEND1_CLUT` writer - Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode."]
pub type RDWR_WORD_BLEND1_CLUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode."]
    #[inline(always)]
    pub fn rdwr_word_blend1_clut(&self) -> RDWR_WORD_BLEND1_CLUT_R {
        RDWR_WORD_BLEND1_CLUT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND1_CLUT_DATA")
            .field("rdwr_word_blend1_clut", &self.rdwr_word_blend1_clut())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode."]
    #[inline(always)]
    #[must_use]
    pub fn rdwr_word_blend1_clut(&mut self) -> RDWR_WORD_BLEND1_CLUT_W<BLEND1_CLUT_DATA_SPEC> {
        RDWR_WORD_BLEND1_CLUT_W::new(self, 0)
    }
}
#[doc = "CLUT sram data read/write register in foreground plane of blender\n\nYou can [`read`](crate::Reg::read) this register and get [`blend1_clut_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend1_clut_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEND1_CLUT_DATA_SPEC;
impl crate::RegisterSpec for BLEND1_CLUT_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend1_clut_data::R`](R) reader structure"]
impl crate::Readable for BLEND1_CLUT_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blend1_clut_data::W`](W) writer structure"]
impl crate::Writable for BLEND1_CLUT_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLEND1_CLUT_DATA to value 0"]
impl crate::Resettable for BLEND1_CLUT_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
