#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub type R = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub type W = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Field `FUNC_OUT_SEL` reader - The value of the bits: 0<=s<=256. Set the value to select output signal. s=0-127: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=128: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
pub type FUNC_OUT_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `FUNC_OUT_SEL` writer - The value of the bits: 0<=s<=256. Set the value to select output signal. s=0-127: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=128: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
pub type FUNC_OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `FUNC_OUT_INV_SEL` reader - set this bit to invert output signal.1:invert.0:not invert."]
pub type FUNC_OUT_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC_OUT_INV_SEL` writer - set this bit to invert output signal.1:invert.0:not invert."]
pub type FUNC_OUT_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUNC_OE_SEL` reader - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
pub type FUNC_OE_SEL_R = crate::BitReader;
#[doc = "Field `FUNC_OE_SEL` writer - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
pub type FUNC_OE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUNC_OE_INV_SEL` reader - set this bit to invert output enable signal.1:invert.0:not invert."]
pub type FUNC_OE_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC_OE_INV_SEL` writer - set this bit to invert output enable signal.1:invert.0:not invert."]
pub type FUNC_OE_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - The value of the bits: 0<=s<=256. Set the value to select output signal. s=0-127: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=128: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
    #[inline(always)]
    pub fn func_out_sel(&self) -> FUNC_OUT_SEL_R {
        FUNC_OUT_SEL_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - set this bit to invert output signal.1:invert.0:not invert."]
    #[inline(always)]
    pub fn func_out_inv_sel(&self) -> FUNC_OUT_INV_SEL_R {
        FUNC_OUT_INV_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
    #[inline(always)]
    pub fn func_oe_sel(&self) -> FUNC_OE_SEL_R {
        FUNC_OE_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - set this bit to invert output enable signal.1:invert.0:not invert."]
    #[inline(always)]
    pub fn func_oe_inv_sel(&self) -> FUNC_OE_INV_SEL_R {
        FUNC_OE_INV_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_OUT_SEL_CFG")
            .field("func_out_sel", &self.func_out_sel())
            .field("func_out_inv_sel", &self.func_out_inv_sel())
            .field("func_oe_sel", &self.func_oe_sel())
            .field("func_oe_inv_sel", &self.func_oe_inv_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - The value of the bits: 0<=s<=256. Set the value to select output signal. s=0-127: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=128: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
    #[inline(always)]
    pub fn func_out_sel(&mut self) -> FUNC_OUT_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        FUNC_OUT_SEL_W::new(self, 0)
    }
    #[doc = "Bit 9 - set this bit to invert output signal.1:invert.0:not invert."]
    #[inline(always)]
    pub fn func_out_inv_sel(&mut self) -> FUNC_OUT_INV_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        FUNC_OUT_INV_SEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
    #[inline(always)]
    pub fn func_oe_sel(&mut self) -> FUNC_OE_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        FUNC_OE_SEL_W::new(self, 10)
    }
    #[doc = "Bit 11 - set this bit to invert output enable signal.1:invert.0:not invert."]
    #[inline(always)]
    pub fn func_oe_inv_sel(&mut self) -> FUNC_OE_INV_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        FUNC_OE_INV_SEL_W::new(self, 11)
    }
}
#[doc = "GPIO output function select register\n\nYou can [`read`](crate::Reg::read) this register and get [`func_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
