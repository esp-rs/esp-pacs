#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `TX_PCM_CONF` reader - Compress/Decompress module configuration bits. 0: decompress transmitted data 1:compress transmitted data"]
pub type TX_PCM_CONF_R = crate::FieldReader;
#[doc = "Field `TX_PCM_CONF` writer - Compress/Decompress module configuration bits. 0: decompress transmitted data 1:compress transmitted data"]
pub type TX_PCM_CONF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TX_PCM_BYPASS` reader - Set this bit to bypass Compress/Decompress module for transmitted data."]
pub type TX_PCM_BYPASS_R = crate::BitReader;
#[doc = "Field `TX_PCM_BYPASS` writer - Set this bit to bypass Compress/Decompress module for transmitted data."]
pub type TX_PCM_BYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_PCM_CONF` reader - Compress/Decompress module configuration bits. 0: decompress received data 1:compress received data"]
pub type RX_PCM_CONF_R = crate::FieldReader;
#[doc = "Field `RX_PCM_CONF` writer - Compress/Decompress module configuration bits. 0: decompress received data 1:compress received data"]
pub type RX_PCM_CONF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RX_PCM_BYPASS` reader - Set this bit to bypass Compress/Decompress module for received data."]
pub type RX_PCM_BYPASS_R = crate::BitReader;
#[doc = "Field `RX_PCM_BYPASS` writer - Set this bit to bypass Compress/Decompress module for received data."]
pub type RX_PCM_BYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_STOP_EN` reader - Set this bit to stop the output of BCK signal and WS signal when TX FIFO is empty."]
pub type TX_STOP_EN_R = crate::BitReader;
#[doc = "Field `TX_STOP_EN` writer - Set this bit to stop the output of BCK signal and WS signal when TX FIFO is empty."]
pub type TX_STOP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_ZEROS_RM_EN` reader - Reserved."]
pub type TX_ZEROS_RM_EN_R = crate::BitReader;
#[doc = "Field `TX_ZEROS_RM_EN` writer - Reserved."]
pub type TX_ZEROS_RM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Compress/Decompress module configuration bits. 0: decompress transmitted data 1:compress transmitted data"]
    #[inline(always)]
    pub fn tx_pcm_conf(&self) -> TX_PCM_CONF_R {
        TX_PCM_CONF_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Set this bit to bypass Compress/Decompress module for transmitted data."]
    #[inline(always)]
    pub fn tx_pcm_bypass(&self) -> TX_PCM_BYPASS_R {
        TX_PCM_BYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Compress/Decompress module configuration bits. 0: decompress received data 1:compress received data"]
    #[inline(always)]
    pub fn rx_pcm_conf(&self) -> RX_PCM_CONF_R {
        RX_PCM_CONF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Set this bit to bypass Compress/Decompress module for received data."]
    #[inline(always)]
    pub fn rx_pcm_bypass(&self) -> RX_PCM_BYPASS_R {
        RX_PCM_BYPASS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to stop the output of BCK signal and WS signal when TX FIFO is empty."]
    #[inline(always)]
    pub fn tx_stop_en(&self) -> TX_STOP_EN_R {
        TX_STOP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved."]
    #[inline(always)]
    pub fn tx_zeros_rm_en(&self) -> TX_ZEROS_RM_EN_R {
        TX_ZEROS_RM_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field(
                "tx_pcm_conf",
                &format_args!("{}", self.tx_pcm_conf().bits()),
            )
            .field(
                "tx_pcm_bypass",
                &format_args!("{}", self.tx_pcm_bypass().bit()),
            )
            .field(
                "rx_pcm_conf",
                &format_args!("{}", self.rx_pcm_conf().bits()),
            )
            .field(
                "rx_pcm_bypass",
                &format_args!("{}", self.rx_pcm_bypass().bit()),
            )
            .field("tx_stop_en", &format_args!("{}", self.tx_stop_en().bit()))
            .field(
                "tx_zeros_rm_en",
                &format_args!("{}", self.tx_zeros_rm_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Compress/Decompress module configuration bits. 0: decompress transmitted data 1:compress transmitted data"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pcm_conf(&mut self) -> TX_PCM_CONF_W<CONF1_SPEC, 0> {
        TX_PCM_CONF_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to bypass Compress/Decompress module for transmitted data."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pcm_bypass(&mut self) -> TX_PCM_BYPASS_W<CONF1_SPEC, 3> {
        TX_PCM_BYPASS_W::new(self)
    }
    #[doc = "Bits 4:6 - Compress/Decompress module configuration bits. 0: decompress received data 1:compress received data"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pcm_conf(&mut self) -> RX_PCM_CONF_W<CONF1_SPEC, 4> {
        RX_PCM_CONF_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to bypass Compress/Decompress module for received data."]
    #[inline(always)]
    #[must_use]
    pub fn rx_pcm_bypass(&mut self) -> RX_PCM_BYPASS_W<CONF1_SPEC, 7> {
        RX_PCM_BYPASS_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to stop the output of BCK signal and WS signal when TX FIFO is empty."]
    #[inline(always)]
    #[must_use]
    pub fn tx_stop_en(&mut self) -> TX_STOP_EN_W<CONF1_SPEC, 8> {
        TX_STOP_EN_W::new(self)
    }
    #[doc = "Bit 9 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tx_zeros_rm_en(&mut self) -> TX_ZEROS_RM_EN_W<CONF1_SPEC, 9> {
        TX_ZEROS_RM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0x89"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x89;
}
