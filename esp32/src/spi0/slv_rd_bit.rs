#[doc = "Register `SLV_RD_BIT` reader"]
pub type R = crate::R<SLV_RD_BIT_SPEC>;
#[doc = "Register `SLV_RD_BIT` writer"]
pub type W = crate::W<SLV_RD_BIT_SPEC>;
#[doc = "Field `SLV_RDATA_BIT` reader - In the slave mode it is the bit length of read data. The value is the length - 1."]
pub type SLV_RDATA_BIT_R = crate::FieldReader<u32>;
#[doc = "Field `SLV_RDATA_BIT` writer - In the slave mode it is the bit length of read data. The value is the length - 1."]
pub type SLV_RDATA_BIT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - In the slave mode it is the bit length of read data. The value is the length - 1."]
    #[inline(always)]
    pub fn slv_rdata_bit(&self) -> SLV_RDATA_BIT_R {
        SLV_RDATA_BIT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLV_RD_BIT")
            .field("slv_rdata_bit", &self.slv_rdata_bit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - In the slave mode it is the bit length of read data. The value is the length - 1."]
    #[inline(always)]
    pub fn slv_rdata_bit(&mut self) -> SLV_RDATA_BIT_W<SLV_RD_BIT_SPEC> {
        SLV_RDATA_BIT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`slv_rd_bit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_rd_bit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLV_RD_BIT_SPEC;
impl crate::RegisterSpec for SLV_RD_BIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slv_rd_bit::R`](R) reader structure"]
impl crate::Readable for SLV_RD_BIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slv_rd_bit::W`](W) writer structure"]
impl crate::Writable for SLV_RD_BIT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLV_RD_BIT to value 0"]
impl crate::Resettable for SLV_RD_BIT_SPEC {
    const RESET_VALUE: u32 = 0;
}
