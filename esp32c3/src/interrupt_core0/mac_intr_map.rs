#[doc = "Register `MAC_INTR_MAP` reader"]
pub struct R(crate::R<MAC_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_INTR_MAP` writer"]
pub struct W(crate::W<MAC_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_INTR_MAP_SPEC>;
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
impl From<crate::W<MAC_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_INTR_MAP` reader - core0_mac_intr_map"]
pub type MAC_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `MAC_INTR_MAP` writer - core0_mac_intr_map"]
pub type MAC_INTR_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, MAC_INTR_MAP_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - core0_mac_intr_map"]
    #[inline(always)]
    pub fn mac_intr_map(&self) -> MAC_INTR_MAP_R {
        MAC_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_INTR_MAP")
            .field(
                "mac_intr_map",
                &format_args!("{}", self.mac_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MAC_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - core0_mac_intr_map"]
    #[inline(always)]
    #[must_use]
    pub fn mac_intr_map(&mut self) -> MAC_INTR_MAP_W<0> {
        MAC_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mac intr map register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_intr_map](index.html) module"]
pub struct MAC_INTR_MAP_SPEC;
impl crate::RegisterSpec for MAC_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_intr_map::R](R) reader structure"]
impl crate::Readable for MAC_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_intr_map::W](W) writer structure"]
impl crate::Writable for MAC_INTR_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_INTR_MAP to value 0"]
impl crate::Resettable for MAC_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
