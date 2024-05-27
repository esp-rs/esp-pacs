#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `TX_PCM_CONF` reader - "]
pub type TX_PCM_CONF_R = crate::FieldReader;
#[doc = "Field `TX_PCM_CONF` writer - "]
pub type TX_PCM_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TX_PCM_BYPASS` reader - "]
pub type TX_PCM_BYPASS_R = crate::BitReader;
#[doc = "Field `TX_PCM_BYPASS` writer - "]
pub type TX_PCM_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PCM_CONF` reader - "]
pub type RX_PCM_CONF_R = crate::FieldReader;
#[doc = "Field `RX_PCM_CONF` writer - "]
pub type RX_PCM_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_PCM_BYPASS` reader - "]
pub type RX_PCM_BYPASS_R = crate::BitReader;
#[doc = "Field `RX_PCM_BYPASS` writer - "]
pub type RX_PCM_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STOP_EN` reader - "]
pub type TX_STOP_EN_R = crate::BitReader;
#[doc = "Field `TX_STOP_EN` writer - "]
pub type TX_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ZEROS_RM_EN` reader - "]
pub type TX_ZEROS_RM_EN_R = crate::BitReader;
#[doc = "Field `TX_ZEROS_RM_EN` writer - "]
pub type TX_ZEROS_RM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tx_pcm_conf(&self) -> TX_PCM_CONF_R {
        TX_PCM_CONF_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_pcm_bypass(&self) -> TX_PCM_BYPASS_R {
        TX_PCM_BYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rx_pcm_conf(&self) -> RX_PCM_CONF_R {
        RX_PCM_CONF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_pcm_bypass(&self) -> RX_PCM_BYPASS_R {
        RX_PCM_BYPASS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_stop_en(&self) -> TX_STOP_EN_R {
        TX_STOP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tx_zeros_rm_en(&self) -> TX_ZEROS_RM_EN_R {
        TX_ZEROS_RM_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("tx_pcm_conf", &self.tx_pcm_conf())
            .field("tx_pcm_bypass", &self.tx_pcm_bypass())
            .field("rx_pcm_conf", &self.rx_pcm_conf())
            .field("rx_pcm_bypass", &self.rx_pcm_bypass())
            .field("tx_stop_en", &self.tx_stop_en())
            .field("tx_zeros_rm_en", &self.tx_zeros_rm_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pcm_conf(&mut self) -> TX_PCM_CONF_W<CONF1_SPEC> {
        TX_PCM_CONF_W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pcm_bypass(&mut self) -> TX_PCM_BYPASS_W<CONF1_SPEC> {
        TX_PCM_BYPASS_W::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pcm_conf(&mut self) -> RX_PCM_CONF_W<CONF1_SPEC> {
        RX_PCM_CONF_W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pcm_bypass(&mut self) -> RX_PCM_BYPASS_W<CONF1_SPEC> {
        RX_PCM_BYPASS_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_stop_en(&mut self) -> TX_STOP_EN_W<CONF1_SPEC> {
        TX_STOP_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tx_zeros_rm_en(&mut self) -> TX_ZEROS_RM_EN_W<CONF1_SPEC> {
        TX_ZEROS_RM_EN_W::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0x89"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0x89;
}
