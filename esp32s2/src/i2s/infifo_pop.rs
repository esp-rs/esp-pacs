#[doc = "Register `INFIFO_POP` reader"]
pub struct R(crate::R<INFIFO_POP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFIFO_POP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFIFO_POP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFIFO_POP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INFIFO_POP` writer"]
pub struct W(crate::W<INFIFO_POP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INFIFO_POP_SPEC>;
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
impl From<crate::W<INFIFO_POP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INFIFO_POP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INFIFO_RDATA` reader - APB in FIFO read data."]
pub type INFIFO_RDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INFIFO_POP` reader - APB in FIFO pop."]
pub type INFIFO_POP_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_POP` writer - APB in FIFO pop."]
pub type INFIFO_POP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INFIFO_POP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - APB in FIFO read data."]
    #[inline(always)]
    pub fn infifo_rdata(&self) -> INFIFO_RDATA_R {
        INFIFO_RDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - APB in FIFO pop."]
    #[inline(always)]
    pub fn infifo_pop(&self) -> INFIFO_POP_R {
        INFIFO_POP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - APB in FIFO pop."]
    #[inline(always)]
    pub fn infifo_pop(&mut self) -> INFIFO_POP_W<16> {
        INFIFO_POP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB in FIFO mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [infifo_pop](index.html) module"]
pub struct INFIFO_POP_SPEC;
impl crate::RegisterSpec for INFIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [infifo_pop::R](R) reader structure"]
impl crate::Readable for INFIFO_POP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [infifo_pop::W](W) writer structure"]
impl crate::Writable for INFIFO_POP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INFIFO_POP to value 0"]
impl crate::Resettable for INFIFO_POP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
