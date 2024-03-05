#[doc = "Register `FILTER_CTRL0` reader"]
pub type R = crate::R<FILTER_CTRL0_SPEC>;
#[doc = "Register `FILTER_CTRL0` writer"]
pub type W = crate::W<FILTER_CTRL0_SPEC>;
#[doc = "Field `APB_SARADC_FILTER_CHANNEL1` reader - configure filter1 to adc channel"]
pub type APB_SARADC_FILTER_CHANNEL1_R = crate::FieldReader;
#[doc = "Field `APB_SARADC_FILTER_CHANNEL1` writer - configure filter1 to adc channel"]
pub type APB_SARADC_FILTER_CHANNEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `APB_SARADC_FILTER_CHANNEL0` reader - configure filter0 to adc channel"]
pub type APB_SARADC_FILTER_CHANNEL0_R = crate::FieldReader;
#[doc = "Field `APB_SARADC_FILTER_CHANNEL0` writer - configure filter0 to adc channel"]
pub type APB_SARADC_FILTER_CHANNEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `APB_SARADC_FILTER_RESET` reader - enable apb_adc1_filter"]
pub type APB_SARADC_FILTER_RESET_R = crate::BitReader;
#[doc = "Field `APB_SARADC_FILTER_RESET` writer - enable apb_adc1_filter"]
pub type APB_SARADC_FILTER_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 18:21 - configure filter1 to adc channel"]
    #[inline(always)]
    pub fn apb_saradc_filter_channel1(&self) -> APB_SARADC_FILTER_CHANNEL1_R {
        APB_SARADC_FILTER_CHANNEL1_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - configure filter0 to adc channel"]
    #[inline(always)]
    pub fn apb_saradc_filter_channel0(&self) -> APB_SARADC_FILTER_CHANNEL0_R {
        APB_SARADC_FILTER_CHANNEL0_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - enable apb_adc1_filter"]
    #[inline(always)]
    pub fn apb_saradc_filter_reset(&self) -> APB_SARADC_FILTER_RESET_R {
        APB_SARADC_FILTER_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CTRL0")
            .field(
                "apb_saradc_filter_channel1",
                &format_args!("{}", self.apb_saradc_filter_channel1().bits()),
            )
            .field(
                "apb_saradc_filter_channel0",
                &format_args!("{}", self.apb_saradc_filter_channel0().bits()),
            )
            .field(
                "apb_saradc_filter_reset",
                &format_args!("{}", self.apb_saradc_filter_reset().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FILTER_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 18:21 - configure filter1 to adc channel"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_filter_channel1(
        &mut self,
    ) -> APB_SARADC_FILTER_CHANNEL1_W<FILTER_CTRL0_SPEC> {
        APB_SARADC_FILTER_CHANNEL1_W::new(self, 18)
    }
    #[doc = "Bits 22:25 - configure filter0 to adc channel"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_filter_channel0(
        &mut self,
    ) -> APB_SARADC_FILTER_CHANNEL0_W<FILTER_CTRL0_SPEC> {
        APB_SARADC_FILTER_CHANNEL0_W::new(self, 22)
    }
    #[doc = "Bit 31 - enable apb_adc1_filter"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_filter_reset(&mut self) -> APB_SARADC_FILTER_RESET_W<FILTER_CTRL0_SPEC> {
        APB_SARADC_FILTER_RESET_W::new(self, 31)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_CTRL0_SPEC;
impl crate::RegisterSpec for FILTER_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_ctrl0::R`](R) reader structure"]
impl crate::Readable for FILTER_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_ctrl0::W`](W) writer structure"]
impl crate::Writable for FILTER_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTER_CTRL0 to value 0x0374_0000"]
impl crate::Resettable for FILTER_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0374_0000;
}
