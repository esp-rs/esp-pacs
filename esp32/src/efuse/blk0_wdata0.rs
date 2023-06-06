#[doc = "Register `BLK0_WDATA0` reader"]
pub struct R(crate::R<BLK0_WDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_WDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_WDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_WDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_WDATA0` writer"]
pub struct W(crate::W<BLK0_WDATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_WDATA0_SPEC>;
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
impl From<crate::W<BLK0_WDATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_WDATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_DIS` reader - program for efuse_wr_disable"]
pub type WR_DIS_R = crate::FieldReader<u16>;
#[doc = "Field `WR_DIS` writer - program for efuse_wr_disable"]
pub type WR_DIS_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA0_SPEC, 16, O, u16>;
#[doc = "Field `RD_DIS` reader - program for efuse_rd_disable"]
pub type RD_DIS_R = crate::FieldReader;
#[doc = "Field `RD_DIS` writer - program for efuse_rd_disable"]
pub type RD_DIS_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA0_SPEC, 4, O>;
#[doc = "Field `FLASH_CRYPT_CNT` reader - program for flash_crypt_cnt"]
pub type FLASH_CRYPT_CNT_R = crate::FieldReader;
#[doc = "Field `FLASH_CRYPT_CNT` writer - program for flash_crypt_cnt"]
pub type FLASH_CRYPT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA0_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:15 - program for efuse_wr_disable"]
    #[inline(always)]
    pub fn wr_dis(&self) -> WR_DIS_R {
        WR_DIS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - program for efuse_rd_disable"]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:26 - program for flash_crypt_cnt"]
    #[inline(always)]
    pub fn flash_crypt_cnt(&self) -> FLASH_CRYPT_CNT_R {
        FLASH_CRYPT_CNT_R::new(((self.bits >> 20) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA0")
            .field("wr_dis", &format_args!("{}", self.wr_dis().bits()))
            .field("rd_dis", &format_args!("{}", self.rd_dis().bits()))
            .field(
                "flash_crypt_cnt",
                &format_args!("{}", self.flash_crypt_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_WDATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - program for efuse_wr_disable"]
    #[inline(always)]
    #[must_use]
    pub fn wr_dis(&mut self) -> WR_DIS_W<0> {
        WR_DIS_W::new(self)
    }
    #[doc = "Bits 16:19 - program for efuse_rd_disable"]
    #[inline(always)]
    #[must_use]
    pub fn rd_dis(&mut self) -> RD_DIS_W<16> {
        RD_DIS_W::new(self)
    }
    #[doc = "Bits 20:26 - program for flash_crypt_cnt"]
    #[inline(always)]
    #[must_use]
    pub fn flash_crypt_cnt(&mut self) -> FLASH_CRYPT_CNT_W<20> {
        FLASH_CRYPT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_wdata0](index.html) module"]
pub struct BLK0_WDATA0_SPEC;
impl crate::RegisterSpec for BLK0_WDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_wdata0::R](R) reader structure"]
impl crate::Readable for BLK0_WDATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_wdata0::W](W) writer structure"]
impl crate::Writable for BLK0_WDATA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA0 to value 0"]
impl crate::Resettable for BLK0_WDATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
