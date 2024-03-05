#[doc = "Register `ESCAPE_CONF` reader"]
pub type R = crate::R<ESCAPE_CONF_SPEC>;
#[doc = "Register `ESCAPE_CONF` writer"]
pub type W = crate::W<ESCAPE_CONF_SPEC>;
#[doc = "Field `TX_C0_ESC_EN` reader - a"]
pub type TX_C0_ESC_EN_R = crate::BitReader;
#[doc = "Field `TX_C0_ESC_EN` writer - a"]
pub type TX_C0_ESC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DB_ESC_EN` reader - a"]
pub type TX_DB_ESC_EN_R = crate::BitReader;
#[doc = "Field `TX_DB_ESC_EN` writer - a"]
pub type TX_DB_ESC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_11_ESC_EN` reader - a"]
pub type TX_11_ESC_EN_R = crate::BitReader;
#[doc = "Field `TX_11_ESC_EN` writer - a"]
pub type TX_11_ESC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_13_ESC_EN` reader - a"]
pub type TX_13_ESC_EN_R = crate::BitReader;
#[doc = "Field `TX_13_ESC_EN` writer - a"]
pub type TX_13_ESC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_C0_ESC_EN` reader - a"]
pub type RX_C0_ESC_EN_R = crate::BitReader;
#[doc = "Field `RX_C0_ESC_EN` writer - a"]
pub type RX_C0_ESC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DB_ESC_EN` reader - a"]
pub type RX_DB_ESC_EN_R = crate::BitReader;
#[doc = "Field `RX_DB_ESC_EN` writer - a"]
pub type RX_DB_ESC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_11_ESC_EN` reader - a"]
pub type RX_11_ESC_EN_R = crate::BitReader;
#[doc = "Field `RX_11_ESC_EN` writer - a"]
pub type RX_11_ESC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_13_ESC_EN` reader - a"]
pub type RX_13_ESC_EN_R = crate::BitReader;
#[doc = "Field `RX_13_ESC_EN` writer - a"]
pub type RX_13_ESC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - a"]
    #[inline(always)]
    pub fn tx_c0_esc_en(&self) -> TX_C0_ESC_EN_R {
        TX_C0_ESC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - a"]
    #[inline(always)]
    pub fn tx_db_esc_en(&self) -> TX_DB_ESC_EN_R {
        TX_DB_ESC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - a"]
    #[inline(always)]
    pub fn tx_11_esc_en(&self) -> TX_11_ESC_EN_R {
        TX_11_ESC_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    pub fn tx_13_esc_en(&self) -> TX_13_ESC_EN_R {
        TX_13_ESC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a"]
    #[inline(always)]
    pub fn rx_c0_esc_en(&self) -> RX_C0_ESC_EN_R {
        RX_C0_ESC_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - a"]
    #[inline(always)]
    pub fn rx_db_esc_en(&self) -> RX_DB_ESC_EN_R {
        RX_DB_ESC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - a"]
    #[inline(always)]
    pub fn rx_11_esc_en(&self) -> RX_11_ESC_EN_R {
        RX_11_ESC_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - a"]
    #[inline(always)]
    pub fn rx_13_esc_en(&self) -> RX_13_ESC_EN_R {
        RX_13_ESC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESCAPE_CONF")
            .field(
                "tx_c0_esc_en",
                &format_args!("{}", self.tx_c0_esc_en().bit()),
            )
            .field(
                "tx_db_esc_en",
                &format_args!("{}", self.tx_db_esc_en().bit()),
            )
            .field(
                "tx_11_esc_en",
                &format_args!("{}", self.tx_11_esc_en().bit()),
            )
            .field(
                "tx_13_esc_en",
                &format_args!("{}", self.tx_13_esc_en().bit()),
            )
            .field(
                "rx_c0_esc_en",
                &format_args!("{}", self.rx_c0_esc_en().bit()),
            )
            .field(
                "rx_db_esc_en",
                &format_args!("{}", self.rx_db_esc_en().bit()),
            )
            .field(
                "rx_11_esc_en",
                &format_args!("{}", self.rx_11_esc_en().bit()),
            )
            .field(
                "rx_13_esc_en",
                &format_args!("{}", self.rx_13_esc_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ESCAPE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - a"]
    #[inline(always)]
    #[must_use]
    pub fn tx_c0_esc_en(&mut self) -> TX_C0_ESC_EN_W<ESCAPE_CONF_SPEC> {
        TX_C0_ESC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - a"]
    #[inline(always)]
    #[must_use]
    pub fn tx_db_esc_en(&mut self) -> TX_DB_ESC_EN_W<ESCAPE_CONF_SPEC> {
        TX_DB_ESC_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - a"]
    #[inline(always)]
    #[must_use]
    pub fn tx_11_esc_en(&mut self) -> TX_11_ESC_EN_W<ESCAPE_CONF_SPEC> {
        TX_11_ESC_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    #[must_use]
    pub fn tx_13_esc_en(&mut self) -> TX_13_ESC_EN_W<ESCAPE_CONF_SPEC> {
        TX_13_ESC_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - a"]
    #[inline(always)]
    #[must_use]
    pub fn rx_c0_esc_en(&mut self) -> RX_C0_ESC_EN_W<ESCAPE_CONF_SPEC> {
        RX_C0_ESC_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - a"]
    #[inline(always)]
    #[must_use]
    pub fn rx_db_esc_en(&mut self) -> RX_DB_ESC_EN_W<ESCAPE_CONF_SPEC> {
        RX_DB_ESC_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - a"]
    #[inline(always)]
    #[must_use]
    pub fn rx_11_esc_en(&mut self) -> RX_11_ESC_EN_W<ESCAPE_CONF_SPEC> {
        RX_11_ESC_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - a"]
    #[inline(always)]
    #[must_use]
    pub fn rx_13_esc_en(&mut self) -> RX_13_ESC_EN_W<ESCAPE_CONF_SPEC> {
        RX_13_ESC_EN_W::new(self, 7)
    }
}
#[doc = "a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`escape_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`escape_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESCAPE_CONF_SPEC;
impl crate::RegisterSpec for ESCAPE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`escape_conf::R`](R) reader structure"]
impl crate::Readable for ESCAPE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`escape_conf::W`](W) writer structure"]
impl crate::Writable for ESCAPE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESCAPE_CONF to value 0x33"]
impl crate::Resettable for ESCAPE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x33;
}
