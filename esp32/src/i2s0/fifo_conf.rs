#[doc = "Register `FIFO_CONF` reader"]
pub type R = crate::R<FIFO_CONF_SPEC>;
#[doc = "Register `FIFO_CONF` writer"]
pub type W = crate::W<FIFO_CONF_SPEC>;
#[doc = "Field `RX_DATA_NUM` reader - "]
pub type RX_DATA_NUM_R = crate::FieldReader;
#[doc = "Field `RX_DATA_NUM` writer - "]
pub type RX_DATA_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TX_DATA_NUM` reader - "]
pub type TX_DATA_NUM_R = crate::FieldReader;
#[doc = "Field `TX_DATA_NUM` writer - "]
pub type TX_DATA_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DSCR_EN` reader - "]
pub type DSCR_EN_R = crate::BitReader;
#[doc = "Field `DSCR_EN` writer - "]
pub type DSCR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_MOD` reader - "]
pub type TX_FIFO_MOD_R = crate::FieldReader;
#[doc = "Field `TX_FIFO_MOD` writer - "]
pub type TX_FIFO_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_FIFO_MOD` reader - "]
pub type RX_FIFO_MOD_R = crate::FieldReader;
#[doc = "Field `RX_FIFO_MOD` writer - "]
pub type RX_FIFO_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TX_FIFO_MOD_FORCE_EN` reader - "]
pub type TX_FIFO_MOD_FORCE_EN_R = crate::BitReader;
#[doc = "Field `TX_FIFO_MOD_FORCE_EN` writer - "]
pub type TX_FIFO_MOD_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_MOD_FORCE_EN` reader - "]
pub type RX_FIFO_MOD_FORCE_EN_R = crate::BitReader;
#[doc = "Field `RX_FIFO_MOD_FORCE_EN` writer - "]
pub type RX_FIFO_MOD_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rx_data_num(&self) -> RX_DATA_NUM_R {
        RX_DATA_NUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn tx_data_num(&self) -> TX_DATA_NUM_R {
        TX_DATA_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dscr_en(&self) -> DSCR_EN_R {
        DSCR_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn tx_fifo_mod(&self) -> TX_FIFO_MOD_R {
        TX_FIFO_MOD_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rx_fifo_mod(&self) -> RX_FIFO_MOD_R {
        RX_FIFO_MOD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_fifo_mod_force_en(&self) -> TX_FIFO_MOD_FORCE_EN_R {
        TX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_fifo_mod_force_en(&self) -> RX_FIFO_MOD_FORCE_EN_R {
        RX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CONF")
            .field("rx_data_num", &self.rx_data_num())
            .field("tx_data_num", &self.tx_data_num())
            .field("dscr_en", &self.dscr_en())
            .field("tx_fifo_mod", &self.tx_fifo_mod())
            .field("rx_fifo_mod", &self.rx_fifo_mod())
            .field("tx_fifo_mod_force_en", &self.tx_fifo_mod_force_en())
            .field("rx_fifo_mod_force_en", &self.rx_fifo_mod_force_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_num(&mut self) -> RX_DATA_NUM_W<FIFO_CONF_SPEC> {
        RX_DATA_NUM_W::new(self, 0)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_num(&mut self) -> TX_DATA_NUM_W<FIFO_CONF_SPEC> {
        TX_DATA_NUM_W::new(self, 6)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dscr_en(&mut self) -> DSCR_EN_W<FIFO_CONF_SPEC> {
        DSCR_EN_W::new(self, 12)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_mod(&mut self) -> TX_FIFO_MOD_W<FIFO_CONF_SPEC> {
        TX_FIFO_MOD_W::new(self, 13)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_mod(&mut self) -> RX_FIFO_MOD_W<FIFO_CONF_SPEC> {
        RX_FIFO_MOD_W::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_mod_force_en(&mut self) -> TX_FIFO_MOD_FORCE_EN_W<FIFO_CONF_SPEC> {
        TX_FIFO_MOD_FORCE_EN_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_mod_force_en(&mut self) -> RX_FIFO_MOD_FORCE_EN_W<FIFO_CONF_SPEC> {
        RX_FIFO_MOD_FORCE_EN_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_conf::R`](R) reader structure"]
impl crate::Readable for FIFO_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_conf::W`](W) writer structure"]
impl crate::Writable for FIFO_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0x1820"]
impl crate::Resettable for FIFO_CONF_SPEC {
    const RESET_VALUE: u32 = 0x1820;
}
