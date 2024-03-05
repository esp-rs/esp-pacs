#[doc = "Register `TX_TAILING_BITS` reader"]
pub type R = crate::R<TX_TAILING_BITS_SPEC>;
#[doc = "Register `TX_TAILING_BITS` writer"]
pub type W = crate::W<TX_TAILING_BITS_SPEC>;
#[doc = "Field `TX_TAILING_BITS` reader - write this bits to specify the extra data bit length after getting EOF"]
pub type TX_TAILING_BITS_R = crate::FieldReader<u16>;
#[doc = "Field `TX_TAILING_BITS` writer - write this bits to specify the extra data bit length after getting EOF"]
pub type TX_TAILING_BITS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - write this bits to specify the extra data bit length after getting EOF"]
    #[inline(always)]
    pub fn tx_tailing_bits(&self) -> TX_TAILING_BITS_R {
        TX_TAILING_BITS_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_TAILING_BITS")
            .field(
                "tx_tailing_bits",
                &format_args!("{}", self.tx_tailing_bits().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_TAILING_BITS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - write this bits to specify the extra data bit length after getting EOF"]
    #[inline(always)]
    #[must_use]
    pub fn tx_tailing_bits(&mut self) -> TX_TAILING_BITS_W<TX_TAILING_BITS_SPEC> {
        TX_TAILING_BITS_W::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_tailing_bits::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_tailing_bits::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_TAILING_BITS_SPEC;
impl crate::RegisterSpec for TX_TAILING_BITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_tailing_bits::R`](R) reader structure"]
impl crate::Readable for TX_TAILING_BITS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_tailing_bits::W`](W) writer structure"]
impl crate::Writable for TX_TAILING_BITS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_TAILING_BITS to value 0"]
impl crate::Resettable for TX_TAILING_BITS_SPEC {
    const RESET_VALUE: u32 = 0;
}
