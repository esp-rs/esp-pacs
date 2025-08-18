#[doc = "Register `SCL_EXT_LOW_TIME` reader"]
pub type R = crate::R<SCL_EXT_LOW_TIME_SPEC>;
#[doc = "Register `SCL_EXT_LOW_TIME` writer"]
pub type W = crate::W<SCL_EXT_LOW_TIME_SPEC>;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD1` reader - NA"]
pub type REG_I3C_MST_EXT_LOW_PERIOD1_R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD1` writer - NA"]
pub type REG_I3C_MST_EXT_LOW_PERIOD1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD2` reader - NA"]
pub type REG_I3C_MST_EXT_LOW_PERIOD2_R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD2` writer - NA"]
pub type REG_I3C_MST_EXT_LOW_PERIOD2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD3` reader - NA"]
pub type REG_I3C_MST_EXT_LOW_PERIOD3_R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD3` writer - NA"]
pub type REG_I3C_MST_EXT_LOW_PERIOD3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD4` reader - NA"]
pub type REG_I3C_MST_EXT_LOW_PERIOD4_R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_EXT_LOW_PERIOD4` writer - NA"]
pub type REG_I3C_MST_EXT_LOW_PERIOD4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period1(&self) -> REG_I3C_MST_EXT_LOW_PERIOD1_R {
        REG_I3C_MST_EXT_LOW_PERIOD1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period2(&self) -> REG_I3C_MST_EXT_LOW_PERIOD2_R {
        REG_I3C_MST_EXT_LOW_PERIOD2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period3(&self) -> REG_I3C_MST_EXT_LOW_PERIOD3_R {
        REG_I3C_MST_EXT_LOW_PERIOD3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period4(&self) -> REG_I3C_MST_EXT_LOW_PERIOD4_R {
        REG_I3C_MST_EXT_LOW_PERIOD4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_EXT_LOW_TIME")
            .field(
                "reg_i3c_mst_ext_low_period1",
                &self.reg_i3c_mst_ext_low_period1(),
            )
            .field(
                "reg_i3c_mst_ext_low_period2",
                &self.reg_i3c_mst_ext_low_period2(),
            )
            .field(
                "reg_i3c_mst_ext_low_period3",
                &self.reg_i3c_mst_ext_low_period3(),
            )
            .field(
                "reg_i3c_mst_ext_low_period4",
                &self.reg_i3c_mst_ext_low_period4(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period1(
        &mut self,
    ) -> REG_I3C_MST_EXT_LOW_PERIOD1_W<'_, SCL_EXT_LOW_TIME_SPEC> {
        REG_I3C_MST_EXT_LOW_PERIOD1_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period2(
        &mut self,
    ) -> REG_I3C_MST_EXT_LOW_PERIOD2_W<'_, SCL_EXT_LOW_TIME_SPEC> {
        REG_I3C_MST_EXT_LOW_PERIOD2_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period3(
        &mut self,
    ) -> REG_I3C_MST_EXT_LOW_PERIOD3_W<'_, SCL_EXT_LOW_TIME_SPEC> {
        REG_I3C_MST_EXT_LOW_PERIOD3_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period4(
        &mut self,
    ) -> REG_I3C_MST_EXT_LOW_PERIOD4_W<'_, SCL_EXT_LOW_TIME_SPEC> {
        REG_I3C_MST_EXT_LOW_PERIOD4_W::new(self, 24)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_ext_low_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_ext_low_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_EXT_LOW_TIME_SPEC;
impl crate::RegisterSpec for SCL_EXT_LOW_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_ext_low_time::R`](R) reader structure"]
impl crate::Readable for SCL_EXT_LOW_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_ext_low_time::W`](W) writer structure"]
impl crate::Writable for SCL_EXT_LOW_TIME_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_EXT_LOW_TIME to value 0"]
impl crate::Resettable for SCL_EXT_LOW_TIME_SPEC {}
