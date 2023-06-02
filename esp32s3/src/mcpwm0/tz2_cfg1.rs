#[doc = "Register `TZ2_CFG1` reader"]
pub struct R(crate::R<TZ2_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZ2_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZ2_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZ2_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZ2_CFG1` writer"]
pub struct W(crate::W<TZ2_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZ2_CFG1_SPEC>;
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
impl From<crate::W<TZ2_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZ2_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZ2_CLR_OST` reader - a rising edge will clear on going one-shot mode action"]
pub type TZ2_CLR_OST_R = crate::BitReader;
#[doc = "Field `TZ2_CLR_OST` writer - a rising edge will clear on going one-shot mode action"]
pub type TZ2_CLR_OST_W<'a, const O: u8> = crate::BitWriter<'a, TZ2_CFG1_SPEC, O>;
#[doc = "Field `TZ2_CBCPULSE` reader - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
pub type TZ2_CBCPULSE_R = crate::FieldReader;
#[doc = "Field `TZ2_CBCPULSE` writer - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
pub type TZ2_CBCPULSE_W<'a, const O: u8> = crate::FieldWriter<'a, TZ2_CFG1_SPEC, 2, O>;
#[doc = "Field `TZ2_FORCE_CBC` reader - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ2_FORCE_CBC_R = crate::BitReader;
#[doc = "Field `TZ2_FORCE_CBC` writer - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ2_FORCE_CBC_W<'a, const O: u8> = crate::BitWriter<'a, TZ2_CFG1_SPEC, O>;
#[doc = "Field `TZ2_FORCE_OST` reader - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ2_FORCE_OST_R = crate::BitReader;
#[doc = "Field `TZ2_FORCE_OST` writer - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ2_FORCE_OST_W<'a, const O: u8> = crate::BitWriter<'a, TZ2_CFG1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn tz2_clr_ost(&self) -> TZ2_CLR_OST_R {
        TZ2_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
    #[inline(always)]
    pub fn tz2_cbcpulse(&self) -> TZ2_CBCPULSE_R {
        TZ2_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn tz2_force_cbc(&self) -> TZ2_FORCE_CBC_R {
        TZ2_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn tz2_force_ost(&self) -> TZ2_FORCE_OST_R {
        TZ2_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZ2_CFG1")
            .field("tz2_clr_ost", &format_args!("{}", self.tz2_clr_ost().bit()))
            .field(
                "tz2_cbcpulse",
                &format_args!("{}", self.tz2_cbcpulse().bits()),
            )
            .field(
                "tz2_force_cbc",
                &format_args!("{}", self.tz2_force_cbc().bit()),
            )
            .field(
                "tz2_force_ost",
                &format_args!("{}", self.tz2_force_ost().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TZ2_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_clr_ost(&mut self) -> TZ2_CLR_OST_W<0> {
        TZ2_CLR_OST_W::new(self)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_cbcpulse(&mut self) -> TZ2_CBCPULSE_W<1> {
        TZ2_CBCPULSE_W::new(self)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_force_cbc(&mut self) -> TZ2_FORCE_CBC_W<3> {
        TZ2_FORCE_CBC_W::new(self)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_force_ost(&mut self) -> TZ2_FORCE_OST_W<4> {
        TZ2_FORCE_OST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software triggers for fault handler actions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tz2_cfg1](index.html) module"]
pub struct TZ2_CFG1_SPEC;
impl crate::RegisterSpec for TZ2_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tz2_cfg1::R](R) reader structure"]
impl crate::Readable for TZ2_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tz2_cfg1::W](W) writer structure"]
impl crate::Writable for TZ2_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TZ2_CFG1 to value 0"]
impl crate::Resettable for TZ2_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
