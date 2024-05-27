///Register `SAR_TSENS_CTRL` reader
pub type R = crate::R<SAR_TSENS_CTRL_SPEC>;
///Register `SAR_TSENS_CTRL` writer
pub type W = crate::W<SAR_TSENS_CTRL_SPEC>;
///Field `TSENS_XPD_WAIT` reader -
pub type TSENS_XPD_WAIT_R = crate::FieldReader<u16>;
///Field `TSENS_XPD_WAIT` writer -
pub type TSENS_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `TSENS_XPD_FORCE` reader -
pub type TSENS_XPD_FORCE_R = crate::BitReader;
///Field `TSENS_XPD_FORCE` writer -
pub type TSENS_XPD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSENS_CLK_INV` reader -
pub type TSENS_CLK_INV_R = crate::BitReader;
///Field `TSENS_CLK_INV` writer -
pub type TSENS_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSENS_CLK_GATED` reader -
pub type TSENS_CLK_GATED_R = crate::BitReader;
///Field `TSENS_CLK_GATED` writer -
pub type TSENS_CLK_GATED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSENS_IN_INV` reader - invert temperature sensor data
pub type TSENS_IN_INV_R = crate::BitReader;
///Field `TSENS_IN_INV` writer - invert temperature sensor data
pub type TSENS_IN_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSENS_CLK_DIV` reader - temperature sensor clock divider
pub type TSENS_CLK_DIV_R = crate::FieldReader;
///Field `TSENS_CLK_DIV` writer - temperature sensor clock divider
pub type TSENS_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TSENS_POWER_UP` reader - temperature sensor power up
pub type TSENS_POWER_UP_R = crate::BitReader;
///Field `TSENS_POWER_UP` writer - temperature sensor power up
pub type TSENS_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSENS_POWER_UP_FORCE` reader - 1: dump out &amp; power up controlled by SW 0: by FSM
pub type TSENS_POWER_UP_FORCE_R = crate::BitReader;
///Field `TSENS_POWER_UP_FORCE` writer - 1: dump out &amp; power up controlled by SW 0: by FSM
pub type TSENS_POWER_UP_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSENS_DUMP_OUT` reader - temperature sensor dump out only active when reg_tsens_power_up_force = 1
pub type TSENS_DUMP_OUT_R = crate::BitReader;
///Field `TSENS_DUMP_OUT` writer - temperature sensor dump out only active when reg_tsens_power_up_force = 1
pub type TSENS_DUMP_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn tsens_xpd_wait(&self) -> TSENS_XPD_WAIT_R {
        TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12
    #[inline(always)]
    pub fn tsens_xpd_force(&self) -> TSENS_XPD_FORCE_R {
        TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn tsens_clk_inv(&self) -> TSENS_CLK_INV_R {
        TSENS_CLK_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn tsens_clk_gated(&self) -> TSENS_CLK_GATED_R {
        TSENS_CLK_GATED_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - invert temperature sensor data
    #[inline(always)]
    pub fn tsens_in_inv(&self) -> TSENS_IN_INV_R {
        TSENS_IN_INV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - temperature sensor clock divider
    #[inline(always)]
    pub fn tsens_clk_div(&self) -> TSENS_CLK_DIV_R {
        TSENS_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - temperature sensor power up
    #[inline(always)]
    pub fn tsens_power_up(&self) -> TSENS_POWER_UP_R {
        TSENS_POWER_UP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - 1: dump out &amp; power up controlled by SW 0: by FSM
    #[inline(always)]
    pub fn tsens_power_up_force(&self) -> TSENS_POWER_UP_FORCE_R {
        TSENS_POWER_UP_FORCE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - temperature sensor dump out only active when reg_tsens_power_up_force = 1
    #[inline(always)]
    pub fn tsens_dump_out(&self) -> TSENS_DUMP_OUT_R {
        TSENS_DUMP_OUT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TSENS_CTRL")
            .field("tsens_xpd_wait", &self.tsens_xpd_wait())
            .field("tsens_xpd_force", &self.tsens_xpd_force())
            .field("tsens_clk_inv", &self.tsens_clk_inv())
            .field("tsens_clk_gated", &self.tsens_clk_gated())
            .field("tsens_in_inv", &self.tsens_in_inv())
            .field("tsens_clk_div", &self.tsens_clk_div())
            .field("tsens_power_up", &self.tsens_power_up())
            .field("tsens_power_up_force", &self.tsens_power_up_force())
            .field("tsens_dump_out", &self.tsens_dump_out())
            .finish()
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    #[must_use]
    pub fn tsens_xpd_wait(&mut self) -> TSENS_XPD_WAIT_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_XPD_WAIT_W::new(self, 0)
    }
    ///Bit 12
    #[inline(always)]
    #[must_use]
    pub fn tsens_xpd_force(&mut self) -> TSENS_XPD_FORCE_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_XPD_FORCE_W::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_inv(&mut self) -> TSENS_CLK_INV_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_CLK_INV_W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_gated(&mut self) -> TSENS_CLK_GATED_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_CLK_GATED_W::new(self, 14)
    }
    ///Bit 15 - invert temperature sensor data
    #[inline(always)]
    #[must_use]
    pub fn tsens_in_inv(&mut self) -> TSENS_IN_INV_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_IN_INV_W::new(self, 15)
    }
    ///Bits 16:23 - temperature sensor clock divider
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_div(&mut self) -> TSENS_CLK_DIV_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_CLK_DIV_W::new(self, 16)
    }
    ///Bit 24 - temperature sensor power up
    #[inline(always)]
    #[must_use]
    pub fn tsens_power_up(&mut self) -> TSENS_POWER_UP_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_POWER_UP_W::new(self, 24)
    }
    ///Bit 25 - 1: dump out &amp; power up controlled by SW 0: by FSM
    #[inline(always)]
    #[must_use]
    pub fn tsens_power_up_force(&mut self) -> TSENS_POWER_UP_FORCE_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_POWER_UP_FORCE_W::new(self, 25)
    }
    ///Bit 26 - temperature sensor dump out only active when reg_tsens_power_up_force = 1
    #[inline(always)]
    #[must_use]
    pub fn tsens_dump_out(&mut self) -> TSENS_DUMP_OUT_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_DUMP_OUT_W::new(self, 26)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sar_tsens_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_tsens_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TSENS_CTRL_SPEC;
impl crate::RegisterSpec for SAR_TSENS_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_tsens_ctrl::R`](R) reader structure
impl crate::Readable for SAR_TSENS_CTRL_SPEC {}
///`write(|w| ..)` method takes [`sar_tsens_ctrl::W`](W) writer structure
impl crate::Writable for SAR_TSENS_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_TSENS_CTRL to value 0x0006_6002
impl crate::Resettable for SAR_TSENS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0006_6002;
}
