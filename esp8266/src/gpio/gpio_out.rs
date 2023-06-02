#[doc = "Register `GPIO_OUT` reader"]
pub struct R(crate::R<GPIO_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_OUT` writer"]
pub struct W(crate::W<GPIO_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIO_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_OUT_DATA` reader - The output value when the GPIO pin is set as output."]
pub type GPIO_OUT_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GPIO_OUT_DATA` writer - The output value when the GPIO pin is set as output."]
pub type GPIO_OUT_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_OUT_SPEC, 16, O, u16, u16>;
#[doc = "Field `GPIO_BT_SEL` reader - BT-Coexist Selection register"]
pub type GPIO_BT_SEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GPIO_BT_SEL` writer - BT-Coexist Selection register"]
pub type GPIO_BT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_OUT_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The output value when the GPIO pin is set as output."]
    #[inline(always)]
    pub fn gpio_out_data(&self) -> GPIO_OUT_DATA_R {
        GPIO_OUT_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - BT-Coexist Selection register"]
    #[inline(always)]
    pub fn gpio_bt_sel(&self) -> GPIO_BT_SEL_R {
        GPIO_BT_SEL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_OUT")
            .field(
                "gpio_bt_sel",
                &format_args!("{}", self.gpio_bt_sel().bits()),
            )
            .field(
                "gpio_out_data",
                &format_args!("{}", self.gpio_out_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The output value when the GPIO pin is set as output."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_data(&mut self) -> GPIO_OUT_DATA_W<0> {
        GPIO_OUT_DATA_W::new(self)
    }
    #[doc = "Bits 16:31 - BT-Coexist Selection register"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_bt_sel(&mut self) -> GPIO_BT_SEL_W<16> {
        GPIO_BT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BT-Coexist Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_out](index.html) module"]
pub struct GPIO_OUT_SPEC;
impl crate::RegisterSpec for GPIO_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_out::R](R) reader structure"]
impl crate::Readable for GPIO_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_out::W](W) writer structure"]
impl crate::Writable for GPIO_OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_OUT to value 0"]
impl crate::Resettable for GPIO_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
