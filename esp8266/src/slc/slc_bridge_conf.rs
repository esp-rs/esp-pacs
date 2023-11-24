#[doc = "Register `SLC_BRIDGE_CONF` reader"]
pub type R = crate::R<SLC_BRIDGE_CONF_SPEC>;
#[doc = "Register `SLC_BRIDGE_CONF` writer"]
pub type W = crate::W<SLC_BRIDGE_CONF_SPEC>;
#[doc = "Field `SLC_TXEOF_ENA` reader - "]
pub type SLC_TXEOF_ENA_R = crate::FieldReader;
#[doc = "Field `SLC_TXEOF_ENA` writer - "]
pub type SLC_TXEOF_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SLC_FIFO_MAP_ENA` reader - "]
pub type SLC_FIFO_MAP_ENA_R = crate::FieldReader;
#[doc = "Field `SLC_FIFO_MAP_ENA` writer - "]
pub type SLC_FIFO_MAP_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SLC_TX_DUMMY_MODE` reader - "]
pub type SLC_TX_DUMMY_MODE_R = crate::BitReader;
#[doc = "Field `SLC_TX_DUMMY_MODE` writer - "]
pub type SLC_TX_DUMMY_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_TX_PUSH_IDLE_NUM` reader - "]
pub type SLC_TX_PUSH_IDLE_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `SLC_TX_PUSH_IDLE_NUM` writer - "]
pub type SLC_TX_PUSH_IDLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn slc_txeof_ena(&self) -> SLC_TXEOF_ENA_R {
        SLC_TXEOF_ENA_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn slc_fifo_map_ena(&self) -> SLC_FIFO_MAP_ENA_R {
        SLC_FIFO_MAP_ENA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_tx_dummy_mode(&self) -> SLC_TX_DUMMY_MODE_R {
        SLC_TX_DUMMY_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn slc_tx_push_idle_num(&self) -> SLC_TX_PUSH_IDLE_NUM_R {
        SLC_TX_PUSH_IDLE_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_BRIDGE_CONF")
            .field(
                "slc_tx_push_idle_num",
                &format_args!("{}", self.slc_tx_push_idle_num().bits()),
            )
            .field(
                "slc_tx_dummy_mode",
                &format_args!("{}", self.slc_tx_dummy_mode().bit()),
            )
            .field(
                "slc_fifo_map_ena",
                &format_args!("{}", self.slc_fifo_map_ena().bits()),
            )
            .field(
                "slc_txeof_ena",
                &format_args!("{}", self.slc_txeof_ena().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_BRIDGE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txeof_ena(&mut self) -> SLC_TXEOF_ENA_W<SLC_BRIDGE_CONF_SPEC> {
        SLC_TXEOF_ENA_W::new(self, 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn slc_fifo_map_ena(&mut self) -> SLC_FIFO_MAP_ENA_W<SLC_BRIDGE_CONF_SPEC> {
        SLC_FIFO_MAP_ENA_W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_dummy_mode(&mut self) -> SLC_TX_DUMMY_MODE_W<SLC_BRIDGE_CONF_SPEC> {
        SLC_TX_DUMMY_MODE_W::new(self, 12)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_push_idle_num(&mut self) -> SLC_TX_PUSH_IDLE_NUM_W<SLC_BRIDGE_CONF_SPEC> {
        SLC_TX_PUSH_IDLE_NUM_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLC_BRIDGE_CONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_bridge_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_bridge_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_BRIDGE_CONF_SPEC;
impl crate::RegisterSpec for SLC_BRIDGE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_bridge_conf::R`](R) reader structure"]
impl crate::Readable for SLC_BRIDGE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_bridge_conf::W`](W) writer structure"]
impl crate::Writable for SLC_BRIDGE_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_BRIDGE_CONF to value 0"]
impl crate::Resettable for SLC_BRIDGE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
