#[doc = "Register `RX_DSCR_CONF` reader"]
pub type R = crate::R<RX_DSCR_CONF_SPEC>;
#[doc = "Register `RX_DSCR_CONF` writer"]
pub type W = crate::W<RX_DSCR_CONF_SPEC>;
#[doc = "Field `SLC0_TOKEN_NO_REPLACE` reader - "]
pub type SLC0_TOKEN_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC0_TOKEN_NO_REPLACE` writer - "]
pub type SLC0_TOKEN_NO_REPLACE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_INFOR_NO_REPLACE` reader - "]
pub type SLC0_INFOR_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC0_INFOR_NO_REPLACE` writer - "]
pub type SLC0_INFOR_NO_REPLACE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_FILL_MODE` reader - "]
pub type SLC0_RX_FILL_MODE_R = crate::BitReader;
#[doc = "Field `SLC0_RX_FILL_MODE` writer - "]
pub type SLC0_RX_FILL_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_EOF_MODE` reader - "]
pub type SLC0_RX_EOF_MODE_R = crate::BitReader;
#[doc = "Field `SLC0_RX_EOF_MODE` writer - "]
pub type SLC0_RX_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_FILL_EN` reader - "]
pub type SLC0_RX_FILL_EN_R = crate::BitReader;
#[doc = "Field `SLC0_RX_FILL_EN` writer - "]
pub type SLC0_RX_FILL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RD_RETRY_THRESHOLD` reader - "]
pub type SLC0_RD_RETRY_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `SLC0_RD_RETRY_THRESHOLD` writer - "]
pub type SLC0_RD_RETRY_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SLC1_TOKEN_NO_REPLACE` reader - "]
pub type SLC1_TOKEN_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN_NO_REPLACE` writer - "]
pub type SLC1_TOKEN_NO_REPLACE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_INFOR_NO_REPLACE` reader - "]
pub type SLC1_INFOR_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC1_INFOR_NO_REPLACE` writer - "]
pub type SLC1_INFOR_NO_REPLACE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RX_FILL_MODE` reader - "]
pub type SLC1_RX_FILL_MODE_R = crate::BitReader;
#[doc = "Field `SLC1_RX_FILL_MODE` writer - "]
pub type SLC1_RX_FILL_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RX_EOF_MODE` reader - "]
pub type SLC1_RX_EOF_MODE_R = crate::BitReader;
#[doc = "Field `SLC1_RX_EOF_MODE` writer - "]
pub type SLC1_RX_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RX_FILL_EN` reader - "]
pub type SLC1_RX_FILL_EN_R = crate::BitReader;
#[doc = "Field `SLC1_RX_FILL_EN` writer - "]
pub type SLC1_RX_FILL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RD_RETRY_THRESHOLD` reader - "]
pub type SLC1_RD_RETRY_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `SLC1_RD_RETRY_THRESHOLD` writer - "]
pub type SLC1_RD_RETRY_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_token_no_replace(&self) -> SLC0_TOKEN_NO_REPLACE_R {
        SLC0_TOKEN_NO_REPLACE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_infor_no_replace(&self) -> SLC0_INFOR_NO_REPLACE_R {
        SLC0_INFOR_NO_REPLACE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_fill_mode(&self) -> SLC0_RX_FILL_MODE_R {
        SLC0_RX_FILL_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc0_rx_eof_mode(&self) -> SLC0_RX_EOF_MODE_R {
        SLC0_RX_EOF_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_rx_fill_en(&self) -> SLC0_RX_FILL_EN_R {
        SLC0_RX_FILL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn slc0_rd_retry_threshold(&self) -> SLC0_RD_RETRY_THRESHOLD_R {
        SLC0_RD_RETRY_THRESHOLD_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_token_no_replace(&self) -> SLC1_TOKEN_NO_REPLACE_R {
        SLC1_TOKEN_NO_REPLACE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_infor_no_replace(&self) -> SLC1_INFOR_NO_REPLACE_R {
        SLC1_INFOR_NO_REPLACE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_fill_mode(&self) -> SLC1_RX_FILL_MODE_R {
        SLC1_RX_FILL_MODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_rx_eof_mode(&self) -> SLC1_RX_EOF_MODE_R {
        SLC1_RX_EOF_MODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_fill_en(&self) -> SLC1_RX_FILL_EN_R {
        SLC1_RX_FILL_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn slc1_rd_retry_threshold(&self) -> SLC1_RD_RETRY_THRESHOLD_R {
        SLC1_RD_RETRY_THRESHOLD_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_DSCR_CONF")
            .field("slc0_token_no_replace", &self.slc0_token_no_replace())
            .field("slc0_infor_no_replace", &self.slc0_infor_no_replace())
            .field("slc0_rx_fill_mode", &self.slc0_rx_fill_mode())
            .field("slc0_rx_eof_mode", &self.slc0_rx_eof_mode())
            .field("slc0_rx_fill_en", &self.slc0_rx_fill_en())
            .field("slc0_rd_retry_threshold", &self.slc0_rd_retry_threshold())
            .field("slc1_token_no_replace", &self.slc1_token_no_replace())
            .field("slc1_infor_no_replace", &self.slc1_infor_no_replace())
            .field("slc1_rx_fill_mode", &self.slc1_rx_fill_mode())
            .field("slc1_rx_eof_mode", &self.slc1_rx_eof_mode())
            .field("slc1_rx_fill_en", &self.slc1_rx_fill_en())
            .field("slc1_rd_retry_threshold", &self.slc1_rd_retry_threshold())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_token_no_replace(&mut self) -> SLC0_TOKEN_NO_REPLACE_W<RX_DSCR_CONF_SPEC> {
        SLC0_TOKEN_NO_REPLACE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_infor_no_replace(&mut self) -> SLC0_INFOR_NO_REPLACE_W<RX_DSCR_CONF_SPEC> {
        SLC0_INFOR_NO_REPLACE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_fill_mode(&mut self) -> SLC0_RX_FILL_MODE_W<RX_DSCR_CONF_SPEC> {
        SLC0_RX_FILL_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc0_rx_eof_mode(&mut self) -> SLC0_RX_EOF_MODE_W<RX_DSCR_CONF_SPEC> {
        SLC0_RX_EOF_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_rx_fill_en(&mut self) -> SLC0_RX_FILL_EN_W<RX_DSCR_CONF_SPEC> {
        SLC0_RX_FILL_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn slc0_rd_retry_threshold(&mut self) -> SLC0_RD_RETRY_THRESHOLD_W<RX_DSCR_CONF_SPEC> {
        SLC0_RD_RETRY_THRESHOLD_W::new(self, 5)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_token_no_replace(&mut self) -> SLC1_TOKEN_NO_REPLACE_W<RX_DSCR_CONF_SPEC> {
        SLC1_TOKEN_NO_REPLACE_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_infor_no_replace(&mut self) -> SLC1_INFOR_NO_REPLACE_W<RX_DSCR_CONF_SPEC> {
        SLC1_INFOR_NO_REPLACE_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_fill_mode(&mut self) -> SLC1_RX_FILL_MODE_W<RX_DSCR_CONF_SPEC> {
        SLC1_RX_FILL_MODE_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_rx_eof_mode(&mut self) -> SLC1_RX_EOF_MODE_W<RX_DSCR_CONF_SPEC> {
        SLC1_RX_EOF_MODE_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_fill_en(&mut self) -> SLC1_RX_FILL_EN_W<RX_DSCR_CONF_SPEC> {
        SLC1_RX_FILL_EN_W::new(self, 20)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn slc1_rd_retry_threshold(&mut self) -> SLC1_RD_RETRY_THRESHOLD_W<RX_DSCR_CONF_SPEC> {
        SLC1_RD_RETRY_THRESHOLD_W::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_dscr_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_dscr_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DSCR_CONF_SPEC;
impl crate::RegisterSpec for RX_DSCR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_dscr_conf::R`](R) reader structure"]
impl crate::Readable for RX_DSCR_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_dscr_conf::W`](W) writer structure"]
impl crate::Writable for RX_DSCR_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_DSCR_CONF to value 0x101b_101a"]
impl crate::Resettable for RX_DSCR_CONF_SPEC {
    const RESET_VALUE: u32 = 0x101b_101a;
}
