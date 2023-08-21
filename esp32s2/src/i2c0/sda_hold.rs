#[doc = "Register `SDA_HOLD` reader"]
pub type R = crate::R<SDA_HOLD_SPEC>;
#[doc = "Register `SDA_HOLD` writer"]
pub type W = crate::W<SDA_HOLD_SPEC>;
#[doc = "Field `TIME` reader - This register is used to configure the interval between changing the SDA output level and the falling edge of SCL, in I2C module clock cycles."]
pub type TIME_R = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - This register is used to configure the interval between changing the SDA output level and the falling edge of SCL, in I2C module clock cycles."]
pub type TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the interval between changing the SDA output level and the falling edge of SCL, in I2C module clock cycles."]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_HOLD")
            .field("time", &format_args!("{}", self.time().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDA_HOLD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the interval between changing the SDA output level and the falling edge of SCL, in I2C module clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<SDA_HOLD_SPEC, 0> {
        TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configures the hold time after a negative SCL edge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDA_HOLD_SPEC;
impl crate::RegisterSpec for SDA_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_hold::R`](R) reader structure"]
impl crate::Readable for SDA_HOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sda_hold::W`](W) writer structure"]
impl crate::Writable for SDA_HOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDA_HOLD to value 0"]
impl crate::Resettable for SDA_HOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
