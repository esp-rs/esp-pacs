///Register `CACHE_SOURCE_1` reader
pub type R = crate::R<CACHE_SOURCE_1_SPEC>;
///Register `CACHE_SOURCE_1` writer
pub type W = crate::W<CACHE_SOURCE_1_SPEC>;
///Field `PRO_CACHE_I_SOURCE_PRO_IRAM1` reader - xx
pub type PRO_CACHE_I_SOURCE_PRO_IRAM1_R = crate::BitReader;
///Field `PRO_CACHE_I_SOURCE_PRO_IRAM1` writer - xx
pub type PRO_CACHE_I_SOURCE_PRO_IRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_I_SOURCE_PRO_IROM0` reader - xx
pub type PRO_CACHE_I_SOURCE_PRO_IROM0_R = crate::BitReader;
///Field `PRO_CACHE_I_SOURCE_PRO_IROM0` writer - xx
pub type PRO_CACHE_I_SOURCE_PRO_IROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_I_SOURCE_PRO_DROM0` reader - xx
pub type PRO_CACHE_I_SOURCE_PRO_DROM0_R = crate::BitReader;
///Field `PRO_CACHE_I_SOURCE_PRO_DROM0` writer - xx
pub type PRO_CACHE_I_SOURCE_PRO_DROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_D_SOURCE_PRO_DRAM0` reader - xx
pub type PRO_CACHE_D_SOURCE_PRO_DRAM0_R = crate::BitReader;
///Field `PRO_CACHE_D_SOURCE_PRO_DRAM0` writer - xx
pub type PRO_CACHE_D_SOURCE_PRO_DRAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_D_SOURCE_PRO_DPORT` reader - xx
pub type PRO_CACHE_D_SOURCE_PRO_DPORT_R = crate::BitReader;
///Field `PRO_CACHE_D_SOURCE_PRO_DPORT` writer - xx
pub type PRO_CACHE_D_SOURCE_PRO_DPORT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_D_SOURCE_PRO_DROM0` reader - xx
pub type PRO_CACHE_D_SOURCE_PRO_DROM0_R = crate::BitReader;
///Field `PRO_CACHE_D_SOURCE_PRO_DROM0` writer - xx
pub type PRO_CACHE_D_SOURCE_PRO_DROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - xx
    #[inline(always)]
    pub fn pro_cache_i_source_pro_iram1(&self) -> PRO_CACHE_I_SOURCE_PRO_IRAM1_R {
        PRO_CACHE_I_SOURCE_PRO_IRAM1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - xx
    #[inline(always)]
    pub fn pro_cache_i_source_pro_irom0(&self) -> PRO_CACHE_I_SOURCE_PRO_IROM0_R {
        PRO_CACHE_I_SOURCE_PRO_IROM0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - xx
    #[inline(always)]
    pub fn pro_cache_i_source_pro_drom0(&self) -> PRO_CACHE_I_SOURCE_PRO_DROM0_R {
        PRO_CACHE_I_SOURCE_PRO_DROM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - xx
    #[inline(always)]
    pub fn pro_cache_d_source_pro_dram0(&self) -> PRO_CACHE_D_SOURCE_PRO_DRAM0_R {
        PRO_CACHE_D_SOURCE_PRO_DRAM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - xx
    #[inline(always)]
    pub fn pro_cache_d_source_pro_dport(&self) -> PRO_CACHE_D_SOURCE_PRO_DPORT_R {
        PRO_CACHE_D_SOURCE_PRO_DPORT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - xx
    #[inline(always)]
    pub fn pro_cache_d_source_pro_drom0(&self) -> PRO_CACHE_D_SOURCE_PRO_DROM0_R {
        PRO_CACHE_D_SOURCE_PRO_DROM0_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SOURCE_1")
            .field(
                "pro_cache_i_source_pro_iram1",
                &self.pro_cache_i_source_pro_iram1(),
            )
            .field(
                "pro_cache_i_source_pro_irom0",
                &self.pro_cache_i_source_pro_irom0(),
            )
            .field(
                "pro_cache_i_source_pro_drom0",
                &self.pro_cache_i_source_pro_drom0(),
            )
            .field(
                "pro_cache_d_source_pro_dram0",
                &self.pro_cache_d_source_pro_dram0(),
            )
            .field(
                "pro_cache_d_source_pro_dport",
                &self.pro_cache_d_source_pro_dport(),
            )
            .field(
                "pro_cache_d_source_pro_drom0",
                &self.pro_cache_d_source_pro_drom0(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - xx
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_i_source_pro_iram1(
        &mut self,
    ) -> PRO_CACHE_I_SOURCE_PRO_IRAM1_W<CACHE_SOURCE_1_SPEC> {
        PRO_CACHE_I_SOURCE_PRO_IRAM1_W::new(self, 0)
    }
    ///Bit 1 - xx
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_i_source_pro_irom0(
        &mut self,
    ) -> PRO_CACHE_I_SOURCE_PRO_IROM0_W<CACHE_SOURCE_1_SPEC> {
        PRO_CACHE_I_SOURCE_PRO_IROM0_W::new(self, 1)
    }
    ///Bit 2 - xx
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_i_source_pro_drom0(
        &mut self,
    ) -> PRO_CACHE_I_SOURCE_PRO_DROM0_W<CACHE_SOURCE_1_SPEC> {
        PRO_CACHE_I_SOURCE_PRO_DROM0_W::new(self, 2)
    }
    ///Bit 3 - xx
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_d_source_pro_dram0(
        &mut self,
    ) -> PRO_CACHE_D_SOURCE_PRO_DRAM0_W<CACHE_SOURCE_1_SPEC> {
        PRO_CACHE_D_SOURCE_PRO_DRAM0_W::new(self, 3)
    }
    ///Bit 4 - xx
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_d_source_pro_dport(
        &mut self,
    ) -> PRO_CACHE_D_SOURCE_PRO_DPORT_W<CACHE_SOURCE_1_SPEC> {
        PRO_CACHE_D_SOURCE_PRO_DPORT_W::new(self, 4)
    }
    ///Bit 5 - xx
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_d_source_pro_drom0(
        &mut self,
    ) -> PRO_CACHE_D_SOURCE_PRO_DROM0_W<CACHE_SOURCE_1_SPEC> {
        PRO_CACHE_D_SOURCE_PRO_DROM0_W::new(self, 5)
    }
}
/**Cache access permission control register 1.

You can [`read`](crate::generic::Reg::read) this register and get [`cache_source_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_source_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_SOURCE_1_SPEC;
impl crate::RegisterSpec for CACHE_SOURCE_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_source_1::R`](R) reader structure
impl crate::Readable for CACHE_SOURCE_1_SPEC {}
///`write(|w| ..)` method takes [`cache_source_1::W`](W) writer structure
impl crate::Writable for CACHE_SOURCE_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_SOURCE_1 to value 0
impl crate::Resettable for CACHE_SOURCE_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
