///Register `DS_CONF` reader
pub type R = crate::R<DS_CONF_SPEC>;
///Register `DS_CONF` writer
pub type W = crate::W<DS_CONF_SPEC>;
///Field `DS_CLK_EN` reader - Set 1 to enable ds clock
pub type DS_CLK_EN_R = crate::BitReader;
///Field `DS_CLK_EN` writer - Set 1 to enable ds clock
pub type DS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DS_RST_EN` reader - Set 0 to reset ds module
pub type DS_RST_EN_R = crate::BitReader;
///Field `DS_RST_EN` writer - Set 0 to reset ds module
pub type DS_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable ds clock
    #[inline(always)]
    pub fn ds_clk_en(&self) -> DS_CLK_EN_R {
        DS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset ds module
    #[inline(always)]
    pub fn ds_rst_en(&self) -> DS_RST_EN_R {
        DS_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DS_CONF")
            .field("ds_clk_en", &self.ds_clk_en())
            .field("ds_rst_en", &self.ds_rst_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable ds clock
    #[inline(always)]
    #[must_use]
    pub fn ds_clk_en(&mut self) -> DS_CLK_EN_W<DS_CONF_SPEC> {
        DS_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset ds module
    #[inline(always)]
    #[must_use]
    pub fn ds_rst_en(&mut self) -> DS_RST_EN_W<DS_CONF_SPEC> {
        DS_RST_EN_W::new(self, 1)
    }
}
/**DS configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ds_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ds_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DS_CONF_SPEC;
impl crate::RegisterSpec for DS_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ds_conf::R`](R) reader structure
impl crate::Readable for DS_CONF_SPEC {}
///`write(|w| ..)` method takes [`ds_conf::W`](W) writer structure
impl crate::Writable for DS_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DS_CONF to value 0x01
impl crate::Resettable for DS_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
