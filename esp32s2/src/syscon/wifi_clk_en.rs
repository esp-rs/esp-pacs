///Register `WIFI_CLK_EN` reader
pub type R = crate::R<WIFI_CLK_EN_SPEC>;
///Register `WIFI_CLK_EN` writer
pub type W = crate::W<WIFI_CLK_EN_SPEC>;
///Field `WIFI_CLK_EN` reader -
pub type WIFI_CLK_EN_R = crate::FieldReader<u32>;
///Field `WIFI_CLK_EN` writer -
pub type WIFI_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn wifi_clk_en(&self) -> WIFI_CLK_EN_R {
        WIFI_CLK_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_CLK_EN")
            .field("wifi_clk_en", &self.wifi_clk_en())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn wifi_clk_en(&mut self) -> WIFI_CLK_EN_W<WIFI_CLK_EN_SPEC> {
        WIFI_CLK_EN_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`wifi_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WIFI_CLK_EN_SPEC;
impl crate::RegisterSpec for WIFI_CLK_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`wifi_clk_en::R`](R) reader structure
impl crate::Readable for WIFI_CLK_EN_SPEC {}
///`write(|w| ..)` method takes [`wifi_clk_en::W`](W) writer structure
impl crate::Writable for WIFI_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WIFI_CLK_EN to value 0xfffc_e030
impl crate::Resettable for WIFI_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0xfffc_e030;
}
