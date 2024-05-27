#[doc = "Register `BLK0_RDATA6` reader"]
pub type R = crate::R<BLK0_RDATA6_SPEC>;
#[doc = "Register `BLK0_RDATA6` writer"]
pub type W = crate::W<BLK0_RDATA6_SPEC>;
#[doc = "Field `RD_CODING_SCHEME` reader - "]
pub type RD_CODING_SCHEME_R = crate::FieldReader;
#[doc = "Field `RD_CONSOLE_DEBUG_DISABLE` reader - "]
pub type RD_CONSOLE_DEBUG_DISABLE_R = crate::BitReader;
#[doc = "Field `RD_DISABLE_SDIO_HOST` reader - "]
pub type RD_DISABLE_SDIO_HOST_R = crate::BitReader;
#[doc = "Field `RD_ABS_DONE_0` reader - "]
pub type RD_ABS_DONE_0_R = crate::BitReader;
#[doc = "Field `RD_ABS_DONE_1` reader - "]
pub type RD_ABS_DONE_1_R = crate::BitReader;
#[doc = "Field `RD_JTAG_DISABLE` reader - "]
pub type RD_JTAG_DISABLE_R = crate::BitReader;
#[doc = "Field `RD_DISABLE_DL_ENCRYPT` reader - "]
pub type RD_DISABLE_DL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `RD_DISABLE_DL_DECRYPT` reader - "]
pub type RD_DISABLE_DL_DECRYPT_R = crate::BitReader;
#[doc = "Field `RD_DISABLE_DL_CACHE` reader - "]
pub type RD_DISABLE_DL_CACHE_R = crate::BitReader;
#[doc = "Field `RD_KEY_STATUS` reader - "]
pub type RD_KEY_STATUS_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_203` reader - "]
pub type RD_RESERVE_0_203_R = crate::FieldReader<u32>;
#[doc = "Field `RD_RESERVE_0_203` writer - "]
pub type RD_RESERVE_0_203_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rd_coding_scheme(&self) -> RD_CODING_SCHEME_R {
        RD_CODING_SCHEME_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rd_console_debug_disable(&self) -> RD_CONSOLE_DEBUG_DISABLE_R {
        RD_CONSOLE_DEBUG_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rd_disable_sdio_host(&self) -> RD_DISABLE_SDIO_HOST_R {
        RD_DISABLE_SDIO_HOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rd_abs_done_0(&self) -> RD_ABS_DONE_0_R {
        RD_ABS_DONE_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rd_abs_done_1(&self) -> RD_ABS_DONE_1_R {
        RD_ABS_DONE_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_jtag_disable(&self) -> RD_JTAG_DISABLE_R {
        RD_JTAG_DISABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rd_disable_dl_encrypt(&self) -> RD_DISABLE_DL_ENCRYPT_R {
        RD_DISABLE_DL_ENCRYPT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rd_disable_dl_decrypt(&self) -> RD_DISABLE_DL_DECRYPT_R {
        RD_DISABLE_DL_DECRYPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rd_disable_dl_cache(&self) -> RD_DISABLE_DL_CACHE_R {
        RD_DISABLE_DL_CACHE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rd_key_status(&self) -> RD_KEY_STATUS_R {
        RD_KEY_STATUS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn rd_reserve_0_203(&self) -> RD_RESERVE_0_203_R {
        RD_RESERVE_0_203_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA6")
            .field("rd_coding_scheme", &self.rd_coding_scheme())
            .field("rd_console_debug_disable", &self.rd_console_debug_disable())
            .field("rd_disable_sdio_host", &self.rd_disable_sdio_host())
            .field("rd_abs_done_0", &self.rd_abs_done_0())
            .field("rd_abs_done_1", &self.rd_abs_done_1())
            .field("rd_jtag_disable", &self.rd_jtag_disable())
            .field("rd_disable_dl_encrypt", &self.rd_disable_dl_encrypt())
            .field("rd_disable_dl_decrypt", &self.rd_disable_dl_decrypt())
            .field("rd_disable_dl_cache", &self.rd_disable_dl_cache())
            .field("rd_key_status", &self.rd_key_status())
            .field("rd_reserve_0_203", &self.rd_reserve_0_203())
            .finish()
    }
}
impl W {
    #[doc = "Bits 11:31"]
    #[inline(always)]
    #[must_use]
    pub fn rd_reserve_0_203(&mut self) -> RD_RESERVE_0_203_W<BLK0_RDATA6_SPEC> {
        RD_RESERVE_0_203_W::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_RDATA6_SPEC;
impl crate::RegisterSpec for BLK0_RDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_rdata6::R`](R) reader structure"]
impl crate::Readable for BLK0_RDATA6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_rdata6::W`](W) writer structure"]
impl crate::Writable for BLK0_RDATA6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK0_RDATA6 to value 0"]
impl crate::Resettable for BLK0_RDATA6_SPEC {
    const RESET_VALUE: u32 = 0;
}
