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
#[doc = "Field `RX_REG_UPDATE` writer - Set this bit to update rx register configuration."]
pub type RX_REG_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, REG_UPDATE_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to update rx register configuration."]
    #[inline(always)]
    #[must_use]
    pub fn rx_reg_update(&mut self) -> RX_REG_UPDATE_W<31> {
        RX_REG_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel IO FIFO configuration register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_update](index.html) module"]
pub struct REG_UPDATE_SPEC;
impl crate::RegisterSpec for REG_UPDATE_SPEC {
    type Ux = u32;
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
