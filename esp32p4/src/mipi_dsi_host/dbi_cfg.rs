///Register `DBI_CFG` reader
pub type R = crate::R<DBI_CFG_SPEC>;
///Register `DBI_CFG` writer
pub type W = crate::W<DBI_CFG_SPEC>;
///Field `IN_DBI_CONF` reader - NA
pub type IN_DBI_CONF_R = crate::FieldReader;
///Field `IN_DBI_CONF` writer - NA
pub type IN_DBI_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OUT_DBI_CONF` reader - NA
pub type OUT_DBI_CONF_R = crate::FieldReader;
///Field `OUT_DBI_CONF` writer - NA
pub type OUT_DBI_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LUT_SIZE_CONF` reader - NA
pub type LUT_SIZE_CONF_R = crate::FieldReader;
///Field `LUT_SIZE_CONF` writer - NA
pub type LUT_SIZE_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - NA
    #[inline(always)]
    pub fn in_dbi_conf(&self) -> IN_DBI_CONF_R {
        IN_DBI_CONF_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - NA
    #[inline(always)]
    pub fn out_dbi_conf(&self) -> OUT_DBI_CONF_R {
        OUT_DBI_CONF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:17 - NA
    #[inline(always)]
    pub fn lut_size_conf(&self) -> LUT_SIZE_CONF_R {
        LUT_SIZE_CONF_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBI_CFG")
            .field("in_dbi_conf", &self.in_dbi_conf())
            .field("out_dbi_conf", &self.out_dbi_conf())
            .field("lut_size_conf", &self.lut_size_conf())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - NA
    #[inline(always)]
    #[must_use]
    pub fn in_dbi_conf(&mut self) -> IN_DBI_CONF_W<DBI_CFG_SPEC> {
        IN_DBI_CONF_W::new(self, 0)
    }
    ///Bits 8:11 - NA
    #[inline(always)]
    #[must_use]
    pub fn out_dbi_conf(&mut self) -> OUT_DBI_CONF_W<DBI_CFG_SPEC> {
        OUT_DBI_CONF_W::new(self, 8)
    }
    ///Bits 16:17 - NA
    #[inline(always)]
    #[must_use]
    pub fn lut_size_conf(&mut self) -> LUT_SIZE_CONF_W<DBI_CFG_SPEC> {
        LUT_SIZE_CONF_W::new(self, 16)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`dbi_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DBI_CFG_SPEC;
impl crate::RegisterSpec for DBI_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dbi_cfg::R`](R) reader structure
impl crate::Readable for DBI_CFG_SPEC {}
///`write(|w| ..)` method takes [`dbi_cfg::W`](W) writer structure
impl crate::Writable for DBI_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBI_CFG to value 0
impl crate::Resettable for DBI_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
