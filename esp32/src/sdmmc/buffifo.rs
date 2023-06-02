#[doc = "Register `BUFFIFO` reader"]
pub struct R(crate::R<BUFFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFFIFO` writer"]
pub struct W(crate::W<BUFFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFFIFO_SPEC>;
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
impl From<crate::W<BUFFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFFIFO` reader - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
pub type BUFFIFO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUFFIFO` writer - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
pub type BUFFIFO_W<'a, const O: u8> = crate::FieldWriter<'a, BUFFIFO_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
    #[inline(always)]
    pub fn buffifo(&self) -> BUFFIFO_R {
        BUFFIFO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUFFIFO")
            .field("buffifo", &format_args!("{}", self.buffifo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUFFIFO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
    #[inline(always)]
    #[must_use]
    pub fn buffifo(&mut self) -> BUFFIFO_W<0> {
        BUFFIFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU write and read transmit data by FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buffifo](index.html) module"]
pub struct BUFFIFO_SPEC;
impl crate::RegisterSpec for BUFFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buffifo::R](R) reader structure"]
impl crate::Readable for BUFFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buffifo::W](W) writer structure"]
impl crate::Writable for BUFFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFFIFO to value 0"]
impl crate::Resettable for BUFFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
