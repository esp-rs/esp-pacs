#[doc = "Register `TO` reader"]
pub type R = crate::R<TO_SPEC>;
#[doc = "Register `TO` writer"]
pub type W = crate::W<TO_SPEC>;
#[doc = "Field `TIME_OUT_VALUE` reader - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
pub type TIME_OUT_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `TIME_OUT_VALUE` writer - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
pub type TIME_OUT_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `TIME_OUT_EN` reader - This is the enable bit for time out control."]
pub type TIME_OUT_EN_R = crate::BitReader;
#[doc = "Field `TIME_OUT_EN` writer - This is the enable bit for time out control."]
pub type TIME_OUT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:23 - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
    #[inline(always)]
    pub fn time_out_value(&self) -> TIME_OUT_VALUE_R {
        TIME_OUT_VALUE_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - This is the enable bit for time out control."]
    #[inline(always)]
    pub fn time_out_en(&self) -> TIME_OUT_EN_R {
        TIME_OUT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TO")
            .field(
                "time_out_value",
                &format_args!("{}", self.time_out_value().bits()),
            )
            .field("time_out_en", &format_args!("{}", self.time_out_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn time_out_value(&mut self) -> TIME_OUT_VALUE_W<TO_SPEC, 0> {
        TIME_OUT_VALUE_W::new(self)
    }
    #[doc = "Bit 24 - This is the enable bit for time out control."]
    #[inline(always)]
    #[must_use]
    pub fn time_out_en(&mut self) -> TIME_OUT_EN_W<TO_SPEC, 24> {
        TIME_OUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Setting time out control for receiving data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`to::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TO_SPEC;
impl crate::RegisterSpec for TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`to::R`](R) reader structure"]
impl crate::Readable for TO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`to::W`](W) writer structure"]
impl crate::Writable for TO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TO to value 0"]
impl crate::Resettable for TO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
