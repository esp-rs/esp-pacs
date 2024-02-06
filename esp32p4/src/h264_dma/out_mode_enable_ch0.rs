#[doc = "Register `OUT_MODE_ENABLE_CH0` reader"]
pub type R = crate::R<OUT_MODE_ENABLE_CH0_SPEC>;
#[doc = "Register `OUT_MODE_ENABLE_CH0` writer"]
pub type W = crate::W<OUT_MODE_ENABLE_CH0_SPEC>;
#[doc = "Field `OUT_TEST_MODE_ENABLE_CH0` reader - tx CH0 test mode enable.0 : H264_DMA work in normal mode.1 : H264_DMA work in test mode"]
pub type OUT_TEST_MODE_ENABLE_CH0_R = crate::BitReader;
#[doc = "Field `OUT_TEST_MODE_ENABLE_CH0` writer - tx CH0 test mode enable.0 : H264_DMA work in normal mode.1 : H264_DMA work in test mode"]
pub type OUT_TEST_MODE_ENABLE_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - tx CH0 test mode enable.0 : H264_DMA work in normal mode.1 : H264_DMA work in test mode"]
    #[inline(always)]
    pub fn out_test_mode_enable_ch0(&self) -> OUT_TEST_MODE_ENABLE_CH0_R {
        OUT_TEST_MODE_ENABLE_CH0_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_MODE_ENABLE_CH0")
            .field(
                "out_test_mode_enable_ch0",
                &format_args!("{}", self.out_test_mode_enable_ch0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_MODE_ENABLE_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - tx CH0 test mode enable.0 : H264_DMA work in normal mode.1 : H264_DMA work in test mode"]
    #[inline(always)]
    #[must_use]
    pub fn out_test_mode_enable_ch0(
        &mut self,
    ) -> OUT_TEST_MODE_ENABLE_CH0_W<OUT_MODE_ENABLE_CH0_SPEC> {
        OUT_TEST_MODE_ENABLE_CH0_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "tx CH0 mode enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_mode_enable_ch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_mode_enable_ch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_MODE_ENABLE_CH0_SPEC;
impl crate::RegisterSpec for OUT_MODE_ENABLE_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_mode_enable_ch0::R`](R) reader structure"]
impl crate::Readable for OUT_MODE_ENABLE_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_mode_enable_ch0::W`](W) writer structure"]
impl crate::Writable for OUT_MODE_ENABLE_CH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_MODE_ENABLE_CH0 to value 0"]
impl crate::Resettable for OUT_MODE_ENABLE_CH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
