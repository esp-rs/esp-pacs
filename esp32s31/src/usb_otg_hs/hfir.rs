#[doc = "Register `HFIR` reader"]
pub type R = crate::R<HFIR_SPEC>;
#[doc = "Register `HFIR` writer"]
pub type W = crate::W<HFIR_SPEC>;
#[doc = "Field `FRINT` reader - Frame Interval (FrInt) The value that the application programs to this field specifies the interval between two consecutive SOFs (FS) or micro- SOFs (HS) or Keep-Alive tokens (HS). This field contains the number of PHY clocks that constitute the required frame interval. The Default value set in this field is for FS operation when the PHY clock frequency is 60 MHz. The application can write a value to this register only after the Port Enable bit of the Host Port Control and Status register (HPRT.PrtEnaPort) has been Set. If no value is programmed, the core calculates the value based on the PHY clock specified in the FS/LS PHY Clock Select field of the Host Configuration register (HCFG.FSLSPclkSel). Do not change the value of this field after the initial configuration. - 125 us * (PHY clock frequency for HS) - 1 ms * (PHY clock frequency for FS/LS)"]
pub type FRINT_R = crate::FieldReader<u16>;
#[doc = "Field `FRINT` writer - Frame Interval (FrInt) The value that the application programs to this field specifies the interval between two consecutive SOFs (FS) or micro- SOFs (HS) or Keep-Alive tokens (HS). This field contains the number of PHY clocks that constitute the required frame interval. The Default value set in this field is for FS operation when the PHY clock frequency is 60 MHz. The application can write a value to this register only after the Port Enable bit of the Host Port Control and Status register (HPRT.PrtEnaPort) has been Set. If no value is programmed, the core calculates the value based on the PHY clock specified in the FS/LS PHY Clock Select field of the Host Configuration register (HCFG.FSLSPclkSel). Do not change the value of this field after the initial configuration. - 125 us * (PHY clock frequency for HS) - 1 ms * (PHY clock frequency for FS/LS)"]
pub type FRINT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HFIRRLDCTRL` reader - Reload Control (HFIRRldCtrl) This bit allows dynamic reloading of the HFIR register during run time. - 1'b0 : The HFIR cannot be reloaded dynamically - 1'b1: the HFIR can be dynamically reloaded during runtime. This bit needs to be programmed during initial configuration and its value must not be changed during runtime."]
pub type HFIRRLDCTRL_R = crate::BitReader;
#[doc = "Field `HFIRRLDCTRL` writer - Reload Control (HFIRRldCtrl) This bit allows dynamic reloading of the HFIR register during run time. - 1'b0 : The HFIR cannot be reloaded dynamically - 1'b1: the HFIR can be dynamically reloaded during runtime. This bit needs to be programmed during initial configuration and its value must not be changed during runtime."]
pub type HFIRRLDCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Frame Interval (FrInt) The value that the application programs to this field specifies the interval between two consecutive SOFs (FS) or micro- SOFs (HS) or Keep-Alive tokens (HS). This field contains the number of PHY clocks that constitute the required frame interval. The Default value set in this field is for FS operation when the PHY clock frequency is 60 MHz. The application can write a value to this register only after the Port Enable bit of the Host Port Control and Status register (HPRT.PrtEnaPort) has been Set. If no value is programmed, the core calculates the value based on the PHY clock specified in the FS/LS PHY Clock Select field of the Host Configuration register (HCFG.FSLSPclkSel). Do not change the value of this field after the initial configuration. - 125 us * (PHY clock frequency for HS) - 1 ms * (PHY clock frequency for FS/LS)"]
    #[inline(always)]
    pub fn frint(&self) -> FRINT_R {
        FRINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Reload Control (HFIRRldCtrl) This bit allows dynamic reloading of the HFIR register during run time. - 1'b0 : The HFIR cannot be reloaded dynamically - 1'b1: the HFIR can be dynamically reloaded during runtime. This bit needs to be programmed during initial configuration and its value must not be changed during runtime."]
    #[inline(always)]
    pub fn hfirrldctrl(&self) -> HFIRRLDCTRL_R {
        HFIRRLDCTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFIR")
            .field("frint", &self.frint())
            .field("hfirrldctrl", &self.hfirrldctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Interval (FrInt) The value that the application programs to this field specifies the interval between two consecutive SOFs (FS) or micro- SOFs (HS) or Keep-Alive tokens (HS). This field contains the number of PHY clocks that constitute the required frame interval. The Default value set in this field is for FS operation when the PHY clock frequency is 60 MHz. The application can write a value to this register only after the Port Enable bit of the Host Port Control and Status register (HPRT.PrtEnaPort) has been Set. If no value is programmed, the core calculates the value based on the PHY clock specified in the FS/LS PHY Clock Select field of the Host Configuration register (HCFG.FSLSPclkSel). Do not change the value of this field after the initial configuration. - 125 us * (PHY clock frequency for HS) - 1 ms * (PHY clock frequency for FS/LS)"]
    #[inline(always)]
    pub fn frint(&mut self) -> FRINT_W<'_, HFIR_SPEC> {
        FRINT_W::new(self, 0)
    }
    #[doc = "Bit 16 - Reload Control (HFIRRldCtrl) This bit allows dynamic reloading of the HFIR register during run time. - 1'b0 : The HFIR cannot be reloaded dynamically - 1'b1: the HFIR can be dynamically reloaded during runtime. This bit needs to be programmed during initial configuration and its value must not be changed during runtime."]
    #[inline(always)]
    pub fn hfirrldctrl(&mut self) -> HFIRRLDCTRL_W<'_, HFIR_SPEC> {
        HFIRRLDCTRL_W::new(self, 16)
    }
}
#[doc = "This register is used to control the interval between two consecutive SOFs.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFIR_SPEC;
impl crate::RegisterSpec for HFIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfir::R`](R) reader structure"]
impl crate::Readable for HFIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfir::W`](W) writer structure"]
impl crate::Writable for HFIR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFIR to value 0xea60"]
impl crate::Resettable for HFIR_SPEC {
    const RESET_VALUE: u32 = 0xea60;
}
