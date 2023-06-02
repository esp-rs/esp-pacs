#[doc = "Register `Redundant_ECO_Ctrl` reader"]
pub struct R(crate::R<REDUNDANT_ECO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REDUNDANT_ECO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REDUNDANT_ECO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REDUNDANT_ECO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Redundant_ECO_Ctrl` writer"]
pub struct W(crate::W<REDUNDANT_ECO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REDUNDANT_ECO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<REDUNDANT_ECO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REDUNDANT_ECO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDUNDANT_ECO_DRIVE` reader - The redundant ECO drive bit to avoid optimization in circuits."]
pub type REDUNDANT_ECO_DRIVE_R = crate::BitReader;
#[doc = "Field `REDUNDANT_ECO_DRIVE` writer - The redundant ECO drive bit to avoid optimization in circuits."]
pub type REDUNDANT_ECO_DRIVE_W<'a, const O: u8> = crate::BitWriter<'a, REDUNDANT_ECO_CTRL_SPEC, O>;
#[doc = "Field `REDUNDANT_ECO_RESULT` reader - The redundant ECO result bit to avoid optimization in circuits."]
pub type REDUNDANT_ECO_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The redundant ECO drive bit to avoid optimization in circuits."]
    #[inline(always)]
    pub fn redundant_eco_drive(&self) -> REDUNDANT_ECO_DRIVE_R {
        REDUNDANT_ECO_DRIVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The redundant ECO result bit to avoid optimization in circuits."]
    #[inline(always)]
    pub fn redundant_eco_result(&self) -> REDUNDANT_ECO_RESULT_R {
        REDUNDANT_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Redundant_ECO_Ctrl")
            .field(
                "redundant_eco_drive",
                &format_args!("{}", self.redundant_eco_drive().bit()),
            )
            .field(
                "redundant_eco_result",
                &format_args!("{}", self.redundant_eco_result().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REDUNDANT_ECO_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The redundant ECO drive bit to avoid optimization in circuits."]
    #[inline(always)]
    #[must_use]
    pub fn redundant_eco_drive(&mut self) -> REDUNDANT_ECO_DRIVE_W<0> {
        REDUNDANT_ECO_DRIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Redundant ECO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redundant_eco_ctrl](index.html) module"]
pub struct REDUNDANT_ECO_CTRL_SPEC;
impl crate::RegisterSpec for REDUNDANT_ECO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [redundant_eco_ctrl::R](R) reader structure"]
impl crate::Readable for REDUNDANT_ECO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [redundant_eco_ctrl::W](W) writer structure"]
impl crate::Writable for REDUNDANT_ECO_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Redundant_ECO_Ctrl to value 0"]
impl crate::Resettable for REDUNDANT_ECO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
