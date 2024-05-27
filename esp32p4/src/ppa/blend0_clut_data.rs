///Register `BLEND0_CLUT_DATA` reader
pub type R = crate::R<BLEND0_CLUT_DATA_SPEC>;
///Register `BLEND0_CLUT_DATA` writer
pub type W = crate::W<BLEND0_CLUT_DATA_SPEC>;
///Field `RDWR_WORD_BLEND0_CLUT` reader - Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode.
pub type RDWR_WORD_BLEND0_CLUT_R = crate::FieldReader<u32>;
///Field `RDWR_WORD_BLEND0_CLUT` writer - Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode.
pub type RDWR_WORD_BLEND0_CLUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode.
    #[inline(always)]
    pub fn rdwr_word_blend0_clut(&self) -> RDWR_WORD_BLEND0_CLUT_R {
        RDWR_WORD_BLEND0_CLUT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND0_CLUT_DATA")
            .field("rdwr_word_blend0_clut", &self.rdwr_word_blend0_clut())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode.
    #[inline(always)]
    #[must_use]
    pub fn rdwr_word_blend0_clut(&mut self) -> RDWR_WORD_BLEND0_CLUT_W<BLEND0_CLUT_DATA_SPEC> {
        RDWR_WORD_BLEND0_CLUT_W::new(self, 0)
    }
}
/**CLUT sram data read/write register in background plane of blender

You can [`read`](crate::generic::Reg::read) this register and get [`blend0_clut_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend0_clut_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLEND0_CLUT_DATA_SPEC;
impl crate::RegisterSpec for BLEND0_CLUT_DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blend0_clut_data::R`](R) reader structure
impl crate::Readable for BLEND0_CLUT_DATA_SPEC {}
///`write(|w| ..)` method takes [`blend0_clut_data::W`](W) writer structure
impl crate::Writable for BLEND0_CLUT_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BLEND0_CLUT_DATA to value 0
impl crate::Resettable for BLEND0_CLUT_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
