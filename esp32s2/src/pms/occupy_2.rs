#[doc = "Register `OCCUPY_2` reader"]
pub struct R(crate::R<OCCUPY_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCCUPY_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCCUPY_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCCUPY_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCCUPY_2` writer"]
pub struct W(crate::W<OCCUPY_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCCUPY_2_SPEC>;
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
impl From<crate::W<OCCUPY_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCCUPY_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCCUPY_MAC_DUMP` reader - Configure whether SRAM Block 18-21 is used as mac dump."]
pub type OCCUPY_MAC_DUMP_R = crate::FieldReader;
#[doc = "Field `OCCUPY_MAC_DUMP` writer - Configure whether SRAM Block 18-21 is used as mac dump."]
pub type OCCUPY_MAC_DUMP_W<'a, const O: u8> = crate::FieldWriter<'a, OCCUPY_2_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Configure whether SRAM Block 18-21 is used as mac dump."]
    #[inline(always)]
    pub fn occupy_mac_dump(&self) -> OCCUPY_MAC_DUMP_R {
        OCCUPY_MAC_DUMP_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCCUPY_2")
            .field(
                "occupy_mac_dump",
                &format_args!("{}", self.occupy_mac_dump().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OCCUPY_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure whether SRAM Block 18-21 is used as mac dump."]
    #[inline(always)]
    #[must_use]
    pub fn occupy_mac_dump(&mut self) -> OCCUPY_MAC_DUMP_W<0> {
        OCCUPY_MAC_DUMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Occupy permission control register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [occupy_2](index.html) module"]
pub struct OCCUPY_2_SPEC;
impl crate::RegisterSpec for OCCUPY_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [occupy_2::R](R) reader structure"]
impl crate::Readable for OCCUPY_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [occupy_2::W](W) writer structure"]
impl crate::Writable for OCCUPY_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCCUPY_2 to value 0"]
impl crate::Resettable for OCCUPY_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
