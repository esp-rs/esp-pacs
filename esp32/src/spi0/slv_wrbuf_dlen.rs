#[doc = "Register `SLV_WRBUF_DLEN` reader"]
pub type R = crate::R<SLV_WRBUF_DLEN_SPEC>;
#[doc = "Register `SLV_WRBUF_DLEN` writer"]
pub type W = crate::W<SLV_WRBUF_DLEN_SPEC>;
#[doc = "Field `SLV_WRBUF_DBITLEN` reader - In the slave mode it is the length in bits for write-buffer operations. The register value shall be (bit_num-1)."]
pub type SLV_WRBUF_DBITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `SLV_WRBUF_DBITLEN` writer - In the slave mode it is the length in bits for write-buffer operations. The register value shall be (bit_num-1)."]
pub type SLV_WRBUF_DBITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - In the slave mode it is the length in bits for write-buffer operations. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_wrbuf_dbitlen(&self) -> SLV_WRBUF_DBITLEN_R {
        SLV_WRBUF_DBITLEN_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLV_WRBUF_DLEN")
            .field(
                "slv_wrbuf_dbitlen",
                &format_args!("{}", self.slv_wrbuf_dbitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLV_WRBUF_DLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:23 - In the slave mode it is the length in bits for write-buffer operations. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrbuf_dbitlen(&mut self) -> SLV_WRBUF_DBITLEN_W<SLV_WRBUF_DLEN_SPEC> {
        SLV_WRBUF_DBITLEN_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_wrbuf_dlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_wrbuf_dlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLV_WRBUF_DLEN_SPEC;
impl crate::RegisterSpec for SLV_WRBUF_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slv_wrbuf_dlen::R`](R) reader structure"]
impl crate::Readable for SLV_WRBUF_DLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slv_wrbuf_dlen::W`](W) writer structure"]
impl crate::Writable for SLV_WRBUF_DLEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLV_WRBUF_DLEN to value 0"]
impl crate::Resettable for SLV_WRBUF_DLEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
