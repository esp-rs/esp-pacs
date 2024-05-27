///Register `MODEM_32K_CLK_CONF` reader
pub type R = crate::R<MODEM_32K_CLK_CONF_SPEC>;
///Register `MODEM_32K_CLK_CONF` writer
pub type W = crate::W<MODEM_32K_CLK_CONF_SPEC>;
///Field `CLK_MODEM_32K_SEL` reader -
pub type CLK_MODEM_32K_SEL_R = crate::FieldReader;
///Field `CLK_MODEM_32K_SEL` writer -
pub type CLK_MODEM_32K_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn clk_modem_32k_sel(&self) -> CLK_MODEM_32K_SEL_R {
        CLK_MODEM_32K_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_32K_CLK_CONF")
            .field("clk_modem_32k_sel", &self.clk_modem_32k_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_32k_sel(&mut self) -> CLK_MODEM_32K_SEL_W<MODEM_32K_CLK_CONF_SPEC> {
        CLK_MODEM_32K_SEL_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`modem_32k_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modem_32k_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MODEM_32K_CLK_CONF_SPEC;
impl crate::RegisterSpec for MODEM_32K_CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`modem_32k_clk_conf::R`](R) reader structure
impl crate::Readable for MODEM_32K_CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`modem_32k_clk_conf::W`](W) writer structure
impl crate::Writable for MODEM_32K_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODEM_32K_CLK_CONF to value 0
impl crate::Resettable for MODEM_32K_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
