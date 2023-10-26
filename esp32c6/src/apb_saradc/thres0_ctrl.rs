#[doc = "Register `THRES0_CTRL` reader"]
pub type R = crate::R<THRES0_CTRL_SPEC>;
#[doc = "Register `THRES0_CTRL` writer"]
pub type W = crate::W<THRES0_CTRL_SPEC>;
#[doc = "Field `APB_SARADC_THRES0_CHANNEL` reader - configure thres0 to adc channel"]
pub type APB_SARADC_THRES0_CHANNEL_R = crate::FieldReader;
#[doc = "Field `APB_SARADC_THRES0_CHANNEL` writer - configure thres0 to adc channel"]
pub type APB_SARADC_THRES0_CHANNEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `APB_SARADC_THRES0_HIGH` reader - saradc thres0 monitor thres"]
pub type APB_SARADC_THRES0_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `APB_SARADC_THRES0_HIGH` writer - saradc thres0 monitor thres"]
pub type APB_SARADC_THRES0_HIGH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
#[doc = "Field `APB_SARADC_THRES0_LOW` reader - saradc thres0 monitor thres"]
pub type APB_SARADC_THRES0_LOW_R = crate::FieldReader<u16>;
#[doc = "Field `APB_SARADC_THRES0_LOW` writer - saradc thres0 monitor thres"]
pub type APB_SARADC_THRES0_LOW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:3 - configure thres0 to adc channel"]
    #[inline(always)]
    pub fn apb_saradc_thres0_channel(&self) -> APB_SARADC_THRES0_CHANNEL_R {
        APB_SARADC_THRES0_CHANNEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:17 - saradc thres0 monitor thres"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high(&self) -> APB_SARADC_THRES0_HIGH_R {
        APB_SARADC_THRES0_HIGH_R::new(((self.bits >> 5) & 0x1fff) as u16)
    }
    #[doc = "Bits 18:30 - saradc thres0 monitor thres"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low(&self) -> APB_SARADC_THRES0_LOW_R {
        APB_SARADC_THRES0_LOW_R::new(((self.bits >> 18) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES0_CTRL")
            .field(
                "apb_saradc_thres0_channel",
                &format_args!("{}", self.apb_saradc_thres0_channel().bits()),
            )
            .field(
                "apb_saradc_thres0_high",
                &format_args!("{}", self.apb_saradc_thres0_high().bits()),
            )
            .field(
                "apb_saradc_thres0_low",
                &format_args!("{}", self.apb_saradc_thres0_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<THRES0_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - configure thres0 to adc channel"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_channel(
        &mut self,
    ) -> APB_SARADC_THRES0_CHANNEL_W<THRES0_CTRL_SPEC, 0> {
        APB_SARADC_THRES0_CHANNEL_W::new(self)
    }
    #[doc = "Bits 5:17 - saradc thres0 monitor thres"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_high(&mut self) -> APB_SARADC_THRES0_HIGH_W<THRES0_CTRL_SPEC, 5> {
        APB_SARADC_THRES0_HIGH_W::new(self)
    }
    #[doc = "Bits 18:30 - saradc thres0 monitor thres"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_low(&mut self) -> APB_SARADC_THRES0_LOW_W<THRES0_CTRL_SPEC, 18> {
        APB_SARADC_THRES0_LOW_W::new(self)
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
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES0_CTRL_SPEC;
impl crate::RegisterSpec for THRES0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres0_ctrl::R`](R) reader structure"]
impl crate::Readable for THRES0_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres0_ctrl::W`](W) writer structure"]
impl crate::Writable for THRES0_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets THRES0_CTRL to value 0x0003_ffed"]
impl crate::Resettable for THRES0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_ffed;
}
