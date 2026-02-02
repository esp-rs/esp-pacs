#[doc = "Register `SDIO_SLAVE_LDO_CONF` reader"]
pub type R = crate::R<SDIO_SLAVE_LDO_CONF_SPEC>;
#[doc = "Register `SDIO_SLAVE_LDO_CONF` writer"]
pub type W = crate::W<SDIO_SLAVE_LDO_CONF_SPEC>;
#[doc = "Field `LDO_READY_CTL_IN_EN` reader - "]
pub type LDO_READY_CTL_IN_EN_R = crate::BitReader;
#[doc = "Field `LDO_READY_CTL_IN_EN` writer - "]
pub type LDO_READY_CTL_IN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDO_READY_THRES` reader - "]
pub type LDO_READY_THRES_R = crate::FieldReader;
#[doc = "Field `LDO_READY_THRES` writer - "]
pub type LDO_READY_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LDO_READY_IGNORE_EN` reader - "]
pub type LDO_READY_IGNORE_EN_R = crate::BitReader;
#[doc = "Field `LDO_READY_IGNORE_EN` writer - "]
pub type LDO_READY_IGNORE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ldo_ready_ctl_in_en(&self) -> LDO_READY_CTL_IN_EN_R {
        LDO_READY_CTL_IN_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn ldo_ready_thres(&self) -> LDO_READY_THRES_R {
        LDO_READY_THRES_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6"]
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ldo_ready_ctl_in_en(&mut self) -> LDO_READY_CTL_IN_EN_W<'_, SDIO_SLAVE_LDO_CONF_SPEC> {
        LDO_READY_CTL_IN_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn ldo_ready_thres(&mut self) -> LDO_READY_THRES_W<'_, SDIO_SLAVE_LDO_CONF_SPEC> {
        LDO_READY_THRES_W::new(self, 1)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ldo_ready_ignore_en(&mut self) -> LDO_READY_IGNORE_EN_W<'_, SDIO_SLAVE_LDO_CONF_SPEC> {
        LDO_READY_IGNORE_EN_W::new(self, 6)
    }
}
#[doc = "SDIO_SLAVE_LDO_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_slave_ldo_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_slave_ldo_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_SLAVE_LDO_CONF_SPEC;
impl crate::RegisterSpec for SDIO_SLAVE_LDO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_slave_ldo_conf::R`](R) reader structure"]
impl crate::Readable for SDIO_SLAVE_LDO_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_slave_ldo_conf::W`](W) writer structure"]
impl crate::Writable for SDIO_SLAVE_LDO_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_SLAVE_LDO_CONF to value 0x14"]
impl crate::Resettable for SDIO_SLAVE_LDO_CONF_SPEC {
    const RESET_VALUE: u32 = 0x14;
}
