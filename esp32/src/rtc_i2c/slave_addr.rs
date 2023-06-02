#[doc = "Register `SLAVE_ADDR` reader"]
pub struct R(crate::R<SLAVE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE_ADDR` writer"]
pub struct W(crate::W<SLAVE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_ADDR_SPEC>;
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
impl From<crate::W<SLAVE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_ADDR` reader - local slave address"]
pub type SLAVE_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLAVE_ADDR` writer - local slave address"]
pub type SLAVE_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, SLAVE_ADDR_SPEC, 15, O, u16, u16>;
#[doc = "Field `_10BIT` reader - Set if local slave address is 10-bit"]
pub type _10BIT_R = crate::BitReader;
#[doc = "Field `_10BIT` writer - Set if local slave address is 10-bit"]
pub type _10BIT_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_ADDR_SPEC, O>;
impl R {
    #[doc = "Bits 0:14 - local slave address"]
    #[inline(always)]
    pub fn slave_addr(&self) -> SLAVE_ADDR_R {
        SLAVE_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Set if local slave address is 10-bit"]
    #[inline(always)]
    pub fn _10bit(&self) -> _10BIT_R {
        _10BIT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE_ADDR")
            .field("slave_addr", &format_args!("{}", self.slave_addr().bits()))
            .field("_10bit", &format_args!("{}", self._10bit().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLAVE_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:14 - local slave address"]
    #[inline(always)]
    #[must_use]
    pub fn slave_addr(&mut self) -> SLAVE_ADDR_W<0> {
        SLAVE_ADDR_W::new(self)
    }
    #[doc = "Bit 31 - Set if local slave address is 10-bit"]
    #[inline(always)]
    #[must_use]
    pub fn _10bit(&mut self) -> _10BIT_W<31> {
        _10BIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_addr](index.html) module"]
pub struct SLAVE_ADDR_SPEC;
impl crate::RegisterSpec for SLAVE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave_addr::R](R) reader structure"]
impl crate::Readable for SLAVE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave_addr::W](W) writer structure"]
impl crate::Writable for SLAVE_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLAVE_ADDR to value 0"]
impl crate::Resettable for SLAVE_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
