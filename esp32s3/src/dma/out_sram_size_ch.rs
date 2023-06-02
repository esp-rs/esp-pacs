#[doc = "Register `OUT_SRAM_SIZE_CH%s` reader"]
pub struct R(crate::R<OUT_SRAM_SIZE_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_SRAM_SIZE_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_SRAM_SIZE_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_SRAM_SIZE_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_SRAM_SIZE_CH%s` writer"]
pub struct W(crate::W<OUT_SRAM_SIZE_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_SRAM_SIZE_CH_SPEC>;
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
impl From<crate::W<OUT_SRAM_SIZE_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_SRAM_SIZE_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_SIZE` reader - This register is used to configure the size of L2 Tx FIFO for Tx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
pub type OUT_SIZE_R = crate::FieldReader;
#[doc = "Field `OUT_SIZE` writer - This register is used to configure the size of L2 Tx FIFO for Tx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
pub type OUT_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, OUT_SRAM_SIZE_CH_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - This register is used to configure the size of L2 Tx FIFO for Tx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
    #[inline(always)]
    pub fn out_size(&self) -> OUT_SIZE_R {
        OUT_SIZE_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_SRAM_SIZE_CH")
            .field("out_size", &format_args!("{}", self.out_size().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_SRAM_SIZE_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - This register is used to configure the size of L2 Tx FIFO for Tx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn out_size(&mut self) -> OUT_SIZE_W<0> {
        OUT_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit L2 FIFO depth of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_sram_size_ch](index.html) module"]
pub struct OUT_SRAM_SIZE_CH_SPEC;
impl crate::RegisterSpec for OUT_SRAM_SIZE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_sram_size_ch::R](R) reader structure"]
impl crate::Readable for OUT_SRAM_SIZE_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_sram_size_ch::W](W) writer structure"]
impl crate::Writable for OUT_SRAM_SIZE_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_SRAM_SIZE_CH%s to value 0x0e"]
impl crate::Resettable for OUT_SRAM_SIZE_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e;
}
