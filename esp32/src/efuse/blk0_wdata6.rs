#[doc = "Register `BLK0_WDATA6` reader"]
pub struct R(crate::R<BLK0_WDATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_WDATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_WDATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_WDATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_WDATA6` writer"]
pub struct W(crate::W<BLK0_WDATA6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_WDATA6_SPEC>;
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
impl From<crate::W<BLK0_WDATA6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_WDATA6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODING_SCHEME` reader - "]
pub type CODING_SCHEME_R = crate::FieldReader;
#[doc = "Field `CODING_SCHEME` writer - "]
pub type CODING_SCHEME_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA6_SPEC, 2, O>;
#[doc = "Field `CONSOLE_DEBUG_DISABLE` reader - "]
pub type CONSOLE_DEBUG_DISABLE_R = crate::BitReader;
#[doc = "Field `CONSOLE_DEBUG_DISABLE` writer - "]
pub type CONSOLE_DEBUG_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA6_SPEC, O>;
#[doc = "Field `DISABLE_SDIO_HOST` reader - "]
pub type DISABLE_SDIO_HOST_R = crate::BitReader;
#[doc = "Field `DISABLE_SDIO_HOST` writer - "]
pub type DISABLE_SDIO_HOST_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA6_SPEC, O>;
#[doc = "Field `ABS_DONE_0` reader - "]
pub type ABS_DONE_0_R = crate::BitReader;
#[doc = "Field `ABS_DONE_0` writer - "]
pub type ABS_DONE_0_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA6_SPEC, O>;
#[doc = "Field `ABS_DONE_1` reader - "]
pub type ABS_DONE_1_R = crate::BitReader;
#[doc = "Field `ABS_DONE_1` writer - "]
pub type ABS_DONE_1_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA6_SPEC, O>;
#[doc = "Field `DISABLE_JTAG` reader - "]
pub type DISABLE_JTAG_R = crate::BitReader;
#[doc = "Field `DISABLE_JTAG` writer - "]
pub type DISABLE_JTAG_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA6_SPEC, O>;
#[doc = "Field `DISABLE_DL_ENCRYPT` reader - "]
pub type DISABLE_DL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `DISABLE_DL_ENCRYPT` writer - "]
pub type DISABLE_DL_ENCRYPT_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA6_SPEC, O>;
#[doc = "Field `DISABLE_DL_DECRYPT` reader - "]
pub type DISABLE_DL_DECRYPT_R = crate::BitReader;
#[doc = "Field `DISABLE_DL_DECRYPT` writer - "]
pub type DISABLE_DL_DECRYPT_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA6_SPEC, O>;
#[doc = "Field `DISABLE_DL_CACHE` reader - "]
pub type DISABLE_DL_CACHE_R = crate::BitReader;
#[doc = "Field `DISABLE_DL_CACHE` writer - "]
pub type DISABLE_DL_CACHE_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA6_SPEC, O>;
#[doc = "Field `KEY_STATUS` reader - "]
pub type KEY_STATUS_R = crate::BitReader;
#[doc = "Field `KEY_STATUS` writer - "]
pub type KEY_STATUS_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA6_SPEC, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn coding_scheme(&self) -> CODING_SCHEME_R {
        CODING_SCHEME_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn console_debug_disable(&self) -> CONSOLE_DEBUG_DISABLE_R {
        CONSOLE_DEBUG_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn disable_sdio_host(&self) -> DISABLE_SDIO_HOST_R {
        DISABLE_SDIO_HOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn abs_done_0(&self) -> ABS_DONE_0_R {
        ABS_DONE_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn abs_done_1(&self) -> ABS_DONE_1_R {
        ABS_DONE_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn disable_jtag(&self) -> DISABLE_JTAG_R {
        DISABLE_JTAG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn disable_dl_encrypt(&self) -> DISABLE_DL_ENCRYPT_R {
        DISABLE_DL_ENCRYPT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn disable_dl_decrypt(&self) -> DISABLE_DL_DECRYPT_R {
        DISABLE_DL_DECRYPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn disable_dl_cache(&self) -> DISABLE_DL_CACHE_R {
        DISABLE_DL_CACHE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn key_status(&self) -> KEY_STATUS_R {
        KEY_STATUS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA6")
            .field(
                "coding_scheme",
                &format_args!("{}", self.coding_scheme().bits()),
            )
            .field(
                "console_debug_disable",
                &format_args!("{}", self.console_debug_disable().bit()),
            )
            .field(
                "disable_sdio_host",
                &format_args!("{}", self.disable_sdio_host().bit()),
            )
            .field("abs_done_0", &format_args!("{}", self.abs_done_0().bit()))
            .field("abs_done_1", &format_args!("{}", self.abs_done_1().bit()))
            .field(
                "disable_jtag",
                &format_args!("{}", self.disable_jtag().bit()),
            )
            .field(
                "disable_dl_encrypt",
                &format_args!("{}", self.disable_dl_encrypt().bit()),
            )
            .field(
                "disable_dl_decrypt",
                &format_args!("{}", self.disable_dl_decrypt().bit()),
            )
            .field(
                "disable_dl_cache",
                &format_args!("{}", self.disable_dl_cache().bit()),
            )
            .field("key_status", &format_args!("{}", self.key_status().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_WDATA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn coding_scheme(&mut self) -> CODING_SCHEME_W<0> {
        CODING_SCHEME_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn console_debug_disable(&mut self) -> CONSOLE_DEBUG_DISABLE_W<2> {
        CONSOLE_DEBUG_DISABLE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn disable_sdio_host(&mut self) -> DISABLE_SDIO_HOST_W<3> {
        DISABLE_SDIO_HOST_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn abs_done_0(&mut self) -> ABS_DONE_0_W<4> {
        ABS_DONE_0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn abs_done_1(&mut self) -> ABS_DONE_1_W<5> {
        ABS_DONE_1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn disable_jtag(&mut self) -> DISABLE_JTAG_W<6> {
        DISABLE_JTAG_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn disable_dl_encrypt(&mut self) -> DISABLE_DL_ENCRYPT_W<7> {
        DISABLE_DL_ENCRYPT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn disable_dl_decrypt(&mut self) -> DISABLE_DL_DECRYPT_W<8> {
        DISABLE_DL_DECRYPT_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn disable_dl_cache(&mut self) -> DISABLE_DL_CACHE_W<9> {
        DISABLE_DL_CACHE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn key_status(&mut self) -> KEY_STATUS_W<10> {
        KEY_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_wdata6](index.html) module"]
pub struct BLK0_WDATA6_SPEC;
impl crate::RegisterSpec for BLK0_WDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_wdata6::R](R) reader structure"]
impl crate::Readable for BLK0_WDATA6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_wdata6::W](W) writer structure"]
impl crate::Writable for BLK0_WDATA6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA6 to value 0"]
impl crate::Resettable for BLK0_WDATA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
