#[doc = "Register `JTAG_CTRL_7` writer"]
pub struct W(crate::W<JTAG_CTRL_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAG_CTRL_7_SPEC>;
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
impl From<crate::W<JTAG_CTRL_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAG_CTRL_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_7` writer - Stores the 0 to 224 bits of the 255 bits register used to cancel the temporary disable of eFuse to JTAG."]
pub type CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, JTAG_CTRL_7_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Stores the 0 to 224 bits of the 255 bits register used to cancel the temporary disable of eFuse to JTAG."]
    #[inline(always)]
    pub fn cancel_efuse_disable_jtag_temporary_7(
        &mut self,
    ) -> CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_7_W<0> {
        CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG configuration register 7\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtag_ctrl_7](index.html) module"]
pub struct JTAG_CTRL_7_SPEC;
impl crate::RegisterSpec for JTAG_CTRL_7_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [jtag_ctrl_7::W](W) writer structure"]
impl crate::Writable for JTAG_CTRL_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JTAG_CTRL_7 to value 0"]
impl crate::Resettable for JTAG_CTRL_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
