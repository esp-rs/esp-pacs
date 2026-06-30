#[doc = "Register `HP_CACHE_PWR_CTRL` reader"]
pub type R = crate::R<HP_CACHE_PWR_CTRL_SPEC>;
#[doc = "Register `HP_CACHE_PWR_CTRL` writer"]
pub type W = crate::W<HP_CACHE_PWR_CTRL_SPEC>;
#[doc = "Field `HP_REG_AC_MEM_FO` reader - need_des"]
pub type HP_REG_AC_MEM_FO_R = crate::FieldReader;
#[doc = "Field `HP_REG_AC_MEM_FO` writer - need_des"]
pub type HP_REG_AC_MEM_FO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HP_REG_IC_MEM_FO` reader - need_des"]
pub type HP_REG_IC_MEM_FO_R = crate::FieldReader;
#[doc = "Field `HP_REG_IC_MEM_FO` writer - need_des"]
pub type HP_REG_IC_MEM_FO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HP_REG_DC_MEM_FO` reader - need_des"]
pub type HP_REG_DC_MEM_FO_R = crate::FieldReader;
#[doc = "Field `HP_REG_DC_MEM_FO` writer - need_des"]
pub type HP_REG_DC_MEM_FO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn hp_reg_ac_mem_fo(&self) -> HP_REG_AC_MEM_FO_R {
        HP_REG_AC_MEM_FO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn hp_reg_ic_mem_fo(&self) -> HP_REG_IC_MEM_FO_R {
        HP_REG_IC_MEM_FO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn hp_reg_dc_mem_fo(&self) -> HP_REG_DC_MEM_FO_R {
        HP_REG_DC_MEM_FO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CACHE_PWR_CTRL")
            .field("hp_reg_ac_mem_fo", &self.hp_reg_ac_mem_fo())
            .field("hp_reg_ic_mem_fo", &self.hp_reg_ic_mem_fo())
            .field("hp_reg_dc_mem_fo", &self.hp_reg_dc_mem_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn hp_reg_ac_mem_fo(&mut self) -> HP_REG_AC_MEM_FO_W<'_, HP_CACHE_PWR_CTRL_SPEC> {
        HP_REG_AC_MEM_FO_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn hp_reg_ic_mem_fo(&mut self) -> HP_REG_IC_MEM_FO_W<'_, HP_CACHE_PWR_CTRL_SPEC> {
        HP_REG_IC_MEM_FO_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn hp_reg_dc_mem_fo(&mut self) -> HP_REG_DC_MEM_FO_W<'_, HP_CACHE_PWR_CTRL_SPEC> {
        HP_REG_DC_MEM_FO_W::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cache_pwr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cache_pwr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CACHE_PWR_CTRL_SPEC;
impl crate::RegisterSpec for HP_CACHE_PWR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_cache_pwr_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_CACHE_PWR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_cache_pwr_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_CACHE_PWR_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_CACHE_PWR_CTRL to value 0"]
impl crate::Resettable for HP_CACHE_PWR_CTRL_SPEC {}
