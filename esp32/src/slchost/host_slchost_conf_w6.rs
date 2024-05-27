///Register `HOST_SLCHOST_CONF_W6` reader
pub type R = crate::R<HOST_SLCHOST_CONF_W6_SPEC>;
///Register `HOST_SLCHOST_CONF_W6` writer
pub type W = crate::W<HOST_SLCHOST_CONF_W6_SPEC>;
///Field `HOST_SLCHOST_CONF24` reader -
pub type HOST_SLCHOST_CONF24_R = crate::FieldReader;
///Field `HOST_SLCHOST_CONF24` writer -
pub type HOST_SLCHOST_CONF24_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HOST_SLCHOST_CONF25` reader -
pub type HOST_SLCHOST_CONF25_R = crate::FieldReader;
///Field `HOST_SLCHOST_CONF25` writer -
pub type HOST_SLCHOST_CONF25_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HOST_SLCHOST_CONF26` reader -
pub type HOST_SLCHOST_CONF26_R = crate::FieldReader;
///Field `HOST_SLCHOST_CONF26` writer -
pub type HOST_SLCHOST_CONF26_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HOST_SLCHOST_CONF27` reader -
pub type HOST_SLCHOST_CONF27_R = crate::FieldReader;
///Field `HOST_SLCHOST_CONF27` writer -
pub type HOST_SLCHOST_CONF27_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn host_slchost_conf24(&self) -> HOST_SLCHOST_CONF24_R {
        HOST_SLCHOST_CONF24_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn host_slchost_conf25(&self) -> HOST_SLCHOST_CONF25_R {
        HOST_SLCHOST_CONF25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23
    #[inline(always)]
    pub fn host_slchost_conf26(&self) -> HOST_SLCHOST_CONF26_R {
        HOST_SLCHOST_CONF26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn host_slchost_conf27(&self) -> HOST_SLCHOST_CONF27_R {
        HOST_SLCHOST_CONF27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W6")
            .field("host_slchost_conf24", &self.host_slchost_conf24())
            .field("host_slchost_conf25", &self.host_slchost_conf25())
            .field("host_slchost_conf26", &self.host_slchost_conf26())
            .field("host_slchost_conf27", &self.host_slchost_conf27())
            .finish()
    }
}
impl W {
    ///Bits 0:7
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf24(&mut self) -> HOST_SLCHOST_CONF24_W<HOST_SLCHOST_CONF_W6_SPEC> {
        HOST_SLCHOST_CONF24_W::new(self, 0)
    }
    ///Bits 8:15
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf25(&mut self) -> HOST_SLCHOST_CONF25_W<HOST_SLCHOST_CONF_W6_SPEC> {
        HOST_SLCHOST_CONF25_W::new(self, 8)
    }
    ///Bits 16:23
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf26(&mut self) -> HOST_SLCHOST_CONF26_W<HOST_SLCHOST_CONF_W6_SPEC> {
        HOST_SLCHOST_CONF26_W::new(self, 16)
    }
    ///Bits 24:31
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf27(&mut self) -> HOST_SLCHOST_CONF27_W<HOST_SLCHOST_CONF_W6_SPEC> {
        HOST_SLCHOST_CONF27_W::new(self, 24)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLCHOST_CONF_W6_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_slchost_conf_w6::R`](R) reader structure
impl crate::Readable for HOST_SLCHOST_CONF_W6_SPEC {}
///`write(|w| ..)` method takes [`host_slchost_conf_w6::W`](W) writer structure
impl crate::Writable for HOST_SLCHOST_CONF_W6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HOST_SLCHOST_CONF_W6 to value 0
impl crate::Resettable for HOST_SLCHOST_CONF_W6_SPEC {
    const RESET_VALUE: u32 = 0;
}
