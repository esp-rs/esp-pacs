#[doc = "Register `PRO_BOOT_LOCATION_1` reader"]
pub type R = crate::R<PRO_BOOT_LOCATION_1_SPEC>;
#[doc = "Register `PRO_BOOT_LOCATION_1` writer"]
pub type W = crate::W<PRO_BOOT_LOCATION_1_SPEC>;
#[doc = "Field `PRO_BOOT_REMAP` reader - If set to 1, enable boot remap function."]
pub type PRO_BOOT_REMAP_R = crate::BitReader;
#[doc = "Field `PRO_BOOT_REMAP` writer - If set to 1, enable boot remap function."]
pub type PRO_BOOT_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - If set to 1, enable boot remap function."]
    #[inline(always)]
    pub fn pro_boot_remap(&self) -> PRO_BOOT_REMAP_R {
        PRO_BOOT_REMAP_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_BOOT_LOCATION_1")
            .field(
                "pro_boot_remap",
                &format_args!("{}", self.pro_boot_remap().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_BOOT_LOCATION_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - If set to 1, enable boot remap function."]
    #[inline(always)]
    #[must_use]
    pub fn pro_boot_remap(&mut self) -> PRO_BOOT_REMAP_W<PRO_BOOT_LOCATION_1_SPEC, 0> {
        PRO_BOOT_REMAP_W::new(self)
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
#[doc = "Boot permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_boot_location_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_boot_location_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_BOOT_LOCATION_1_SPEC;
impl crate::RegisterSpec for PRO_BOOT_LOCATION_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_boot_location_1::R`](R) reader structure"]
impl crate::Readable for PRO_BOOT_LOCATION_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_boot_location_1::W`](W) writer structure"]
impl crate::Writable for PRO_BOOT_LOCATION_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_BOOT_LOCATION_1 to value 0"]
impl crate::Resettable for PRO_BOOT_LOCATION_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
