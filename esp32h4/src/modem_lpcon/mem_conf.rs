#[doc = "Register `MEM_CONF` reader"]
pub type R = crate::R<MEM_CONF_SPEC>;
#[doc = "Register `MEM_CONF` writer"]
pub type W = crate::W<MEM_CONF_SPEC>;
#[doc = "Field `DC_MEM_MODE` reader - "]
pub type DC_MEM_MODE_R = crate::FieldReader;
#[doc = "Field `DC_MEM_MODE` writer - "]
pub type DC_MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DC_MEM_FORCE` reader - "]
pub type DC_MEM_FORCE_R = crate::BitReader;
#[doc = "Field `DC_MEM_FORCE` writer - "]
pub type DC_MEM_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_MEM_MODE` reader - "]
pub type AGC_MEM_MODE_R = crate::FieldReader;
#[doc = "Field `AGC_MEM_MODE` writer - "]
pub type AGC_MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AGC_MEM_FORCE` reader - "]
pub type AGC_MEM_FORCE_R = crate::BitReader;
#[doc = "Field `AGC_MEM_FORCE` writer - "]
pub type AGC_MEM_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBUS_MEM_MODE` reader - "]
pub type PBUS_MEM_MODE_R = crate::FieldReader;
#[doc = "Field `PBUS_MEM_MODE` writer - "]
pub type PBUS_MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PBUS_MEM_FORCE` reader - "]
pub type PBUS_MEM_FORCE_R = crate::BitReader;
#[doc = "Field `PBUS_MEM_FORCE` writer - "]
pub type PBUS_MEM_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC_MEM_MODE` reader - "]
pub type BC_MEM_MODE_R = crate::FieldReader;
#[doc = "Field `BC_MEM_MODE` writer - "]
pub type BC_MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BC_MEM_FORCE` reader - "]
pub type BC_MEM_FORCE_R = crate::BitReader;
#[doc = "Field `BC_MEM_FORCE` writer - "]
pub type BC_MEM_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_MST_MEM_MODE` reader - "]
pub type I2C_MST_MEM_MODE_R = crate::FieldReader;
#[doc = "Field `I2C_MST_MEM_MODE` writer - "]
pub type I2C_MST_MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `I2C_MST_MEM_FORCE` reader - "]
pub type I2C_MST_MEM_FORCE_R = crate::BitReader;
#[doc = "Field `I2C_MST_MEM_FORCE` writer - "]
pub type I2C_MST_MEM_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN_FREQ_MEM_MODE` reader - "]
pub type CHAN_FREQ_MEM_MODE_R = crate::FieldReader;
#[doc = "Field `CHAN_FREQ_MEM_MODE` writer - "]
pub type CHAN_FREQ_MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CHAN_FREQ_MEM_FORCE` reader - "]
pub type CHAN_FREQ_MEM_FORCE_R = crate::BitReader;
#[doc = "Field `CHAN_FREQ_MEM_FORCE` writer - "]
pub type CHAN_FREQ_MEM_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dc_mem_mode(&self) -> DC_MEM_MODE_R {
        DC_MEM_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dc_mem_force(&self) -> DC_MEM_FORCE_R {
        DC_MEM_FORCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn agc_mem_mode(&self) -> AGC_MEM_MODE_R {
        AGC_MEM_MODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn agc_mem_force(&self) -> AGC_MEM_FORCE_R {
        AGC_MEM_FORCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pbus_mem_mode(&self) -> PBUS_MEM_MODE_R {
        PBUS_MEM_MODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pbus_mem_force(&self) -> PBUS_MEM_FORCE_R {
        PBUS_MEM_FORCE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn bc_mem_mode(&self) -> BC_MEM_MODE_R {
        BC_MEM_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn bc_mem_force(&self) -> BC_MEM_FORCE_R {
        BC_MEM_FORCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn i2c_mst_mem_mode(&self) -> I2C_MST_MEM_MODE_R {
        I2C_MST_MEM_MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn i2c_mst_mem_force(&self) -> I2C_MST_MEM_FORCE_R {
        I2C_MST_MEM_FORCE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn chan_freq_mem_mode(&self) -> CHAN_FREQ_MEM_MODE_R {
        CHAN_FREQ_MEM_MODE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn chan_freq_mem_force(&self) -> CHAN_FREQ_MEM_FORCE_R {
        CHAN_FREQ_MEM_FORCE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CONF")
            .field("dc_mem_mode", &self.dc_mem_mode())
            .field("dc_mem_force", &self.dc_mem_force())
            .field("agc_mem_mode", &self.agc_mem_mode())
            .field("agc_mem_force", &self.agc_mem_force())
            .field("pbus_mem_mode", &self.pbus_mem_mode())
            .field("pbus_mem_force", &self.pbus_mem_force())
            .field("bc_mem_mode", &self.bc_mem_mode())
            .field("bc_mem_force", &self.bc_mem_force())
            .field("i2c_mst_mem_mode", &self.i2c_mst_mem_mode())
            .field("i2c_mst_mem_force", &self.i2c_mst_mem_force())
            .field("chan_freq_mem_mode", &self.chan_freq_mem_mode())
            .field("chan_freq_mem_force", &self.chan_freq_mem_force())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dc_mem_mode(&mut self) -> DC_MEM_MODE_W<'_, MEM_CONF_SPEC> {
        DC_MEM_MODE_W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dc_mem_force(&mut self) -> DC_MEM_FORCE_W<'_, MEM_CONF_SPEC> {
        DC_MEM_FORCE_W::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn agc_mem_mode(&mut self) -> AGC_MEM_MODE_W<'_, MEM_CONF_SPEC> {
        AGC_MEM_MODE_W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn agc_mem_force(&mut self) -> AGC_MEM_FORCE_W<'_, MEM_CONF_SPEC> {
        AGC_MEM_FORCE_W::new(self, 7)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pbus_mem_mode(&mut self) -> PBUS_MEM_MODE_W<'_, MEM_CONF_SPEC> {
        PBUS_MEM_MODE_W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pbus_mem_force(&mut self) -> PBUS_MEM_FORCE_W<'_, MEM_CONF_SPEC> {
        PBUS_MEM_FORCE_W::new(self, 11)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn bc_mem_mode(&mut self) -> BC_MEM_MODE_W<'_, MEM_CONF_SPEC> {
        BC_MEM_MODE_W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn bc_mem_force(&mut self) -> BC_MEM_FORCE_W<'_, MEM_CONF_SPEC> {
        BC_MEM_FORCE_W::new(self, 15)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn i2c_mst_mem_mode(&mut self) -> I2C_MST_MEM_MODE_W<'_, MEM_CONF_SPEC> {
        I2C_MST_MEM_MODE_W::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn i2c_mst_mem_force(&mut self) -> I2C_MST_MEM_FORCE_W<'_, MEM_CONF_SPEC> {
        I2C_MST_MEM_FORCE_W::new(self, 19)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn chan_freq_mem_mode(&mut self) -> CHAN_FREQ_MEM_MODE_W<'_, MEM_CONF_SPEC> {
        CHAN_FREQ_MEM_MODE_W::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn chan_freq_mem_force(&mut self) -> CHAN_FREQ_MEM_FORCE_W<'_, MEM_CONF_SPEC> {
        CHAN_FREQ_MEM_FORCE_W::new(self, 23)
    }
}
#[doc = "MEM_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_conf::R`](R) reader structure"]
impl crate::Readable for MEM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_conf::W`](W) writer structure"]
impl crate::Writable for MEM_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CONF to value 0x0088_8888"]
impl crate::Resettable for MEM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0088_8888;
}
