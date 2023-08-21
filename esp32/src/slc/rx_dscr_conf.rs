#[doc = "Register `RX_DSCR_CONF` reader"]
pub type R = crate::R<RX_DSCR_CONF_SPEC>;
#[doc = "Register `RX_DSCR_CONF` writer"]
pub type W = crate::W<RX_DSCR_CONF_SPEC>;
#[doc = "Field `SLC0_TOKEN_NO_REPLACE` reader - "]
pub type SLC0_TOKEN_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC0_TOKEN_NO_REPLACE` writer - "]
pub type SLC0_TOKEN_NO_REPLACE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_INFOR_NO_REPLACE` reader - "]
pub type SLC0_INFOR_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC0_INFOR_NO_REPLACE` writer - "]
pub type SLC0_INFOR_NO_REPLACE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RX_FILL_MODE` reader - "]
pub type SLC0_RX_FILL_MODE_R = crate::BitReader;
#[doc = "Field `SLC0_RX_FILL_MODE` writer - "]
pub type SLC0_RX_FILL_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RX_EOF_MODE` reader - "]
pub type SLC0_RX_EOF_MODE_R = crate::BitReader;
#[doc = "Field `SLC0_RX_EOF_MODE` writer - "]
pub type SLC0_RX_EOF_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RX_FILL_EN` reader - "]
pub type SLC0_RX_FILL_EN_R = crate::BitReader;
#[doc = "Field `SLC0_RX_FILL_EN` writer - "]
pub type SLC0_RX_FILL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RD_RETRY_THRESHOLD` reader - "]
pub type SLC0_RD_RETRY_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `SLC0_RD_RETRY_THRESHOLD` writer - "]
pub type SLC0_RD_RETRY_THRESHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `SLC1_TOKEN_NO_REPLACE` reader - "]
pub type SLC1_TOKEN_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN_NO_REPLACE` writer - "]
pub type SLC1_TOKEN_NO_REPLACE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC1_INFOR_NO_REPLACE` reader - "]
pub type SLC1_INFOR_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC1_INFOR_NO_REPLACE` writer - "]
pub type SLC1_INFOR_NO_REPLACE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC1_RX_FILL_MODE` reader - "]
pub type SLC1_RX_FILL_MODE_R = crate::BitReader;
#[doc = "Field `SLC1_RX_FILL_MODE` writer - "]
pub type SLC1_RX_FILL_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC1_RX_EOF_MODE` reader - "]
pub type SLC1_RX_EOF_MODE_R = crate::BitReader;
#[doc = "Field `SLC1_RX_EOF_MODE` writer - "]
pub type SLC1_RX_EOF_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC1_RX_FILL_EN` reader - "]
pub type SLC1_RX_FILL_EN_R = crate::BitReader;
#[doc = "Field `SLC1_RX_FILL_EN` writer - "]
pub type SLC1_RX_FILL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC1_RD_RETRY_THRESHOLD` reader - "]
pub type SLC1_RD_RETRY_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `SLC1_RD_RETRY_THRESHOLD` writer - "]
pub type SLC1_RD_RETRY_THRESHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
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
            .field(
                "slc0_token_no_replace",
                &format_args!("{}", self.slc0_token_no_replace().bit()),
            )
            .field(
                "slc0_infor_no_replace",
                &format_args!("{}", self.slc0_infor_no_replace().bit()),
            )
            .field(
                "slc0_rx_fill_mode",
                &format_args!("{}", self.slc0_rx_fill_mode().bit()),
            )
            .field(
                "slc0_rx_eof_mode",
                &format_args!("{}", self.slc0_rx_eof_mode().bit()),
            )
            .field(
                "slc0_rx_fill_en",
                &format_args!("{}", self.slc0_rx_fill_en().bit()),
            )
            .field(
                "slc0_rd_retry_threshold",
                &format_args!("{}", self.slc0_rd_retry_threshold().bits()),
            )
            .field(
                "slc1_token_no_replace",
                &format_args!("{}", self.slc1_token_no_replace().bit()),
            )
            .field(
                "slc1_infor_no_replace",
                &format_args!("{}", self.slc1_infor_no_replace().bit()),
            )
            .field(
                "slc1_rx_fill_mode",
                &format_args!("{}", self.slc1_rx_fill_mode().bit()),
            )
            .field(
                "slc1_rx_eof_mode",
                &format_args!("{}", self.slc1_rx_eof_mode().bit()),
            )
            .field(
                "slc1_rx_fill_en",
                &format_args!("{}", self.slc1_rx_fill_en().bit()),
            )
            .field(
                "slc1_rd_retry_threshold",
                &format_args!("{}", self.slc1_rd_retry_threshold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_DSCR_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token_no_replace(&mut self) -> SLC0_TOKEN_NO_REPLACE_W<RX_DSCR_CONF_SPEC, 0> {
        SLC0_TOKEN_NO_REPLACE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_infor_no_replace(&mut self) -> SLC0_INFOR_NO_REPLACE_W<RX_DSCR_CONF_SPEC, 1> {
        SLC0_INFOR_NO_REPLACE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_fill_mode(&mut self) -> SLC0_RX_FILL_MODE_W<RX_DSCR_CONF_SPEC, 2> {
        SLC0_RX_FILL_MODE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_eof_mode(&mut self) -> SLC0_RX_EOF_MODE_W<RX_DSCR_CONF_SPEC, 3> {
        SLC0_RX_EOF_MODE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_fill_en(&mut self) -> SLC0_RX_FILL_EN_W<RX_DSCR_CONF_SPEC, 4> {
        SLC0_RX_FILL_EN_W::new(self)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rd_retry_threshold(&mut self) -> SLC0_RD_RETRY_THRESHOLD_W<RX_DSCR_CONF_SPEC, 5> {
        SLC0_RD_RETRY_THRESHOLD_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token_no_replace(&mut self) -> SLC1_TOKEN_NO_REPLACE_W<RX_DSCR_CONF_SPEC, 16> {
        SLC1_TOKEN_NO_REPLACE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_infor_no_replace(&mut self) -> SLC1_INFOR_NO_REPLACE_W<RX_DSCR_CONF_SPEC, 17> {
        SLC1_INFOR_NO_REPLACE_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_fill_mode(&mut self) -> SLC1_RX_FILL_MODE_W<RX_DSCR_CONF_SPEC, 18> {
        SLC1_RX_FILL_MODE_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_eof_mode(&mut self) -> SLC1_RX_EOF_MODE_W<RX_DSCR_CONF_SPEC, 19> {
        SLC1_RX_EOF_MODE_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_fill_en(&mut self) -> SLC1_RX_FILL_EN_W<RX_DSCR_CONF_SPEC, 20> {
        SLC1_RX_FILL_EN_W::new(self)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rd_retry_threshold(&mut self) -> SLC1_RD_RETRY_THRESHOLD_W<RX_DSCR_CONF_SPEC, 21> {
        SLC1_RD_RETRY_THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_dscr_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_dscr_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DSCR_CONF_SPEC;
impl crate::RegisterSpec for RX_DSCR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_dscr_conf::R`](R) reader structure"]
impl crate::Readable for RX_DSCR_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_dscr_conf::W`](W) writer structure"]
impl crate::Writable for RX_DSCR_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_DSCR_CONF to value 0x101b_101a"]
impl crate::Resettable for RX_DSCR_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x101b_101a;
}
