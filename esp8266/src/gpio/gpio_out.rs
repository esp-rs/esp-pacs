#[doc = "Register `GPIO_OUT` reader"]
pub type R = crate::R<GPIO_OUT_SPEC>;
#[doc = "Register `GPIO_OUT` writer"]
pub type W = crate::W<GPIO_OUT_SPEC>;
#[doc = "Field `GPIO_OUT_DATA` reader - The output value when the GPIO pin is set as output."]
pub type GPIO_OUT_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO_OUT_DATA` writer - The output value when the GPIO pin is set as output."]
pub type GPIO_OUT_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `GPIO_BT_SEL` reader - BT-Coexist Selection register"]
pub type GPIO_BT_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO_BT_SEL` writer - BT-Coexist Selection register"]
pub type GPIO_BT_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
    pub fn gpio_out_data(&mut self) -> GPIO_OUT_DATA_W<GPIO_OUT_SPEC, 0> {
        GPIO_OUT_DATA_W::new(self)
    }
    #[doc = "Bits 16:31 - BT-Coexist Selection register"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_bt_sel(&mut self) -> GPIO_BT_SEL_W<GPIO_OUT_SPEC, 16> {
        GPIO_BT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "BT-Coexist Selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OUT_SPEC;
impl crate::RegisterSpec for GPIO_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_out::R`](R) reader structure"]
impl crate::Readable for GPIO_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_out::W`](W) writer structure"]
impl crate::Writable for GPIO_OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_OUT to value 0"]
impl crate::Resettable for GPIO_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
