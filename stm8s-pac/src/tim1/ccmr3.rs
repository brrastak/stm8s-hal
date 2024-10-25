#[doc = "Register `CCMR3` reader"]
pub type R = crate::R<CCMR3_SPEC>;
#[doc = "Register `CCMR3` writer"]
pub type W = crate::W<CCMR3_SPEC>;
#[doc = "Field `CC3S` reader - "]
pub type CC3S_R = crate::FieldReader;
#[doc = "Field `CC3S` writer - "]
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC3FE` reader - "]
pub type OC3FE_R = crate::BitReader;
#[doc = "Field `OC3FE` writer - "]
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - "]
pub type OC3PE_R = crate::BitReader;
#[doc = "Field `OC3PE` writer - "]
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M` reader - "]
pub type OC3M_R = crate::FieldReader;
#[doc = "Field `OC3M` writer - "]
pub type OC3M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC3CE` reader - "]
pub type OC3CE_R = crate::BitReader;
#[doc = "Field `OC3CE` writer - "]
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new(self.bits & 3)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<CCMR3_SPEC> {
        CC3S_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn oc3fe(&mut self) -> OC3FE_W<CCMR3_SPEC> {
        OC3FE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc3pe(&mut self) -> OC3PE_W<CCMR3_SPEC> {
        OC3PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn oc3m(&mut self) -> OC3M_W<CCMR3_SPEC> {
        OC3M_W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn oc3ce(&mut self) -> OC3CE_W<CCMR3_SPEC> {
        OC3CE_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TIM1 capture/compare mode register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR3_SPEC;
impl crate::RegisterSpec for CCMR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccmr3::R`](R) reader structure"]
impl crate::Readable for CCMR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr3::W`](W) writer structure"]
impl crate::Writable for CCMR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCMR3 to value 0"]
impl crate::Resettable for CCMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
