#[doc = "Register `FLASH_SUS_CTRL` reader"]
pub type R = crate::R<FLASH_SUS_CTRL_SPEC>;
#[doc = "Register `FLASH_SUS_CTRL` writer"]
pub type W = crate::W<FLASH_SUS_CTRL_SPEC>;
#[doc = "Field `FLASH_PES_EN` reader - Set this bit to enable auto-suspend function."]
pub type FLASH_PES_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PES_EN` writer - Set this bit to enable auto-suspend function."]
pub type FLASH_PES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PER_COMMAND` reader - Program/Erase resume command value."]
pub type FLASH_PER_COMMAND_R = crate::FieldReader;
#[doc = "Field `FLASH_PER_COMMAND` writer - Program/Erase resume command value."]
pub type FLASH_PER_COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FLASH_PES_COMMAND` reader - Program/Erase suspend command value."]
pub type FLASH_PES_COMMAND_R = crate::FieldReader;
#[doc = "Field `FLASH_PES_COMMAND` writer - Program/Erase suspend command value."]
pub type FLASH_PES_COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable auto-suspend function."]
    #[inline(always)]
    pub fn flash_pes_en(&self) -> FLASH_PES_EN_R {
        FLASH_PES_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Program/Erase resume command value."]
    #[inline(always)]
    pub fn flash_per_command(&self) -> FLASH_PER_COMMAND_R {
        FLASH_PER_COMMAND_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - Program/Erase suspend command value."]
    #[inline(always)]
    pub fn flash_pes_command(&self) -> FLASH_PES_COMMAND_R {
        FLASH_PES_COMMAND_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_SUS_CTRL")
            .field(
                "flash_pes_en",
                &format_args!("{}", self.flash_pes_en().bit()),
            )
            .field(
                "flash_per_command",
                &format_args!("{}", self.flash_per_command().bits()),
            )
            .field(
                "flash_pes_command",
                &format_args!("{}", self.flash_pes_command().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FLASH_SUS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable auto-suspend function."]
    #[inline(always)]
    #[must_use]
    pub fn flash_pes_en(&mut self) -> FLASH_PES_EN_W<FLASH_SUS_CTRL_SPEC> {
        FLASH_PES_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:8 - Program/Erase resume command value."]
    #[inline(always)]
    #[must_use]
    pub fn flash_per_command(&mut self) -> FLASH_PER_COMMAND_W<FLASH_SUS_CTRL_SPEC> {
        FLASH_PER_COMMAND_W::new(self, 1)
    }
    #[doc = "Bits 9:16 - Program/Erase suspend command value."]
    #[inline(always)]
    #[must_use]
    pub fn flash_pes_command(&mut self) -> FLASH_PES_COMMAND_W<FLASH_SUS_CTRL_SPEC> {
        FLASH_PES_COMMAND_W::new(self, 9)
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
#[doc = "SPI1 flash suspend command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sus_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sus_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SUS_CTRL_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_sus_ctrl::R`](R) reader structure"]
impl crate::Readable for FLASH_SUS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_sus_ctrl::W`](W) writer structure"]
impl crate::Writable for FLASH_SUS_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_SUS_CTRL to value 0xeaf4"]
impl crate::Resettable for FLASH_SUS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xeaf4;
}
