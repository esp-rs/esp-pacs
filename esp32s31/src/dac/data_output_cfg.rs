#[doc = "Register `DATA_OUTPUT_CFG` reader"]
pub type R = crate::R<DATA_OUTPUT_CFG_SPEC>;
#[doc = "Register `DATA_OUTPUT_CFG` writer"]
pub type W = crate::W<DATA_OUTPUT_CFG_SPEC>;
#[doc = "Field `PAD_0_DATA_SEL` reader - 1:output pdma data 0:output sintx data"]
pub type PAD_0_DATA_SEL_R = crate::BitReader;
#[doc = "Field `PAD_0_DATA_SEL` writer - 1:output pdma data 0:output sintx data"]
pub type PAD_0_DATA_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_1_DATA_SEL` reader - 1:output pdma data 0:output sintx data"]
pub type PAD_1_DATA_SEL_R = crate::BitReader;
#[doc = "Field `PAD_1_DATA_SEL` writer - 1:output pdma data 0:output sintx data"]
pub type PAD_1_DATA_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1:output pdma data 0:output sintx data"]
    #[inline(always)]
    pub fn pad_0_data_sel(&self) -> PAD_0_DATA_SEL_R {
        PAD_0_DATA_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:output pdma data 0:output sintx data"]
    #[inline(always)]
    pub fn pad_1_data_sel(&self) -> PAD_1_DATA_SEL_R {
        PAD_1_DATA_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_OUTPUT_CFG")
            .field("pad_0_data_sel", &self.pad_0_data_sel())
            .field("pad_1_data_sel", &self.pad_1_data_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1:output pdma data 0:output sintx data"]
    #[inline(always)]
    pub fn pad_0_data_sel(&mut self) -> PAD_0_DATA_SEL_W<'_, DATA_OUTPUT_CFG_SPEC> {
        PAD_0_DATA_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:output pdma data 0:output sintx data"]
    #[inline(always)]
    pub fn pad_1_data_sel(&mut self) -> PAD_1_DATA_SEL_W<'_, DATA_OUTPUT_CFG_SPEC> {
        PAD_1_DATA_SEL_W::new(self, 1)
    }
}
#[doc = "dac DATA OUTPUT cfg register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_output_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_output_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_OUTPUT_CFG_SPEC;
impl crate::RegisterSpec for DATA_OUTPUT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_output_cfg::R`](R) reader structure"]
impl crate::Readable for DATA_OUTPUT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_output_cfg::W`](W) writer structure"]
impl crate::Writable for DATA_OUTPUT_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_OUTPUT_CFG to value 0"]
impl crate::Resettable for DATA_OUTPUT_CFG_SPEC {}
