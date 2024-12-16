#[doc = "Register `RETENTION_CTRL2` reader"]
pub type R = crate::R<RETENTION_CTRL2_SPEC>;
#[doc = "Register `RETENTION_CTRL2` writer"]
pub type W = crate::W<RETENTION_CTRL2_SPEC>;
#[doc = "Field `RET_ICACHE_SIZE` reader - ******* Description ***********"]
pub type RET_ICACHE_SIZE_R = crate::FieldReader;
#[doc = "Field `RET_ICACHE_SIZE` writer - ******* Description ***********"]
pub type RET_ICACHE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RET_ICACHE_VLD_SIZE` reader - ******* Description ***********"]
pub type RET_ICACHE_VLD_SIZE_R = crate::FieldReader;
#[doc = "Field `RET_ICACHE_VLD_SIZE` writer - ******* Description ***********"]
pub type RET_ICACHE_VLD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RET_ICACHE_START_POINT` reader - ******* Description ***********"]
pub type RET_ICACHE_START_POINT_R = crate::FieldReader;
#[doc = "Field `RET_ICACHE_START_POINT` writer - ******* Description ***********"]
pub type RET_ICACHE_START_POINT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RET_ICACHE_ENABLE` reader - ******* Description ***********"]
pub type RET_ICACHE_ENABLE_R = crate::BitReader;
#[doc = "Field `RET_ICACHE_ENABLE` writer - ******* Description ***********"]
pub type RET_ICACHE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:11 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_icache_size(&self) -> RET_ICACHE_SIZE_R {
        RET_ICACHE_SIZE_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 13:20 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_icache_vld_size(&self) -> RET_ICACHE_VLD_SIZE_R {
        RET_ICACHE_VLD_SIZE_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 22:29 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_icache_start_point(&self) -> RET_ICACHE_START_POINT_R {
        RET_ICACHE_START_POINT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bit 31 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_icache_enable(&self) -> RET_ICACHE_ENABLE_R {
        RET_ICACHE_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL2")
            .field("ret_icache_size", &self.ret_icache_size())
            .field("ret_icache_vld_size", &self.ret_icache_vld_size())
            .field("ret_icache_start_point", &self.ret_icache_start_point())
            .field("ret_icache_enable", &self.ret_icache_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:11 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_icache_size(&mut self) -> RET_ICACHE_SIZE_W<RETENTION_CTRL2_SPEC> {
        RET_ICACHE_SIZE_W::new(self, 4)
    }
    #[doc = "Bits 13:20 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_icache_vld_size(&mut self) -> RET_ICACHE_VLD_SIZE_W<RETENTION_CTRL2_SPEC> {
        RET_ICACHE_VLD_SIZE_W::new(self, 13)
    }
    #[doc = "Bits 22:29 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_icache_start_point(&mut self) -> RET_ICACHE_START_POINT_W<RETENTION_CTRL2_SPEC> {
        RET_ICACHE_START_POINT_W::new(self, 22)
    }
    #[doc = "Bit 31 - ******* Description ***********"]
    #[inline(always)]
    pub fn ret_icache_enable(&mut self) -> RET_ICACHE_ENABLE_W<RETENTION_CTRL2_SPEC> {
        RET_ICACHE_ENABLE_W::new(self, 31)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`retention_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`retention_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RETENTION_CTRL2_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retention_ctrl2::R`](R) reader structure"]
impl crate::Readable for RETENTION_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`retention_ctrl2::W`](W) writer structure"]
impl crate::Writable for RETENTION_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RETENTION_CTRL2 to value 0x001f_eff0"]
impl crate::Resettable for RETENTION_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x001f_eff0;
}
