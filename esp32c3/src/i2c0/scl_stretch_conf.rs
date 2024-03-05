#[doc = "Register `SCL_STRETCH_CONF` reader"]
pub type R = crate::R<SCL_STRETCH_CONF_SPEC>;
#[doc = "Register `SCL_STRETCH_CONF` writer"]
pub type W = crate::W<SCL_STRETCH_CONF_SPEC>;
#[doc = "Field `STRETCH_PROTECT_NUM` reader - reg_stretch_protect_num"]
pub type STRETCH_PROTECT_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `STRETCH_PROTECT_NUM` writer - reg_stretch_protect_num"]
pub type STRETCH_PROTECT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SLAVE_SCL_STRETCH_EN` reader - reg_slave_scl_stretch_en"]
pub type SLAVE_SCL_STRETCH_EN_R = crate::BitReader;
#[doc = "Field `SLAVE_SCL_STRETCH_EN` writer - reg_slave_scl_stretch_en"]
pub type SLAVE_SCL_STRETCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_SCL_STRETCH_CLR` writer - reg_slave_scl_stretch_clr"]
pub type SLAVE_SCL_STRETCH_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_BYTE_ACK_CTL_EN` reader - reg_slave_byte_ack_ctl_en"]
pub type SLAVE_BYTE_ACK_CTL_EN_R = crate::BitReader;
#[doc = "Field `SLAVE_BYTE_ACK_CTL_EN` writer - reg_slave_byte_ack_ctl_en"]
pub type SLAVE_BYTE_ACK_CTL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_BYTE_ACK_LVL` reader - reg_slave_byte_ack_lvl"]
pub type SLAVE_BYTE_ACK_LVL_R = crate::BitReader;
#[doc = "Field `SLAVE_BYTE_ACK_LVL` writer - reg_slave_byte_ack_lvl"]
pub type SLAVE_BYTE_ACK_LVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - reg_stretch_protect_num"]
    #[inline(always)]
    pub fn stretch_protect_num(&self) -> STRETCH_PROTECT_NUM_R {
        STRETCH_PROTECT_NUM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - reg_slave_scl_stretch_en"]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&self) -> SLAVE_SCL_STRETCH_EN_R {
        SLAVE_SCL_STRETCH_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_slave_byte_ack_ctl_en"]
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&self) -> SLAVE_BYTE_ACK_CTL_EN_R {
        SLAVE_BYTE_ACK_CTL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_slave_byte_ack_lvl"]
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&self) -> SLAVE_BYTE_ACK_LVL_R {
        SLAVE_BYTE_ACK_LVL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_STRETCH_CONF")
            .field(
                "stretch_protect_num",
                &format_args!("{}", self.stretch_protect_num().bits()),
            )
            .field(
                "slave_scl_stretch_en",
                &format_args!("{}", self.slave_scl_stretch_en().bit()),
            )
            .field(
                "slave_byte_ack_ctl_en",
                &format_args!("{}", self.slave_byte_ack_ctl_en().bit()),
            )
            .field(
                "slave_byte_ack_lvl",
                &format_args!("{}", self.slave_byte_ack_lvl().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_STRETCH_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - reg_stretch_protect_num"]
    #[inline(always)]
    #[must_use]
    pub fn stretch_protect_num(&mut self) -> STRETCH_PROTECT_NUM_W<SCL_STRETCH_CONF_SPEC> {
        STRETCH_PROTECT_NUM_W::new(self, 0)
    }
    #[doc = "Bit 10 - reg_slave_scl_stretch_en"]
    #[inline(always)]
    #[must_use]
    pub fn slave_scl_stretch_en(&mut self) -> SLAVE_SCL_STRETCH_EN_W<SCL_STRETCH_CONF_SPEC> {
        SLAVE_SCL_STRETCH_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - reg_slave_scl_stretch_clr"]
    #[inline(always)]
    #[must_use]
    pub fn slave_scl_stretch_clr(&mut self) -> SLAVE_SCL_STRETCH_CLR_W<SCL_STRETCH_CONF_SPEC> {
        SLAVE_SCL_STRETCH_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - reg_slave_byte_ack_ctl_en"]
    #[inline(always)]
    #[must_use]
    pub fn slave_byte_ack_ctl_en(&mut self) -> SLAVE_BYTE_ACK_CTL_EN_W<SCL_STRETCH_CONF_SPEC> {
        SLAVE_BYTE_ACK_CTL_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - reg_slave_byte_ack_lvl"]
    #[inline(always)]
    #[must_use]
    pub fn slave_byte_ack_lvl(&mut self) -> SLAVE_BYTE_ACK_LVL_W<SCL_STRETCH_CONF_SPEC> {
        SLAVE_BYTE_ACK_LVL_W::new(self, 13)
    }
}
#[doc = "I2C_SCL_STRETCH_CONF_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stretch_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stretch_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_STRETCH_CONF_SPEC;
impl crate::RegisterSpec for SCL_STRETCH_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_stretch_conf::R`](R) reader structure"]
impl crate::Readable for SCL_STRETCH_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_stretch_conf::W`](W) writer structure"]
impl crate::Writable for SCL_STRETCH_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCL_STRETCH_CONF to value 0"]
impl crate::Resettable for SCL_STRETCH_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
