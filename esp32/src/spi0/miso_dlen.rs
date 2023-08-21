#[doc = "Register `MISO_DLEN` reader"]
pub type R = crate::R<MISO_DLEN_SPEC>;
#[doc = "Register `MISO_DLEN` writer"]
pub type W = crate::W<MISO_DLEN_SPEC>;
#[doc = "Field `USR_MISO_DBITLEN` reader - The length in bits of read-data. The register value shall be (bit_num-1)."]
pub type USR_MISO_DBITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `USR_MISO_DBITLEN` writer - The length in bits of read-data. The register value shall be (bit_num-1)."]
pub type USR_MISO_DBITLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - The length in bits of read-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn usr_miso_dbitlen(&self) -> USR_MISO_DBITLEN_R {
        USR_MISO_DBITLEN_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISO_DLEN")
            .field(
                "usr_miso_dbitlen",
                &format_args!("{}", self.usr_miso_dbitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MISO_DLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - The length in bits of read-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn usr_miso_dbitlen(&mut self) -> USR_MISO_DBITLEN_W<MISO_DLEN_SPEC, 0> {
        USR_MISO_DBITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`miso_dlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miso_dlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISO_DLEN_SPEC;
impl crate::RegisterSpec for MISO_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miso_dlen::R`](R) reader structure"]
impl crate::Readable for MISO_DLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`miso_dlen::W`](W) writer structure"]
impl crate::Writable for MISO_DLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISO_DLEN to value 0"]
impl crate::Resettable for MISO_DLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
