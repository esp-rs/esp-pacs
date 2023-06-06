#[doc = "Register `EXT1` reader"]
pub struct R(crate::R<EXT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT1` writer"]
pub struct W(crate::W<EXT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT1_SPEC>;
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
impl From<crate::W<EXT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_ERASE_TIME` reader - erase flash delay time by system clock."]
pub type T_ERASE_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `T_ERASE_TIME` writer - erase flash delay time by system clock."]
pub type T_ERASE_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, EXT1_SPEC, 12, O, u16>;
#[doc = "Field `T_ERASE_SHIFT` reader - erase flash delay time shift."]
pub type T_ERASE_SHIFT_R = crate::FieldReader;
#[doc = "Field `T_ERASE_SHIFT` writer - erase flash delay time shift."]
pub type T_ERASE_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, EXT1_SPEC, 4, O>;
#[doc = "Field `T_ERASE_ENA` reader - erase flash delay enable."]
pub type T_ERASE_ENA_R = crate::BitReader;
#[doc = "Field `T_ERASE_ENA` writer - erase flash delay enable."]
pub type T_ERASE_ENA_W<'a, const O: u8> = crate::BitWriter<'a, EXT1_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - erase flash delay time by system clock."]
    #[inline(always)]
    pub fn t_erase_time(&self) -> T_ERASE_TIME_R {
        T_ERASE_TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - erase flash delay time shift."]
    #[inline(always)]
    pub fn t_erase_shift(&self) -> T_ERASE_SHIFT_R {
        T_ERASE_SHIFT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - erase flash delay enable."]
    #[inline(always)]
    pub fn t_erase_ena(&self) -> T_ERASE_ENA_R {
        T_ERASE_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT1")
            .field(
                "t_erase_time",
                &format_args!("{}", self.t_erase_time().bits()),
            )
            .field(
                "t_erase_shift",
                &format_args!("{}", self.t_erase_shift().bits()),
            )
            .field("t_erase_ena", &format_args!("{}", self.t_erase_ena().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - erase flash delay time by system clock."]
    #[inline(always)]
    #[must_use]
    pub fn t_erase_time(&mut self) -> T_ERASE_TIME_W<0> {
        T_ERASE_TIME_W::new(self)
    }
    #[doc = "Bits 16:19 - erase flash delay time shift."]
    #[inline(always)]
    #[must_use]
    pub fn t_erase_shift(&mut self) -> T_ERASE_SHIFT_W<16> {
        T_ERASE_SHIFT_W::new(self)
    }
    #[doc = "Bit 31 - erase flash delay enable."]
    #[inline(always)]
    #[must_use]
    pub fn t_erase_ena(&mut self) -> T_ERASE_ENA_W<31> {
        T_ERASE_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext1](index.html) module"]
pub struct EXT1_SPEC;
impl crate::RegisterSpec for EXT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext1::R](R) reader structure"]
impl crate::Readable for EXT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext1::W](W) writer structure"]
impl crate::Writable for EXT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT1 to value 0x800f_0000"]
impl crate::Resettable for EXT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x800f_0000;
}
