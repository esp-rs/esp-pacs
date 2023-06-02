#[doc = "Register `IN_SRAM_SIZE_CH%s` reader"]
pub struct R(crate::R<IN_SRAM_SIZE_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_SRAM_SIZE_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_SRAM_SIZE_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_SRAM_SIZE_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_SRAM_SIZE_CH%s` writer"]
pub struct W(crate::W<IN_SRAM_SIZE_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_SRAM_SIZE_CH_SPEC>;
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
impl From<crate::W<IN_SRAM_SIZE_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_SRAM_SIZE_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_SIZE` reader - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
pub type IN_SIZE_R = crate::FieldReader;
#[doc = "Field `IN_SIZE` writer - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
pub type IN_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, IN_SRAM_SIZE_CH_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
    #[inline(always)]
    pub fn in_size(&self) -> IN_SIZE_R {
        IN_SIZE_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_SRAM_SIZE_CH")
            .field("in_size", &format_args!("{}", self.in_size().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_SRAM_SIZE_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn in_size(&mut self) -> IN_SIZE_W<0> {
        IN_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive L2 FIFO depth of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_sram_size_ch](index.html) module"]
pub struct IN_SRAM_SIZE_CH_SPEC;
impl crate::RegisterSpec for IN_SRAM_SIZE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_sram_size_ch::R](R) reader structure"]
impl crate::Readable for IN_SRAM_SIZE_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_sram_size_ch::W](W) writer structure"]
impl crate::Writable for IN_SRAM_SIZE_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_SRAM_SIZE_CH%s to value 0x0e"]
impl crate::Resettable for IN_SRAM_SIZE_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e;
}
