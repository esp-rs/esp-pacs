#[doc = "Register `CORE_0_RCD_EN` reader"]
pub struct R(crate::R<CORE_0_RCD_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_RCD_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_RCD_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_RCD_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_RCD_EN` writer"]
pub struct W(crate::W<CORE_0_RCD_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_RCD_EN_SPEC>;
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
impl From<crate::W<CORE_0_RCD_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_RCD_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_RCD_RECORDEN` reader - Set 1 to enable record PC"]
pub type CORE_0_RCD_RECORDEN_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_RCD_RECORDEN` writer - Set 1 to enable record PC"]
pub type CORE_0_RCD_RECORDEN_W<'a> = crate::BitWriter<'a, u32, CORE_0_RCD_EN_SPEC, bool, 0>;
#[doc = "Field `CORE_0_RCD_PDEBUGEN` reader - Set 1 to enable cpu pdebug function, must set this bit can get cpu PC"]
pub type CORE_0_RCD_PDEBUGEN_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_RCD_PDEBUGEN` writer - Set 1 to enable cpu pdebug function, must set this bit can get cpu PC"]
pub type CORE_0_RCD_PDEBUGEN_W<'a> = crate::BitWriter<'a, u32, CORE_0_RCD_EN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable record PC"]
    #[inline(always)]
    pub fn core_0_rcd_recorden(&self) -> CORE_0_RCD_RECORDEN_R {
        CORE_0_RCD_RECORDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to enable cpu pdebug function, must set this bit can get cpu PC"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugen(&self) -> CORE_0_RCD_PDEBUGEN_R {
        CORE_0_RCD_PDEBUGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable record PC"]
    #[inline(always)]
    pub fn core_0_rcd_recorden(&mut self) -> CORE_0_RCD_RECORDEN_W {
        CORE_0_RCD_RECORDEN_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 to enable cpu pdebug function, must set this bit can get cpu PC"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugen(&mut self) -> CORE_0_RCD_PDEBUGEN_W {
        CORE_0_RCD_PDEBUGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "record enable configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_rcd_en](index.html) module"]
pub struct CORE_0_RCD_EN_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_rcd_en::R](R) reader structure"]
impl crate::Readable for CORE_0_RCD_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_rcd_en::W](W) writer structure"]
impl crate::Writable for CORE_0_RCD_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_RCD_EN to value 0"]
impl crate::Resettable for CORE_0_RCD_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
