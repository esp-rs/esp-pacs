#[doc = "Register `SRAM_CTRL_0` reader"]
pub struct R(crate::R<SRAM_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_CTRL_0` writer"]
pub struct W(crate::W<SRAM_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CTRL_0_SPEC>;
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
impl From<crate::W<SRAM_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_FO` reader - This field is used to force on clock gate of internal SRAM."]
pub type SRAM_FO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRAM_FO` writer - This field is used to force on clock gate of internal SRAM."]
pub type SRAM_FO_W<'a, const O: u8> = crate::FieldWriter<'a, SRAM_CTRL_0_SPEC, 22, O, u32, u32>;
impl R {
    #[doc = "Bits 0:21 - This field is used to force on clock gate of internal SRAM."]
    #[inline(always)]
    pub fn sram_fo(&self) -> SRAM_FO_R {
        SRAM_FO_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CTRL_0")
            .field("sram_fo", &format_args!("{}", self.sram_fo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_CTRL_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - This field is used to force on clock gate of internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn sram_fo(&mut self) -> SRAM_FO_W<0> {
        SRAM_FO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System SRAM configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_ctrl_0](index.html) module"]
pub struct SRAM_CTRL_0_SPEC;
impl crate::RegisterSpec for SRAM_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_ctrl_0::R](R) reader structure"]
impl crate::Readable for SRAM_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_ctrl_0::W](W) writer structure"]
impl crate::Writable for SRAM_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_CTRL_0 to value 0x003f_ffff"]
impl crate::Resettable for SRAM_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_ffff;
}
