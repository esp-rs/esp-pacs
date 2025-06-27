#[doc = "Register `RETENTION_CTRL4` reader"]
pub type R = crate::R<RETENTION_CTRL4_SPEC>;
#[doc = "Register `RETENTION_CTRL4` writer"]
pub type W = crate::W<RETENTION_CTRL4_SPEC>;
#[doc = "Field `RETENTION_INV_CFG` reader - ******* Description ***********"]
pub type RETENTION_INV_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `RETENTION_INV_CFG` writer - ******* Description ***********"]
pub type RETENTION_INV_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_inv_cfg(&self) -> RETENTION_INV_CFG_R {
        RETENTION_INV_CFG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL4")
            .field("retention_inv_cfg", &self.retention_inv_cfg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_inv_cfg(&mut self) -> RETENTION_INV_CFG_W<RETENTION_CTRL4_SPEC> {
        RETENTION_INV_CFG_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`retention_ctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`retention_ctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RETENTION_CTRL4_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retention_ctrl4::R`](R) reader structure"]
impl crate::Readable for RETENTION_CTRL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`retention_ctrl4::W`](W) writer structure"]
impl crate::Writable for RETENTION_CTRL4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RETENTION_CTRL4 to value 0xffff_ffff"]
impl crate::Resettable for RETENTION_CTRL4_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
