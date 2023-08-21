#[doc = "Register `I2SRXEOF_NUM` reader"]
pub type R = crate::R<I2SRXEOF_NUM_SPEC>;
#[doc = "Register `I2SRXEOF_NUM` writer"]
pub type W = crate::W<I2SRXEOF_NUM_SPEC>;
#[doc = "Field `I2S_I2S_RX_EOF_NUM` reader - "]
pub type I2S_I2S_RX_EOF_NUM_R = crate::FieldReader<u32>;
#[doc = "Field `I2S_I2S_RX_EOF_NUM` writer - "]
pub type I2S_I2S_RX_EOF_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_i2s_rx_eof_num(&self) -> I2S_I2S_RX_EOF_NUM_R {
        I2S_I2S_RX_EOF_NUM_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SRXEOF_NUM")
            .field(
                "i2s_i2s_rx_eof_num",
                &format_args!("{}", self.i2s_i2s_rx_eof_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2SRXEOF_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_eof_num(&mut self) -> I2S_I2S_RX_EOF_NUM_W<I2SRXEOF_NUM_SPEC, 0> {
        I2S_I2S_RX_EOF_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2SRXEOF_NUM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2srxeof_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2srxeof_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SRXEOF_NUM_SPEC;
impl crate::RegisterSpec for I2SRXEOF_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2srxeof_num::R`](R) reader structure"]
impl crate::Readable for I2SRXEOF_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2srxeof_num::W`](W) writer structure"]
impl crate::Writable for I2SRXEOF_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SRXEOF_NUM to value 0"]
impl crate::Resettable for I2SRXEOF_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
