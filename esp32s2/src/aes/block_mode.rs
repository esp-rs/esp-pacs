#[doc = "Register `BLOCK_MODE` reader"]
pub type R = crate::R<BLOCK_MODE_SPEC>;
#[doc = "Register `BLOCK_MODE` writer"]
pub type W = crate::W<BLOCK_MODE_SPEC>;
#[doc = "Field `BLOCK_MODE` reader - Defines the operation type of the AES Accelerator operating under the DMA-AES working mode. For details, see Table 8. & 3'h0(BLOCK_MODE_ECB): ECB # 3'h1(BLOCK_MODE_CBC): CBC # 3'h2(BLOCK_MODE_OFB): OFB # 3'h3(BLOCK_MODE_CTR): CTR # 3'h4(BLOCK_MODE_CFB8): CFB-8 # 3'h5(BLOCK_MODE_CFB128): CFB-128 # 3'h6(BLOCK_MODE_GCM): GCM &"]
pub type BLOCK_MODE_R = crate::FieldReader;
#[doc = "Field `BLOCK_MODE` writer - Defines the operation type of the AES Accelerator operating under the DMA-AES working mode. For details, see Table 8. & 3'h0(BLOCK_MODE_ECB): ECB # 3'h1(BLOCK_MODE_CBC): CBC # 3'h2(BLOCK_MODE_OFB): OFB # 3'h3(BLOCK_MODE_CTR): CTR # 3'h4(BLOCK_MODE_CFB8): CFB-8 # 3'h5(BLOCK_MODE_CFB128): CFB-128 # 3'h6(BLOCK_MODE_GCM): GCM &"]
pub type BLOCK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Defines the operation type of the AES Accelerator operating under the DMA-AES working mode. For details, see Table 8. & 3'h0(BLOCK_MODE_ECB): ECB # 3'h1(BLOCK_MODE_CBC): CBC # 3'h2(BLOCK_MODE_OFB): OFB # 3'h3(BLOCK_MODE_CTR): CTR # 3'h4(BLOCK_MODE_CFB8): CFB-8 # 3'h5(BLOCK_MODE_CFB128): CFB-128 # 3'h6(BLOCK_MODE_GCM): GCM &"]
    #[inline(always)]
    pub fn block_mode(&self) -> BLOCK_MODE_R {
        BLOCK_MODE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLOCK_MODE")
            .field("block_mode", &self.block_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Defines the operation type of the AES Accelerator operating under the DMA-AES working mode. For details, see Table 8. & 3'h0(BLOCK_MODE_ECB): ECB # 3'h1(BLOCK_MODE_CBC): CBC # 3'h2(BLOCK_MODE_OFB): OFB # 3'h3(BLOCK_MODE_CTR): CTR # 3'h4(BLOCK_MODE_CFB8): CFB-8 # 3'h5(BLOCK_MODE_CFB128): CFB-128 # 3'h6(BLOCK_MODE_GCM): GCM &"]
    #[inline(always)]
    pub fn block_mode(&mut self) -> BLOCK_MODE_W<BLOCK_MODE_SPEC> {
        BLOCK_MODE_W::new(self, 0)
    }
}
#[doc = "Block operation type register\n\nYou can [`read`](crate::Reg::read) this register and get [`block_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLOCK_MODE_SPEC;
impl crate::RegisterSpec for BLOCK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`block_mode::R`](R) reader structure"]
impl crate::Readable for BLOCK_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`block_mode::W`](W) writer structure"]
impl crate::Writable for BLOCK_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLOCK_MODE to value 0"]
impl crate::Resettable for BLOCK_MODE_SPEC {}
