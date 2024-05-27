#[doc = "Register `BLK0_WDATA6` reader"]
pub type R = crate::R<BLK0_WDATA6_SPEC>;
#[doc = "Register `BLK0_WDATA6` writer"]
pub type W = crate::W<BLK0_WDATA6_SPEC>;
#[doc = "Field `CODING_SCHEME` reader - "]
pub type CODING_SCHEME_R = crate::FieldReader;
#[doc = "Field `CODING_SCHEME` writer - "]
pub type CODING_SCHEME_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CONSOLE_DEBUG_DISABLE` reader - "]
pub type CONSOLE_DEBUG_DISABLE_R = crate::BitReader;
#[doc = "Field `CONSOLE_DEBUG_DISABLE` writer - "]
pub type CONSOLE_DEBUG_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_SDIO_HOST` reader - "]
pub type DISABLE_SDIO_HOST_R = crate::BitReader;
#[doc = "Field `DISABLE_SDIO_HOST` writer - "]
pub type DISABLE_SDIO_HOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABS_DONE_0` reader - "]
pub type ABS_DONE_0_R = crate::BitReader;
#[doc = "Field `ABS_DONE_0` writer - "]
pub type ABS_DONE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABS_DONE_1` reader - "]
pub type ABS_DONE_1_R = crate::BitReader;
#[doc = "Field `ABS_DONE_1` writer - "]
pub type ABS_DONE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_JTAG` reader - "]
pub type DISABLE_JTAG_R = crate::BitReader;
#[doc = "Field `DISABLE_JTAG` writer - "]
pub type DISABLE_JTAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_DL_ENCRYPT` reader - "]
pub type DISABLE_DL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `DISABLE_DL_ENCRYPT` writer - "]
pub type DISABLE_DL_ENCRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_DL_DECRYPT` reader - "]
pub type DISABLE_DL_DECRYPT_R = crate::BitReader;
#[doc = "Field `DISABLE_DL_DECRYPT` writer - "]
pub type DISABLE_DL_DECRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_DL_CACHE` reader - "]
pub type DISABLE_DL_CACHE_R = crate::BitReader;
#[doc = "Field `DISABLE_DL_CACHE` writer - "]
pub type DISABLE_DL_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_STATUS` reader - "]
pub type KEY_STATUS_R = crate::BitReader;
#[doc = "Field `KEY_STATUS` writer - "]
pub type KEY_STATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("coding_scheme", &self.coding_scheme())
            .field("console_debug_disable", &self.console_debug_disable())
            .field("disable_sdio_host", &self.disable_sdio_host())
            .field("abs_done_0", &self.abs_done_0())
            .field("abs_done_1", &self.abs_done_1())
            .field("disable_jtag", &self.disable_jtag())
            .field("disable_dl_encrypt", &self.disable_dl_encrypt())
            .field("disable_dl_decrypt", &self.disable_dl_decrypt())
            .field("disable_dl_cache", &self.disable_dl_cache())
            .field("key_status", &self.key_status())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn coding_scheme(&mut self) -> CODING_SCHEME_W<BLK0_WDATA6_SPEC> {
        CODING_SCHEME_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn console_debug_disable(&mut self) -> CONSOLE_DEBUG_DISABLE_W<BLK0_WDATA6_SPEC> {
        CONSOLE_DEBUG_DISABLE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn disable_sdio_host(&mut self) -> DISABLE_SDIO_HOST_W<BLK0_WDATA6_SPEC> {
        DISABLE_SDIO_HOST_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn abs_done_0(&mut self) -> ABS_DONE_0_W<BLK0_WDATA6_SPEC> {
        ABS_DONE_0_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn abs_done_1(&mut self) -> ABS_DONE_1_W<BLK0_WDATA6_SPEC> {
        ABS_DONE_1_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn disable_jtag(&mut self) -> DISABLE_JTAG_W<BLK0_WDATA6_SPEC> {
        DISABLE_JTAG_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn disable_dl_encrypt(&mut self) -> DISABLE_DL_ENCRYPT_W<BLK0_WDATA6_SPEC> {
        DISABLE_DL_ENCRYPT_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn disable_dl_decrypt(&mut self) -> DISABLE_DL_DECRYPT_W<BLK0_WDATA6_SPEC> {
        DISABLE_DL_DECRYPT_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn disable_dl_cache(&mut self) -> DISABLE_DL_CACHE_W<BLK0_WDATA6_SPEC> {
        DISABLE_DL_CACHE_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn key_status(&mut self) -> KEY_STATUS_W<BLK0_WDATA6_SPEC> {
        KEY_STATUS_W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_WDATA6_SPEC;
impl crate::RegisterSpec for BLK0_WDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_wdata6::R`](R) reader structure"]
impl crate::Readable for BLK0_WDATA6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_wdata6::W`](W) writer structure"]
impl crate::Writable for BLK0_WDATA6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA6 to value 0"]
impl crate::Resettable for BLK0_WDATA6_SPEC {
    const RESET_VALUE: u32 = 0;
}
