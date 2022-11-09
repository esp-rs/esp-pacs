#[doc = "Register `PRO_IRAM0_4` reader"]
pub struct R(crate::R<PRO_IRAM0_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_IRAM0_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_IRAM0_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_IRAM0_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_IRAM0_4` writer"]
pub struct W(crate::W<PRO_IRAM0_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_IRAM0_4_SPEC>;
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
impl From<crate::W<PRO_IRAM0_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_IRAM0_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_IRAM0_ILG_CLR` reader - The clear signal for IBUS access interrupt."]
pub type PRO_IRAM0_ILG_CLR_R = crate::BitReader<bool>;
#[doc = "Field `PRO_IRAM0_ILG_CLR` writer - The clear signal for IBUS access interrupt."]
pub type PRO_IRAM0_ILG_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_IRAM0_4_SPEC, bool, O>;
#[doc = "Field `PRO_IRAM0_ILG_EN` reader - The enable signal for IBUS access interrupt."]
pub type PRO_IRAM0_ILG_EN_R = crate::BitReader<bool>;
#[doc = "Field `PRO_IRAM0_ILG_EN` writer - The enable signal for IBUS access interrupt."]
pub type PRO_IRAM0_ILG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRO_IRAM0_4_SPEC, bool, O>;
#[doc = "Field `PRO_IRAM0_ILG_INTR` reader - IBUS access interrupt signal."]
pub type PRO_IRAM0_ILG_INTR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The clear signal for IBUS access interrupt."]
    #[inline(always)]
    pub fn pro_iram0_ilg_clr(&self) -> PRO_IRAM0_ILG_CLR_R {
        PRO_IRAM0_ILG_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for IBUS access interrupt."]
    #[inline(always)]
    pub fn pro_iram0_ilg_en(&self) -> PRO_IRAM0_ILG_EN_R {
        PRO_IRAM0_ILG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IBUS access interrupt signal."]
    #[inline(always)]
    pub fn pro_iram0_ilg_intr(&self) -> PRO_IRAM0_ILG_INTR_R {
        PRO_IRAM0_ILG_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for IBUS access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pro_iram0_ilg_clr(&mut self) -> PRO_IRAM0_ILG_CLR_W<0> {
        PRO_IRAM0_ILG_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The enable signal for IBUS access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pro_iram0_ilg_en(&mut self) -> PRO_IRAM0_ILG_EN_W<1> {
        PRO_IRAM0_ILG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IBUS permission control register 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_iram0_4](index.html) module"]
pub struct PRO_IRAM0_4_SPEC;
impl crate::RegisterSpec for PRO_IRAM0_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_iram0_4::R](R) reader structure"]
impl crate::Readable for PRO_IRAM0_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_iram0_4::W](W) writer structure"]
impl crate::Writable for PRO_IRAM0_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_IRAM0_4 to value 0"]
impl crate::Resettable for PRO_IRAM0_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
