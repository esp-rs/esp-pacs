#[doc = "Register `SDA_DUTY` reader"]
pub struct R(crate::R<SDA_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDA_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDA_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDA_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDA_DUTY` writer"]
pub struct W(crate::W<SDA_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDA_DUTY_SPEC>;
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
impl From<crate::W<SDA_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDA_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUM` reader - time period for SDA to toggle after SCL goes low"]
pub type NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NUM` writer - time period for SDA to toggle after SCL goes low"]
pub type NUM_W<'a, const O: u8> = crate::FieldWriter<'a, SDA_DUTY_SPEC, 20, O, u32, u32>;
impl R {
    #[doc = "Bits 0:19 - time period for SDA to toggle after SCL goes low"]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_DUTY")
            .field("num", &format_args!("{}", self.num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDA_DUTY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - time period for SDA to toggle after SCL goes low"]
    #[inline(always)]
    #[must_use]
    pub fn num(&mut self) -> NUM_W<0> {
        NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure sda duty\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sda_duty](index.html) module"]
pub struct SDA_DUTY_SPEC;
impl crate::RegisterSpec for SDA_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sda_duty::R](R) reader structure"]
impl crate::Readable for SDA_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sda_duty::W](W) writer structure"]
impl crate::Writable for SDA_DUTY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDA_DUTY to value 0x10"]
impl crate::Resettable for SDA_DUTY_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
