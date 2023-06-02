#[doc = "Register `MAC_DUMP_0` reader"]
pub struct R(crate::R<MAC_DUMP_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_DUMP_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_DUMP_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_DUMP_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_DUMP_0` writer"]
pub struct W(crate::W<MAC_DUMP_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_DUMP_0_SPEC>;
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
impl From<crate::W<MAC_DUMP_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_DUMP_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_DUMP_LOCK` reader - Lock register. Setting to 1 locks MAC dump permission control registers."]
pub type MAC_DUMP_LOCK_R = crate::BitReader;
#[doc = "Field `MAC_DUMP_LOCK` writer - Lock register. Setting to 1 locks MAC dump permission control registers."]
pub type MAC_DUMP_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, MAC_DUMP_0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks MAC dump permission control registers."]
    #[inline(always)]
    pub fn mac_dump_lock(&self) -> MAC_DUMP_LOCK_R {
        MAC_DUMP_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_DUMP_0")
            .field(
                "mac_dump_lock",
                &format_args!("{}", self.mac_dump_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MAC_DUMP_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks MAC dump permission control registers."]
    #[inline(always)]
    #[must_use]
    pub fn mac_dump_lock(&mut self) -> MAC_DUMP_LOCK_W<0> {
        MAC_DUMP_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC dump permission control register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_dump_0](index.html) module"]
pub struct MAC_DUMP_0_SPEC;
impl crate::RegisterSpec for MAC_DUMP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_dump_0::R](R) reader structure"]
impl crate::Readable for MAC_DUMP_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_dump_0::W](W) writer structure"]
impl crate::Writable for MAC_DUMP_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_DUMP_0 to value 0"]
impl crate::Resettable for MAC_DUMP_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
