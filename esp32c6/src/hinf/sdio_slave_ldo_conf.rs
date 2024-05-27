///Register `SDIO_SLAVE_LDO_CONF` reader
pub type R = crate::R<SDIO_SLAVE_LDO_CONF_SPEC>;
///Register `SDIO_SLAVE_LDO_CONF` writer
pub type W = crate::W<SDIO_SLAVE_LDO_CONF_SPEC>;
///Field `LDO_READY_CTL_IN_EN` reader - control ldo ready signal by sdio slave itself
pub type LDO_READY_CTL_IN_EN_R = crate::BitReader;
///Field `LDO_READY_CTL_IN_EN` writer - control ldo ready signal by sdio slave itself
pub type LDO_READY_CTL_IN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDO_READY_THRES` reader - configure ldo ready counting threshold value, the actual counting target is 2^(ldo_ready_thres)-1
pub type LDO_READY_THRES_R = crate::FieldReader;
///Field `LDO_READY_THRES` writer - configure ldo ready counting threshold value, the actual counting target is 2^(ldo_ready_thres)-1
pub type LDO_READY_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `LDO_READY_IGNORE_EN` reader - ignore ldo ready signal
pub type LDO_READY_IGNORE_EN_R = crate::BitReader;
///Field `LDO_READY_IGNORE_EN` writer - ignore ldo ready signal
pub type LDO_READY_IGNORE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - control ldo ready signal by sdio slave itself
    #[inline(always)]
    pub fn ldo_ready_ctl_in_en(&self) -> LDO_READY_CTL_IN_EN_R {
        LDO_READY_CTL_IN_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:5 - configure ldo ready counting threshold value, the actual counting target is 2^(ldo_ready_thres)-1
    #[inline(always)]
    pub fn ldo_ready_thres(&self) -> LDO_READY_THRES_R {
        LDO_READY_THRES_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    ///Bit 6 - ignore ldo ready signal
    #[inline(always)]
    pub fn ldo_ready_ignore_en(&self) -> LDO_READY_IGNORE_EN_R {
        LDO_READY_IGNORE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_SLAVE_LDO_CONF")
            .field("ldo_ready_ctl_in_en", &self.ldo_ready_ctl_in_en())
            .field("ldo_ready_thres", &self.ldo_ready_thres())
            .field("ldo_ready_ignore_en", &self.ldo_ready_ignore_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - control ldo ready signal by sdio slave itself
    #[inline(always)]
    #[must_use]
    pub fn ldo_ready_ctl_in_en(&mut self) -> LDO_READY_CTL_IN_EN_W<SDIO_SLAVE_LDO_CONF_SPEC> {
        LDO_READY_CTL_IN_EN_W::new(self, 0)
    }
    ///Bits 1:5 - configure ldo ready counting threshold value, the actual counting target is 2^(ldo_ready_thres)-1
    #[inline(always)]
    #[must_use]
    pub fn ldo_ready_thres(&mut self) -> LDO_READY_THRES_W<SDIO_SLAVE_LDO_CONF_SPEC> {
        LDO_READY_THRES_W::new(self, 1)
    }
    ///Bit 6 - ignore ldo ready signal
    #[inline(always)]
    #[must_use]
    pub fn ldo_ready_ignore_en(&mut self) -> LDO_READY_IGNORE_EN_W<SDIO_SLAVE_LDO_CONF_SPEC> {
        LDO_READY_IGNORE_EN_W::new(self, 6)
    }
}
/**sdio slave ldo control register

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_slave_ldo_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_slave_ldo_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDIO_SLAVE_LDO_CONF_SPEC;
impl crate::RegisterSpec for SDIO_SLAVE_LDO_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sdio_slave_ldo_conf::R`](R) reader structure
impl crate::Readable for SDIO_SLAVE_LDO_CONF_SPEC {}
///`write(|w| ..)` method takes [`sdio_slave_ldo_conf::W`](W) writer structure
impl crate::Writable for SDIO_SLAVE_LDO_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDIO_SLAVE_LDO_CONF to value 0x14
impl crate::Resettable for SDIO_SLAVE_LDO_CONF_SPEC {
    const RESET_VALUE: u32 = 0x14;
}
