#[doc = "Register `ROM_CTRL_0` reader"]
pub type R = crate::R<ROM_CTRL_0_SPEC>;
#[doc = "Register `ROM_CTRL_0` writer"]
pub type W = crate::W<ROM_CTRL_0_SPEC>;
#[doc = "Field `ROM_FO` reader - This field is used to force on clock gate of internal ROM."]
pub type ROM_FO_R = crate::FieldReader;
#[doc = "Field `ROM_FO` writer - This field is used to force on clock gate of internal ROM."]
pub type ROM_FO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - This field is used to force on clock gate of internal ROM."]
    #[inline(always)]
    pub fn rom_fo(&self) -> ROM_FO_R {
        ROM_FO_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_CTRL_0")
            .field("rom_fo", &self.rom_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to force on clock gate of internal ROM."]
    #[inline(always)]
    pub fn rom_fo(&mut self) -> ROM_FO_W<ROM_CTRL_0_SPEC> {
        ROM_FO_W::new(self, 0)
    }
}
#[doc = "System ROM configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_CTRL_0_SPEC;
impl crate::RegisterSpec for ROM_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_ctrl_0::R`](R) reader structure"]
impl crate::Readable for ROM_CTRL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_ctrl_0::W`](W) writer structure"]
impl crate::Writable for ROM_CTRL_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROM_CTRL_0 to value 0x03"]
impl crate::Resettable for ROM_CTRL_0_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
