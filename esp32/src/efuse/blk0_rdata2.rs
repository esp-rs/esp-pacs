#[doc = "Register `BLK0_RDATA2` reader"]
pub struct R(crate::R<BLK0_RDATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_RDATA2` writer"]
pub struct W(crate::W<BLK0_RDATA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_RDATA2_SPEC>;
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
impl From<crate::W<BLK0_RDATA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_RDATA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_MAC_1` reader - "]
pub type RD_MAC_1_R = crate::FieldReader<u16>;
#[doc = "Field `RD_MAC_CRC` reader - "]
pub type RD_MAC_CRC_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_88` reader - "]
pub type RD_RESERVE_0_88_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_88` writer - "]
pub type RD_RESERVE_0_88_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_RDATA2_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rd_mac_1(&self) -> RD_MAC_1_R {
        RD_MAC_1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rd_mac_crc(&self) -> RD_MAC_CRC_R {
        RD_MAC_CRC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rd_reserve_0_88(&self) -> RD_RESERVE_0_88_R {
        RD_RESERVE_0_88_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA2")
            .field("rd_mac_1", &format_args!("{}", self.rd_mac_1().bits()))
            .field("rd_mac_crc", &format_args!("{}", self.rd_mac_crc().bits()))
            .field(
                "rd_reserve_0_88",
                &format_args!("{}", self.rd_reserve_0_88().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_RDATA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn rd_reserve_0_88(&mut self) -> RD_RESERVE_0_88_W<24> {
        RD_RESERVE_0_88_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata2](index.html) module"]
pub struct BLK0_RDATA2_SPEC;
impl crate::RegisterSpec for BLK0_RDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata2::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_rdata2::W](W) writer structure"]
impl crate::Writable for BLK0_RDATA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_RDATA2 to value 0"]
impl crate::Resettable for BLK0_RDATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
