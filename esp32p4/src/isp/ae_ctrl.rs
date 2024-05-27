///Register `AE_CTRL` reader
pub type R = crate::R<AE_CTRL_SPEC>;
///Register `AE_CTRL` writer
pub type W = crate::W<AE_CTRL_SPEC>;
///Field `AE_UPDATE` writer - write 1 to this bit triggers one statistic event
pub type AE_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AE_SELECT` reader - this field configures ae input data source, 0: data from median, 1: data from gama
pub type AE_SELECT_R = crate::BitReader;
///Field `AE_SELECT` writer - this field configures ae input data source, 0: data from median, 1: data from gama
pub type AE_SELECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - this field configures ae input data source, 0: data from median, 1: data from gama
    #[inline(always)]
    pub fn ae_select(&self) -> AE_SELECT_R {
        AE_SELECT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_CTRL")
            .field("ae_select", &self.ae_select())
            .finish()
    }
}
impl W {
    ///Bit 0 - write 1 to this bit triggers one statistic event
    #[inline(always)]
    #[must_use]
    pub fn ae_update(&mut self) -> AE_UPDATE_W<AE_CTRL_SPEC> {
        AE_UPDATE_W::new(self, 0)
    }
    ///Bit 1 - this field configures ae input data source, 0: data from median, 1: data from gama
    #[inline(always)]
    #[must_use]
    pub fn ae_select(&mut self) -> AE_SELECT_W<AE_CTRL_SPEC> {
        AE_SELECT_W::new(self, 1)
    }
}
/**ae control register

You can [`read`](crate::generic::Reg::read) this register and get [`ae_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AE_CTRL_SPEC;
impl crate::RegisterSpec for AE_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ae_ctrl::R`](R) reader structure
impl crate::Readable for AE_CTRL_SPEC {}
///`write(|w| ..)` method takes [`ae_ctrl::W`](W) writer structure
impl crate::Writable for AE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AE_CTRL to value 0
impl crate::Resettable for AE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
