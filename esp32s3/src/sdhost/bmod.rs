#[doc = "Register `BMOD` reader"]
pub type R = crate::R<BMOD_SPEC>;
#[doc = "Register `BMOD` writer"]
pub type W = crate::W<BMOD_SPEC>;
#[doc = "Field `SWR` reader - Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle."]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle."]
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB` reader - Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations."]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations."]
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE` reader - IDMAC Enable. When set, the IDMAC is enabled."]
pub type DE_R = crate::BitReader;
#[doc = "Field `DE` writer - IDMAC Enable. When set, the IDMAC is enabled."]
pub type DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBL` reader - Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows: 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer. PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access."]
pub type PBL_R = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows: 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer. PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access."]
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle."]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations."]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - IDMAC Enable. When set, the IDMAC is enabled."]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows: 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer. PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access."]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMOD")
            .field("swr", &self.swr())
            .field("fb", &self.fb())
            .field("de", &self.de())
            .field("pbl", &self.pbl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle."]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<'_, BMOD_SPEC> {
        SWR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations."]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<'_, BMOD_SPEC> {
        FB_W::new(self, 1)
    }
    #[doc = "Bit 7 - IDMAC Enable. When set, the IDMAC is enabled."]
    #[inline(always)]
    pub fn de(&mut self) -> DE_W<'_, BMOD_SPEC> {
        DE_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows: 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer. PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access."]
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W<'_, BMOD_SPEC> {
        PBL_W::new(self, 8)
    }
}
#[doc = "Burst mode transfer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMOD_SPEC;
impl crate::RegisterSpec for BMOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmod::R`](R) reader structure"]
impl crate::Readable for BMOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmod::W`](W) writer structure"]
impl crate::Writable for BMOD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMOD to value 0"]
impl crate::Resettable for BMOD_SPEC {}
