#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub type R = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub type W = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Field `OUT_SEL` reader - The value of the bits: 0&lt;=s&lt;=256. Set the value to select output signal. s=0-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
pub type OUT_SEL_R = crate::FieldReader;
#[doc = "Field `OUT_SEL` writer - The value of the bits: 0&lt;=s&lt;=256. Set the value to select output signal. s=0-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
pub type OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INV_SEL` reader - set this bit to invert output signal.1:invert.0:not invert."]
pub type INV_SEL_R = crate::BitReader;
#[doc = "Field `INV_SEL` writer - set this bit to invert output signal.1:invert.0:not invert."]
pub type INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN_SEL` reader - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
pub type OEN_SEL_R = crate::BitReader;
#[doc = "Field `OEN_SEL` writer - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
pub type OEN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN_INV_SEL` reader - set this bit to invert output enable signal.1:invert.0:not invert."]
pub type OEN_INV_SEL_R = crate::BitReader;
#[doc = "Field `OEN_INV_SEL` writer - set this bit to invert output enable signal.1:invert.0:not invert."]
pub type OEN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - The value of the bits: 0&lt;=s&lt;=256. Set the value to select output signal. s=0-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - set this bit to invert output signal.1:invert.0:not invert."]
    #[inline(always)]
    pub fn inv_sel(&self) -> INV_SEL_R {
        INV_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
    #[inline(always)]
    pub fn oen_sel(&self) -> OEN_SEL_R {
        OEN_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - set this bit to invert output enable signal.1:invert.0:not invert."]
    #[inline(always)]
    pub fn oen_inv_sel(&self) -> OEN_INV_SEL_R {
        OEN_INV_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_OUT_SEL_CFG")
            .field("out_sel", &format_args!("{}", self.out_sel().bits()))
            .field("inv_sel", &format_args!("{}", self.inv_sel().bit()))
            .field("oen_sel", &format_args!("{}", self.oen_sel().bit()))
            .field("oen_inv_sel", &format_args!("{}", self.oen_inv_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FUNC_OUT_SEL_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - The value of the bits: 0&lt;=s&lt;=256. Set the value to select output signal. s=0-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
    #[inline(always)]
    #[must_use]
    pub fn out_sel(&mut self) -> OUT_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        OUT_SEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - set this bit to invert output signal.1:invert.0:not invert."]
    #[inline(always)]
    #[must_use]
    pub fn inv_sel(&mut self) -> INV_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        INV_SEL_W::new(self, 8)
    }
    #[doc = "Bit 9 - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn oen_sel(&mut self) -> OEN_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        OEN_SEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - set this bit to invert output enable signal.1:invert.0:not invert."]
    #[inline(always)]
    #[must_use]
    pub fn oen_inv_sel(&mut self) -> OEN_INV_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        OEN_INV_SEL_W::new(self, 10)
    }
}
#[doc = "GPIO output function select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0x80"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
