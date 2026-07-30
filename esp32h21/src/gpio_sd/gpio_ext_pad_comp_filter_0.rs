#[doc = "Register `GPIO_EXT_PAD_COMP_FILTER_0` reader"]
pub type R = crate::R<GPIO_EXT_PAD_COMP_FILTER_0_SPEC>;
#[doc = "Register `GPIO_EXT_PAD_COMP_FILTER_0` writer"]
pub type W = crate::W<GPIO_EXT_PAD_COMP_FILTER_0_SPEC>;
#[doc = "Field `GPIO_EXT_ZERO_DET_FILTER_CNT_0` reader - Zero Detect filter cycle length"]
pub type GPIO_EXT_ZERO_DET_FILTER_CNT_0_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_EXT_ZERO_DET_FILTER_CNT_0` writer - Zero Detect filter cycle length"]
pub type GPIO_EXT_ZERO_DET_FILTER_CNT_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Zero Detect filter cycle length"]
    #[inline(always)]
    pub fn gpio_ext_zero_det_filter_cnt_0(&self) -> GPIO_EXT_ZERO_DET_FILTER_CNT_0_R {
        GPIO_EXT_ZERO_DET_FILTER_CNT_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_EXT_PAD_COMP_FILTER_0")
            .field(
                "gpio_ext_zero_det_filter_cnt_0",
                &self.gpio_ext_zero_det_filter_cnt_0(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Zero Detect filter cycle length"]
    #[inline(always)]
    pub fn gpio_ext_zero_det_filter_cnt_0(
        &mut self,
    ) -> GPIO_EXT_ZERO_DET_FILTER_CNT_0_W<'_, GPIO_EXT_PAD_COMP_FILTER_0_SPEC> {
        GPIO_EXT_ZERO_DET_FILTER_CNT_0_W::new(self, 0)
    }
}
#[doc = "Zero Detect filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_pad_comp_filter_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_pad_comp_filter_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_EXT_PAD_COMP_FILTER_0_SPEC;
impl crate::RegisterSpec for GPIO_EXT_PAD_COMP_FILTER_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ext_pad_comp_filter_0::R`](R) reader structure"]
impl crate::Readable for GPIO_EXT_PAD_COMP_FILTER_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_ext_pad_comp_filter_0::W`](W) writer structure"]
impl crate::Writable for GPIO_EXT_PAD_COMP_FILTER_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_EXT_PAD_COMP_FILTER_0 to value 0"]
impl crate::Resettable for GPIO_EXT_PAD_COMP_FILTER_0_SPEC {}
