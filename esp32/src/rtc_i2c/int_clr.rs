#[doc = "Register `INT_CLR` reader"]
pub struct R(crate::R<INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_TRANS_COMPLETE_INT_CLR` reader - "]
pub type SLAVE_TRANS_COMPLETE_INT_CLR_R = crate::BitReader;
#[doc = "Field `SLAVE_TRANS_COMPLETE_INT_CLR` writer - "]
pub type SLAVE_TRANS_COMPLETE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `ARBITRATION_LOST_INT_CLR` reader - "]
pub type ARBITRATION_LOST_INT_CLR_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_CLR` writer - "]
pub type ARBITRATION_LOST_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `MASTER_TRANS_COMPLETE_INT_CLR` reader - "]
pub type MASTER_TRANS_COMPLETE_INT_CLR_R = crate::BitReader;
#[doc = "Field `MASTER_TRANS_COMPLETE_INT_CLR` writer - "]
pub type MASTER_TRANS_COMPLETE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TRANS_COMPLETE_INT_CLR` reader - "]
pub type TRANS_COMPLETE_INT_CLR_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE_INT_CLR` writer - "]
pub type TRANS_COMPLETE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIME_OUT_INT_CLR` writer - "]
pub type TIME_OUT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slave_trans_complete_int_clr(&self) -> SLAVE_TRANS_COMPLETE_INT_CLR_R {
        SLAVE_TRANS_COMPLETE_INT_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn arbitration_lost_int_clr(&self) -> ARBITRATION_LOST_INT_CLR_R {
        ARBITRATION_LOST_INT_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn master_trans_complete_int_clr(&self) -> MASTER_TRANS_COMPLETE_INT_CLR_R {
        MASTER_TRANS_COMPLETE_INT_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trans_complete_int_clr(&self) -> TRANS_COMPLETE_INT_CLR_R {
        TRANS_COMPLETE_INT_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLR")
            .field(
                "slave_trans_complete_int_clr",
                &format_args!("{}", self.slave_trans_complete_int_clr().bit()),
            )
            .field(
                "arbitration_lost_int_clr",
                &format_args!("{}", self.arbitration_lost_int_clr().bit()),
            )
            .field(
                "master_trans_complete_int_clr",
                &format_args!("{}", self.master_trans_complete_int_clr().bit()),
            )
            .field(
                "trans_complete_int_clr",
                &format_args!("{}", self.trans_complete_int_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn slave_trans_complete_int_clr(&mut self) -> SLAVE_TRANS_COMPLETE_INT_CLR_W<4> {
        SLAVE_TRANS_COMPLETE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_clr(&mut self) -> ARBITRATION_LOST_INT_CLR_W<5> {
        ARBITRATION_LOST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn master_trans_complete_int_clr(&mut self) -> MASTER_TRANS_COMPLETE_INT_CLR_W<6> {
        MASTER_TRANS_COMPLETE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete_int_clr(&mut self) -> TRANS_COMPLETE_INT_CLR_W<7> {
        TRANS_COMPLETE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn time_out_int_clr(&mut self) -> TIME_OUT_INT_CLR_W<8> {
        TIME_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_clr::R](R) reader structure"]
impl crate::Readable for INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
