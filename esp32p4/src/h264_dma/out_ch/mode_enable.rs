///Register `MODE_ENABLE` reader
pub type R = crate::R<MODE_ENABLE_SPEC>;
///Register `MODE_ENABLE` writer
pub type W = crate::W<MODE_ENABLE_SPEC>;
///Field `OUT_TEST_MODE_ENABLE` reader - tx CH0 test mode enable.0 : H264_DMA work in normal mode.1 : H264_DMA work in test mode
pub type OUT_TEST_MODE_ENABLE_R = crate::BitReader;
///Field `OUT_TEST_MODE_ENABLE` writer - tx CH0 test mode enable.0 : H264_DMA work in normal mode.1 : H264_DMA work in test mode
pub type OUT_TEST_MODE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - tx CH0 test mode enable.0 : H264_DMA work in normal mode.1 : H264_DMA work in test mode
    #[inline(always)]
    pub fn out_test_mode_enable(&self) -> OUT_TEST_MODE_ENABLE_R {
        OUT_TEST_MODE_ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE_ENABLE")
            .field("out_test_mode_enable", &self.out_test_mode_enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - tx CH0 test mode enable.0 : H264_DMA work in normal mode.1 : H264_DMA work in test mode
    #[inline(always)]
    #[must_use]
    pub fn out_test_mode_enable(&mut self) -> OUT_TEST_MODE_ENABLE_W<MODE_ENABLE_SPEC> {
        OUT_TEST_MODE_ENABLE_W::new(self, 0)
    }
}
/**TX CHx mode enable register. Available on CH0

You can [`read`](crate::generic::Reg::read) this register and get [`mode_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MODE_ENABLE_SPEC;
impl crate::RegisterSpec for MODE_ENABLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mode_enable::R`](R) reader structure
impl crate::Readable for MODE_ENABLE_SPEC {}
///`write(|w| ..)` method takes [`mode_enable::W`](W) writer structure
impl crate::Writable for MODE_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODE_ENABLE to value 0
impl crate::Resettable for MODE_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
