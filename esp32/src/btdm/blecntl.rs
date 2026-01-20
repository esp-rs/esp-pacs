#[doc = "Register `BLECNTL` reader"]
pub type R = crate::R<BLECNTL_SPEC>;
#[doc = "Register `BLECNTL` writer"]
pub type W = crate::W<BLECNTL_SPEC>;
#[doc = "Field `SYNCERR` reader - Max number of bit errors allowed to recognize a syncword"]
pub type SYNCERR_R = crate::FieldReader;
#[doc = "Field `SYNCERR` writer - Max number of bit errors allowed to recognize a syncword"]
pub type SYNCERR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXWINSZDEF` reader - Default RX window size in μs"]
pub type RXWINSZDEF_R = crate::FieldReader;
#[doc = "Field `RXWINSZDEF` writer - Default RX window size in μs"]
pub type RXWINSZDEF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RWBLE_EN` reader - RW-BLE Core Exchange table pre-fetch mechanism enable"]
pub type RWBLE_EN_R = crate::BitReader;
#[doc = "Field `RWBLE_EN` writer - RW-BLE Core Exchange table pre-fetch mechanism enable"]
pub type RWBLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADVERTFILT_EN` reader - Filter to only correctly received packets"]
pub type ADVERTFILT_EN_R = crate::BitReader;
#[doc = "Field `ADVERTFILT_EN` writer - Filter to only correctly received packets"]
pub type ADVERTFILT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOP_REMAP_DSB` reader - Hop remapping algorithm disable"]
pub type HOP_REMAP_DSB_R = crate::BitReader;
#[doc = "Field `HOP_REMAP_DSB` writer - Hop remapping algorithm disable"]
pub type HOP_REMAP_DSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_DSB` reader - If enabled, CRC is removed from stream, when disabled, CRC is appended for user check"]
pub type CRC_DSB_R = crate::BitReader;
#[doc = "Field `CRC_DSB` writer - If enabled, CRC is removed from stream, when disabled, CRC is appended for user check"]
pub type CRC_DSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WHIT_DSB` reader - Whitening disable"]
pub type WHIT_DSB_R = crate::BitReader;
#[doc = "Field `WHIT_DSB` writer - Whitening disable"]
pub type WHIT_DSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPT_DSB` reader - Encryption and decryption disable"]
pub type CRYPT_DSB_R = crate::BitReader;
#[doc = "Field `CRYPT_DSB` writer - Encryption and decryption disable"]
pub type CRYPT_DSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NESN_DSB` reader - Acknowledge scheme disable"]
pub type NESN_DSB_R = crate::BitReader;
#[doc = "Field `NESN_DSB` writer - Acknowledge scheme disable"]
pub type NESN_DSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SN_DSB` reader - Sequence number disable"]
pub type SN_DSB_R = crate::BitReader;
#[doc = "Field `SN_DSB` writer - Sequence number disable"]
pub type SN_DSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MD_DSB` reader - More data bit disable (0=Normal operation, 1=Allow a one Tx/Rx exchange whatever the MD bits are)"]
pub type MD_DSB_R = crate::BitReader;
#[doc = "Field `MD_DSB` writer - More data bit disable (0=Normal operation, 1=Allow a one Tx/Rx exchange whatever the MD bits are)"]
pub type MD_DSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_ABORT` reader - Abort the current scan window"]
pub type SCAN_ABORT_R = crate::BitReader;
#[doc = "Field `SCAN_ABORT` writer - Abort the current scan window"]
pub type SCAN_ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADVERT_ABORT` reader - Abort the current advertising event"]
pub type ADVERT_ABORT_R = crate::BitReader;
#[doc = "Field `ADVERT_ABORT` writer - Abort the current advertising event"]
pub type ADVERT_ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFTEST_ABORT` reader - Abort the current RF test"]
pub type RFTEST_ABORT_R = crate::BitReader;
#[doc = "Field `RFTEST_ABORT` writer - Abort the current RF test"]
pub type RFTEST_ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINT_REQ` reader - Force a software interrupt"]
pub type SWINT_REQ_R = crate::BitReader;
#[doc = "Field `SWINT_REQ` writer - Force a software interrupt"]
pub type SWINT_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SOFT_RST` reader - Reset of the complete register block"]
pub type REG_SOFT_RST_R = crate::BitReader;
#[doc = "Field `REG_SOFT_RST` writer - Reset of the complete register block"]
pub type REG_SOFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_TGSOFT_RST` reader - Master timing generator reset"]
pub type MASTER_TGSOFT_RST_R = crate::BitReader;
#[doc = "Field `MASTER_TGSOFT_RST` writer - Master timing generator reset"]
pub type MASTER_TGSOFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_SOFT_RST` reader - Reset the complete BLE Core except registers and timing generator (Master state machine reset)"]
pub type MASTER_SOFT_RST_R = crate::BitReader;
#[doc = "Field `MASTER_SOFT_RST` writer - Reset the complete BLE Core except registers and timing generator (Master state machine reset)"]
pub type MASTER_SOFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Max number of bit errors allowed to recognize a syncword"]
    #[inline(always)]
    pub fn syncerr(&self) -> SYNCERR_R {
        SYNCERR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Default RX window size in μs"]
    #[inline(always)]
    pub fn rxwinszdef(&self) -> RXWINSZDEF_R {
        RXWINSZDEF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - RW-BLE Core Exchange table pre-fetch mechanism enable"]
    #[inline(always)]
    pub fn rwble_en(&self) -> RWBLE_EN_R {
        RWBLE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter to only correctly received packets"]
    #[inline(always)]
    pub fn advertfilt_en(&self) -> ADVERTFILT_EN_R {
        ADVERTFILT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Hop remapping algorithm disable"]
    #[inline(always)]
    pub fn hop_remap_dsb(&self) -> HOP_REMAP_DSB_R {
        HOP_REMAP_DSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If enabled, CRC is removed from stream, when disabled, CRC is appended for user check"]
    #[inline(always)]
    pub fn crc_dsb(&self) -> CRC_DSB_R {
        CRC_DSB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Whitening disable"]
    #[inline(always)]
    pub fn whit_dsb(&self) -> WHIT_DSB_R {
        WHIT_DSB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Encryption and decryption disable"]
    #[inline(always)]
    pub fn crypt_dsb(&self) -> CRYPT_DSB_R {
        CRYPT_DSB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Acknowledge scheme disable"]
    #[inline(always)]
    pub fn nesn_dsb(&self) -> NESN_DSB_R {
        NESN_DSB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Sequence number disable"]
    #[inline(always)]
    pub fn sn_dsb(&self) -> SN_DSB_R {
        SN_DSB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - More data bit disable (0=Normal operation, 1=Allow a one Tx/Rx exchange whatever the MD bits are)"]
    #[inline(always)]
    pub fn md_dsb(&self) -> MD_DSB_R {
        MD_DSB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Abort the current scan window"]
    #[inline(always)]
    pub fn scan_abort(&self) -> SCAN_ABORT_R {
        SCAN_ABORT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Abort the current advertising event"]
    #[inline(always)]
    pub fn advert_abort(&self) -> ADVERT_ABORT_R {
        ADVERT_ABORT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Abort the current RF test"]
    #[inline(always)]
    pub fn rftest_abort(&self) -> RFTEST_ABORT_R {
        RFTEST_ABORT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Force a software interrupt"]
    #[inline(always)]
    pub fn swint_req(&self) -> SWINT_REQ_R {
        SWINT_REQ_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reset of the complete register block"]
    #[inline(always)]
    pub fn reg_soft_rst(&self) -> REG_SOFT_RST_R {
        REG_SOFT_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Master timing generator reset"]
    #[inline(always)]
    pub fn master_tgsoft_rst(&self) -> MASTER_TGSOFT_RST_R {
        MASTER_TGSOFT_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reset the complete BLE Core except registers and timing generator (Master state machine reset)"]
    #[inline(always)]
    pub fn master_soft_rst(&self) -> MASTER_SOFT_RST_R {
        MASTER_SOFT_RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLECNTL")
            .field("syncerr", &self.syncerr())
            .field("rxwinszdef", &self.rxwinszdef())
            .field("rwble_en", &self.rwble_en())
            .field("advertfilt_en", &self.advertfilt_en())
            .field("hop_remap_dsb", &self.hop_remap_dsb())
            .field("crc_dsb", &self.crc_dsb())
            .field("whit_dsb", &self.whit_dsb())
            .field("crypt_dsb", &self.crypt_dsb())
            .field("nesn_dsb", &self.nesn_dsb())
            .field("sn_dsb", &self.sn_dsb())
            .field("md_dsb", &self.md_dsb())
            .field("scan_abort", &self.scan_abort())
            .field("advert_abort", &self.advert_abort())
            .field("rftest_abort", &self.rftest_abort())
            .field("swint_req", &self.swint_req())
            .field("reg_soft_rst", &self.reg_soft_rst())
            .field("master_tgsoft_rst", &self.master_tgsoft_rst())
            .field("master_soft_rst", &self.master_soft_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Max number of bit errors allowed to recognize a syncword"]
    #[inline(always)]
    pub fn syncerr(&mut self) -> SYNCERR_W<'_, BLECNTL_SPEC> {
        SYNCERR_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Default RX window size in μs"]
    #[inline(always)]
    pub fn rxwinszdef(&mut self) -> RXWINSZDEF_W<'_, BLECNTL_SPEC> {
        RXWINSZDEF_W::new(self, 4)
    }
    #[doc = "Bit 8 - RW-BLE Core Exchange table pre-fetch mechanism enable"]
    #[inline(always)]
    pub fn rwble_en(&mut self) -> RWBLE_EN_W<'_, BLECNTL_SPEC> {
        RWBLE_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter to only correctly received packets"]
    #[inline(always)]
    pub fn advertfilt_en(&mut self) -> ADVERTFILT_EN_W<'_, BLECNTL_SPEC> {
        ADVERTFILT_EN_W::new(self, 9)
    }
    #[doc = "Bit 16 - Hop remapping algorithm disable"]
    #[inline(always)]
    pub fn hop_remap_dsb(&mut self) -> HOP_REMAP_DSB_W<'_, BLECNTL_SPEC> {
        HOP_REMAP_DSB_W::new(self, 16)
    }
    #[doc = "Bit 17 - If enabled, CRC is removed from stream, when disabled, CRC is appended for user check"]
    #[inline(always)]
    pub fn crc_dsb(&mut self) -> CRC_DSB_W<'_, BLECNTL_SPEC> {
        CRC_DSB_W::new(self, 17)
    }
    #[doc = "Bit 18 - Whitening disable"]
    #[inline(always)]
    pub fn whit_dsb(&mut self) -> WHIT_DSB_W<'_, BLECNTL_SPEC> {
        WHIT_DSB_W::new(self, 18)
    }
    #[doc = "Bit 19 - Encryption and decryption disable"]
    #[inline(always)]
    pub fn crypt_dsb(&mut self) -> CRYPT_DSB_W<'_, BLECNTL_SPEC> {
        CRYPT_DSB_W::new(self, 19)
    }
    #[doc = "Bit 20 - Acknowledge scheme disable"]
    #[inline(always)]
    pub fn nesn_dsb(&mut self) -> NESN_DSB_W<'_, BLECNTL_SPEC> {
        NESN_DSB_W::new(self, 20)
    }
    #[doc = "Bit 21 - Sequence number disable"]
    #[inline(always)]
    pub fn sn_dsb(&mut self) -> SN_DSB_W<'_, BLECNTL_SPEC> {
        SN_DSB_W::new(self, 21)
    }
    #[doc = "Bit 22 - More data bit disable (0=Normal operation, 1=Allow a one Tx/Rx exchange whatever the MD bits are)"]
    #[inline(always)]
    pub fn md_dsb(&mut self) -> MD_DSB_W<'_, BLECNTL_SPEC> {
        MD_DSB_W::new(self, 22)
    }
    #[doc = "Bit 24 - Abort the current scan window"]
    #[inline(always)]
    pub fn scan_abort(&mut self) -> SCAN_ABORT_W<'_, BLECNTL_SPEC> {
        SCAN_ABORT_W::new(self, 24)
    }
    #[doc = "Bit 25 - Abort the current advertising event"]
    #[inline(always)]
    pub fn advert_abort(&mut self) -> ADVERT_ABORT_W<'_, BLECNTL_SPEC> {
        ADVERT_ABORT_W::new(self, 25)
    }
    #[doc = "Bit 26 - Abort the current RF test"]
    #[inline(always)]
    pub fn rftest_abort(&mut self) -> RFTEST_ABORT_W<'_, BLECNTL_SPEC> {
        RFTEST_ABORT_W::new(self, 26)
    }
    #[doc = "Bit 28 - Force a software interrupt"]
    #[inline(always)]
    pub fn swint_req(&mut self) -> SWINT_REQ_W<'_, BLECNTL_SPEC> {
        SWINT_REQ_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reset of the complete register block"]
    #[inline(always)]
    pub fn reg_soft_rst(&mut self) -> REG_SOFT_RST_W<'_, BLECNTL_SPEC> {
        REG_SOFT_RST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Master timing generator reset"]
    #[inline(always)]
    pub fn master_tgsoft_rst(&mut self) -> MASTER_TGSOFT_RST_W<'_, BLECNTL_SPEC> {
        MASTER_TGSOFT_RST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Reset the complete BLE Core except registers and timing generator (Master state machine reset)"]
    #[inline(always)]
    pub fn master_soft_rst(&mut self) -> MASTER_SOFT_RST_W<'_, BLECNTL_SPEC> {
        MASTER_SOFT_RST_W::new(self, 31)
    }
}
#[doc = "BLE control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blecntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blecntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLECNTL_SPEC;
impl crate::RegisterSpec for BLECNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blecntl::R`](R) reader structure"]
impl crate::Readable for BLECNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blecntl::W`](W) writer structure"]
impl crate::Writable for BLECNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLECNTL to value 0"]
impl crate::Resettable for BLECNTL_SPEC {}
