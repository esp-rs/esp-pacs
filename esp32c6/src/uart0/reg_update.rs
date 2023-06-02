#[doc = "Register `REG_UPDATE` reader"]
pub struct R(crate::R<REG_UPDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_UPDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_UPDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_UPDATE` writer"]
pub struct W(crate::W<REG_UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_UPDATE_SPEC>;
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
impl From<crate::W<REG_UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_UPDATE` reader - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
pub type REG_UPDATE_R = crate::BitReader;
#[doc = "Field `REG_UPDATE` writer - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
pub type REG_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, REG_UPDATE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
    #[inline(always)]
    pub fn reg_update(&self) -> REG_UPDATE_R {
        REG_UPDATE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_UPDATE")
            .field("reg_update", &format_args!("{}", self.reg_update().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
    #[inline(always)]
    #[must_use]
    pub fn reg_update(&mut self) -> REG_UPDATE_W<0> {
        REG_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Registers Configuration Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_update](index.html) module"]
pub struct REG_UPDATE_SPEC;
impl crate::RegisterSpec for REG_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_update::R](R) reader structure"]
impl crate::Readable for REG_UPDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_update::W](W) writer structure"]
impl crate::Writable for REG_UPDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_UPDATE to value 0"]
impl crate::Resettable for REG_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
