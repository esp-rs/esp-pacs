#[doc = "Register `SDA_SAMPLE` reader"]
pub struct R(crate::R<SDA_SAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDA_SAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDA_SAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDA_SAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDA_SAMPLE` writer"]
pub struct W(crate::W<SDA_SAMPLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDA_SAMPLE_SPEC>;
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
impl From<crate::W<SDA_SAMPLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDA_SAMPLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - This register is used to configure the clock num I2C used to sample data on SDA after the posedge of SCL"]
pub type TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIME` writer - This register is used to configure the clock num I2C used to sample data on SDA after the posedge of SCL"]
pub type TIME_W<'a, const O: u8> = crate::FieldWriter<'a, SDA_SAMPLE_SPEC, 10, O, u16, u16>;
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the clock num I2C used to sample data on SDA after the posedge of SCL"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_SAMPLE")
            .field("time", &format_args!("{}", self.time().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDA_SAMPLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the clock num I2C used to sample data on SDA after the posedge of SCL"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<0> {
        TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sda_sample](index.html) module"]
pub struct SDA_SAMPLE_SPEC;
impl crate::RegisterSpec for SDA_SAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sda_sample::R](R) reader structure"]
impl crate::Readable for SDA_SAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sda_sample::W](W) writer structure"]
impl crate::Writable for SDA_SAMPLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDA_SAMPLE to value 0"]
impl crate::Resettable for SDA_SAMPLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
