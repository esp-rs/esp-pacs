#[doc = "Register `ROM_MPU_TABLE0` reader"]
pub type R = crate::R<ROM_MPU_TABLE0_SPEC>;
#[doc = "Register `ROM_MPU_TABLE0` writer"]
pub type W = crate::W<ROM_MPU_TABLE0_SPEC>;
#[doc = "Field `ROM_MPU_TABLE0` reader - "]
pub type ROM_MPU_TABLE0_R = crate::FieldReader;
#[doc = "Field `ROM_MPU_TABLE0` writer - "]
pub type ROM_MPU_TABLE0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_mpu_table0(&self) -> ROM_MPU_TABLE0_R {
        ROM_MPU_TABLE0_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_MPU_TABLE0")
            .field(
                "rom_mpu_table0",
                &format_args!("{}", self.rom_mpu_table0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ROM_MPU_TABLE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn rom_mpu_table0(&mut self) -> ROM_MPU_TABLE0_W<ROM_MPU_TABLE0_SPEC> {
        ROM_MPU_TABLE0_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_mpu_table0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_mpu_table0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_MPU_TABLE0_SPEC;
impl crate::RegisterSpec for ROM_MPU_TABLE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_mpu_table0::R`](R) reader structure"]
impl crate::Readable for ROM_MPU_TABLE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_mpu_table0::W`](W) writer structure"]
impl crate::Writable for ROM_MPU_TABLE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROM_MPU_TABLE0 to value 0x01"]
impl crate::Resettable for ROM_MPU_TABLE0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
