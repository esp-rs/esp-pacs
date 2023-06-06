#[doc = "Register `BUS_TIMEOUT` reader"]
pub struct R(crate::R<BUS_TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_TIMEOUT` writer"]
pub struct W(crate::W<BUS_TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_TIMEOUT_SPEC>;
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
impl From<crate::W<BUS_TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_PERI_TIMEOUT_THRES` reader - need_des"]
pub type LP_PERI_TIMEOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `LP_PERI_TIMEOUT_THRES` writer - need_des"]
pub type LP_PERI_TIMEOUT_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, BUS_TIMEOUT_SPEC, 16, O, u16>;
#[doc = "Field `LP_PERI_TIMEOUT_INT_CLEAR` writer - need_des"]
pub type LP_PERI_TIMEOUT_INT_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, BUS_TIMEOUT_SPEC, O>;
#[doc = "Field `LP_PERI_TIMEOUT_PROTECT_EN` reader - need_des"]
pub type LP_PERI_TIMEOUT_PROTECT_EN_R = crate::BitReader;
#[doc = "Field `LP_PERI_TIMEOUT_PROTECT_EN` writer - need_des"]
pub type LP_PERI_TIMEOUT_PROTECT_EN_W<'a, const O: u8> = crate::BitWriter<'a, BUS_TIMEOUT_SPEC, O>;
impl R {
    #[doc = "Bits 14:29 - need_des"]
    #[inline(always)]
    pub fn lp_peri_timeout_thres(&self) -> LP_PERI_TIMEOUT_THRES_R {
        LP_PERI_TIMEOUT_THRES_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_peri_timeout_protect_en(&self) -> LP_PERI_TIMEOUT_PROTECT_EN_R {
        LP_PERI_TIMEOUT_PROTECT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_TIMEOUT")
            .field(
                "lp_peri_timeout_thres",
                &format_args!("{}", self.lp_peri_timeout_thres().bits()),
            )
            .field(
                "lp_peri_timeout_protect_en",
                &format_args!("{}", self.lp_peri_timeout_protect_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUS_TIMEOUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 14:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_timeout_thres(&mut self) -> LP_PERI_TIMEOUT_THRES_W<14> {
        LP_PERI_TIMEOUT_THRES_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_timeout_int_clear(&mut self) -> LP_PERI_TIMEOUT_INT_CLEAR_W<30> {
        LP_PERI_TIMEOUT_INT_CLEAR_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_timeout_protect_en(&mut self) -> LP_PERI_TIMEOUT_PROTECT_EN_W<31> {
        LP_PERI_TIMEOUT_PROTECT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_timeout](index.html) module"]
pub struct BUS_TIMEOUT_SPEC;
impl crate::RegisterSpec for BUS_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_timeout::R](R) reader structure"]
impl crate::Readable for BUS_TIMEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_timeout::W](W) writer structure"]
impl crate::Writable for BUS_TIMEOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS_TIMEOUT to value 0xbfff_c000"]
impl crate::Resettable for BUS_TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0xbfff_c000;
}
