#[doc = "Register `BMOD` reader"]
pub struct R(crate::R<BMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMOD` writer"]
pub struct W(crate::W<BMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMOD_SPEC>;
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
impl From<crate::W<BMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWR` reader - Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle."]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle."]
pub type SWR_W<'a, const O: u8> = crate::BitWriter<'a, BMOD_SPEC, O>;
#[doc = "Field `FB` reader - Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations."]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations."]
pub type FB_W<'a, const O: u8> = crate::BitWriter<'a, BMOD_SPEC, O>;
#[doc = "Field `DE` reader - IDMAC Enable. When set, the IDMAC is enabled."]
pub type DE_R = crate::BitReader;
#[doc = "Field `DE` writer - IDMAC Enable. When set, the IDMAC is enabled."]
pub type DE_W<'a, const O: u8> = crate::BitWriter<'a, BMOD_SPEC, O>;
#[doc = "Field `PBL` reader - Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows: 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer. PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access."]
pub type PBL_R = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows: 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer. PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access."]
pub type PBL_W<'a, const O: u8> = crate::FieldWriter<'a, BMOD_SPEC, 3, O>;
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
            .field("swr", &format_args!("{}", self.swr().bit()))
            .field("fb", &format_args!("{}", self.fb().bit()))
            .field("de", &format_args!("{}", self.de().bit()))
            .field("pbl", &format_args!("{}", self.pbl().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BMOD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset. When set, the DMA Controller resets all its internal registers. It is automatically cleared after one clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<0> {
        SWR_W::new(self)
    }
    #[doc = "Bit 1 - Fixed Burst. Controls whether the AHB Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AHB will use SINGLE and INCR burst transfer operations."]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<1> {
        FB_W::new(self)
    }
    #[doc = "Bit 7 - IDMAC Enable. When set, the IDMAC is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn de(&mut self) -> DE_W<7> {
        DE_W::new(self)
    }
    #[doc = "Bits 8:10 - Programmable Burst Length. These bits indicate the maximum number of beats to be performed in one IDMAC???Internal DMA Control???transaction. The IDMAC will always attempt to burst as specified in PBL each time it starts a burst transfer on the host bus. The permissible values are 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of MSIZE of FIFOTH register. In order to change this value, write the required value to FIFOTH register. This is an encode value as follows: 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer. PBL is a read-only value and is applicable only for data access, it does not apply to descriptor access."]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<8> {
        PBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst mode transfer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmod](index.html) module"]
pub struct BMOD_SPEC;
impl crate::RegisterSpec for BMOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmod::R](R) reader structure"]
impl crate::Readable for BMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmod::W](W) writer structure"]
impl crate::Writable for BMOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMOD to value 0"]
impl crate::Resettable for BMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
