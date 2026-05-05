#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `LP_REG_CS0_DIS` reader - SPI CS$n pin enable, 1: disable CS$n, 0: spi_cs$n signal is from/to CS$n pin. Can be configured in CONF state."]
pub type LP_REG_CS0_DIS_R = crate::BitReader;
#[doc = "Field `LP_REG_CS0_DIS` writer - SPI CS$n pin enable, 1: disable CS$n, 0: spi_cs$n signal is from/to CS$n pin. Can be configured in CONF state."]
pub type LP_REG_CS0_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_CK_DIS` reader - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type LP_REG_CK_DIS_R = crate::BitReader;
#[doc = "Field `LP_REG_CK_DIS` writer - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type LP_REG_CK_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
pub type LP_REG_MASTER_CS_POL_R = crate::FieldReader;
#[doc = "Field `LP_REG_MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
pub type LP_REG_MASTER_CS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_REG_SLAVE_CS_POL` reader - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type LP_REG_SLAVE_CS_POL_R = crate::BitReader;
#[doc = "Field `LP_REG_SLAVE_CS_POL` writer - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type LP_REG_SLAVE_CS_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type LP_REG_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `LP_REG_CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type LP_REG_CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type LP_REG_CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `LP_REG_CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type LP_REG_CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI CS$n pin enable, 1: disable CS$n, 0: spi_cs$n signal is from/to CS$n pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs0_dis(&self) -> LP_REG_CS0_DIS_R {
        LP_REG_CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_ck_dis(&self) -> LP_REG_CK_DIS_R {
        LP_REG_CK_DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_master_cs_pol(&self) -> LP_REG_MASTER_CS_POL_R {
        LP_REG_MASTER_CS_POL_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_slave_cs_pol(&self) -> LP_REG_SLAVE_CS_POL_R {
        LP_REG_SLAVE_CS_POL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_ck_idle_edge(&self) -> LP_REG_CK_IDLE_EDGE_R {
        LP_REG_CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs_keep_active(&self) -> LP_REG_CS_KEEP_ACTIVE_R {
        LP_REG_CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("lp_reg_cs0_dis", &self.lp_reg_cs0_dis())
            .field("lp_reg_ck_dis", &self.lp_reg_ck_dis())
            .field("lp_reg_master_cs_pol", &self.lp_reg_master_cs_pol())
            .field("lp_reg_slave_cs_pol", &self.lp_reg_slave_cs_pol())
            .field("lp_reg_ck_idle_edge", &self.lp_reg_ck_idle_edge())
            .field("lp_reg_cs_keep_active", &self.lp_reg_cs_keep_active())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI CS$n pin enable, 1: disable CS$n, 0: spi_cs$n signal is from/to CS$n pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs0_dis(&mut self) -> LP_REG_CS0_DIS_W<'_, MISC_SPEC> {
        LP_REG_CS0_DIS_W::new(self, 0)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_ck_dis(&mut self) -> LP_REG_CK_DIS_W<'_, MISC_SPEC> {
        LP_REG_CK_DIS_W::new(self, 6)
    }
    #[doc = "Bits 7:9 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_master_cs_pol(&mut self) -> LP_REG_MASTER_CS_POL_W<'_, MISC_SPEC> {
        LP_REG_MASTER_CS_POL_W::new(self, 7)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_slave_cs_pol(&mut self) -> LP_REG_SLAVE_CS_POL_W<'_, MISC_SPEC> {
        LP_REG_SLAVE_CS_POL_W::new(self, 23)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_ck_idle_edge(&mut self) -> LP_REG_CK_IDLE_EDGE_W<'_, MISC_SPEC> {
        LP_REG_CK_IDLE_EDGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs_keep_active(&mut self) -> LP_REG_CS_KEEP_ACTIVE_W<'_, MISC_SPEC> {
        LP_REG_CS_KEEP_ACTIVE_W::new(self, 30)
    }
}
#[doc = "SPI misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {}
