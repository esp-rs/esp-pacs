///Register `L1_ICACHE0_PRELOCK_SCT_SIZE` reader
pub type R = crate::R<L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC>;
///Register `L1_ICACHE0_PRELOCK_SCT_SIZE` writer
pub type W = crate::W<L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC>;
///Field `L1_ICACHE0_PRELOCK_SCT0_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT0_ADDR_REG
pub type L1_ICACHE0_PRELOCK_SCT0_SIZE_R = crate::FieldReader<u16>;
///Field `L1_ICACHE0_PRELOCK_SCT0_SIZE` writer - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT0_ADDR_REG
pub type L1_ICACHE0_PRELOCK_SCT0_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `L1_ICACHE0_PRELOCK_SCT1_SIZE` reader - Those bits are used to configure the size of the second section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT1_ADDR_REG
pub type L1_ICACHE0_PRELOCK_SCT1_SIZE_R = crate::FieldReader<u16>;
///Field `L1_ICACHE0_PRELOCK_SCT1_SIZE` writer - Those bits are used to configure the size of the second section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT1_ADDR_REG
pub type L1_ICACHE0_PRELOCK_SCT1_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT0_ADDR_REG
    #[inline(always)]
    pub fn l1_icache0_prelock_sct0_size(&self) -> L1_ICACHE0_PRELOCK_SCT0_SIZE_R {
        L1_ICACHE0_PRELOCK_SCT0_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 16:29 - Those bits are used to configure the size of the second section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT1_ADDR_REG
    #[inline(always)]
    pub fn l1_icache0_prelock_sct1_size(&self) -> L1_ICACHE0_PRELOCK_SCT1_SIZE_R {
        L1_ICACHE0_PRELOCK_SCT1_SIZE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE0_PRELOCK_SCT_SIZE")
            .field(
                "l1_icache0_prelock_sct0_size",
                &self.l1_icache0_prelock_sct0_size(),
            )
            .field(
                "l1_icache0_prelock_sct1_size",
                &self.l1_icache0_prelock_sct1_size(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT0_ADDR_REG
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_prelock_sct0_size(
        &mut self,
    ) -> L1_ICACHE0_PRELOCK_SCT0_SIZE_W<L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC> {
        L1_ICACHE0_PRELOCK_SCT0_SIZE_W::new(self, 0)
    }
    ///Bits 16:29 - Those bits are used to configure the size of the second section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT1_ADDR_REG
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_prelock_sct1_size(
        &mut self,
    ) -> L1_ICACHE0_PRELOCK_SCT1_SIZE_W<L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC> {
        L1_ICACHE0_PRELOCK_SCT1_SIZE_W::new(self, 16)
    }
}
/**L1 instruction Cache 0 prelock section size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_prelock_sct_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_prelock_sct_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC;
impl crate::RegisterSpec for L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_icache0_prelock_sct_size::R`](R) reader structure
impl crate::Readable for L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC {}
///`write(|w| ..)` method takes [`l1_icache0_prelock_sct_size::W`](W) writer structure
impl crate::Writable for L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_ICACHE0_PRELOCK_SCT_SIZE to value 0x3fff_3fff
impl crate::Resettable for L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x3fff_3fff;
}
