#[doc = "Register `CPUSDIO_INT1` reader"]
pub struct R(crate::R<CPUSDIO_INT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUSDIO_INT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUSDIO_INT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUSDIO_INT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUSDIO_INT1` writer"]
pub struct W(crate::W<CPUSDIO_INT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUSDIO_INT1_SPEC>;
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
impl From<crate::W<CPUSDIO_INT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUSDIO_INT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_INT_H` reader - SDIO's extent GPIO32~39 interrupt"]
pub type SDIO_INT_H_R = crate::FieldReader;
#[doc = "Field `PIN_PAD_DRIVER` reader - "]
pub type PIN_PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `PIN_PAD_DRIVER` writer - "]
pub type PIN_PAD_DRIVER_W<'a, const O: u8> = crate::BitWriter<'a, CPUSDIO_INT1_SPEC, O>;
#[doc = "Field `PIN_INT_TYPE` reader - "]
pub type PIN_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `PIN_INT_TYPE` writer - "]
pub type PIN_INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, CPUSDIO_INT1_SPEC, 3, O>;
#[doc = "Field `PIN_WAKEUP_ENABLE` reader - "]
pub type PIN_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `PIN_WAKEUP_ENABLE` writer - "]
pub type PIN_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, CPUSDIO_INT1_SPEC, O>;
#[doc = "Field `PIN_CONFIG` reader - "]
pub type PIN_CONFIG_R = crate::FieldReader;
#[doc = "Field `PIN_CONFIG` writer - "]
pub type PIN_CONFIG_W<'a, const O: u8> = crate::FieldWriter<'a, CPUSDIO_INT1_SPEC, 2, O>;
#[doc = "Field `PIN_INT_ENA` reader - "]
pub type PIN_INT_ENA_R = crate::FieldReader;
#[doc = "Field `PIN_INT_ENA` writer - "]
pub type PIN_INT_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, CPUSDIO_INT1_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:7 - SDIO's extent GPIO32~39 interrupt"]
    #[inline(always)]
    pub fn sdio_int_h(&self) -> SDIO_INT_H_R {
        SDIO_INT_H_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin_pad_driver(&self) -> PIN_PAD_DRIVER_R {
        PIN_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn pin_int_type(&self) -> PIN_INT_TYPE_R {
        PIN_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin_wakeup_enable(&self) -> PIN_WAKEUP_ENABLE_R {
        PIN_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn pin_config(&self) -> PIN_CONFIG_R {
        PIN_CONFIG_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn pin_int_ena(&self) -> PIN_INT_ENA_R {
        PIN_INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUSDIO_INT1")
            .field("sdio_int_h", &format_args!("{}", self.sdio_int_h().bits()))
            .field(
                "pin_pad_driver",
                &format_args!("{}", self.pin_pad_driver().bit()),
            )
            .field(
                "pin_int_type",
                &format_args!("{}", self.pin_int_type().bits()),
            )
            .field(
                "pin_wakeup_enable",
                &format_args!("{}", self.pin_wakeup_enable().bit()),
            )
            .field("pin_config", &format_args!("{}", self.pin_config().bits()))
            .field(
                "pin_int_ena",
                &format_args!("{}", self.pin_int_ena().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPUSDIO_INT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pin_pad_driver(&mut self) -> PIN_PAD_DRIVER_W<2> {
        PIN_PAD_DRIVER_W::new(self)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    #[must_use]
    pub fn pin_int_type(&mut self) -> PIN_INT_TYPE_W<7> {
        PIN_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pin_wakeup_enable(&mut self) -> PIN_WAKEUP_ENABLE_W<10> {
        PIN_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn pin_config(&mut self) -> PIN_CONFIG_W<11> {
        PIN_CONFIG_W::new(self)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    #[must_use]
    pub fn pin_int_ena(&mut self) -> PIN_INT_ENA_W<13> {
        PIN_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpusdio_int1](index.html) module"]
pub struct CPUSDIO_INT1_SPEC;
impl crate::RegisterSpec for CPUSDIO_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpusdio_int1::R](R) reader structure"]
impl crate::Readable for CPUSDIO_INT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpusdio_int1::W](W) writer structure"]
impl crate::Writable for CPUSDIO_INT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUSDIO_INT1 to value 0"]
impl crate::Resettable for CPUSDIO_INT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
