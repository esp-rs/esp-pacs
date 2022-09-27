#[doc = "Register `DCACHE_OCCUPY_CTRL` reader"]
pub struct R(crate::R<DCACHE_OCCUPY_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_OCCUPY_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_OCCUPY_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_OCCUPY_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_OCCUPY_CTRL` writer"]
pub struct W(crate::W<DCACHE_OCCUPY_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_OCCUPY_CTRL_SPEC>;
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
impl From<crate::W<DCACHE_OCCUPY_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_OCCUPY_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_OCCUPY_ENA` reader - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
pub type DCACHE_OCCUPY_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DCACHE_OCCUPY_ENA` writer - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
pub type DCACHE_OCCUPY_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCACHE_OCCUPY_CTRL_SPEC, bool, O>;
#[doc = "Field `DCACHE_OCCUPY_DONE` reader - The bit is used to indicate occupy operation is finished."]
pub type DCACHE_OCCUPY_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
    #[inline(always)]
    pub fn dcache_occupy_ena(&self) -> DCACHE_OCCUPY_ENA_R {
        DCACHE_OCCUPY_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate occupy operation is finished."]
    #[inline(always)]
    pub fn dcache_occupy_done(&self) -> DCACHE_OCCUPY_DONE_R {
        DCACHE_OCCUPY_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
    #[inline(always)]
    pub fn dcache_occupy_ena(&mut self) -> DCACHE_OCCUPY_ENA_W<0> {
        DCACHE_OCCUPY_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_occupy_ctrl](index.html) module"]
pub struct DCACHE_OCCUPY_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_OCCUPY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_occupy_ctrl::R](R) reader structure"]
impl crate::Readable for DCACHE_OCCUPY_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_occupy_ctrl::W](W) writer structure"]
impl crate::Writable for DCACHE_OCCUPY_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_OCCUPY_CTRL to value 0x02"]
impl crate::Resettable for DCACHE_OCCUPY_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
