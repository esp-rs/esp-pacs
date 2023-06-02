#[doc = "Register `JTAG_SEL` reader"]
pub struct R(crate::R<JTAG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JTAG_SEL` writer"]
pub struct W(crate::W<JTAG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAG_SEL_SPEC>;
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
impl From<crate::W<JTAG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFT` reader - If strapping_sel_jtag feature is disabled by efuse, and if neither pad_jtag or usb_jtag is disabled by efuse, this field determines which one jtag between usb_jtag and pad_jtag will be used. 1'b1(default): usb_jtag, 1'b0: pad_jtag."]
pub type SOFT_R = crate::BitReader;
#[doc = "Field `SOFT` writer - If strapping_sel_jtag feature is disabled by efuse, and if neither pad_jtag or usb_jtag is disabled by efuse, this field determines which one jtag between usb_jtag and pad_jtag will be used. 1'b1(default): usb_jtag, 1'b0: pad_jtag."]
pub type SOFT_W<'a, const O: u8> = crate::BitWriter<'a, JTAG_SEL_SPEC, O>;
impl R {
    #[doc = "Bit 31 - If strapping_sel_jtag feature is disabled by efuse, and if neither pad_jtag or usb_jtag is disabled by efuse, this field determines which one jtag between usb_jtag and pad_jtag will be used. 1'b1(default): usb_jtag, 1'b0: pad_jtag."]
    #[inline(always)]
    pub fn soft(&self) -> SOFT_R {
        SOFT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAG_SEL")
            .field("soft", &format_args!("{}", self.soft().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<JTAG_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - If strapping_sel_jtag feature is disabled by efuse, and if neither pad_jtag or usb_jtag is disabled by efuse, this field determines which one jtag between usb_jtag and pad_jtag will be used. 1'b1(default): usb_jtag, 1'b0: pad_jtag."]
    #[inline(always)]
    #[must_use]
    pub fn soft(&mut self) -> SOFT_W<31> {
        SOFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtag_sel](index.html) module"]
pub struct JTAG_SEL_SPEC;
impl crate::RegisterSpec for JTAG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtag_sel::R](R) reader structure"]
impl crate::Readable for JTAG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jtag_sel::W](W) writer structure"]
impl crate::Writable for JTAG_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JTAG_SEL to value 0x8000_0000"]
impl crate::Resettable for JTAG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
