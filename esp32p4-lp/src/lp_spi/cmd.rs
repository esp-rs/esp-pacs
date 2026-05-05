#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `LP_REG_UPDATE` writer - Set this bit to synchronize SPI registers from APB clock domain into SPI module clock domain, which is only used in SPI master mode."]
pub type LP_REG_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_USR` reader - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf."]
pub type LP_REG_USR_R = crate::BitReader;
#[doc = "Field `LP_REG_USR` writer - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf."]
pub type LP_REG_USR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn lp_reg_usr(&self) -> LP_REG_USR_R {
        LP_REG_USR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("lp_reg_usr", &self.lp_reg_usr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 23 - Set this bit to synchronize SPI registers from APB clock domain into SPI module clock domain, which is only used in SPI master mode."]
    #[inline(always)]
    pub fn lp_reg_update(&mut self) -> LP_REG_UPDATE_W<'_, CMD_SPEC> {
        LP_REG_UPDATE_W::new(self, 23)
    }
    #[doc = "Bit 24 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn lp_reg_usr(&mut self) -> LP_REG_USR_W<'_, CMD_SPEC> {
        LP_REG_USR_W::new(self, 24)
    }
}
#[doc = "Command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {}
