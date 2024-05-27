///Register `HOST_SLC0HOST_LEN_WD` reader
pub type R = crate::R<HOST_SLC0HOST_LEN_WD_SPEC>;
///Register `HOST_SLC0HOST_LEN_WD` writer
pub type W = crate::W<HOST_SLC0HOST_LEN_WD_SPEC>;
///Field `HOST_SLC0HOST_LEN_WD` reader -
pub type HOST_SLC0HOST_LEN_WD_R = crate::FieldReader<u32>;
///Field `HOST_SLC0HOST_LEN_WD` writer -
pub type HOST_SLC0HOST_LEN_WD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn host_slc0host_len_wd(&self) -> HOST_SLC0HOST_LEN_WD_R {
        HOST_SLC0HOST_LEN_WD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC0HOST_LEN_WD")
            .field("host_slc0host_len_wd", &self.host_slc0host_len_wd())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_len_wd(&mut self) -> HOST_SLC0HOST_LEN_WD_W<HOST_SLC0HOST_LEN_WD_SPEC> {
        HOST_SLC0HOST_LEN_WD_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_len_wd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc0host_len_wd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLC0HOST_LEN_WD_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_LEN_WD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_slc0host_len_wd::R`](R) reader structure
impl crate::Readable for HOST_SLC0HOST_LEN_WD_SPEC {}
///`write(|w| ..)` method takes [`host_slc0host_len_wd::W`](W) writer structure
impl crate::Writable for HOST_SLC0HOST_LEN_WD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HOST_SLC0HOST_LEN_WD to value 0
impl crate::Resettable for HOST_SLC0HOST_LEN_WD_SPEC {
    const RESET_VALUE: u32 = 0;
}
