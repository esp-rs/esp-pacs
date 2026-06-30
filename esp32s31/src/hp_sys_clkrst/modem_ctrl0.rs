#[doc = "Register `MODEM_CTRL0` reader"]
pub type R = crate::R<MODEM_CTRL0_SPEC>;
#[doc = "Register `MODEM_CTRL0` writer"]
pub type W = crate::W<MODEM_CTRL0_SPEC>;
#[doc = "Field `REG_MODEM_CLK_EN` reader - need_des"]
pub type REG_MODEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_MODEM_CLK_EN` writer - need_des"]
pub type REG_MODEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_modem_clk_en(&self) -> REG_MODEM_CLK_EN_R {
        REG_MODEM_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_CTRL0")
            .field("reg_modem_clk_en", &self.reg_modem_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_modem_clk_en(&mut self) -> REG_MODEM_CLK_EN_W<'_, MODEM_CTRL0_SPEC> {
        REG_MODEM_CLK_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_CTRL0_SPEC;
impl crate::RegisterSpec for MODEM_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_ctrl0::R`](R) reader structure"]
impl crate::Readable for MODEM_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_ctrl0::W`](W) writer structure"]
impl crate::Writable for MODEM_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_CTRL0 to value 0x01"]
impl crate::Resettable for MODEM_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
