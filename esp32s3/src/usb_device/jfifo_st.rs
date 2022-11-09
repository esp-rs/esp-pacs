#[doc = "Register `JFIFO_ST` reader"]
pub struct R(crate::R<JFIFO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JFIFO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JFIFO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JFIFO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JFIFO_ST` writer"]
pub struct W(crate::W<JFIFO_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JFIFO_ST_SPEC>;
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
impl From<crate::W<JFIFO_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JFIFO_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_FIFO_CNT` reader - JTAT in fifo counter."]
pub type IN_FIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN_FIFO_EMPTY` reader - 1: JTAG in fifo is empty."]
pub type IN_FIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `IN_FIFO_FULL` reader - 1: JTAG in fifo is full."]
pub type IN_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `OUT_FIFO_CNT` reader - JTAT out fifo counter."]
pub type OUT_FIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_FIFO_EMPTY` reader - 1: JTAG out fifo is empty."]
pub type OUT_FIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `OUT_FIFO_FULL` reader - 1: JTAG out fifo is full."]
pub type OUT_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `IN_FIFO_RESET` reader - Write 1 to reset JTAG in fifo."]
pub type IN_FIFO_RESET_R = crate::BitReader<bool>;
#[doc = "Field `IN_FIFO_RESET` writer - Write 1 to reset JTAG in fifo."]
pub type IN_FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, JFIFO_ST_SPEC, bool, O>;
#[doc = "Field `OUT_FIFO_RESET` reader - Write 1 to reset JTAG out fifo."]
pub type OUT_FIFO_RESET_R = crate::BitReader<bool>;
#[doc = "Field `OUT_FIFO_RESET` writer - Write 1 to reset JTAG out fifo."]
pub type OUT_FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, JFIFO_ST_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - JTAT in fifo counter."]
    #[inline(always)]
    pub fn in_fifo_cnt(&self) -> IN_FIFO_CNT_R {
        IN_FIFO_CNT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 1: JTAG in fifo is empty."]
    #[inline(always)]
    pub fn in_fifo_empty(&self) -> IN_FIFO_EMPTY_R {
        IN_FIFO_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: JTAG in fifo is full."]
    #[inline(always)]
    pub fn in_fifo_full(&self) -> IN_FIFO_FULL_R {
        IN_FIFO_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - JTAT out fifo counter."]
    #[inline(always)]
    pub fn out_fifo_cnt(&self) -> OUT_FIFO_CNT_R {
        OUT_FIFO_CNT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 1: JTAG out fifo is empty."]
    #[inline(always)]
    pub fn out_fifo_empty(&self) -> OUT_FIFO_EMPTY_R {
        OUT_FIFO_EMPTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: JTAG out fifo is full."]
    #[inline(always)]
    pub fn out_fifo_full(&self) -> OUT_FIFO_FULL_R {
        OUT_FIFO_FULL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write 1 to reset JTAG in fifo."]
    #[inline(always)]
    pub fn in_fifo_reset(&self) -> IN_FIFO_RESET_R {
        IN_FIFO_RESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1 to reset JTAG out fifo."]
    #[inline(always)]
    pub fn out_fifo_reset(&self) -> OUT_FIFO_RESET_R {
        OUT_FIFO_RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Write 1 to reset JTAG in fifo."]
    #[inline(always)]
    #[must_use]
    pub fn in_fifo_reset(&mut self) -> IN_FIFO_RESET_W<8> {
        IN_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 9 - Write 1 to reset JTAG out fifo."]
    #[inline(always)]
    #[must_use]
    pub fn out_fifo_reset(&mut self) -> OUT_FIFO_RESET_W<9> {
        OUT_FIFO_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB-JTAG FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jfifo_st](index.html) module"]
pub struct JFIFO_ST_SPEC;
impl crate::RegisterSpec for JFIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jfifo_st::R](R) reader structure"]
impl crate::Readable for JFIFO_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jfifo_st::W](W) writer structure"]
impl crate::Writable for JFIFO_ST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JFIFO_ST to value 0x44"]
impl crate::Resettable for JFIFO_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0x44;
}
